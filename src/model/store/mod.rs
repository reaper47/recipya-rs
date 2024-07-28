use std::sync::Arc;

use crate::ctx::Ctx;
use crate::model;
use crate::model::user::{User, UserForCreate};
use axum::async_trait;

pub use self::error::{Error, Result};

pub mod error;
pub mod mock;
pub mod psql;
mod queries;

pub type Db = Arc<dyn RepositoryService + Sync + Send>;

#[async_trait]
pub trait RepositoryService {
    async fn user_create(&self, ctx: &Ctx, user_c: UserForCreate) -> model::Result<i64>;
    async fn user_get(&self, ctx: &Ctx, id: i64) -> model::Result<User>;
    async fn user_delete(&self, _ctx: &Ctx, id: i64) -> Result<()>;
    /*async fn register(&self, email: &str, hashed_password: &str) -> std::result::Result<i64, crate::web::Error>;
    async fn users(&self) -> Vec<models::User>;*/
}
