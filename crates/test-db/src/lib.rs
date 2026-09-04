use std::{env, time::Duration};

use diesel::{Connection, sql_query};
use diesel_async::{
    AsyncConnection, AsyncPgConnection, RunQueryDsl,
    pooled_connection::{AsyncDieselConnectionManager, ManagerConfig, bb8},
};
use diesel_migrations::MigrationHarness;
use futures_util::FutureExt;
use tokio::sync::OnceCell;
use url::Url;

use config::{Config, States};
use repository::{DbPool, MIGRATIONS, ModelManager, make_db_pool};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const TEMPLATE_DB: &str = "recipya_test_template";

const TEST_DATABASE_NAME: &str = "recipya_test";

/// Provides a default config for the tests.
pub fn default_config() -> Config {
    Config {
        base_url: "http://localhost:8078".into(),
        database_url: test_database_url(),
        states: States::default(),
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
        format!("{base}/recipya_test")
    }
}

/// Sets up the test environment by loading `.env` file and validating required variables.
///
/// # Panics
///
/// Panics if the `DATABASE_URL` environment variable is not set.
pub fn setup_env() {
    let _ = dotenvy::dotenv();

    unsafe {
        std::env::set_var("APP_ENV", "test");
    }

    assert!(
        std::env::var("DATABASE_URL").is_ok(),
        "Environment variable 'DATABASE_URL' to be set: NotPresent"
    );
    assert!(
        std::env::var("APP_ENV").is_ok(),
        "Environment variable 'APP_ENV' to be set: NotPresent"
    );
}

static TEST_DB_INIT: OnceCell<(TestDb, Config)> = OnceCell::const_new();

/// Initializes the test database if it has not yet been initialized.
///
/// # Panics
///
/// Panics when the test database initialisation fails.
pub async fn init_test_db<'a>() -> &'a (TestDb, Config) {
    TEST_DB_INIT
        .get_or_init(async || TestDb::new(None).await.unwrap())
        .await
}

/// A struct representing a test database, containing the database name and URL.
pub struct TestDb {
    pub db_url: String,
}

impl TestDb {
    /// Creates a new test database by generating a unique name, creating the database,
    /// and setting up the connection URL.
    ///
    /// # Panics
    ///
    /// Panics when the database migrations failed to run.
    pub async fn new(config: Option<Config>) -> Result<(Self, Config)> {
        use diesel_async::RunQueryDsl;

        let (db_name, db_url) = generate_db()?;
        let admin_url = admin_database_url();
        ensure_template_exists(&admin_url).await?;

        {
            let conn = make_db_pool(&admin_url).await?;
            sql_query(format!("DROP DATABASE IF EXISTS \"{db_name}\";"))
                .execute(&mut conn.get().await?)
                .await?;
            sql_query(format!(
                "CREATE DATABASE \"{db_name}\" TEMPLATE \"{TEMPLATE_DB}\";"
            ))
            .execute(&mut conn.get().await?)
            .await?;
        }

        {
            let migration_url = db_url.clone();
            tokio::task::spawn_blocking(move || {
                diesel::PgConnection::establish(&migration_url)
                    .expect("Failed to connect for migrations")
                    .run_pending_migrations(MIGRATIONS)
                    .expect("Failed to run migrations");
            })
            .await
            .expect("Migration task failed");
        }

        let mut config = config.unwrap_or_default();
        config.database_url.clone_from(&db_url);

        Ok((
            Self {
                db_url: db_url.clone(),
            },
            config,
        ))
    }
}

fn admin_database_url() -> String {
    let _ = dotenvy::from_filename(concat!(env!("CARGO_MANIFEST_DIR"), "/../../.env"));
    std::env::var("DATABASE_URL").expect("DATABASE_URL not set")
}

const TEMPLATE_LOCK_ID: i64 = 918_273_645;

/// AI usage:
///
/// Assisted by Claude *Sonnet 5* because I struggled understanding why so many tests were
/// failing randomly while refactoring the tests to drastically improve the time it takes to run them.
async fn ensure_template_exists(admin_url: &str) -> Result<()> {
    let pool = make_db_pool(admin_url).await?;
    let mut conn = pool.get().await?;

    sql_query(format!("SELECT pg_advisory_lock({TEMPLATE_LOCK_ID})"))
        .execute(&mut conn)
        .await?;

    let result = async {
        let _ = sql_query(format!("CREATE DATABASE \"{TEMPLATE_DB}\";"))
            .execute(&mut conn)
            .await;

        let mut url = Url::parse(admin_url)?;
        url.set_path(TEMPLATE_DB);
        tokio::task::spawn_blocking(move || {
            diesel::PgConnection::establish(url.as_ref())
                .expect("connect to template db")
                .run_pending_migrations(MIGRATIONS)
                .expect("migrate template db");
        })
        .await?;

        Ok(())
    }
    .await;

    sql_query(format!("SELECT pg_advisory_unlock({TEMPLATE_LOCK_ID})"))
        .execute(&mut conn)
        .await?;

    result
}

/// Creates the model manager for use in tests.
///
/// # Panics
///
/// Panics when the database pool fails to build.
pub async fn test_model_manager() -> ModelManager {
    let (_, config) = init_test_db().await;
    let url = &config.database_url;

    let mut cfg = ManagerConfig::<AsyncPgConnection>::default();

    cfg.custom_setup = Box::new(|url| {
        async move {
            let mut conn = AsyncPgConnection::establish(url).await?;
            conn.begin_test_transaction()
                .await
                .expect("BEGIN test transaction");
            Ok(conn)
        }
        .boxed()
    });

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(url, cfg);
    let pool = bb8::Pool::builder()
        .max_size(1)
        .min_idle(Some(1))
        .connection_timeout(Duration::from_secs(3))
        .build(manager)
        .await
        .expect("build per-test pool");

    ModelManager { pool: DbPool(pool) }
}

/// Generates a unique test database name and URL.
pub fn generate_db() -> Result<(String, String)> {
    let db_name = format!("{TEST_DATABASE_NAME}_{}", uuid::Uuid::new_v4().simple());
    let db_url = test_db_url(&db_name)?;
    Ok((db_name, db_url))
}

fn test_db_url(db_name: &str) -> Result<String> {
    let base_url = admin_database_url();
    let mut url = Url::parse(&base_url)?;
    url.set_path(db_name);
    Ok(url.to_string())
}
