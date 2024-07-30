use diesel_async::{AsyncPgConnection, pooled_connection::bb8};

use crate::{
    config,
    model::store::{new_db_pool, Pool},
    services::email::Sendgrid,
};

pub use self::error::{Error, Result};

mod error;
mod store;

pub mod payloads;
pub mod user;

#[derive(Clone)]
pub struct ModelManager {
    db: Pool,
    email: Option<Sendgrid>,
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
