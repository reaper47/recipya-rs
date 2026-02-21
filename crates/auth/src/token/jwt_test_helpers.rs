#[cfg(feature = "test-helpers")]
use crate::token::jwt::Claims;

#[cfg(feature = "test-helpers")]
pub fn encode_claims_for_test(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{EncodingKey, Header, encode};

    use crate::config::auth_config;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(auth_config().jwt_secret.as_ref()),
    )
}
