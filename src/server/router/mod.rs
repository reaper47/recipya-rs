mod auth_router;
mod general_router;
mod handlers;
pub(crate) mod middleware;
mod recipes_routes;
mod static_files_router;

use axum::Router;

use crate::error::Result;
use crate::server::AppState;
use crate::server::router::auth_router::auth_routes;
use crate::server::router::general_router::general_routes;
use crate::server::router::recipes_routes::recipes_routes;
use crate::server::router::static_files_router::static_files_routes;

/// Creates the Router for the web server.
pub async fn router(state: AppState) -> Result<Router<AppState>> {
    let router = Router::new()
        .nest("/auth", auth_routes(state.clone()))
        .nest("/recipes", recipes_routes(state.clone()))
        .merge(general_routes(state.clone()))
        .merge(static_files_routes(state.clone()));

    Ok(router)
}
