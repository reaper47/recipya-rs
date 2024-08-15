use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use lib_core::config::config;
use lib_web::handlers::handlers_general;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(handlers_general::index))
        .route("/guide/auth/login", get(handlers_general::redirect_to_login))
        .nest_service("/guide", ServeDir::new(&config().PATHS.DOCS))
        .nest_service("/static", ServeDir::new(&config().PATHS.STATIC))
}
