#![recursion_limit = "256"]

mod admin_router;
mod auth_router;
mod error;
mod general_router;
mod handlers;
mod recipes_router;
mod schemas;
mod settings_router;
mod shared_router;
mod static_files_router;

pub mod middleware;

pub use error::{Error, Result};
pub use handlers::static_files::copy_to_fs;

use axum::Router;

use app::state::AppState;

use crate::admin_router::admin_routes;
use crate::auth_router::auth_routes;
use crate::general_router::general_routes;
use crate::recipes_router::recipes_routes;
use crate::shared_router::shared_routes;
use crate::static_files_router::static_files_routes;

/// Creates the Router for the web server.
pub fn router(state: AppState) -> Result<Router<AppState>> {
    let router = Router::new()
        .nest("/admin", admin_routes(&state))
        .nest("/auth", auth_routes(&state))
        .nest("/recipes", recipes_routes(state.clone()))
        .nest("/settings", settings_router::settings_routes(&state))
        .nest("/shared", shared_routes())
        .merge(general_routes(&state))
        .merge(static_files_routes(state));

    Ok(router)
}
