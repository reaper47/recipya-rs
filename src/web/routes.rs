use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;

use crate::model::ModelManager;
use crate::web;
use crate::web::rpc;

mod auth;
mod general;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .merge(general::routes())
        .merge(auth::routes(mm.clone()))
        .nest("/api", rpc::routes(mm.clone()))
        .layer(middleware::map_response(web::middleware::response::map))
        .layer(middleware::from_fn_with_state(mm.clone(), web::middleware::ctx::resolve))
        .layer(CookieManagerLayer::new())
}
