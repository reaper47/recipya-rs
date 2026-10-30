use axum::{
    extract::{OriginalUri, Query, State},
    response::IntoResponse,
};
use axum_htmx::HxRequest;
use fluent_static::support::axum::RequestLanguage;
use iso8601::DateTime;
use tracing::error;

use app::{message::ToastCtx, state::AppState};
use config::States;
use l10n::Messages;
use models::{
    Recipe,
    data::{AboutData, Data, PaginationData, SearchbarData, ViewRecipe},
    params::SearchParams,
    time::FormattedTimes,
};
use support::states::AuthenticatedState;

use crate::{Error, Result, handlers::get_settings, middleware::mw_auth::RequireAuth};

/// Handles searching recipes.
pub async fn search_recipes_handler(
    HxRequest(is_hx_request): HxRequest,
    Query(params): Query<SearchParams>,
    OriginalUri(uri): OriginalUri,
    RequestLanguage(messages): RequestLanguage<Messages>,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let toast = ToastCtx::<AuthenticatedState>::new(&state, user.id, &messages);

    let recipes = match Recipe::get_page(&state.mm, user.id, &params).await {
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
                            error!(?err, "Error formatting times for recipe");
                            Error::Database
                        })
                })
                .collect();

            if let Ok(mapped) = mapped_recipes {
                mapped
            } else {
                toast
                    .error(&messages.toast_recipes_times_format_failed())
                    .await;
                return Err(Error::Database);
            }
        }
        Err(err) => {
            error!(user = ?user.id, ?params, ?err, "(search_recipes_handler) Error fetching recipes with search params");
            toast.error(&messages.toast_recipes_fetch_failed()).await;
            return Err(Error::Database);
        }
    };

    if recipes.is_empty() {
        let is_favourites = params.is_favourites.unwrap_or_default();

        return Ok(templates::search::no_results(is_favourites, &messages).into_response());
    }

    let settings = get_settings(&state, user.id, &messages).await?;

    Ok(templates::recipes::search_results(
        &state.fs_support,
        uri.path(),
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            states: States {
                autologin: state.config.read().await.states.autologin,
                ..Default::default()
            },
            is_hx_request,
            about: AboutData::new(false, false, DateTime::default(), DateTime::default()),
            pagination: Some(PaginationData::new_for_recipes(
                &params,
                recipes.len().try_into().unwrap_or(i64::MAX),
                is_hx_request,
            )),
            searchbar: Some(SearchbarData::from_params(params)),
            recipes,
            ..Default::default()
        },
        &state.data_dir,
        &settings,
        &messages,
    )
    .into_response())
}
