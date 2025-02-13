use axum::extract::{Path, State};
use axum::response::IntoResponse;
use crate::core::model::Recipe;
use crate::server::AppState;
use crate::server::router::middleware::mw_auth::CtxW;

/// Handles viewing a recipe.
pub async fn recipe_view_handler(ctx: CtxW, Path(recipe_id): Path<i64>, State(state): State<AppState>) -> impl IntoResponse {
    let user_id = ctx.0.user_id();
    let recipe = Recipe::get(&state.mm, user_id, recipe_id).await?;

}
