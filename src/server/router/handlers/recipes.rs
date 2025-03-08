use axum::extract::ws::Message;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, Uri};
use axum::response::IntoResponse;
use reqwest::StatusCode;
use tracing::error;

use crate::core::model::Recipe;
use crate::server::router::handlers::helpers::is_hx_request;
use crate::server::router::handlers::message::{IMessage, MessageHtmx};
use crate::server::router::middleware::mw_auth::CtxW;
use crate::server::router::SearchParams;
use crate::server::templates::data::{
    AboutData, Data, FormattedTimes, PaginationData, PaginationHtmxData, PaginationSearchData,
    SearchbarData, ShareData, ViewRecipe,
};
use crate::server::{templates, AppState};
use crate::server::{Error, Result};

/// Handles deleting a user's recipe.
pub async fn delete_recipe_handler(
    ctx: CtxW,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    match Recipe::delete(&state.mm, recipe_id, user_id).await {
        Ok(_) => (
            StatusCode::NO_CONTENT,
            [(axum_htmx::headers::HX_REDIRECT, "/")],
        )
            .into_response(),
        Err(err) => {
            error!("Error deleting recipe {recipe_id} for user {user_id}: {err}");

            let toast = MessageHtmx::error("Recipe could not be deleted.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json.into())).await;
            }

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handles viewing the recipes.
pub async fn recipes_handler(
    ctx: CtxW,
    headers: HeaderMap,
    Query(search_params): Query<SearchParams>,
    uri: Uri,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let num_recipes = match Recipe::count(&state.mm, user_id).await {
        Ok(count) => count,
        Err(err) => {
            error!("Error counting recipes for user {user_id}: {err}");

            let toast = MessageHtmx::error("Error fetching number of recipes.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state
                    .broadcast(ctx.0.user_id(), Message::Text(json.into()))
                    .await;
            }

            return Error::Database.into_response();
        }
    };

    let recipes = match Recipe::get_page(&state.mm, user_id, &search_params).await {
        Ok(recipes) => {
            let mapped_recipes: std::result::Result<Vec<ViewRecipe>, _> = recipes.into_iter().map(|recipe| {
                FormattedTimes::from_times(&recipe.times).map(|formatted_times| ViewRecipe {
                    recipe_details: recipe,
                    formatted_times,
                }).map_err(async |err| {
                    error!("Error formatting times for recipe: {err}");
                    let toast = MessageHtmx::error("Error formatting recipe times.");
                    if let Ok(json) = serde_json::to_string(&toast) {
                        state.broadcast(ctx.0.user_id(), Message::Text(json.into())).await;
                    }
                    Error::Database
                })
            }).collect();

            match mapped_recipes {
                Ok(mapped) => mapped,
                Err(_) => return Error::Database.into_response(),
            }
        }
        Err(err) => {
            error!("Error fetching recipes for user '{user_id}' with search params '{:?}': {err}", search_params);
            let toast = MessageHtmx::error("Error fetching recipes.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(ctx.0.user_id(), Message::Text(json.into())).await;
            }
            return Error::Database.into_response();
        }
    };

    templates::recipes::index(
        uri.path(),
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.is_autologin,
            is_hx_request: is_hx_request(headers),
            about: AboutData {
                is_update_available: false,
            },
            pagination: PaginationData::new_for_recipes(&search_params, num_recipes, false),
            searchbar: SearchbarData::from_params(search_params),
            share: ShareData {
                is_from_host: false,
                is_shared: false,
            },
            recipes,
        },
        state.data_dir,
    )
        .into_response()
}

/// Handles viewing a recipe.
pub async fn recipe_view_handler(
    ctx: CtxW,
    headers: HeaderMap,
    Path(recipe_id): Path<i64>,
    uri: Uri,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = ctx.0.user_id();

    let recipe = match Recipe::get(&state.mm, user_id, recipe_id).await {
        Ok(recipe) => recipe,
        Err(_) => {
            return Ok(templates::general::simple(
                "Recipe Not Found",
                "The recipe you requested to view is not found.",
            ));
        }
    };

    let formatted_times = FormattedTimes::from_times(&recipe.times)?;
    let view = vec![ViewRecipe {
        recipe_details: recipe,
        formatted_times,
    }];

    templates::recipes::view_recipe(
        uri.path(),
        state.data_dir,
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.is_autologin,
            is_hx_request: is_hx_request(headers),
            about: AboutData {
                is_update_available: false,
            },
            pagination: PaginationData {
                left: vec![],
                middle: vec![],
                right: vec![],
                prev: 0,
                selected: 0,
                next: 0,
                htmx: PaginationHtmxData {
                    is_swap: false,
                    target: "".to_string(),
                },
                search: PaginationSearchData { current_page: 0 },
                is_hidden: false,
                num_pages: 0,
                num_results: 0,
                results_per_page: 0,
                url: "".to_string(),
                url_queries: "".to_string(),
            },
            searchbar: SearchbarData {
                sort: String::from("a-z"),
                term: String::from("a-z"),
            },
            share: ShareData {
                is_from_host: true,
                is_shared: false,
            },
            recipes: view,
        },
    )
}
