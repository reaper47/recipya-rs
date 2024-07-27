use std::ops::DerefMut;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::async_trait;
use deadpool_postgres::{tokio_postgres::NoTls, ManagerConfig, Pool, RecyclingMethod, Runtime};

use crate::app::migrations;
use crate::models::User;
use crate::services::repository::RepositoryService;
use crate::{queries, Error};

pub struct PsqlRepository {
    pool: Pool,
}

impl PsqlRepository {
    pub async fn from_url(url: String) -> Self {
        let mut db_config = deadpool_postgres::Config::new();
        db_config.url = Some(url);
        db_config.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        let pool = db_config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
        {
            let client = pool.get().await.unwrap();
            let rows = client.query("SELECT 1", &[]).await.unwrap();
            let value: i32 = rows[0].get(0);
            assert_eq!(value, 1);
        }

        let mut conn = pool.get().await.unwrap();
        let client = conn.deref_mut().deref_mut();
        migrations::runner().run_async(client).await.unwrap();

        Self { pool }
    }
}

#[async_trait]
impl RepositoryService for PsqlRepository {
    async fn register(&self, email: &str, plain_password: &str) -> Result<i64, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password = argon2
            .hash_password(plain_password.as_bytes(), &salt)?
            .to_string();

        Ok(self
            .pool
            .get()
            .await?
            .query_one(queries::INSERT_USER, &[&email, &password])
            .await?
            .get(0))
    }

    async fn users(&self) -> Vec<User> {
        todo!()
    }
}
