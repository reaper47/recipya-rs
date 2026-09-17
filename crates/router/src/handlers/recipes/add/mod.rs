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
    response::IntoResponse,
};
use axum_htmx::HxRequest;

use app::state::AppState;
use config::States;
use models::data::Data;

use crate::{Result, handlers::get_settings, middleware::mw_auth::RequireAuth};

/// Handles the add recipe page.
pub async fn add_recipes_handler(
    HxRequest(is_hx_request): HxRequest,
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
                autologin: state.config.read().await.states.autologin,
                ..Default::default()
            },
            is_hx_request,
            ..Default::default()
        },
        &settings,
    )
    .into_response())
}
