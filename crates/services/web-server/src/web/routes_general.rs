use axum::{middleware, routing::get, Router};
use tower_http::services::ServeDir;

use lib_web::{handlers::handlers_general, middleware::mw_auth, AppState};

use crate::config::web_config;

pub fn routes_general(state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(handlers_general::index)
                .layer(middleware::from_fn(mw_auth::mw_redirect_if_authenticated)),
        )
        .route(
            "/guide/auth/login",
            get(handlers_general::redirect_to_login),
        )
        .route(
            "/ws",
            get(handlers_general::ws_handler).layer(middleware::from_fn(mw_auth::mw_ctx_require)),
        )
        .nest_service("/guide", ServeDir::new(&web_config().DOCS_FOLDER))
        .nest_service("/static", ServeDir::new(&web_config().WEB_FOLDER))
        .with_state(state)
}
