use axum::extract::{OriginalUri, Path, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use iso8601::DateTime;
use tracing::error;
use uuid::Uuid;

use app::state::AppState;
use models::Error::EntityNotFound;
use models::data::{AboutData, Data, ShareData, ViewRecipe};
use models::settings::UserSettingDetails;
use models::share::ShareRecipe;
use models::time::FormattedTimes;

use crate::Error;
use crate::handlers::helpers::is_hx_request;
use crate::middleware::mw_auth::OptionalAuth;

/// Renders the shared recipe.
pub async fn share_recipe_handler(
    header_map: HeaderMap,
    OriginalUri(uri): OriginalUri,
    State(state): State<AppState>,
    OptionalAuth(user): OptionalAuth,
    Path(link): Path<Uuid>,
) -> impl IntoResponse {
    let (share, recipe) = match ShareRecipe::get_by_link(&state.mm, link).await {
        Ok(share) => share,
        Err(err) => {
            error!("Error fetching shared recipe '{link}': {err}");
            return Error::Model(EntityNotFound {
                id: "-1".into(),
                entity: "shared recipe",
            })
            .into_response();
        }
    };

    let formatted_times = match FormattedTimes::from_times(&recipe.times) {
        Ok(formatted_times) => formatted_times,
        Err(err) => {
            error!("Failed to format recipe times: {err}");
            return Error::BadTimeFormat.into_response();
        }
    };

    let template = match user {
        Some(user) => {
            let settings = UserSettingDetails::get(&state.mm, user.id)
                .await
                .unwrap_or_default();

            templates::recipes::view_recipe(
                &state.fs_support,
                uri.path(),
                &state.data_dir,
                &Data {
                    is_admin: user.is_admin,
                    is_authenticated: true,
                    is_autologin: state.config.read().await.is_autologin,
                    is_hx_request: is_hx_request(&header_map),
                    // TODO: Populate AboutData with good values.
                    is_preview: false,
                    about: AboutData::new(false, false, DateTime::default(), DateTime::default()),
                    pagination: None,
                    searchbar: None,
                    share: Some(ShareData {
                        is_shared: true,
                        is_from_host: user.id == share.user_id,
                    }),
                    recipes: vec![ViewRecipe {
                        recipe_details: recipe,
                        formatted_times,
                    }],
                    reports: None,
                },
                &settings,
            )
        }
        None => templates::recipes::view_recipe(
            &state.fs_support,
            uri.path(),
            &state.data_dir,
            &Data {
                is_admin: false,
                is_authenticated: true,
                is_autologin: state.config.read().await.is_autologin,
                is_hx_request: is_hx_request(&header_map),
                // TODO: Populate AboutData with good values.
                is_preview: false,
                about: AboutData {
                    is_update_available: false,
                    is_check_update: false,
                    last_checked_update_at: DateTime::default(),
                    last_updated_at: DateTime::default(),
                },
                pagination: None,
                searchbar: None,
                share: Some(ShareData {
                    is_shared: true,
                    is_from_host: false,
                }),
                recipes: vec![ViewRecipe {
                    recipe_details: recipe,
                    formatted_times,
                }],
                reports: None,
            },
            &UserSettingDetails::default(),
        ),
    };

    match template {
        Ok(res) => res.into_response(),
        Err(err) => {
            error!("Error rendering shared view recipe page '{link}': {err}");
            Error::Templates.into_response()
        }
    }
}
