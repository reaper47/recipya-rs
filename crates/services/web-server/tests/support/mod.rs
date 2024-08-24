pub mod assert;
pub mod server;

#[cfg(test)]
pub mod test_db {
    use std::{
        collections::HashMap,
        sync::{atomic::AtomicU32, Arc},
    };

    use diesel::{sql_query, Connection, PgConnection, RunQueryDsl};
    use lib_auth::token::{generate_web_token, Token};
    use lib_core::{
        ctx::Ctx,
        model::{
            store::Pool,
            user::{UserBmc, UserForCreate},
            ModelManager,
        },
    };
    use lib_web::AppState;
    use tokio::sync::Mutex;

    static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);

    #[derive(Clone)]
    pub struct TestDb {
        default_db_url: String,
        name: String,
        pub pool: Pool,
    }

    impl TestDb {
        pub async fn new() -> Self {
            let default_db_url = &std::env::var("DATABASE_URL").unwrap();
            let mut conn = PgConnection::establish(default_db_url).unwrap();

            let name = format!(
                "test_db_{}_{}",
                std::process::id(),
                TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            );

            sql_query(format!("CREATE DATABASE {};", name))
                .execute(&mut conn)
                .unwrap();

            let mut url = url::Url::parse(default_db_url).unwrap();
            url.set_path(&name);

            let pool = lib_core::model::store::new_db_pool(url.as_str())
                .await
                .unwrap();

            UserBmc::create(
                &Ctx::root_ctx(),
                &ModelManager {
                    db: pool.clone(),
                    email: None,
                },
                UserForCreate {
                    email: "test@example.com".to_string(),
                    password_clear: "12345678".to_string(),
                },
            )
            .await
            .unwrap();

            Self {
                default_db_url: default_db_url.to_string(),
                name,
                pool,
            }
        }

        pub fn state(&self) -> AppState {
            AppState {
                mm: ModelManager {
                    db: self.pool.clone(),
                    email: None,
                },
                subscribers: Arc::new(Mutex::new(HashMap::new())),
            }
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            /*if thread::panicking() {
                println!("TestDb leaking database {}", self.name);
                return;
            }*/
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

    pub async fn get_token(mm: ModelManager) -> Token {
        let ctx = Ctx::root_ctx();
        let email = "confirm@test.com".to_string();

        UserBmc::create(
            &ctx,
            &mm,
            UserForCreate {
                email: email.clone(),
                password_clear: "12345678".to_string(),
            },
        )
        .await
        .unwrap();

        let user = UserBmc::first_by_email(&ctx, &mm, &email)
            .await
            .unwrap()
            .unwrap();

        generate_web_token(&user.email, user.token_salt).unwrap()
    }
}
