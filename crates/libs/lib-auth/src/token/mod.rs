use hmac::{Hmac, Mac};
use sha2::Sha512;
use uuid::Uuid;

use lib_utils::{
    b64::{b64u_decode_to_string, b64u_encode},
    time::{now_utc, now_utc_plus_sec_str, parse_utc},
};

use crate::auth_config;

pub use self::error::{Error, Result};

mod error;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Token {
    pub ident: String,     // Identifier (username for example).
    pub exp: String,       // Expiration date in Rfc3339.
    pub sign_b64u: String, // Signature, base64url encoded.
}

impl std::str::FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> std::result::Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split('.').collect();
        if splits.len() != 3 {
            return Err(Error::InvalidFormat);
        }
        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64u_decode_to_string(ident_b64u).map_err(|_| Error::CannotDecodeIdent)?,
            exp: b64u_decode_to_string(exp_b64u).map_err(|_| Error::CannotDecodeExp)?,
            sign_b64u: sign_b64u.to_string(),
        })
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64u_encode(&self.ident),
            b64u_encode(&self.exp),
            self.sign_b64u
        )
    }
}

pub fn generate_web_token(user: &str, salt: Uuid) -> Result<Token> {
    let config = &auth_config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn generate_long_lasting_web_token(user: &str, salt: Uuid) -> Result<Token> {
    let config = &auth_config();
    _generate_token(user, 2_678_400., salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: &Token, salt: Uuid) -> Result<()> {
    let config = &auth_config();
    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)?;

    Ok(())
}

fn _generate_token(ident: &str, duration_sec: f64, salt: Uuid, key: &[u8]) -> Result<Token> {
    // Compute the two first components.
    let ident = ident.to_string();
    let exp = now_utc_plus_sec_str(duration_sec);

    // Sign the two first components.
    let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

    Ok(Token {
        ident,
        exp,
        sign_b64u,
    })
}

fn _validate_token_sign_and_exp(origin_token: &Token, salt: Uuid, key: &[u8]) -> Result<()> {
    // Validate signature.
    let new_sign_b64u = _token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;
    if new_sign_b64u != origin_token.sign_b64u {
        return Err(Error::SignatureNotMatching);
    }

    // Validate expiration.
    let origin_exp = parse_utc(&origin_token.exp).map_err(|_| Error::ExpNotIso)?;
    if origin_exp < now_utc() {
        return Err(Error::Expired);
    }

    Ok(())
}

/// Create token signature from token parts and salt.
fn _token_sign_into_b64u(ident: &str, exp: &str, salt: Uuid, key: &[u8]) -> Result<String> {
    let content = format!("{}.{}", b64u_encode(ident), b64u_encode(exp));

    // Create a HMAC-SHA-512 from key.
    let mut hmac_sha512 =
        Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::HmacFailNewFromSlice)?;

    // Add content.
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();
    let result = b64u_encode(result_bytes);

    Ok(result)
}

#[cfg(test)]
mod tests {
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    use super::*;
    use crate::token;
    use std::{thread, time::Duration};

    #[test]
    fn test_token_display_ok() -> Result<()> {
        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sign-b64u-encoded";
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2023-05-17T15:30:00Z".to_string(),
            sign_b64u: "some-sign-b64u-encoded".to_string(),
        };

        assert_eq!(fx_token.to_string(), fx_token_str);
        Ok(())
    }

    #[test]
    fn test_token_from_str_ok() -> Result<()> {
        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyMy0wNS0xN1QxNTozMDowMFo.some-sign-b64u-encoded";
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2023-05-17T15:30:00Z".to_string(),
            sign_b64u: "some-sign-b64u-encoded".to_string(),
        };

        let token: Token = fx_token_str.parse()?;

        assert_eq!(token, fx_token);
        Ok(())
    }

    #[test]
    fn test_validate_web_token_ok() -> Result<()> {
        let fx_user = "user_one";
        let fx_salt = Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453").unwrap();
        let fx_duration_sec = 0.02; // 20ms
        let token_key = &auth_config().TOKEN_KEY;
        let fx_token = _generate_token(fx_user, fx_duration_sec, fx_salt, token_key)?;

        thread::sleep(Duration::from_millis(10));
        let res = validate_web_token(&fx_token, fx_salt);

        res?;
        Ok(())
    }

    #[test]
    fn test_validate_web_token_err_expired() -> Result<()> {
        let fx_user = "user_one";
        let fx_salt = Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453").unwrap();
        let fx_duration_sec = 0.01; // 10ms
        let token_key = &auth_config().TOKEN_KEY;
        let fx_token = _generate_token(fx_user, fx_duration_sec, fx_salt, token_key)?;

        thread::sleep(Duration::from_millis(20));
        let res = validate_web_token(&fx_token, fx_salt);

        assert!(
            matches!(res, Err(token::Error::Expired)),
            "Should have matched `Err(Error::Expired)` but was `{res:?}`"
        );
        Ok(())
    }
}
