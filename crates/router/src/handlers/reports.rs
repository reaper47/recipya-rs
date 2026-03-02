use app::state::AppState;
use axum::{
    extract::{OriginalUri, State},
    http::HeaderMap,
    response::IntoResponse,
};
use models::data::Data;

use crate::{
    Result,
    handlers::{get_settings, helpers::is_hx_request},
    middleware::mw_auth::RequireAuth,
};

pub async fn reports_handler(
    header_map: HeaderMap,
    OriginalUri(uri): OriginalUri,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let settings = get_settings(&state, user.id).await?;

    Ok(templates::reports::index(
        uri.path(),
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            ..Default::default()
        },
        &settings,
    )
    .into_response())
}
