use std::sync::{Arc, OnceLock};

use axum::Router;
use axum::http::Method;
use axum_test::{TestServer, TestServerConfig, TestWebSocket, Transport};
use test_db::TestDb;
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
use router::router;
use support::fs::MockFs;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

/// The email address of the default user in the test database.
pub const TEST_USER_EMAIL: &str = "test@test.com";

/// The password of the default user in the test database.
pub const TEST_USER_PASSWORD: &str = "12345678";

/// The email address of the demo user in the test database.
pub const TEST_DEMO_EMAIL: &str = "demo@demo.com";

pub static TEST_PASSWORD_SALT: OnceLock<Uuid> = OnceLock::new();

pub const TEST_PASSWORD_HASH: &str = "#02#$argon2id$v=19$m=19456,t=2,p=1$bpxlx+8RTH2isz8k1XIyXw$jPya0CutmZGEV2bcveP+eDalHVGi7Nz2D8cjKsy2MN4";

fn get_password_salth() -> Uuid {
    TEST_PASSWORD_SALT
        .get_or_init(|| Uuid::parse_str("6e9c65c7-ef11-4c7d-a2b3-3f24d572325f").unwrap())
        .clone()
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
pub async fn build_server_anonymous(app_config: Config) -> Result<TestServer> {
    let routes = prepare_router(app_config).await?;
    let config = TestServerConfig {
        save_cookies: true,
        ..TestServerConfig::default()
    };

    Ok(TestServer::new_with_config(routes, config))
}

/// Builds a test server with a logged-in user.
///
/// # Panics
///
/// Panics if the test user is not found in the database.
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

    let mut server = TestServer::new_with_config(routes, config);
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

    let mut server = TestServer::new_with_config(routes, config);

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

    User::new_with_hash(
        &mm,
        UserForCreate {
            email: email.clone(),
            password_clear: "12345678".to_string(),
        },
        get_password_salth(),
        TEST_PASSWORD_HASH.to_string(),
    )
    .await?;

    let user = User::get_user_by_email(&mm, &email)
        .await?
        .expect("User not found");

    Ok(generate_access_token(&user.id)?)
}

/// Prepares the router for the test server with the given database URL.
async fn prepare_router(config: Config) -> Result<Router<()>> {
    let state = create_app_state(config).await;
    let app = router(state.clone())?
        .layer(CookieManagerLayer::new())
        .with_state(state.clone());

    User::new_with_hash(
        &state.mm,
        UserForCreate {
            email: TEST_USER_EMAIL.into(),
            password_clear: TEST_USER_PASSWORD.into(),
        },
        get_password_salth(),
        TEST_PASSWORD_HASH.to_string(),
    )
    .await?;

    User::new_with_hash(
        &state.mm,
        UserForCreate {
            email: TEST_DEMO_EMAIL.into(),
            password_clear: TEST_USER_PASSWORD.into(),
        },
        get_password_salth(),
        TEST_PASSWORD_HASH.to_string(),
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
    Ok(User::new_with_hash(
        &create_app_state(config.clone()).await.mm,
        UserForCreate {
            email: email.into(),
            password_clear: TEST_USER_PASSWORD.into(),
        },
        get_password_salth(),
        TEST_PASSWORD_HASH.to_string(),
    )
    .await?)
}

/// Asserts that the user cannot access the specified URI.
pub async fn assert_must_be_logged_in(method: Method, uri: &str) -> Result<()> {
    let (_test_db, config) = TestDb::new(None).await?;
    let server = build_server_anonymous(config).await?;

    let res = match method {
        Method::DELETE => server.delete(uri),
        Method::GET => server.get(uri),
        Method::POST => server.post(uri),
        Method::PATCH => server.patch(uri),
        Method::PUT => server.put(uri),
        _ => unimplemented!(),
    }
    .await;

    res.assert_status_see_other();
    res.assert_header("Location", "/auth/login");
    Ok(())
}
