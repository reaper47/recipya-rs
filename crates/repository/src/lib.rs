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
    pub async fn new(database_url: &str) -> Result<Self> {
        Ok(Self {
            pool: make_db_pool(database_url).await?,
        })
    }

    /// Checks whether the database can be reached.
    pub async fn ping(&self) -> bool {
        let Ok(_) = self.pool.get().await else {
            return false;
        };
        true
    }
}

/// Builds the database string from the environment
///
/// # Panics
///
/// Panics when one of the following situations occurs:
/// - The `DATABASE_USER_FILE` is not set or cannot be read.
/// - The `DATABASE_PASSWORD_FILE` is not set or cannot be read.
/// - The `DATABASE_PORT` is not an integer.
pub fn build_db_url_from_env() -> String {
    let user_file =
        std::env::var("DATABASE_USER_FILE").expect("env variable 'DATABASE_USER_FILE' to be set");

    let password_file = std::env::var("DATABASE_PASSWORD_FILE")
        .expect("env variable 'DATABASE_PASSWORD_FILE' to be set");

    let host = std::env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());

    let port = std::env::var("DATABASE_PORT")
        .unwrap_or_else(|_| "5432".to_string())
        .parse::<i64>()
        .expect("port could not be parsed to an integer");

    let user = std::fs::read_to_string(&user_file)
        .inspect_err(|_| println!("Error reading `DATABASE_USER_FILE` ({user_file})"))
        .unwrap();

    let password = std::fs::read_to_string(&password_file)
        .inspect_err(|_| println!("Error reading `DATABASE_PASSWORD_FILE` ({password_file})"))
        .unwrap();

    format!(
        "postgres://{}:{}@{host}:{port}",
        user.trim(),
        password.trim()
    )
}
