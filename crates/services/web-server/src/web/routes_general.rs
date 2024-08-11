use axum::{response::Redirect, routing::get, Router};
use tower_http::services::ServeDir;

use lib_core::config::config;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/guide/auth/login", get(redirect_to_login))
        .nest_service("/guide", ServeDir::new(&config().PATHS.DOCS))
        .nest_service("/static", ServeDir::new(&config().PATHS.STATIC))
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
