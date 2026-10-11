use axum::{Form, extract::State, response::IntoResponse};
use fluent_static::support::axum::RequestLanguage;
use reqwest::StatusCode;
use tracing::error;

use app::message::{Broadcaster, Toast};
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
        error!(?err, "Error adding recipe category");
        Toast::broadcast_error(
            &state,
            user.id,
            &messages.toast_recipes_add_category_failed(),
            &messages,
        )
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
    let category = form.category;
    if category.is_empty() || category == "uncategorized" {
        Toast::broadcast_error(
            &state,
            user.id,
            &messages.toast_recipes_category_empty(),
            &messages,
        )
        .await;
        return Error::InvalidPayload.into_response();
    }

    if let Err(err) = Recipe::delete_recipe_category(&state.mm, &category, user.id).await {
        error!(?err, "Error deleting recipe category");
        Toast::broadcast_error(
            &state,
            user.id,
            &messages.toast_recipes_category_delete_failed(),
            &messages,
        )
        .await;
        return Error::Database.into_response();
    }

    (StatusCode::NO_CONTENT, "").into_response()
}
