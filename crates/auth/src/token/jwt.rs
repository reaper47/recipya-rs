use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::auth_config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub(crate) fn generate_token(
    user_id: &Uuid,
    duration_sec: u64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: usize::try_from(
            (now + Duration::from_secs(duration_sec))
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        )
        .unwrap_or_default(),
        iat: usize::try_from(now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs())
            .unwrap_or_default(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(auth_config().jwt_secret.as_ref()),
    )
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode(
        token,
        &DecodingKey::from_secret(auth_config().jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
