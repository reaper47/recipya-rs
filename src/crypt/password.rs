use crate::{config, crypt::encrypt_into_b64u};

use super::{EncryptContent, Error, Result};

/// Encrypt the password with the default scheme.
pub fn encrypt(enc_content: &EncryptContent) -> Result<String> {
    let key = &config().PASSWORD_KEY;
    let encrypted = encrypt_into_b64u(key, enc_content)?;
    Ok(format!("#01{encrypted}"))
}

/// Validate if an EncryptContent matches.
pub fn validate(enc_content: &EncryptContent, password_ref: &str) -> Result<()> {
    let password = encrypt(enc_content)?;

    if password == password_ref {
        Ok(())
    } else {
        Err(Error::PasswordNotMatching)
    }
}
