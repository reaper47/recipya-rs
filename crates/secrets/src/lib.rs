mod error;
mod keyring;
mod master_key;

use std::{
    io::Read,
    sync::{Arc, RwLock},
};

use uuid::Uuid;

use crate::keyring::KeyRing;
pub use error::{Error, Result};

/// Shared, thread-safe container for application secrets and key material.
#[derive(Clone)]
pub struct AppSecrets {
    master_key: Arc<RwLock<KeyRing>>,
}

/// Output of an encryption operation, storable as-is for later decryption.
pub struct EncryptedPayload {
    /// Unique per-encryption nonce. Must be stored alongside the ciphertext.
    pub nonce: Vec<u8>,
    /// Encrypted data concatenated with the AEAD authentication tag.
    pub ciphertext: Vec<u8>,
    /// Version identifier of the master key used to encrypt `ciphertext`.
    pub key_version: u16,
}

impl AppSecrets {
    /// Attempts to load and validate an [`AppSecrets`] instance from the provided readers.
    pub fn try_new<R: Read>(master_key_reader: R) -> Result<Self> {
        Ok(Self {
            master_key: Arc::new(RwLock::new(KeyRing::from_reader(master_key_reader)?)),
        })
    }

    /// Encrypts `plaintext` with the current master key, bound to `user_id`.
    pub fn encrypt_with_master_key(
        &self,
        plaintext: &str,
        user_id: Uuid,
    ) -> Result<EncryptedPayload> {
        self.master_key
            .read()
            .map_err(|_| Error::MasterKeyLockPoisoned)?
            .encrypt(plaintext, user_id)
    }

    /// Decrypts a payload produced by [`encrypt_with_master_key`](Self::encrypt_with_master_key).
    pub fn decrypt_with_master_key(
        &self,
        payload: &EncryptedPayload,
        user_id: Uuid,
    ) -> Result<String> {
        self.master_key
            .read()
            .map_err(|_| Error::MasterKeyLockPoisoned)?
            .decrypt(
                &payload.nonce,
                &payload.ciphertext,
                payload.key_version,
                user_id,
            )
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const MASTER_KEY_FILE: &str =
        r#"{"currentVersion": 1, "keys": {"1": "PmJnTYKSH33+zz5XlEFAKz5Q6148etzJIZ/7Z+Pg+Dw="}}"#;
    const API_KEY: &str =
        "sk-ant-api03-AbC123_xYz-EXAMPLEEXAMPLEEXAMPLEEXAMPLEEXAMPLEEXAMPLEEXAMPLE";
    const TEST_USER_ID: Uuid = Uuid::from_u128(0x7b0e4c1a_3f5d_4a8e_9c2b_1d6f8e0a5b37);

    #[test]
    fn test_encrypt_decrypt_plaintext_ok() -> Result<()> {
        let file = Cursor::new(MASTER_KEY_FILE);
        let app_secrets = AppSecrets::try_new(file)?;

        // Encrypt
        let got_encrypted = app_secrets.encrypt_with_master_key(API_KEY, TEST_USER_ID)?;
        assert_eq!(got_encrypted.key_version, 1);
        assert_eq!(got_encrypted.nonce.len(), 12);
        assert_ne!(got_encrypted.ciphertext, [] as [u8; 0]);
        assert_ne!(got_encrypted.ciphertext, API_KEY.as_bytes());

        // Decrypt
        let got_decrypted = app_secrets.decrypt_with_master_key(&got_encrypted, TEST_USER_ID)?;
        assert_eq!(got_decrypted, API_KEY);

        Ok(())
    }

    #[test]
    fn test_decrypt_wrong_user_err() -> Result<()> {
        let file = Cursor::new(MASTER_KEY_FILE);
        let app_secrets = AppSecrets::try_new(file)?;
        let encrypted = app_secrets.encrypt_with_master_key(API_KEY, TEST_USER_ID)?;

        let got = app_secrets.decrypt_with_master_key(&encrypted, Uuid::new_v4());

        assert!(got.is_err());
        Ok(())
    }
}
