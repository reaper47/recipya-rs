use axum::extract::{OriginalUri, Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use tracing::error;
use uuid::Uuid;

use crate::core::model::Error::EntityNotFound;
use crate::core::model::share::ShareRecipe;
use crate::server::Error;
use crate::server::router::handlers::helpers::is_hx_request;
use crate::server::router::middleware::mw_auth::CtxW;
use crate::server::templates::data::{AboutData, Data, FormattedTimes, ShareData, ViewRecipe};
use crate::server::{AppState, templates};

/// Renders the shared recipe.
pub async fn share_recipe_handler(
    ctx: CtxW,
    header_map: HeaderMap,
    OriginalUri(uri): OriginalUri,
    State(state): State<AppState>,
    Path(link): Path<Uuid>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let (share, recipe) = match ShareRecipe::get_by_link(&state.mm, link).await {
        Ok(share) => share,
        Err(err) => {
            error!("Error fetching shared recipe '{link}': {err}");
            return Error::Model(EntityNotFound {
                id: -1,
                entity: "shared recipe",
            })
            .into_response();
        }
    };

    let formatted_times = match FormattedTimes::from_times(&recipe.times) {
        Ok(formatted_times) => formatted_times,
        Err(err) => {
            error!("Failed to format recipe times: {err}");
            return err.into_response();
        }
    };

    templates::recipes::view_recipe(
        uri.path(),
        state.data_dir,
        Data {
            is_admin: user_id == 1,
            is_authenticated: user_id > 0,
            is_autologin: state.config.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            about: AboutData {
                is_update_available: false,
            },
            pagination: None,
            searchbar: None,
            share: Some(ShareData {
                is_shared: true,
                is_from_host: user_id == share.user_id,
            }),
            recipes: vec![ViewRecipe {
                recipe_details: recipe,
                formatted_times,
            }],
        },
    )
    .into_response()
}
