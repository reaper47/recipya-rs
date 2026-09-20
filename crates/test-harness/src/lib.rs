use std::{pin::Pin, sync::Arc, time::Duration};

use axum::Router;
use axum::http::Method;
use axum_test::{TestResponse, TestServer, TestServerConfig, Transport};
use eventsource_stream::{Event as SseEvent, EventStreamError, Eventsource};
use futures_util::{Stream, StreamExt};
use test_db::default_config;
use test_fixtures::{
    TEST_DEMO_EMAIL, TEST_PASSWORD_HASH, TEST_USER_EMAIL, TEST_USER_PASSWORD, get_password_salt,
};
use tokio::time::timeout;
use tower_cookies::{Cookie, CookieManagerLayer};
use tracing::error;

use app::state::AppState;
use auth::token::{generate_access_token, http::AUTH_TOKEN};
use config::Config;
use models::user::{User, UserForCreate};
use recipya_scraper::tests::MockHttpClient;
use router::router;
use support::fs::MockFs;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

/// The HTML returned when the SSE notification should be hidden.
pub const HIDDEN_SSE_NOTIFICATION: &str = r#"<div id="sse-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default hidden"><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1"></p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">-1 of -1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#;

/// A wrapper around an SSE stream for testing.
pub struct TestSse {
    stream: Pin<
        Box<
            dyn Stream<Item = std::result::Result<SseEvent, EventStreamError<reqwest::Error>>>
                + Send,
        >,
    >,
}

impl TestSse {
    /// Connects to the /sse endpoint.
    ///
    /// # Panics
    ///
    /// Panics if the connection fails.
    pub async fn connect(url: &str, auth_token: &str) -> Self {
        Self {
            stream: Box::pin(
                reqwest::Client::new()
                    .get(url)
                    .header(
                        reqwest::header::COOKIE,
                        format!("{AUTH_TOKEN}={auth_token}"),
                    )
                    .send()
                    .await
                    .expect("failed to connect to SSE endpoint")
                    .bytes_stream()
                    .eventsource(),
            ),
        }
    }

    /// Receives a single SSE event.
    ///
    /// # Panics
    ///
    /// Panics if the connection fails.
    pub async fn receive_data(&mut self) -> String {
        timeout(Duration::from_secs(5), self.stream.next())
            .await
            .expect("timed out waiting for SSE message")
            .expect("SSE stream ended unexpectedly")
            .expect("SSE stream error")
            .data
    }

    /// Receives all SSE events up to a given count.
    ///
    /// # Panics
    ///
    /// Panics if the connection fails.
    pub async fn receive_all_data(&mut self, count: usize) -> Vec<String> {
        let mut event_data = Vec::with_capacity(count);
        while event_data.len() < count {
            let event = self
                .stream
                .next()
                .await
                .expect("SSE stream ended unexpectedly")
                .expect("error in event stream");
            event_data.push(event.data);
        }
        event_data
    }
}

/// Creates the `AppState` for testing.
///
/// # Panics
///
/// Panics if the app state fails to initialize.
pub async fn create_app_state(config: Config) -> AppState {
    AppState::new_for_test(
        config,
        test_db::test_model_manager().await,
        Arc::new(MockHttpClient),
        Arc::new(MockFs),
    )
    .map_err(|err| {
        error!(?err, "Could not initialise app state");
        err
    })
    .expect("failed to initialise app state")
}

/// Builds a test server with anonymous access (no user logged in).
pub async fn build_server_anonymous(app_config: Config) -> Result<(TestServer, AppState)> {
    let (routes, state) = prepare_router(app_config).await?;
    let config = TestServerConfig {
        save_cookies: true,
        ..TestServerConfig::default()
    };

    Ok((TestServer::new_with_config(routes, config), state))
}

/// Builds a test server with a logged-in user.
///
/// # Panics
///
/// Panics if the test user is not found in the database.
pub async fn build_server_logged_in(app_config: Config) -> Result<(TestServer, AppState)> {
    let (routes, state) = prepare_router(app_config).await?;
    let config = TestServerConfig {
        save_cookies: true,
        ..TestServerConfig::default()
    };

    let user = User::get_user_by_email(&state.mm, TEST_USER_EMAIL)
        .await?
        .expect("user should be in database");

    let token = generate_access_token(&user.id)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token);
    cookie.set_http_only(true);
    cookie.set_path("/");

    let mut server = TestServer::new_with_config(routes, config);
    server.add_cookie(cookie);
    Ok((server, state))
}

/// Builds and initializes a test server with SSE support.
pub async fn build_server_sse(app_config: Config) -> Result<(TestServer, TestSse, AppState)> {
    build_server_sse_helper(app_config, TEST_USER_EMAIL).await
}

/// Builds and initializes a test server with SSE support for a user other
/// than the test one.
pub async fn build_server_sse_other_user(
    app_config: Config,
    auth_email: &str,
) -> Result<(TestServer, TestSse, AppState)> {
    build_server_sse_helper(app_config, auth_email).await
}

async fn build_server_sse_helper(
    app_config: Config,
    auth_email: &str,
) -> Result<(TestServer, TestSse, AppState)> {
    let (routes, state) = prepare_router(app_config).await?;
    let config = TestServerConfig {
        save_cookies: true,
        transport: Some(Transport::HttpRandomPort),
        ..TestServerConfig::default()
    };

    let user = User::get_user_by_email(&state.mm, auth_email)
        .await?
        .expect("user should be in database");

    let mut server = TestServer::new_with_config(routes, config);

    let token = generate_access_token(&user.id)?;
    let mut cookie = Cookie::new(AUTH_TOKEN, &token);
    cookie.set_http_only(true);
    cookie.set_path("/");
    server.add_cookie(cookie);

    let sse_url = server.server_url("/sse").unwrap().to_string();
    let sse_server = TestSse::connect(&sse_url, &token).await;

    Ok((server, sse_server, state))
}

/// Prepares the router for the test server with the given database URL.
async fn prepare_router(config: Config) -> Result<(Router<()>, AppState)> {
    let state = create_app_state(config).await;
    let app = router(&state)?
        .layer(CookieManagerLayer::new())
        .with_state(state.clone());

    User::new_with_hash(
        &state.mm,
        UserForCreate {
            email: TEST_USER_EMAIL.into(),
            password_clear: TEST_USER_PASSWORD.into(),
        },
        get_password_salt(),
        TEST_PASSWORD_HASH.to_string(),
    )
    .await?;

    User::new_with_hash(
        &state.mm,
        UserForCreate {
            email: TEST_DEMO_EMAIL.into(),
            password_clear: TEST_USER_PASSWORD.into(),
        },
        get_password_salt(),
        TEST_PASSWORD_HASH.to_string(),
    )
    .await?;

    Ok((app, state))
}

/// Asserts that the user cannot access the specified URI.
pub async fn assert_must_be_logged_in(method: Method, uri: &str) -> Result<()> {
    let (server, _) = build_server_anonymous(default_config()).await?;

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

/// Asserts that the response HTML contains all of the expected strings.
pub fn assert_html(got: &TestResponse, want: &[&str]) {
    for s in want {
        got.assert_text_contains(s);
    }
}

/// Asserts that the response HTML does not contain any of the unwanted strings.
///
/// # Panics
///
/// Panics if any of the unwanted strings are found in the response HTML.
pub fn assert_not_in_html(got: &TestResponse, not_want: &[&str]) -> Result<()> {
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

/// Asserts that the SSE server sent the wanted message.
///
/// # Panics
///
/// Panics if the connection fails or the message is not received within 5 seconds.
pub async fn assert_sse_message(server: &mut TestSse, want: &str) {
    let data = server.receive_data().await;
    assert!(
        data.contains(want),
        "Expected SSE message containing '{want}', got:\n{data}",
    );
}

/// Asserts that the SSE server sent all wanted messages, in any order.
///
/// # Panics
///
/// Panics if any of the wanted messages are not received.
pub async fn assert_sse_messages_any_order(server: &mut TestSse, wants: &[&str]) {
    let received = server.receive_all_data(wants.len()).await;

    for want in wants {
        assert!(
            received.iter().any(|msg| msg.contains(want)),
            "Failed to find '{}' in received messages:\n{}",
            want,
            received.join("\n")
        );
    }
}
