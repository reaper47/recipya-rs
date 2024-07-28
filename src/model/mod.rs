use std::sync::Arc;

use crate::model::store::mock::MockRepository;
use crate::model::store::{psql::PsqlRepository, Db};

pub use self::error::{Error, Result};

mod base;
mod error;
mod store;

pub mod payloads;
pub mod user;
pub mod user_old;

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            db: Arc::new(PsqlRepository::new().await?),
        })
    }

    pub fn new_test() -> Self {
        Self {
            db: Arc::new(MockRepository::default()),
        }
    }

    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
