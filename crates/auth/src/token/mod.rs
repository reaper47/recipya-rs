mod http;

pub mod jwt;

#[cfg(feature = "test-helpers")]
pub mod jwt_test_helpers;

use uuid::Uuid;

use crate::token::jwt::generate_token;
pub use crate::{Error, Result};
pub use http::{AUTH_TOKEN, remove_token_cookie, set_token_cookie};

/// Generates a short-lived web token for the given user and salt.
pub fn generate_web_token(user: &Uuid) -> Result<String> {
    Ok(generate_token(user, None)?)
}

/// Generates a long-lasting web token for the given user and salt.
pub fn generate_long_lasting_web_token(user: &Uuid) -> Result<String> {
    Ok(generate_token(user, Some(2_678_400))?)
}

/// Generates a verification token.
pub fn generate_verification_token() -> String {
    Uuid::new_v4().simple().to_string()
}

#[cfg(test)]
mod tests {
    use crate::token::jwt::validate_token;

    use super::*;

    #[test]
    fn test_generate_web_token() {
        let user = Uuid::new_v4();
        let token = generate_web_token(&user).unwrap();

        let claims = validate_token(&token).unwrap();

        assert!(claims.exp >= 1_770_947_000);
    }

    #[test]
    fn test_generate_long_lasting_web_token() {
        let user = Uuid::new_v4();
        let token = generate_long_lasting_web_token(&user).unwrap();

        let claims = validate_token(&token).unwrap();

        assert!(claims.exp >= 11_410_509_000);
    }

    #[test]
    fn test_generate_verification_token() {
        let token = generate_verification_token();

        assert_eq!(token.len(), 32);
    }
}
