use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use itertools::izip;
use serde::Deserialize;
use tracing::error;

use app::state::AppState;
use math::cooking::units::system;
use models::Error::EntityNotFound;
use models::Recipe;

use crate::{Error, handlers::message::broadcast_error, middleware::mw_auth::RequireAuth};

#[derive(Deserialize)]
pub struct YieldQueryParams {
    #[serde(rename = "yield")]
    pub yield_param: u16,
}

/// Handles scaling the recipe's yield.
pub async fn scale_recipe_handler(
    RequireAuth(user): RequireAuth,
    Path(recipe_id): Path<i64>,
    Query(params): Query<YieldQueryParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if params.yield_param == 0 {
        broadcast_error(&state, user.id, "Yield must be greater than zero.").await;
        return Error::InvalidQuery.into_response();
    }

    let recipe = match Recipe::get(&state.mm, user.id, recipe_id).await {
        Ok(mut recipe) => {
            let measurement_system =
                system::MeasurementSystem::from_id(recipe.recipe.measurement_system_id)
                    .unwrap_or_default();

            let factor = f64::from(params.yield_param) / f64::from(recipe.recipe.r#yield);
            let items = recipe.ingredients.items_as_text();
            let scaled = measurement_system.scale(
                items.iter().map(ToString::to_string).collect::<Vec<_>>(),
                factor,
            );

            for (x, y) in izip!(recipe.ingredients.iter_mut(), scaled) {
                x.text = y;
            }

            recipe
        }
        Err(err) => {
            error!(?recipe_id, user = ?user.id, ?err, "Error fetching recipe");
            broadcast_error(&state, user.id, "Recipe not found.").await;
            return Error::Model(EntityNotFound {
                id: recipe_id.to_string(),
                entity: "recipe",
            })
            .into_response();
        }
    };

    templates::recipes::render_ingredients_instructions(&recipe).into_response()
}
