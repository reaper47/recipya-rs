use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha512;

use support::b64::b64u_encode;

use super::{ContentToHash, Error, Result, Scheme};
use crate::config::auth_config;

/// Represents the first password hashing scheme (Scheme 01).
///
/// This struct implements the `Scheme` trait and provides methods to hash and validate
/// passwords using a specific algorithm (HMAC with SHA-512).
pub struct Scheme01;

impl Scheme for Scheme01 {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
        let key = &auth_config().decoded_password_key;
        hash(key, to_hash)
    }

    fn validate(&self, to_hash: &ContentToHash, raw_pwd_ref: &str) -> Result<()> {
        let raw_pwd_new = self.hash(to_hash)?;
        if raw_pwd_new == raw_pwd_ref {
            Ok(())
        } else {
            Err(Error::PwdValidate)
        }
    }
}

/// Hashes the provided content and salt using the HMAC-SHA512 algorithm.
pub fn hash(key: &[u8], to_hash: &ContentToHash) -> Result<String> {
    let ContentToHash { content, salt } = to_hash;

    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::Key)?;
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    let hmac_result = hmac_sha512.finalize();

    let result = b64u_encode(hmac_result.into_bytes());

    Ok(result)
}

#[cfg(test)]
mod tests {
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_scheme_01_hash_into_b64u_ok() -> Result<()> {
        let fx_salt = Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?;
        let fx_key_b64 = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let fx_key = URL_SAFE_NO_PAD.decode(fx_key_b64)?;
        let fx_to_hash = ContentToHash {
            content: "hello world".to_string(),
            salt: fx_salt,
        };
        let fx_res = "3JdP2uZT2cNw5JGSXAN_V-LuuSD4bcB7htUWPXBxv9pnUku9Yp5g5WYubXbITeIpkLbMMrp5tWoeprrbY1aoQg";

        let res = hash(&fx_key, &fx_to_hash)?;

        assert_eq!(res, fx_res);
        Ok(())
    }
}
