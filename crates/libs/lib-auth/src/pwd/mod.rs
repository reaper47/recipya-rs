use uuid::Uuid;

use crate::{config::auth_config, pwd::hmac_hasher::hmac_sha512_hash};

pub use self::error::{Error, Result};

mod error;
mod hmac_hasher;

pub struct ContentToHash {
    pub content: String, // Clear content.
    pub salt: Uuid,      // Clear salt.
}

/// Hash the password with the default scheme.
pub fn hash(to_hash: &ContentToHash) -> Result<String> {
    let key = &auth_config().PASSWORD_KEY;
    let hashed = hmac_sha512_hash(key, to_hash)?;
    Ok(format!("#01{hashed}"))
}

/// Validate whether a ContentToHash matches.
pub fn validate(enc_content: &ContentToHash, password_ref: &str) -> Result<()> {
    let password = hash(enc_content)?;

    if password == password_ref {
        Ok(())
    } else {
        Err(Error::NotMatching)
    }
}
