use axum_test::{TestResponse, TestServer, TestServerConfig};
use tower_cookies::Cookie;

use recipya::{model::ModelManager, web};

pub fn assert_html(got: TestResponse, want: Vec<&str>) {
    for s in want {
        got.assert_text_contains(s);
    }
}

pub fn assert_not_in_html(got: TestResponse, want: Vec<&str>) {
    let text = got.text();
    for s in want {
        assert!(!text.contains(s), "expected `{s}` not to be in html");
    }
}

pub fn build_server(mm: ModelManager) -> TestServer {
    let routes = recipya::web::routes(mm.clone());
    let config = TestServerConfig::builder().save_cookies().build();
    TestServer::new_with_config(routes, config).unwrap()
}

pub fn build_server_logged_in(mm: ModelManager) -> TestServer {
    let routes = recipya::web::routes(mm.clone());
    let config = TestServerConfig::builder().save_cookies().build();

    let mut server = TestServer::new_with_config(routes, config).unwrap();
    // TODO: Create secure cookie
    server.add_cookie(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    server
}
