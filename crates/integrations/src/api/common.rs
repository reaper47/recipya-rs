use uuid::Uuid;

use crate::Error;

pub type FailedRecipes = Vec<(Uuid, Error)>;

/// Maximum number of retry attempts for fetching a single recipe.
pub const MAX_RETRY_ATTEMPTS: usize = 3;

pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    /// Creates a new set of credentials.
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

pub struct AuthenticatedState;
pub struct UnauthenticatedState;
