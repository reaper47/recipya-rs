use std::fmt;

use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use uuid::Uuid;

use crate::{Error, Result};

pub type FailedRecipes = Vec<(Uuid, Error)>;

/// Maximum number of retry attempts for fetching a single recipe.
pub const MAX_RETRY_ATTEMPTS: usize = 3;

pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    /// Creates a new set of credentials.
    pub const fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

pub struct AuthenticatedState;
pub struct UnauthenticatedState;

/// Assembles a token header for authentication.
pub fn assemble_token_header(auth_type: &AuthType, token: &str) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("{auth_type} {token}"))
            .map_err(|err| Error::ApiError(format!("Invalid token: {err}")))?,
    );
    Ok(headers)
}

pub enum AuthType {
    Basic,
    Bearer,
}

impl fmt::Display for AuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Basic => "Basic",
                Self::Bearer => "Bearer",
            }
        )
    }
}
