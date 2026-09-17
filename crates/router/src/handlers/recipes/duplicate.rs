use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use axum_htmx::HxRequest;
use config::States;
use tracing::error;

use app::state::AppState;
use models::Error::EntityNotFound;
use models::data::Data;

use crate::{
    Error, Result,
    handlers::{get_settings, message::broadcast_error, recipes::common::fetch_view_recipe},
    middleware::mw_auth::RequireAuth,
};

/// Handles the duplicate recipe endpoint.
pub async fn duplicate_recipe_handler(
    Path(recipe_id): Path<i64>,
    HxRequest(is_hx_request): HxRequest,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let settings = get_settings(&state, user.id).await?;

    let (mut recipe, categories, keywords) =
        match fetch_view_recipe(&state, user.id, recipe_id).await {
            Ok(res) => res,
            Err(err) => {
                error!(?recipe_id, user = ?user.id, ?err, "Error fetching view recipe");
                broadcast_error(&state, user.id, "Recipe not found.").await;
                return Err(Error::Model(EntityNotFound {
                    id: recipe_id.to_string(),
                    entity: "recipe",
                }));
            }
        };

    recipe.recipe_details.recipe.name = format!("{} (copy)", recipe.recipe_details.recipe.name);

    Ok(templates::recipes::add_recipe_manual(
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            is_hx_request,
            recipes: vec![recipe],
            states: States {
                autologin: state.config.read().await.states.autologin,
                ..Default::default()
            },
            ..Default::default()
        },
        &settings,
        categories,
        keywords,
    )
    .into_response())
}
