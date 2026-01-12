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
