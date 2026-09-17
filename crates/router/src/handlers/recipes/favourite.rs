use axum::{
    Form,
    extract::{Path, State},
    http::Uri,
    response::IntoResponse,
};
use tracing::error;

use app::state::AppState;
use models::{Recipe, data::ViewRecipe, time::FormattedTimes};

use crate::{
    Error, handlers::message::broadcast_error, middleware::mw_auth::RequireAuth,
    recipes_router::params::FavouriteParams,
};

/// Toggles the favourite state of a recipe.
pub async fn toggle_favourite_handler(
    uri: Uri,
    RequireAuth(user): RequireAuth,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
    Form(params): Form<FavouriteParams>,
) -> impl IntoResponse {
    let is_favourite = match Recipe::toggle_favourite(&state.mm, user.id, recipe_id).await {
        Ok(v) => v,
        Err(err) => {
            error!(?recipe_id, user = ?user.id, ?err, "Error toggling the favourite state of recipe");
            broadcast_error(&state, user.id, "Error toggling favourite.").await;
            return Error::Database.into_response();
        }
    };

    if let Ok(r) = Recipe::get(&state.mm, user.id, recipe_id).await
        && let Ok(times) = FormattedTimes::from_times(&r.times)
    {
        let cache_key = (user.id, recipe_id);
        let view_recipe = ViewRecipe {
            recipe_details: r,
            formatted_times: times,
        };

        state.remove_cached_recipe(cache_key).await;
        state.cache_recipe(cache_key, &view_recipe).await;
    }

    let path = uri.path();
    let is_deletable = path.starts_with("/recipes/search") && path.contains("fav=true");

    templates::recipes::render_favourite_button(
        recipe_id,
        is_favourite,
        is_deletable,
        params.is_view_recipe.unwrap_or_default(),
    )
    .into_response()
}
