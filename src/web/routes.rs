use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;

use crate::model::ModelManager;
use crate::web;

mod auth;
mod general;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .merge(general::routes())
        .merge(auth::routes(mm.clone()))
        .layer(middleware::map_response(web::middleware::response::map))
        .layer(middleware::from_fn_with_state(mm.clone(), web::middleware::ctx::resolve))
        .layer(CookieManagerLayer::new())
}
