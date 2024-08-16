//! Model Layer
//!
//! Design:
//!
//! - The Model layer normalizes the application's data type
//!   structures and access.
//! - All application code data access must go through the Model layer.
//! - The `ModelManager` holds the internal states/resources
//!   needed by ModelControllers to access data.
//!   (e.g., db_pool, S3 client, redis client).
//! - Model Controllers (e.g., `TaskBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Task`, `Project`).
//!   (`Bmc` is short for Backend Model Controller).
//! - In frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
//! - ModelManager are designed to be passed as an argument
//!   to all Model Controllers functions.
//!

use diesel_async::{pooled_connection::bb8, AsyncPgConnection};

use lib_email::Sendgrid;

use crate::config::config;
use crate::model::store::{new_db_pool, Pool};

pub use self::error::{Error, Result};

mod error;
pub mod store;

pub(in crate::model) mod schema;
pub mod user;

#[derive(Clone)]
pub struct ModelManager {
    pub db: Pool,
    pub email: Option<Sendgrid>,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            db: new_db_pool(&config().DB_URL).await?,
            email: Sendgrid::new(),
        })
    }

    pub(in crate::model) fn db(&self) -> &Pool {
        &self.db
    }

    pub(in crate::model) async fn connection(
        &self,
    ) -> Result<bb8::PooledConnection<AsyncPgConnection>> {
        Ok(self.db.get().await?)
    }
}
