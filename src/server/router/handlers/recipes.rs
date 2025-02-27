use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;

use crate::core::model::Recipe;
use crate::server::error::Result;
use crate::server::router::middleware::mw_auth::CtxW;
use crate::server::templates::data::{AboutData, Data, FormattedTimes, ShareData, ViewRecipe};
use crate::server::{templates, AppState};

/// Handles viewing a recipe.
pub async fn recipe_view_handler(
    ctx: CtxW,
    headers: HeaderMap,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = ctx.0.user_id();

    let recipe = match Recipe::get(&state.mm, user_id, recipe_id).await {
        Ok(recipe) => recipe,
        Err(_) => {
            return Ok(templates::general::simple("Recipe Not Found", "The recipe you requested to view is not found."));
        }
    };

    let formatted_times = FormattedTimes::from_times(&recipe.times)?;

    Ok(templates::recipes::view_recipe(
        state.data_dir,
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: false,
            is_hx_request: headers
                .get(axum_htmx::headers::HX_REQUEST)
                .map(|v| v == "true")
                .unwrap_or(false),
            about: AboutData {
                is_update_available: false,
            },
            share: ShareData {
                is_from_host: true,
                is_shared: false,
            },
            view: ViewRecipe {
                recipe_details: recipe,
                formatted_times,
            },
        },
    ))
}
