use axum::response::Redirect;
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

use crate::config;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/guide/auth/login", get(redirect_to_login))
        .nest_service("/guide", ServeDir::new(&config().PATHS.DOCS))
        .nest_service("/static", ServeDir::new(&config().PATHS.STATIC))
    //.with_state(state)
}

async fn index() -> Redirect {
    let mut redirect_url = "/guide";
    if config().IS_BYPASS_GUIDE {
        redirect_url = "/auth/login";
    }

    Redirect::to(redirect_url)
}

async fn redirect_to_login() -> Redirect {
    Redirect::permanent("/auth/login")
}
