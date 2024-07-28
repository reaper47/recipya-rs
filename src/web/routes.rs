use std::sync::Arc;

use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;

use crate::model::ModelManager;
use crate::{app::App, web};

mod auth;
mod general;

pub fn routes(app: Arc<App>, mm: ModelManager) -> Router {
    Router::new()
        .merge(general::routes(Arc::clone(&app)))
        .merge(auth::routes(Arc::clone(&app)))
        .layer(middleware::map_response(web::middleware::response::map))
        .layer(middleware::from_fn_with_state(
            mm,
            web::middleware::ctx::resolver,
        ))
        .layer(CookieManagerLayer::new())
}
