use crate::{app, web};
use axum::extract::State;
use axum::routing::get;
use axum::Router;
use maud::Markup;
use std::sync::Arc;

pub fn routes(state: Arc<app::App>) -> Router {
    Router::new()
        .route("/auth/forgot-password", get(handle_forgot_password))
        .route("/auth/login", get(handle_login))
        .route("/auth/register", get(handle_register))
        .with_state(state)
}

async fn handle_forgot_password() -> Markup {
    web::templates::auth::forgot_password().await
}

async fn handle_login(State(app): State<Arc<app::App>>) -> Markup {
    web::templates::auth::login(app.is_demo(), app.is_no_signups()).await
}

async fn handle_register() -> Markup {
    web::templates::auth::register().await
}
