use diesel::{Connection, PgConnection};
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::model::store::Error::FailToCreatePool;

pub use self::error::{Error, Result};

pub type Pool = diesel_async::pooled_connection::bb8::Pool<AsyncPgConnection>;

mod error;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub async fn new_db_pool(url: &str) -> Result<Pool> {
    {
        let mut conn =
            PgConnection::establish(url).unwrap_or_else(|_| panic!("Error connecting to {}", url));
        conn.run_pending_migrations(MIGRATIONS)
            .expect("migrations should have been applied");
    }

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);

    let pool = Pool::builder()
        .build(config)
        .await
        .map_err(|e| FailToCreatePool(e.to_string()))?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    /*#[tokio::test]
    async fn test_foo() {
        let db = TestDb::new().await;
        db.run_test(|pool| {
            async move {
                let conn = &mut pool.get().unwrap();

                let res = diesel::insert_into(users::table)
                    .values(&UserForInsert {
                        email: "test@example.com".to_string(),
                    })
                    .returning(User::as_returning())
                    .get_result(conn)
                    .expect("Error saving new user");


                let results = users
                    .select(User::as_select())
                    .load(conn)
                    .expect("expected results");
                assert_eq!(results.len(), 1);
            }
            .boxed()
        })
        .await;
    }*/
}

#[cfg(test)]
pub mod test_db {
    use std::{sync::atomic::AtomicU32, thread};

    use diesel::{Connection, PgConnection, RunQueryDsl, sql_query};
    use futures::future::BoxFuture;
    use reqwest::Url;

    use crate::config;

    use super::*;

    static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);

    #[derive(Clone)]
    pub struct TestDb {
        default_db_url: String,
        name: String,
        pub pool: Pool,
    }

    impl TestDb {
        pub async fn new() -> Self {
            let default_db_url = &config().DB_URL;
            let mut conn = PgConnection::establish(default_db_url).unwrap();

            let name = format!(
                "test_db_{}_{}",
                std::process::id(),
                TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            );

            sql_query(format!("CREATE DATABASE {};", name))
                .execute(&mut conn)
                .unwrap();

            let mut url = Url::parse(default_db_url).unwrap();
            url.set_path(&name);

            Self {
                default_db_url: default_db_url.to_string(),
                name,
                pool: new_db_pool(&url.to_string()).await.unwrap(),
            }
        }

        pub async fn run_test(&self, test: impl Fn() -> BoxFuture<'static, ()>) {
            test().await;
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            if thread::panicking() {
                eprintln!("TestDb leaking database {}", self.name);
                return;
            }
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
