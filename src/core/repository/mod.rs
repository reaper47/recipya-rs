mod error;
pub mod pool;
pub mod schema;

pub use error::{Error, Result};

use crate::core::repository::pool::{make_db_pool, DbPool};

/// `ModelManager` is a structure responsible for managing database interactions.
#[derive(Clone)]
pub struct ModelManager {
    /// The connection pool for the PostgreSQL database.
    pub pool: DbPool,
}

impl ModelManager {
    /// Creates a new instance of the model manager.
    pub async fn new(database_url: impl Into<String>) -> Result<Self> {
        Ok(Self {
            pool: make_db_pool(&database_url.into()).await?,
        })
    }
}
