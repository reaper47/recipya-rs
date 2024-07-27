use crate::{models, Error};
use axum::async_trait;

mod mock;
mod psql;

pub(crate) use mock::MockRepository;
pub(crate) use psql::PsqlRepository;

#[async_trait]
pub trait RepositoryService {
    async fn register(&self, email: &str, hashed_password: &str) -> Result<i64, Error>;
    async fn users(&self) -> Vec<models::User>;
}
