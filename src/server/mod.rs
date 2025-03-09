mod error;

pub mod router;
pub(super) mod templates;

pub use error::{Error, Result};
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
    use diesel::internal::derives::multiconnection::chrono;
    use diesel::{sql_query, Connection};
    use tower_cookies::{Cookie, CookieManagerLayer};
    use uuid::Uuid;

    use crate::core::auth::token::{generate_web_token, Token};
    use crate::core::config::Config;
    use crate::core::model::recipe::{
        NutritionForCreate, RecipeForCreate, Sections, TimesForCreate, ToolForCreate,
        VideoForCreate,
    };
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
    pub(crate) fn test_database_url() -> String {
        std::env::var("RECIPYA_DATABASE_TEST_URL").unwrap_or(String::from(
            "postgres://postgres:postgres@localhost:5432/recipya_test",
        ))
    }

    /// Provides a default config for the tests.
    pub fn default_config() -> Config {
        Config {
            base_url: String::from("http://localhost:8078"),
            database_url: test_database_url(),
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

            let conn = make_db_pool(&test_database_url()).await?;
            sql_query(format!("CREATE DATABASE \"{db_name}\";"))
                .execute(&mut conn.get().await?)
                .await?;

            let mut config = config.unwrap_or_default();
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

        let state = AppState::new(app_config).await?;
        let user = User::get_user_by_email(&state.mm, auth_email)
            .await?
            .expect("Should be an user");

        let mut server = TestServer::new_with_config(routes, config)?;

        let token = generate_web_token(auth_email, user.token_salt)?;
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
        let mut db_url = test_database_url();
        let mut db_name = String::from("recipya_test");
        let db_id = Uuid::new_v4().to_string();
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
                email: String::from(TEST_USER_EMAIL),
                password_clear: String::from(TEST_USER_PASSWORD),
            },
        )
            .await?;

        User::new(
            &state.mm,
            UserForCreate {
                email: String::from("demo@demo.com"),
                password_clear: String::from(TEST_USER_PASSWORD),
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

    /// Prepares a recipe with all its components filled out.
    pub fn a_complete_recipe() -> RecipeForCreate {
        let main_image = Uuid::new_v4();
        let secondary_image = Uuid::new_v4();
        let video = Uuid::new_v4();

        RecipeForCreate {
            name: String::from("Best Chinese Kale"),
            description: Some(String::from("This is the most delicious recipe!")),
            images: Some(vec![main_image, secondary_image]),
            yield_: Some(4),
            source: Some(String::from(
                "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/",
            )),
            videos: vec![VideoForCreate {
                video,
                duration: Some(chrono::Duration::minutes(7)),
                content_url: Some(String::from("https://example.com/best-food.mp4")),
                embed_url: Some(String::from("https://example.com/embed/j43yfe3.mp4")),
            }],
            category: Some(String::from("dinner")),
            cuisine: Some(String::from("thai")),
            ingredients: Sections::from([
                (
                    String::from("Sauce"),
                    vec![
                        String::from("1 cup blue spinach"),
                        String::from("1/2 tbsp cinnamon"),
                    ],
                ),
                (
                    String::from("Main"),
                    vec![
                        String::from("4 pounds top quality chicken filet"),
                        String::from("1/8 cup lemon juice"),
                    ],
                ),
            ]),
            instructions: Sections::from([
                (
                    String::from("Sauce"),
                    vec![String::from("Mix all these ingredients")],
                ),
                (
                    String::from("Chicken"),
                    vec![
                        String::from("Turn the oven at 300 F"),
                        String::from("Soak the chicken in the lemon juice"),
                        String::from("Bake for 35 minutes"),
                    ],
                ),
            ]),
            keywords: vec![String::from("vegetarian"), String::from("tofu")],
            nutrition: Some(NutritionForCreate {
                calories_kcal: Some(300),
                total_carbohydrates: Some(55),
                sugars_g: Some(43),
                protein_g: Some(7),
                total_fat_g: Some(6),
                saturated_fat_g: Some(1),
                unsaturated_fat_g: Some(2),
                cholesterol_mg: Some(5),
                sodium_mg: Some(12),
                fiber_g: Some(10),
                trans_fat_g: Some(3),
                serving_size: Some(String::from("100g")),
            }),
            times: Some(TimesForCreate {
                prep_seconds: 120,
                cook_seconds: 3600,
            }),
            tools: vec![
                ToolForCreate {
                    name: String::from("wok"),
                    quantity: 1,
                },
                ToolForCreate {
                    name: String::from("frying pan"),
                    quantity: 1,
                },
            ],
        }
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

    /// Asserts that the user cannot access the specified URI.
    pub async fn assert_must_be_logged_in(uri: &str) -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_anonymous(config).await?;

        let res = server.get(uri).await;

        res.assert_status_see_other();
        res.assert_header("Location", "/auth/login");
        Ok(())
    }
}
