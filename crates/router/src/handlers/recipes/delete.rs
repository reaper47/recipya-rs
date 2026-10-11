use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum_htmx::HX_REDIRECT;
use fluent_static::support::axum::RequestLanguage;
use reqwest::StatusCode;
use tracing::error;

use app::message::{Broadcaster, Toast};
use app::state::AppState;
use l10n::Messages;
use models::Recipe;

use crate::middleware::mw_auth::RequireAuth;

/// Handles deleting a user's recipe.
pub async fn delete_recipe_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match Recipe::delete(&state.mm, recipe_id, user.id).await {
        Ok(()) => {
            state.remove_cached_recipe((user.id, recipe_id)).await;
            (StatusCode::NO_CONTENT, [(HX_REDIRECT, "/")]).into_response()
        }
        Err(err) => {
            error!(?recipe_id, user = ?user.id, ?err, "Error deleting recipe");
            Toast::broadcast_error(
                &state,
                user.id,
                &messages.toast_recipes_delete_failed(),
                &messages,
            )
            .await;
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
