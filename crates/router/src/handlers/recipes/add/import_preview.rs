use std::sync::Arc;

use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse},
};
use config::States;
use reqwest::StatusCode;
use tracing::error;

use app::state::AppState;
use models::{
    RecipeDetails,
    data::{AboutData, Data, PaginationData, SearchbarData, ShareData, ViewRecipe},
    recipe::structs::recipe::RecipeForCreate,
    time::FormattedTimes,
};

use crate::{
    Error, Result, handlers::message::broadcast_error, middleware::mw_auth::RequireAuth,
    recipes_router::params::PreviewForm,
};

/// Handles generating a preview of the recipe based on the input JSON recipe schema.
pub async fn add_recipe_import_preview_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(form): Form<PreviewForm>,
) -> Result<impl IntoResponse> {
    match serde_json::from_str::<schema_org::Recipe>(&form.json_input) {
        Ok(schema) => {
            let recipe_c = RecipeForCreate::from(&schema);
            let recipe_details = RecipeDetails::from(recipe_c);
            let formatted_times = FormattedTimes::from_times(&recipe_details.times)?;
            let view_recipe = ViewRecipe {
                recipe_details,
                formatted_times,
            };

            match templates::recipes::view_recipe_helper(
                &Arc::clone(&state.fs_support),
                &state.data_dir,
                &Data {
                    states: States {
                        autologin: state.config.read().await.states.autologin,
                        ..Default::default()
                    },
                    is_admin: user.is_admin,
                    is_authenticated: true,
                    is_hx_request: true,
                    is_preview: true,
                    about: AboutData::default(),
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
            ) {
                Ok(res) => Ok(res.into_response()),
                Err(err) => {
                    error!(user = ?user.id, ?err, "Error rendering view recipe page preview");
                    broadcast_error(&state, user.id, "Error rendering recipe preview.").await;
                    Err(Error::Templates)
                }
            }
        }
        Err(err) => {
            error!(?err, "Error parsing recipe schema JSON");
            Ok((
                StatusCode::OK,
                Html(format!(
                    r#"<div class="text-error">Invalid JSON: {err}</div>"#
                )),
            )
                .into_response())
        }
    }
}
