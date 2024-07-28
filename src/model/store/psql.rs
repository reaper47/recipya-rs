use std::ops::DerefMut;

use axum::async_trait;
use deadpool::Runtime;
use deadpool_postgres::{
    tokio_postgres::NoTls,
    Pool, {ManagerConfig, RecyclingMethod},
};

use crate::ctx::Ctx;
use crate::model::user::UserForCreate;
use crate::{
    app::migrations,
    config, model,
    model::{
        store,
        store::{error, Error::FailToCreatePool, RepositoryService},
        user::User,
    },
};

pub struct PsqlRepository {
    pool: Pool,
}

impl PsqlRepository {
    pub async fn new() -> Result<Self, error::Error> {
        let mut db_config = deadpool_postgres::Config::new();
        db_config.url = Some(config().DB_URL.clone());
        db_config.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        let pool = db_config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|ex| FailToCreatePool(ex.to_string()))?;
        {
            let mut client = pool
                .get()
                .await
                .map_err(|ex| FailToCreatePool(ex.to_string()))?;

            let client = client.deref_mut().deref_mut();
            migrations::runner()
                .run_async(client)
                .await
                .map_err(|ex| FailToCreatePool(ex.to_string()))?;
        }

        Ok(Self { pool })
    }
}

#[async_trait]
impl RepositoryService for PsqlRepository {
    async fn user_create(&self, ctx: &Ctx, user_c: UserForCreate) -> model::Result<i64> {
        todo!()
        /*let rows: Result<Vec<User>, store::Error> = self
            .pool
            .get()
            .await?
            .query("SELECT id, name, age FROM my_table", &[])
            .await?
            .into_iter()
            .map(User::try_from)
            .collect();

        rows*/
    }

    async fn user_get(&self, ctx: &Ctx, id: i64) -> model::Result<User> {
        todo!()
        /*let rows: Result<Vec<User>, store::Error> = self
            .pool
            .get()
            .await?
            .query("SELECT id, name, age FROM my_table", &[])
            .await?
            .into_iter()
            .map(User::try_from)
            .ok_or(Error::EntityNotFound { entity: "user", id })?
            .collect();

        rows*/
    }

    async fn user_delete(&self, _ctx: &Ctx, id: i64) -> store::Result<()> {
        todo!()
    }

    /*async fn register(&self, email: &str, plain_password: &str) -> Result<i64, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let password = match Argon2::default().hash_password(plain_password.as_bytes(), &salt) {
            Ok(hash) => hash.to_string(),
            Err(e) => return Err(Error::RepositoryError(e.to_string())),
        };

        let client = match self.pool.get().await {
            Ok(client) => client,
            Err(e) => return Err(Error::RepositoryError(e.to_string())),
        };

        let row = match client
            .query_one(queries::INSERT_USER, &[&email, &password])
            .await
        {
            Ok(row) => row,
            Err(e) => return Err(Error::RepositoryError(e.to_string())),
        };

        Ok(row.get(0))
    }

    async fn users(&self) -> Vec<User> {
        todo!()
    }*/
}
