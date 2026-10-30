use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::Form;
use axum_htmx::HX_LOCATION;
use fluent_static::support::axum::RequestLanguage;
use reqwest::StatusCode;
use tracing::error;

use app::message::ToastCtx;
use app::state::AppState;
use l10n::Messages;
use models::{Recipe, types::NonEmptyVec};
use support::states::AuthenticatedState;

use crate::{
    Error,
    handlers::recipes::common::fetch_categories_keywords,
    middleware::mw_auth::RequireAuth,
    schemas::recipes::{BulkCategoriseForm, RecipeCategoryForm},
};

/// Handles fetching all user categories as <option/>s.
pub async fn get_all_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match fetch_categories_keywords(&state, user.id, &messages).await {
        Ok((categories, _)) => {
            templates::general::render_categories_for_datalist(&categories).into_response()
        }
        Err(err) => {
            let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);
            error!(user = ?user.id, ?err, "Error fetching user categories");
            toast
                .error(&messages.toast_settings_fetch_categories_failed())
                .await;
            Error::EntityNotFound {
                entity: "categories",
            }
            .into_response()
        }
    }
}

/// Handles updating the category for the selected recipes.
pub async fn bulk_update_handler(
    RequireAuth(user): RequireAuth,
    RequestLanguage(messages): RequestLanguage<Messages>,
    State(state): State<AppState>,
    Form(form): Form<BulkCategoriseForm>,
) -> impl IntoResponse {
    let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);

    let ids: NonEmptyVec<i64> = if let Ok(v) = form.ids.try_into() {
        v
    } else {
        toast.error(&messages.toast_payload_empty()).await;
        return Error::InvalidPayload.into_response();
    };

    match Recipe::update_category(&state.mm, &form.name, &ids, user.id).await {
        Ok(()) => (
            StatusCode::NO_CONTENT,
            [(HX_LOCATION, r##"{"path":"/recipes","target":"#content"}"##)],
        )
            .into_response(),
        Err(err) => {
            let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);
            error!(recipe_ids = ?ids, user = ?user.id, ?err, "Error category bulk update");
            toast.error(&messages.toast_bulk_update_failed()).await;
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

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
