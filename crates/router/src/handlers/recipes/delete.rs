use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum_htmx::HX_REDIRECT;
use reqwest::StatusCode;
use tracing::error;

use app::state::AppState;
use models::Recipe;

use crate::handlers::message::broadcast_error;
use crate::middleware::mw_auth::RequireAuth;

/// Handles deleting a user's recipe.
pub async fn delete_recipe_handler(
    RequireAuth(user): RequireAuth,
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
            broadcast_error(&state, user.id, "Recipe could not be deleted.").await;
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
