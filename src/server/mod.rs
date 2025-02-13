mod error;

pub mod router;
pub(super) mod templates;

pub use error::Result;
pub use router::router;

use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::core::config::{Config, DataDir};
use crate::core::email::EmailClient;
use crate::core::repository::ModelManager;

/// Shared application state for the Axum web server.
#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub data_dir: DataDir,
    pub email_service: Option<EmailClient>,
    pub mm: ModelManager,
    pub subscribers: Arc<Mutex<HashMap<i64, Vec<WebSocket>>>>,
}

impl AppState {
    /// Creates a new instance of `AppState` by initializing the `ModelManager`
    /// with the provided database URL.
    pub async fn new(config: Config) -> Result<Self> {
        let email_service = match EmailClient::new() {
            Ok(email_service) => Some(email_service),
            Err(_) => None,
        };

        let data_dir = DataDir::new()?;
        data_dir.log();

        Ok(Self {
            config: config.clone(),
            data_dir,
            email_service,
            mm: ModelManager::new(config.database_url).await?,
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Broadcasts a message to all active WebSocket subscribers of a given user.
    pub async fn broadcast(&self, user_id: i64, message: Message) {
        if let Some(vec) = self.subscribers.lock().await.get_mut(&user_id) {
            let mut to_remove: Vec<usize> = Vec::new();
            for (idx, ws) in vec.iter_mut().enumerate() {
                if ws.send(Message::Ping(Vec::new().into())).await.is_err() {
                    to_remove.push(idx);
                }

                let _ = ws.send(message.clone()).await;
            }

            for &idx in to_remove.iter().rev() {
                vec.remove(idx);
            }
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    use axum::Router;
    use axum_test::{TestResponse, TestServer, TestServerConfig, TestWebSocket, Transport};
    use diesel::{sql_query, Connection};
    use tower_cookies::{Cookie, CookieManagerLayer};

    use crate::core::auth::token::{generate_web_token, Token};
    use crate::core::config::Config;
    use crate::core::model::user::{User, UserForCreate};
    use crate::core::repository::pool::make_db_pool;
    use crate::core::repository::ModelManager;
    use crate::core::support::token::AUTH_TOKEN;
    use crate::server::router::middleware::mw_auth::mw_ctx_resolver;
    use crate::server::AppState;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    /// The email address of the default user in the test database.
    pub const TEST_USER_EMAIL: &str = "test@test.com";

    /// The password of the default user in the test database.
    pub const TEST_USER_PASSWORD: &str = "12345678";

    /// The database URL used for connecting to the database in test environments.
    pub(crate) const RECIPYA_DATABASE_URL: &'static str =
        "postgres://postgres:postgres@localhost:5432/recipya_test";

    pub fn default_config() -> Config {
        Config {
            base_url: String::from("http://localhost:8078"),
            database_url: RECIPYA_DATABASE_URL.to_string(),
            is_autologin: false,
            is_demo: false,
            is_no_signups: false,
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

            let (db_name, db_url) = generate_db().await?;

            let conn = make_db_pool(RECIPYA_DATABASE_URL).await?;
            sql_query(format!("CREATE DATABASE \"{db_name}\";"))
                .execute(&mut conn.get().await?)
                .await?;

            let mut config = config.unwrap_or(Config::default());
            config.database_url = db_url.clone();

            Ok((
                Self {
                    db_name: db_name.to_string(),
                    db_url: db_url.to_string(),
                },
                config,
            ))
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            use diesel::RunQueryDsl;

            let mut conn = diesel::PgConnection::establish(RECIPYA_DATABASE_URL)
                .unwrap_or_else(|_| panic!("Error connecting to {RECIPYA_DATABASE_URL}"));

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

    /// Builds a test server with anonymous access (no user logged in).
    pub async fn build_server_anonymous(app_config: Config) -> Result<TestServer> {
        let routes = prepare_router(app_config).await?;
        let config = TestServerConfig {
            save_cookies: true,
            ..TestServerConfig::default()
        };

        Ok(TestServer::new_with_config(routes, config)?)
    }

    /// Builds a test server with a logged-in user.
    pub async fn build_server_logged_in(app_config: Config) -> Result<TestServer> {
        let routes = prepare_router(app_config.clone()).await?;
        let config = TestServerConfig {
            save_cookies: true,
            ..TestServerConfig::default()
        };

        let state = AppState::new(app_config).await?;

        let user = User::get_user_by_email(&state.mm, TEST_USER_EMAIL)
            .await?
            .expect("Should be an user");

        let token = generate_web_token(TEST_USER_EMAIL, user.token_salt)?;

        let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
        cookie.set_http_only(true);
        cookie.set_path("/");

        let mut server = TestServer::new_with_config(routes, config)?;
        server.add_cookie(cookie);
        Ok(server)
    }

    /// Builds and initializes a test server with WebSocket support.
    pub async fn build_server_ws(app_config: Config) -> Result<(TestServer, TestWebSocket)> {
        let routes = prepare_router(app_config.clone()).await?;
        let config = TestServerConfig {
            save_cookies: true,
            transport: Some(Transport::HttpRandomPort),
            ..TestServerConfig::default()
        };

        let state = AppState::new(app_config).await?;
        let user = User::get_user_by_email(&state.mm, TEST_USER_EMAIL)
            .await?
            .expect("Should be an user");

        let mut server = TestServer::new_with_config(routes, config)?;

        let token = generate_web_token(TEST_USER_EMAIL, user.token_salt)?;
        let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
        cookie.set_http_only(true);
        cookie.set_path("/");
        server.add_cookie(cookie);

        let ws_server = server.get_websocket("/ws").await.into_websocket().await;

        Ok((server, ws_server))
    }

    /// Generates a token for a newly created test user.
    pub async fn get_token(mm: ModelManager) -> Result<Token> {
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

        Ok(generate_web_token(&user.email, user.token_salt)?)
    }

    /// Generates a unique test database name and URL.
    pub async fn generate_db() -> Result<(String, String)> {
        let mut db_url = RECIPYA_DATABASE_URL.to_string();
        let mut db_name = String::from("recipya_test");
        let db_id = uuid::Uuid::new_v4().to_string();
        db_url.push_str(db_id.as_str());
        db_name.push_str(&db_id);
        Ok((db_name, db_url))
    }

    /// Prepares the router for the test server with the given database URL.
    async fn prepare_router(config: Config) -> Result<Router<()>> {
        let state = AppState::new(config).await?;
        let app = crate::server::router(state.clone())
            .await?
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                mw_ctx_resolver,
            ))
            .layer(CookieManagerLayer::new())
            .with_state(state.clone());

        User::new(
            &state.mm,
            UserForCreate {
                email: String::from("test@test.com"),
                password_clear: String::from("12345678"),
            },
        )
        .await?;
        Ok(app)
    }

    /// Inserts a test user in the database.
    pub async fn insert_user(config: Config) -> Result<User> {
        let user = User::new(
            &AppState::new(config.clone()).await?.mm,
            UserForCreate {
                email: String::from(TEST_USER_EMAIL),
                password_clear: String::from(TEST_USER_PASSWORD),
            },
        )
        .await?;

        Ok(user)
    }

    /// Asserts that the response HTML contains all of the expected strings.
    pub fn assert_html(got: TestResponse, want: Vec<&str>) {
        for s in want {
            got.assert_text_contains(s);
        }
    }

    /// Asserts that the response HTML does not contain any of the unwanted strings.
    pub async fn assert_not_in_html(got: TestResponse, not_want: Vec<&str>) -> Result<()> {
        let text = got.text();
        for s in not_want {
            assert!(!text.contains(s), "expected `{s}` not to be in html");
        }

        Ok(())
    }
}
