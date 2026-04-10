use std::env;

use diesel::{Connection, sql_query};
use uuid::Uuid;

use config::Config;
use repository::make_db_pool;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

/// Provides a default config for the tests.
pub fn default_config() -> Config {
    Config {
        base_url: "http://localhost:8078".into(),
        database_url: test_database_url(),
        is_autologin: false,
        is_demo: false,
        is_no_signups: false,
        is_production: false,
    }
}

/// The database URL used for connecting to the database in test environments.
///
/// # Panics
///
/// Panics if the `DATABASE_URL` environment variable is not set.
pub fn test_database_url() -> String {
    setup_env();

    let base = env::var("DATABASE_URL")
        .expect("Environment variable 'DATABASE_URL' to be set")
        .trim_end_matches('/')
        .trim_end_matches("/recipya")
        .to_string();

    if base.ends_with("/recipya_test") {
        base
    } else {
        format!("{base}/recipya_test",)
    }
}

/// Sets up the test environment by loading `.env` file and validating required variables.
///
/// # Panics
///
/// Panics if the `DATABASE_URL` environment variable is not set.
pub fn setup_env() {
    let _ = dotenvy::dotenv();

    assert!(
        std::env::var("DATABASE_URL").is_ok(),
        "Environment variable 'DATABASE_URL' to be set: NotPresent"
    );
}

/// A struct representing a test database, containing the database name and URL.
pub struct TestDb {
    db_name: String,
    pub db_url: String,
}

impl TestDb {
    /// Creates a new test database by generating a unique name, creating the database,
    /// and setting up the connection URL.
    pub async fn new(config: Option<Config>) -> Result<(Self, Config)> {
        use diesel_async::RunQueryDsl;

        let (db_name, db_url) = generate_db()?;
        let conn = make_db_pool(&test_database_url()).await?;
        sql_query(format!("CREATE DATABASE \"{db_name}\";"))
            .execute(&mut conn.get().await?)
            .await?;

        let mut config = config.unwrap_or_default();
        config.database_url.clone_from(&db_url);

        Ok((
            Self {
                db_name: db_name.clone(),
                db_url: db_url.clone(),
            },
            config,
        ))
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        use diesel::RunQueryDsl;

        let url = test_database_url();

        let mut conn = diesel::PgConnection::establish(&url)
            .unwrap_or_else(|_| panic!("Error connecting to {url}"));

        sql_query(format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
            self.db_name
        ))
        .execute(&mut conn)
        .expect("Error executing pg_terminate_backend query");

        sql_query(format!("DROP DATABASE \"{}\"", self.db_name))
            .execute(&mut conn)
            .expect("Error dropping db");
    }
}

/// Generates a unique test database name and URL.
pub fn generate_db() -> Result<(String, String)> {
    let mut db_url = test_database_url();

    let mut db_name = String::from("recipya_test");
    let db_id = Uuid::new_v4().to_string();
    db_url.push_str(db_id.as_str());
    db_name.push_str(&db_id);
    Ok((db_name, db_url))
}
