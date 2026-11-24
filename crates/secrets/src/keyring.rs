use std::collections::BTreeMap;
use std::io::Read;

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, Generate, KeyInit, Payload},
};
use base64::Engine as _;
use serde::Deserialize;
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::master_key::MasterKey;
use crate::{EncryptedPayload, Error, Result};

/// Versioned collection of [`MasterKey`] instances supporting key rotation.
pub struct KeyRing {
    pub active_version: u16,
    keys: BTreeMap<u16, MasterKey>,
}

impl Drop for KeyRing {
    fn drop(&mut self) {
        self.active_version.zeroize();
        self.keys.clear();
    }
}

impl ZeroizeOnDrop for KeyRing {}

impl std::fmt::Debug for KeyRing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let versions = self.keys.keys().copied().collect::<Vec<_>>();
        f.debug_struct("KeyRing")
            .field("active_version", &self.active_version)
            .field("loaded_versions", &versions)
            .finish()
    }
}

enum KeyDomain {
    KeyRing,
}

impl std::fmt::Display for KeyDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyRing => f.write_str("keyring"),
        }
    }
}

#[derive(Deserialize)]
struct KeyRingJson {
    #[serde(rename = "currentVersion")]
    current_version: u16,
    keys: BTreeMap<String, String>,
}

impl KeyRing {
    /// Loads and validates a [`KeyRing`] from the master key secret file.
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;

        let keyring: KeyRingJson = serde_json::from_str(&buf)?;
        if keyring.keys.is_empty() {
            return Err(Error::KeyError("master_key.json contains no keys".into()));
        }

        let active_version = keyring.current_version;
        let mut keys = BTreeMap::new();
        for (version, b64_key) in keyring.keys {
            let version = version
                .parse::<u16>()
                .map_err(|_| Error::KeyError(format!("Invalid version '{version}'")))?;

            let mut decoded_key = base64::engine::general_purpose::STANDARD
                .decode(&b64_key)
                .map_err(|err| {
                    Error::KeyError(format!(
                        "Failed to decode base64 key for version '{version}': {err}"
                    ))
                })?;

            if decoded_key.len() != 32 {
                decoded_key.zeroize();
                return Err(Error::KeyError(format!(
                    "key version '{version}' must be 32 bytes after base64 decode"
                )));
            }

            let mut arr = [0u8; 32];
            arr.copy_from_slice(&decoded_key);
            decoded_key.zeroize();

            keys.insert(version, MasterKey::new(arr));
        }

        if !keys.contains_key(&active_version) {
            return Err(Error::KeyError(format!(
                "active_version '{active_version}' is not present in the keys"
            )));
        }

        Ok(Self {
            active_version,
            keys,
        })
    }

    /// Encrypts `plaintext` with the active key using AES-256-GCM.
    pub fn encrypt(&self, plaintext: &str, user_id: Uuid) -> Result<EncryptedPayload> {
        let version = self.active_version;
        let key = self.active_key()?;

        let nonce = Nonce::generate();
        let ciphertext = Aes256Gcm::new_from_slice(key.as_bytes())
            .map_err(|_| Error::InvalidKeyLength)?
            .encrypt(
                &nonce,
                Payload {
                    msg: plaintext.as_bytes(),
                    aad: &build_aad(user_id, &KeyDomain::KeyRing, version),
                },
            )?;

        Ok(EncryptedPayload {
            nonce: nonce.to_vec(),
            ciphertext,
            key_version: version,
        })
    }

    /// Decrypts an AES-256-GCM payload using the key identified by `version`.
    pub fn decrypt(
        &self,
        nonce_bytes: &[u8],
        ciphertext: &[u8],
        version: u16,
        user_id: Uuid,
    ) -> Result<String> {
        let key = self.key_for_version(version)?;
        let nonce: Nonce<_> = nonce_bytes.try_into().map_err(|_| Error::InvalidNonce)?;
        let plaintext_bytes = Aes256Gcm::new_from_slice(key.as_bytes())
            .map_err(|_| Error::InvalidKeyLength)?
            .decrypt(
                &nonce,
                Payload {
                    msg: ciphertext,
                    aad: &build_aad(user_id, &KeyDomain::KeyRing, version),
                },
            )?;

        String::from_utf8(plaintext_bytes).map_err(|_| Error::InvalidUtf8)
    }

    fn active_key(&self) -> Result<MasterKey> {
        self.key_for_version(self.active_version)
    }

    fn key_for_version(&self, version: u16) -> Result<MasterKey> {
        self.keys.get(&version).map_or_else(
            || Err(Error::KeyNotFound),
            |k| Ok(MasterKey::new(*k.as_bytes())),
        )
    }
}

fn build_aad(user_id: Uuid, domain: &KeyDomain, version: u16) -> Vec<u8> {
    const UUID_LEN_BYTES: usize = 16;
    const DOMAIN_LEN_BYTES: usize = 4;
    const VERSION_LEN_BYTES: usize = 2;

    let domain = domain.to_string();
    let domain_len = u32::try_from(domain.len()).unwrap();
    let domain_bytes = domain.as_bytes();

    let mut aad = Vec::with_capacity(
        UUID_LEN_BYTES + DOMAIN_LEN_BYTES + domain_bytes.len() + VERSION_LEN_BYTES,
    );
    aad.extend(user_id.as_bytes());
    aad.extend(domain_len.to_be_bytes());
    aad.extend(domain_bytes);
    aad.extend(version.to_be_bytes());
    aad
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_keyring_debug_print() {
        let file = Cursor::new(
            r#"{"currentVersion": 1, "keys": {"1": "PmJnTYKSH33+zz5XlEFAKz5Q6148etzJIZ/7Z+Pg+Dw="}}"#,
        );

        let got = KeyRing::from_reader(file).unwrap();

        assert_eq!(
            format!("{got:?}"),
            "KeyRing { active_version: 1, loaded_versions: [1] }"
        );
    }

    mod from_reader {
        use std::{assert_matches, io::Cursor};

        use super::*;

        #[test]
        fn test_no_keys() {
            let file = Cursor::new(r#"{"currentVersion": 1, "keys": {}}"#);

            let got = KeyRing::from_reader(file);

            assert_matches!(got, Err(Error::KeyError(msg)) if msg == "master_key.json contains no keys");
        }

        #[test]
        fn test_version_invalid() {
            let file = Cursor::new(
                r#"{"currentVersion": 1, "keys": {"-1": "rf7rctXDS9Pm4Jj7QM6C36YK4d18KTRDsP5woVVSDv0="}}"#,
            );

            let got = KeyRing::from_reader(file);

            assert_matches!(got, Err(Error::KeyError(msg)) if msg == "Invalid version '-1'");
        }

        #[test]
        fn test_invalid_b64_key() {
            let file = Cursor::new(
                r#"{"currentVersion": 1, "keys": {"1": "1234567891234567891234567891234"}}"#,
            );

            let got = KeyRing::from_reader(file);

            assert_matches!(got, Err(Error::KeyError(msg)) if msg == "Failed to decode base64 key for version '1': Invalid padding");
        }

        #[test]
        fn test_valid_b64_not_valid_master_key() {
            let file = Cursor::new(
                r#"{"currentVersion": 1, "keys": {"1": "s5g/3HnADGT7W/Aaf6FqUoRZyU2Vj1vtPwwVd6uqgM6f"}}"#,
            );

            let got = KeyRing::from_reader(file);

            assert_matches!(got, Err(Error::KeyError(msg)) if msg == "key version '1' must be 32 bytes after base64 decode");
        }

        #[test]
        fn test_active_version_not_in_keys() {
            let file = Cursor::new(
                r#"{"currentVersion": 2, "keys": {"1": "PmJnTYKSH33+zz5XlEFAKz5Q6148etzJIZ/7Z+Pg+Dw="}}"#,
            );

            let got = KeyRing::from_reader(file);

            assert_matches!(got, Err(Error::KeyError(msg)) if msg == "active_version '2' is not present in the keys");
        }

        #[test]
        fn test_valid_file() {
            let file = Cursor::new(
                r#"{"currentVersion": 1, "keys": {"1": "PmJnTYKSH33+zz5XlEFAKz5Q6148etzJIZ/7Z+Pg+Dw="}}"#,
            );

            let got = KeyRing::from_reader(file).unwrap();

            assert_eq!(got.active_version, 1);
            assert_eq!(got.keys.len(), 1);
        }
    }
}
