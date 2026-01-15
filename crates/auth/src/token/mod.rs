pub mod http;
pub mod jwt;

#[cfg(feature = "test-helpers")]
pub mod jwt_test_helpers;

pub use crate::{Error, Result};

use uuid::Uuid;

use crate::token::jwt::generate_token;

/// Generates a short-lived access token for the given user.
pub fn generate_access_token(user: &Uuid) -> Result<String> {
    Ok(generate_token(user, 15 * 60)?)
}

/// Generates a long-lasting web token for the given user.
pub fn generate_long_lasting_web_token(user: &Uuid) -> Result<String> {
    Ok(generate_token(user, 2_678_400)?)
}

/// Generates a verification token, which is essentially a stringified simple UUID.
pub fn generate_verification_token() -> String {
    Uuid::new_v4().simple().to_string()
}

/// Generates a refresh token, which is essentially a stringified UUID.
pub fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
}
