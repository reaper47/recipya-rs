use axum::{
    extract::{OriginalUri, Path, Query, State},
    http::HeaderMap,
    response::IntoResponse,
};
use config::States;
use iso8601::DateTime;
use tracing::error;

use app::state::AppState;
use models::{
    Recipe,
    data::{AboutData, Data, PaginationData, SearchbarData, ShareData, ViewRecipe},
    params::SearchParams,
    settings::UserSettingDetails,
    time::FormattedTimes,
};

use crate::{
    Error, Result,
    handlers::{get_settings, helpers::is_hx_request, message::broadcast_error},
    middleware::mw_auth::RequireAuth,
};

/// Handles viewing the recipes.
pub async fn recipes_handler(
    headers: HeaderMap,
    Query(search_params): Query<SearchParams>,
    OriginalUri(uri): OriginalUri,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let settings = get_settings(&state, user.id).await?;

    let num_recipes = match Recipe::count(&state.mm, user.id).await {
        Ok(count) => count,
        Err(err) => {
            error!("Error counting recipes for user {}: {err}", user.id);
            broadcast_error(&state, user.id, "Error fetching number of recipes.").await;
            return Ok(Error::Database.into_response());
        }
    };

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
                "(recipes_handler) Error fetching recipes for user '{:?}' with search params '{search_params:?}': {err}",
                user.id
            );
            broadcast_error(&state, user.id, "Error fetching recipes.").await;
            return Err(Error::Database);
        }
    };

    Ok(templates::recipes::index(
        &state.fs_support,
        uri.path(),
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            states: States {
                autologin: state.config.read().await.states.autologin.clone(),
                ..Default::default()
            },
            is_hx_request: is_hx_request(&headers),
            about: AboutData::new(false, false, DateTime::default(), DateTime::default()),
            pagination: Some(PaginationData::new_for_recipes(
                &search_params,
                num_recipes,
                is_hx_request(&headers),
            )),
            searchbar: Some(SearchbarData::from_params(search_params)),
            recipes,
            ..Default::default()
        },
        &state.data_dir,
        &settings,
    )
    .into_response())
}

/// Handles viewing a recipe.
pub async fn view_recipe_handler(
    header_map: HeaderMap,
    Path(recipe_id): Path<i64>,
    OriginalUri(uri): OriginalUri,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let cache_key = (user.id, recipe_id);
    let mut view_recipe = if let Some(recipe) = state.get_cached_recipe(cache_key).await {
        recipe
    } else {
        let Ok(recipe) = Recipe::get(&state.mm, user.id, recipe_id).await else {
            return Ok(templates::general::simple(
                "Recipe Not Found",
                "The recipe you requested to view is not found.",
            ));
        };

        let formatted_times = FormattedTimes::from_times(&recipe.times)?;
        let view_recipe = ViewRecipe {
            recipe_details: recipe,
            formatted_times,
        };

        state.cache_recipe(cache_key, &view_recipe).await;
        view_recipe
    };

    let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;

    if user_settings.is_bold_ingredients
        && let Err(err) = view_recipe
            .recipe_details
            .bold_ingredients_in_instructions(&state.mm)
            .await
    {
        error!(
            recipe_id = recipe_id,
            user_id = user.id.to_string(),
            err = err.to_string(),
            "Failed to bolden instructions"
        );
    }

    match templates::recipes::view_recipe(
        &state.fs_support,
        uri.path(),
        &state.data_dir,
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            states: States {
                autologin: state.config.read().await.states.autologin.clone(),
                ..Default::default()
            },
            is_hx_request: is_hx_request(&header_map),
            about: AboutData::new(false, false, DateTime::default(), DateTime::default()),
            pagination: Some(PaginationData::hidden()),
            searchbar: Some(SearchbarData {
                is_favourites: false,
                sort: "a-z".into(),
                term: "a-z".into(),
            }),
            share: Some(ShareData {
                is_from_host: true,
                is_shared: false,
            }),
            recipes: vec![view_recipe],
            ..Default::default()
        },
        &user_settings,
    ) {
        Ok(res) => Ok(res),
        Err(err) => {
            error!(
                "Error rendering view recipe page for user {} and recipe {recipe_id}: {err}",
                user.id
            );
            Err(Error::Templates)
        }
    }
}
