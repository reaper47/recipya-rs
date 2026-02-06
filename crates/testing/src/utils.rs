use axum::Router;
use axum_test::{TestResponse, TestServer, TestServerConfig, TestWebSocket, Transport};
use diesel::{Connection, sql_query};
use std::env;
use std::fs::File;
use std::io::{Cursor, Read};
use std::sync::Arc;
use tower_cookies::{Cookie, CookieManagerLayer};
use tracing::error;
use uuid::Uuid;

use app::state::AppState;
use auth::token::generate_access_token;
use auth::token::http::AUTH_TOKEN;
use config::Config;
use models::user::{User, UserForCreate};
use recipya_scraper::tests::MockHttpClient;
use repository::ModelManager;
use repository::make_db_pool;
use router::router;
use support::fs::MockFs;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

/// The email address of the default user in the test database.
pub const TEST_USER_EMAIL: &str = "test@test.com";

/// The password of the default user in the test database.
pub const TEST_USER_PASSWORD: &str = "12345678";

/// The HTML returned when the websocket notification should be hidden.
pub const HIDDEN_WS_NOTIFICATION: &str = r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default hidden"><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1"></p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">-1 of -1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#;

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

/// Creates the `AppState` for testing.
///
/// # Panics
///
/// Panics if the app state fails to initialize.
pub async fn create_app_state(config: Config) -> AppState {
    AppState::new(config, Arc::new(MockHttpClient), Arc::new(MockFs))
        .await
        .map_err(|err| {
            error!("Could not initialise app state: {err}");
            err
        })
        .expect("Failed to initialise app state")
}

/// Builds a test server with anonymous access (no user logged in).
#[cfg(feature = "test-utils")]
pub async fn build_server_anonymous(app_config: Config) -> Result<TestServer> {
    let routes = prepare_router(app_config).await?;
    let config = TestServerConfig {
        save_cookies: true,
        ..TestServerConfig::default()
    };

    Ok(TestServer::new_with_config(routes, config)?)
}

/// Builds a test server with a logged-in user.
///
/// # Panics
///
/// Panics if the test user is not found in the database.
#[cfg(feature = "test-utils")]
pub async fn build_server_logged_in(app_config: Config) -> Result<TestServer> {
    use auth::token::{generate_access_token, http::AUTH_TOKEN};

    let routes = prepare_router(app_config.clone()).await?;
    let config = TestServerConfig {
        save_cookies: true,
        ..TestServerConfig::default()
    };

    let state = create_app_state(app_config).await;

    let user = User::get_user_by_email(&state.mm, TEST_USER_EMAIL)
        .await?
        .expect("User should be in database");

    let token = generate_access_token(&user.id)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token);
    cookie.set_http_only(true);
    cookie.set_path("/");

    let mut server = TestServer::new_with_config(routes, config)?;
    server.add_cookie(cookie);
    Ok(server)
}

/// Builds and initializes a test server with WebSocket support.
pub async fn build_server_ws(app_config: Config) -> Result<(TestServer, TestWebSocket)> {
    build_server_ws_helper(app_config, TEST_USER_EMAIL).await
}

/// Builds and initializes a test server with WebSocket support for a user other
/// than the test one.
pub async fn build_server_ws_other_user(
    app_config: Config,
    auth_email: &str,
) -> Result<(TestServer, TestWebSocket)> {
    build_server_ws_helper(app_config, auth_email).await
}

async fn build_server_ws_helper(
    app_config: Config,
    auth_email: &str,
) -> Result<(TestServer, TestWebSocket)> {
    let routes = prepare_router(app_config.clone()).await?;
    let config = TestServerConfig {
        save_cookies: true,
        transport: Some(Transport::HttpRandomPort),
        ..TestServerConfig::default()
    };

    let state = create_app_state(app_config).await;
    let user = User::get_user_by_email(&state.mm, auth_email)
        .await?
        .expect("User should be in database");

    let mut server = TestServer::new_with_config(routes, config)?;

    let token = generate_access_token(&user.id)?;
    let mut cookie = Cookie::new(AUTH_TOKEN, token.clone());
    cookie.set_http_only(true);
    cookie.set_path("/");
    server.add_cookie(cookie);

    let ws_server = server.get_websocket("/ws").await.into_websocket().await;

    Ok((server, ws_server))
}

/// Generates a token for a newly created test user.
///
/// # Panics
///
/// Panics if the user cannot be found after creation.
pub async fn get_token(mm: ModelManager) -> Result<String> {
    let email = "confirm@test.com".to_string();

    User::new(
        &mm,
        UserForCreate {
            email: email.clone(),
            password_clear: "12345678".to_string(),
        },
    )
    .await?;

    let user = User::get_user_by_email(&mm, &email)
        .await?
        .expect("User not found");

    Ok(generate_access_token(&user.id)?)
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

/// Prepares the router for the test server with the given database URL.
async fn prepare_router(config: Config) -> Result<Router<()>> {
    let state = create_app_state(config).await;
    let app = router(state.clone())?
        .layer(CookieManagerLayer::new())
        .with_state(state.clone());

    User::new(
        &state.mm,
        UserForCreate {
            email: TEST_USER_EMAIL.into(),
            password_clear: TEST_USER_PASSWORD.into(),
        },
    )
    .await?;

    User::new(
        &state.mm,
        UserForCreate {
            email: "demo@demo.com".into(),
            password_clear: TEST_USER_PASSWORD.into(),
        },
    )
    .await?;
    Ok(app)
}

/// Inserts a test user in the database.
pub async fn insert_user(config: Config) -> Result<User> {
    insert_user_helper(config, TEST_USER_EMAIL).await
}

/// Inserts another test user in the database.
pub async fn insert_other_user(config: Config, email: &str) -> Result<User> {
    insert_user_helper(config, email).await
}

async fn insert_user_helper(config: Config, email: &str) -> Result<User> {
    let user = User::new(
        &create_app_state(config.clone()).await.mm,
        UserForCreate {
            email: email.into(),
            password_clear: TEST_USER_PASSWORD.into(),
        },
    )
    .await?;
    Ok(user)
}

/// Asserts that the response HTML contains all of the expected strings.
pub fn assert_html(got: &TestResponse, want: Vec<&str>) {
    for s in want {
        got.assert_text_contains(s);
    }
}

/// Asserts that the response HTML does not contain any of the unwanted strings.
///
/// # Panics
///
/// Panics if any of the unwanted strings are found in the response HTML.
pub fn assert_not_in_html(got: &TestResponse, not_want: Vec<&str>) -> Result<()> {
    let text = got.text();
    for s in not_want {
        assert!(
            !text.contains(s),
            "expected `{s}` not to be in html: {}",
            got.text()
        );
    }

    Ok(())
}

/// Asserts that the websocket server sent the wanted message.
pub async fn assert_ws_message(server: &mut TestWebSocket, want: &str) {
    let _ = server.receive_message().await;
    server.assert_receive_text_contains(want).await;
}

/// Asserts that the user cannot access the specified URI.
pub async fn assert_must_be_logged_in(method: axum::http::Method, uri: &str) -> Result<()> {
    let (_test_db, config) = TestDb::new(None).await?;
    let server = build_server_anonymous(config).await?;

    let res = match method {
        axum::http::Method::DELETE => server.delete(uri),
        axum::http::Method::GET => server.get(uri),
        axum::http::Method::POST => server.post(uri),
        axum::http::Method::PATCH => server.patch(uri),
        axum::http::Method::PUT => server.put(uri),
        _ => unimplemented!(),
    }
    .await;

    res.assert_status_see_other();
    res.assert_header("Location", "/auth/login");
    Ok(())
}

/// Opens a data test file and returns its contents as a `Cursor`.
///
/// # Panics
///
/// Panics if the file does not exist or cannot be read.
pub fn open_test_file(filename: &str) -> Cursor<Vec<u8>> {
    let path = format!("./tests/data/{filename}");
    let mut file = File::open(path).expect("File to exist");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read file");
    Cursor::new(buf)
}
