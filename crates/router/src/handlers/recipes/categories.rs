use axum::{Form, extract::State, response::IntoResponse};
use reqwest::StatusCode;
use tracing::error;

use app::state::AppState;
use models::Recipe;

use crate::{
    Error, handlers::message::broadcast_error, middleware::mw_auth::RequireAuth,
    recipes_router::params::RecipeCategoryForm,
};

/// Handles adding a recipe category into the database.
pub async fn post_recipe_categories_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(form): Form<RecipeCategoryForm>,
) -> impl IntoResponse {
    let category = form.category;
    if category.is_empty() {
        return Error::InvalidPayload.into_response();
    }

    if let Err(err) = Recipe::add_category(&state.mm, &category, user.id).await {
        error!(?err, "Error adding recipe category");
        broadcast_error(&state, user.id, "Failed to add recipe category.").await;
        return Error::Database.into_response();
    }

    templates::settings::new_recipe_category(&category).into_response()
}

/// Handles deleting a recipe category from the database.
pub async fn delete_recipe_categories_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(form): Form<RecipeCategoryForm>,
) -> impl IntoResponse {
    let category = form.category;
    if category.is_empty() || category == "uncategorized" {
        broadcast_error(
            &state,
            user.id,
            "Category cannot be empty or uncategorized.",
        )
        .await;
        return Error::InvalidPayload.into_response();
    }

    if let Err(err) = Recipe::delete_recipe_category(&state.mm, &category, user.id).await {
        error!(?err, "Error deleting recipe category");
        broadcast_error(&state, user.id, "Failed to delete recipe category.").await;
        return Error::Database.into_response();
    }

    (StatusCode::NO_CONTENT, "").into_response()
}
