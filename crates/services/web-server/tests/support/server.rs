use axum_test::{TestServer, TestServerConfig, TestWebSocket};
use lib_auth::token::generate_web_token;
use lib_core::{ctx::Ctx, model::user::UserBmc};
use lib_web::{utils::token::AUTH_TOKEN, AppState};
use recipya::routes_all;
use tower_cookies::Cookie;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub async fn build_server_anonymous(state: AppState) -> Result<TestServer> {
    let routes = routes_all(state).await?;
    let config = TestServerConfig::builder()
        .mock_transport()
        .save_cookies()
        .build();

    Ok(TestServer::new_with_config(routes, config)?)
}

pub async fn build_server_logged_in(state: AppState) -> Result<TestServer> {
    let routes = routes_all(state.clone()).await?;
    let config = TestServerConfig::builder()
        .mock_transport()
        .save_cookies()
        .build();

    let mut server = TestServer::new_with_config(routes, config).unwrap();

    let email = "test@example.com";
    let user = UserBmc::first_by_email(&Ctx::root_ctx(), &state.mm, email)
        .await?
        .unwrap();
    let token = generate_web_token(email, user.token_salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");
    server.add_cookie(cookie);

    Ok(server)
}

pub async fn build_server_ws(state: AppState) -> Result<(TestServer, TestWebSocket)> {
    let routes = routes_all(state.clone()).await?;
    let config = TestServerConfig::builder()
        .http_transport()
        .save_cookies()
        .build();

    let mut server = TestServer::new_with_config(routes, config).unwrap();

    let email = "test@example.com";
    let user = UserBmc::first_by_email(&Ctx::root_ctx(), &state.mm, email)
        .await?
        .unwrap();
    let token = generate_web_token(email, user.token_salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");
    server.add_cookie(cookie);

    let ws_server = server.get_websocket("/ws").await.into_websocket().await;

    Ok((server, ws_server))
}
