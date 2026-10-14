use axum::{Form, extract::State, response::IntoResponse};
use fluent_static::support::axum::RequestLanguage;
use reqwest::StatusCode;
use support::states::AuthenticatedState;
use tracing::error;

use app::message::ToastCtx;
use app::state::AppState;
use l10n::Messages;
use models::Recipe;

use crate::{Error, middleware::mw_auth::RequireAuth, params::RecipeCategoryForm};

/// Handles adding a recipe category into the database.
pub async fn post_recipe_categories_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    Form(form): Form<RecipeCategoryForm>,
) -> impl IntoResponse {
    let category = form.category;
    if category.is_empty() {
        return Error::InvalidPayload.into_response();
    }

    if let Err(err) = Recipe::add_category(&state.mm, &category, user.id).await {
        let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);
        error!(?err, "Error adding recipe category");
        toast
            .error(&messages.toast_recipes_add_category_failed())
            .await;
        return Error::Database.into_response();
    }

    templates::settings::new_recipe_category(&category, &messages).into_response()
}

/// Handles deleting a recipe category from the database.
pub async fn delete_recipe_categories_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    Form(form): Form<RecipeCategoryForm>,
) -> impl IntoResponse {
    let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);

    let category = form.category;
    if category.is_empty() || category == "uncategorized" {
        toast.error(&messages.toast_recipes_category_empty()).await;
        return Error::InvalidPayload.into_response();
    }

    if let Err(err) = Recipe::delete_recipe_category(&state.mm, &category, user.id).await {
        error!(?err, "Error deleting recipe category");
        toast
            .error(&messages.toast_recipes_category_delete_failed())
            .await;
        return Error::Database.into_response();
    }

    (StatusCode::NO_CONTENT, "").into_response()
}
