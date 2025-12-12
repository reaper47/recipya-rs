use std::sync::OnceLock;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};

use super::{ContentToHash, Error, Result, Scheme};
use crate::config::auth_config;

/// Represents the second password hashing scheme (Scheme 02).
///
/// This struct implements the `Scheme` trait and provides methods to hash and validate
/// passwords using the Argon2 algorithm. It offers a more modern, secure hashing method
/// compared to Scheme 01, with better resistance to brute-force attacks.
pub struct Scheme02;

impl Scheme for Scheme02 {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
        let argon2 = get_argon2();

        let salt_b64 = SaltString::encode_b64(to_hash.salt.as_bytes()).map_err(|_| Error::Salt)?;

        let pwd = argon2
            .hash_password(to_hash.content.as_bytes(), &salt_b64)
            .map_err(|_| Error::Hash)?
            .to_string();

        Ok(pwd)
    }

    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()> {
        let argon2 = get_argon2();

        let parsed_hash_ref = PasswordHash::new(pwd_ref).map_err(|_| Error::Hash)?;

        argon2
            .verify_password(to_hash.content.as_bytes(), &parsed_hash_ref)
            .map_err(|_| Error::PwdValidate)
    }
}

fn get_argon2() -> &'static Argon2<'static> {
    static INSTANCE: OnceLock<Argon2<'static>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let key = &auth_config().decoded_password_key;
        Argon2::new_with_secret(
            key,
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::default(),
        )
        .unwrap()
    })
}

#[cfg(test)]
mod tests {
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_scheme_02_hash_into_b64u_ok() -> Result<()> {
        let fx_to_hash = ContentToHash {
            content: "hello world".to_string(),
            salt: Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?,
        };
        let fx_res = "$argon2id$v=19$m=19456,t=2,p=1$8F6JYdatQIaeeKbeBl5UUw$1j7eN2XoMhvQT8hbi4NmpZ70NO/H6e9ItDhIh6OkgBQ";

        let scheme = Scheme02;
        let res = scheme.hash(&fx_to_hash)?;

        assert_eq!(res, fx_res);
        Ok(())
    }
}
