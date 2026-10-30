use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum_extra::extract::Form;
use axum_htmx::{HX_LOCATION, HX_REDIRECT};
use fluent_static::support::axum::RequestLanguage;
use reqwest::StatusCode;
use tracing::error;

use app::message::ToastCtx;
use app::state::AppState;
use l10n::Messages;
use models::Recipe;
use support::states::AuthenticatedState;

use crate::middleware::mw_auth::RequireAuth;
use crate::schemas::recipes::BulkDeleteForm;

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
            let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);
            error!(?recipe_id, user = ?user.id, ?err, "Error deleting recipe");
            toast.error(&messages.toast_recipes_delete_failed(1)).await;
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handles deleting the user's selected recipes at once.
pub async fn bulk_delete_recipes_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    Form(form): Form<BulkDeleteForm>,
) -> impl IntoResponse {
    match Recipe::delete_bulk(&state.mm, &form.ids, user.id).await {
        Ok(()) => (
            StatusCode::NO_CONTENT,
            [(HX_LOCATION, r##"{"path":"/recipes","target":"#content"}"##)],
        )
            .into_response(),
        Err(err) => {
            let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);
            error!(recipe_ids = ?form.ids, user = ?user.id, ?err, "Error deleting recipes");
            toast
                .error(&messages.toast_recipes_delete_failed(form.ids.len()))
                .await;
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
