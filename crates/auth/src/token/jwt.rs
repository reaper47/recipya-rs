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
    duration_sec: Option<u64>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now();
    let duration_hours = duration_sec.unwrap_or(2_678_400 / 3600);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: (now + Duration::from_hours(duration_hours))
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as usize,
        iat: now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() as usize,
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
