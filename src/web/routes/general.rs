use std::sync::Arc;

use axum::extract::State;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

use crate::app;

pub fn routes(state: Arc<app::App>) -> Router {
    Router::new()
        .route("/", get(handle_index))
        .route("/guide/auth/login", get(handle_redirect_to_login))
        .nest_service("/guide", ServeDir::new("docs/public"))
        .nest_service("/static", ServeDir::new("web/static"))
        .with_state(state)
}

async fn handle_index(State(app): State<Arc<app::App>>) -> Redirect {
    let mut redirect_url = "/guide";
    if app.is_bypass_guide() {
        redirect_url = "/auth/login";
    }

    Redirect::to(redirect_url)
}

async fn handle_redirect_to_login() -> Redirect {
    Redirect::permanent("/auth/login")
}
