use crate::{
    config::web_config,
    web::{routes_auth, routes_general},
};
use axum::{middleware, Router};
use lib_core::model::ModelManager;
use lib_web::{
    middleware::{mw_auth::mw_ctx_resolve, mw_res_map::mw_reponse_map},
    routes::routes_static,
};
use tower_cookies::CookieManagerLayer;

mod config;
pub mod web;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub async fn routes_all(mm: ModelManager) -> Result<Router> {
    /* TODO: Figure out how to make this work
    let routes_rpc = web::routes_rpc::routes(mm.clone())
        .route_layer(middleware::from_fn(mw_ctx_require));*/

    let router = Router::new()
        // .nest("/api", routes_rpc)
        .nest("/auth", routes_auth::routes(mm.clone()))
        .merge(routes_general::routes())
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir(&web_config().WEB_FOLDER));

    Ok(router)
}
