use axum_test::{TestServer, TestServerConfig};
use lib_core::model::ModelManager;
use lib_web::utils::token::AUTH_TOKEN;
use recipya::{routes_all, web::routes_auth};
use tower_cookies::Cookie;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub async fn build_server(mm: ModelManager) -> Result<TestServer> {
    let routes = routes_all(mm).await?;
    let config = TestServerConfig::builder().save_cookies().build();
    Ok(TestServer::new_with_config(routes, config)?)
}

pub fn build_server_logged_in(mm: ModelManager) -> TestServer {
    let routes = routes_auth::routes(mm.clone());
    let config = TestServerConfig::builder().save_cookies().build();

    let mut server = TestServer::new_with_config(routes, config).unwrap();
    // TODO: Create secure cookie
    server.add_cookie(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));
    server
}
