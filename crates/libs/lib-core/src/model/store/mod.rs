use diesel::Connection;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub use self::error::{Error, Result};

mod error;

pub type Pool = diesel_async::pooled_connection::bb8::Pool<AsyncPgConnection>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/model/store/migrations");

pub async fn new_db_pool(url: &str) -> Result<Pool> {
    {
        let mut conn = diesel::PgConnection::establish(url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", url));
        conn.run_pending_migrations(MIGRATIONS)
            .expect("migrations should have been applied");
    }

    let pool = Pool::builder()
        .build(AsyncDieselConnectionManager::new(url))
        .await
        .map_err(|e| Error::FailToCreatePool(e.to_string()))?;

    Ok(pool)
}

#[cfg(test)]
pub mod test_db {
    use super::*;
    use crate::{
        config::config,
        ctx::Ctx,
        model::{
            user::{UserBmc, UserForCreate},
            ModelManager,
        },
    };
    use diesel::{sql_query, Connection, PgConnection, RunQueryDsl};
    use reqwest::Url;
    use std::sync::atomic::AtomicU32;

    pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);

    #[derive(Clone)]
    pub struct TestDb {
        default_db_url: String,
        name: String,
        pub pool: Pool,
    }

    impl TestDb {
        pub async fn new() -> Result<Self> {
            let default_db_url = config().DB_URL.to_string();
            let mut conn = PgConnection::establish(&default_db_url).unwrap();

            let name = format!(
                "test_db_{}_{}",
                std::process::id(),
                TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            );

            sql_query(format!("CREATE DATABASE {};", name)).execute(&mut conn)?;

            let mut url = Url::parse(&default_db_url)?;
            url.set_path(&name);
            let pool = new_db_pool(url.as_str()).await?;

            UserBmc::create(
                &Ctx::root_ctx(),
                &ModelManager {
                    db: pool.clone(),
                    email: None,
                },
                UserForCreate {
                    email: "first@user.com".to_string(),
                    password_clear: "12345678".to_string(),
                },
            )
            .await?;

            Ok(Self {
                default_db_url,
                name,
                pool,
            })
        }

        pub fn setup(&self, user_id: i64) -> (ModelManager, Ctx) {
            (
                ModelManager {
                    db: self.pool.clone(),
                    email: None,
                },
                Ctx::new(user_id).unwrap(),
            )
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            let mut conn = PgConnection::establish(&self.default_db_url).unwrap();
            sql_query(format!(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
                self.name
            ))
            .execute(&mut conn)
            .unwrap();
            sql_query(format!("DROP DATABASE {}", self.name))
                .execute(&mut conn)
                .unwrap();
        }
    }
}
