mod auth;
mod general;

use crate::app::App;
use axum::{middleware, Router};
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use crate::web;

pub fn routes(app: Arc<App>) -> Router {
    Router::new()
        .merge(general::routes(Arc::clone(&app)))
        .merge(auth::routes(Arc::clone(&app)))
        .layer(middleware::from_fn_with_state(app, web::middleware::ctx::resolver))
        .layer(CookieManagerLayer::new())
}
