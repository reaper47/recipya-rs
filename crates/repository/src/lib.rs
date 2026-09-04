#![allow(clippy::wildcard_imports)]

mod error;
mod pool;

pub mod extensions;
pub mod schema;

pub use error::{Error, Result};
pub use pool::{DbPool, MIGRATIONS, PgPooledConn, create_database_if_not_exists, make_db_pool};

/// `ModelManager` is a structure responsible for managing database interactions.
#[derive(Clone)]
pub struct ModelManager {
    /// The connection pool for the `PostgreSQL` database.
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
