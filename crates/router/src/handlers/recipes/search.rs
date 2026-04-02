use axum::{
    extract::{OriginalUri, Query, State},
    http::HeaderMap,
    response::IntoResponse,
};
use iso8601::DateTime;
use tracing::error;

use app::state::AppState;
use models::{
    Recipe,
    data::{AboutData, Data, PaginationData, SearchbarData, ViewRecipe},
    params::SearchParams,
    time::FormattedTimes,
};

use crate::{
    Error, Result,
    handlers::{get_settings, helpers::is_hx_request, message::broadcast_error},
    middleware::mw_auth::RequireAuth,
};

/// Handles searching recipes.
pub async fn search_recipes_handler(
    headers: HeaderMap,
    Query(search_params): Query<SearchParams>,
    OriginalUri(uri): OriginalUri,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let recipes = match Recipe::get_page(&state.mm, user.id, &search_params).await {
        Ok(recipes) => {
            let mapped_recipes: std::result::Result<Vec<ViewRecipe>, _> = recipes
                .into_iter()
                .map(|recipe| {
                    FormattedTimes::from_times(&recipe.times)
                        .map(|formatted_times| ViewRecipe {
                            recipe_details: recipe,
                            formatted_times,
                        })
                        .map_err(async |err| {
                            error!("Error formatting times for recipe: {err}");
                            Error::Database
                        })
                })
                .collect();

            if let Ok(mapped) = mapped_recipes {
                mapped
            } else {
                broadcast_error(&state, user.id, "Error formatting recipe times.").await;
                return Err(Error::Database);
            }
        }
        Err(err) => {
            error!(
                "(search_recipes_handler) Error fetching recipes for user '{}' with search params '{search_params:?}': {err}",
                user.id
            );
            broadcast_error(&state, user.id, "Error fetching recipes.").await;
            return Err(Error::Database);
        }
    };

    if recipes.is_empty() {
        let is_favourites = search_params.is_favourites.unwrap_or_default();

        return Ok(templates::search::no_results(is_favourites).into_response());
    }

    let settings = get_settings(&state, user.id).await?;

    Ok(templates::recipes::search_results(
        &state.fs_support,
        uri.path(),
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&headers),
            // TODO: Populate AboutData with good values.
            is_preview: false,
            about: AboutData::new(false, false, DateTime::default(), DateTime::default()),
            pagination: Some(PaginationData::new_for_recipes(
                &search_params,
                recipes.len().try_into().unwrap_or(i64::MAX),
                headers.get(axum_htmx::HX_REQUEST).is_some(),
            )),
            searchbar: Some(SearchbarData::from_params(search_params)),
            share: None,
            recipes,
            reports: None,
        },
        &state.data_dir,
        &settings,
    )
    .into_response())
}
