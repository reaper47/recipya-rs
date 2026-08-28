mod import_api;
mod import_app;
mod import_preview;
mod manual;
mod raw;
mod website;

pub use import_api::*;
pub use import_app::*;
pub use import_preview::*;
pub use manual::*;
pub use raw::*;
pub use website::*;

use axum::{
    extract::{OriginalUri, State},
    http::HeaderMap,
    response::IntoResponse,
};

use app::state::AppState;
use models::data::{Data, States};

use crate::{
    Result,
    handlers::{get_settings, helpers::is_hx_request},
    middleware::mw_auth::RequireAuth,
};

/// Handles the add recipe page.
pub async fn add_recipes_handler(
    header_map: HeaderMap,
    OriginalUri(uri): OriginalUri,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let settings = get_settings(&state, user.id).await?;

    Ok(templates::recipes::add_page(
        uri.path(),
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            states: States {
                autologin: state.config.read().await.states.autologin.clone(),
                ..Default::default()
            },
            is_hx_request: is_hx_request(&header_map),
            ..Default::default()
        },
        &settings,
    )
    .into_response())
}
