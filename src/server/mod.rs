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
    use diesel::internal::derives::multiconnection::chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use diesel::{Connection, sql_query};
    use tower_cookies::{Cookie, CookieManagerLayer};
    use uuid::Uuid;

    use crate::core::auth::token::{Token, generate_web_token};
    use crate::core::config::Config;
    use crate::core::model::recipe::{
        Nutrition, NutritionForCreate, RecipeForCreate, Sections, Times, TimesForCreate,
        ToolForCreate, ToolRecipe, VideoForCreate,
    };
    use crate::core::model::user::{User, UserForCreate};
    use crate::core::model::{Recipe, RecipeDetails};
    use crate::core::repository::ModelManager;
    use crate::core::repository::pool::make_db_pool;
    use crate::core::support::token::AUTH_TOKEN;
    use crate::server::AppState;
    use crate::server::router::middleware::mw_auth::mw_ctx_resolver;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    /// The email address of the default user in the test database.
    pub const TEST_USER_EMAIL: &str = "test@test.com";

    /// The password of the default user in the test database.
    pub const TEST_USER_PASSWORD: &str = "12345678";

    /// The database URL used for connecting to the database in test environments.
    pub(crate) fn test_database_url() -> String {
        std::env::var("RECIPYA_DATABASE_TEST_URL")
            .unwrap_or("postgres://postgres:postgres@localhost:5432/recipya_test".into())
    }

    /// Provides a default config for the tests.
    pub fn default_config() -> Config {
        Config {
            base_url: "http://localhost:8078".into(),
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
            .expect("User should be in database");

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
            .expect("User should be in database");

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
        let user = User::new(
            &AppState::new(config.clone()).await?.mm,
            UserForCreate {
                email: TEST_USER_EMAIL.into(),
                password_clear: TEST_USER_PASSWORD.into(),
            },
        )
        .await?;

        Ok(user)
    }

    /// Inserts another test user in the database.
    pub async fn insert_other_user(config: Config, email: &str) -> Result<User> {
        let user = User::new(
            &AppState::new(config.clone()).await?.mm,
            UserForCreate {
                email: email.into(),
                password_clear: TEST_USER_PASSWORD.into(),
            },
        )
        .await?;

        Ok(user)
    }

    /// Constructs a `RecipeForCreate` instance with all components populated, preparing
    /// it for database insertion.
    pub fn a_complete_recipe_for_create() -> RecipeForCreate {
        let main_image = Uuid::new_v4();
        let secondary_image = Uuid::new_v4();
        let video = Uuid::new_v4();

        RecipeForCreate {
            name: "Best Chinese Kale".into(),
            description: Some("This is the most delicious recipe!".into()),
            images: Some(vec![main_image, secondary_image]),
            yield_: Some(4),
            source: Some(
                "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/".into(),
            ),
            videos: vec![VideoForCreate {
                video,
                duration: Some(chrono::Duration::minutes(7)),
                content_url: Some("https://example.com/best-food.mp4".into()),
                embed_url: Some("https://example.com/embed/j43yfe3.mp4".into()),
            }],
            category: Some("dinner".into()),
            cuisine: Some("thai".into()),
            ingredients: Sections::from([
                (
                    "Sauce".into(),
                    Vec::<String>::from(["1 cup blue spinach".into(), "1/2 tbsp cinnamon".into()]),
                ),
                (
                    "Main".into(),
                    Vec::<String>::from([
                        "4 pounds top quality chicken filet".into(),
                        "1/8 cup lemon juice".into(),
                    ]),
                ),
            ]),
            instructions: Sections::from([
                (
                    "Sauce".into(),
                    Vec::<String>::from(["Mix all these ingredients".into()]),
                ),
                (
                    "Chicken".into(),
                    Vec::<String>::from([
                        "Turn the oven at 300 F".into(),
                        "Soak the chicken in the lemon juice".into(),
                        "Bake for 35 minutes".into(),
                    ]),
                ),
            ]),
            keywords: Vec::<String>::from(["vegetarian".into(), "tofu".into()]),
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
                serving_size: Some("100g".into()),
            }),
            times: Some(TimesForCreate {
                prep_seconds: 120,
                cook_seconds: 3600,
            }),
            tools: vec![
                ToolForCreate {
                    name: "wok".into(),
                    quantity: 1,
                },
                ToolForCreate {
                    name: "frying pan".into(),
                    quantity: 1,
                },
            ],
        }
    }

    /// Constructs a `RecipeDetails` based on a complete `RecipeForCreate` instance.
    pub fn a_complete_recipe() -> RecipeDetails {
        let recipe_c = a_complete_recipe_for_create();

        let images = recipe_c.images.expect("some images");
        let additional_images = images.last().iter().cloned().cloned().collect::<Vec<_>>();

        let created_date = NaiveDate::from_ymd_opt(2012, 12, 31).expect("end of the world");
        let updated_date = NaiveDate::from_ymd_opt(2022, 2, 24).expect("russia invaded Ukraine");
        let time = NaiveTime::from_hms_opt(0, 0, 0).expect("invalid time");

        RecipeDetails {
            recipe: Recipe {
                id: 1,
                name: recipe_c.name,
                description: recipe_c.description,
                image: images.first().cloned().or(None),
                yield_: recipe_c.yield_.ok_or(4).expect("a yield found"),
                language: "en".into(),
                source: recipe_c.source,
                created_at: NaiveDateTime::new(created_date, time),
                updated_at: NaiveDateTime::new(updated_date, time),
                user_id: 1,
            },
            additional_images,
            category: recipe_c.category.expect("a category"),
            cuisine: recipe_c.cuisine,
            ingredients: recipe_c.ingredients,
            instructions: recipe_c.instructions,
            keywords: recipe_c.keywords,
            nutrition: recipe_c.nutrition.map(|n| Nutrition {
                id: 1,
                recipe_id: 1,
                calories_kcal: n.calories_kcal,
                total_carbohydrates: n.total_carbohydrates,
                sugars_g: n.sugars_g,
                protein_g: n.protein_g,
                total_fat_g: n.total_fat_g,
                saturated_fat_g: n.saturated_fat_g,
                unsaturated_fat_g: n.unsaturated_fat_g,
                cholesterol_mg: n.cholesterol_mg,
                sodium_mg: n.sodium_mg,
                fiber_g: n.fiber_g,
                trans_fat_g: n.trans_fat_g,
                serving_size: n.serving_size,
            }),
            times: Times {
                id: 1,
                recipe_id: 1,
                prep_seconds: 3600,
                cook_seconds: 900,
                total_seconds: 4500,
            },
            tools: recipe_c
                .tools
                .iter()
                .enumerate()
                .map(|(i, t)| ToolRecipe {
                    name: t.name.clone(),
                    quantity: t.quantity,
                    tool_order: i as i16,
                })
                .collect(),
            videos: vec![],
        }
    }

    /// Asserts that the response HTML contains all of the expected strings.
    pub fn assert_html(got: TestResponse, want: Vec<&str>) {
        for s in want {
            got.assert_text_contains(s);
        }
    }

    /// Asserts that the response HTML does not contain any of the unwanted strings.
    pub fn assert_not_in_html(got: TestResponse, not_want: Vec<&str>) -> Result<()> {
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

    /// Asserts that the user cannot access the specified URI.
    pub async fn assert_must_be_logged_in(method: axum::http::Method, uri: &str) -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_anonymous(config).await?;

        let res = match method {
            axum::http::Method::GET => server.get(uri),
            axum::http::Method::POST => server.post(uri),
            axum::http::Method::DELETE => server.delete(uri),
            _ => unimplemented!(),
        }
        .await;

        res.assert_status_see_other();
        res.assert_header("Location", "/auth/login");
        Ok(())
    }
}
