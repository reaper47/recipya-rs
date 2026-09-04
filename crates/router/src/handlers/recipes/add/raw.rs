use axum::{Form, extract::State, http::HeaderValue, response::IntoResponse};
use axum_htmx::HX_REDIRECT;
use reqwest::StatusCode;
use tokio::time::Instant;
use tracing::{error, warn};

use app::state::AppState;
use models::{
    Recipe,
    recipe::structs::recipe::RecipeForCreate,
    reports::{
        report::{Items, ReportForCreate},
        report_log::ReportLogForCreate,
        report_types::{ReportTypeFull, TertiaryReportType},
    },
    settings::UserSettingDetails,
};

use crate::{
    Error, handlers::message::broadcast_error, middleware::mw_auth::RequireAuth,
    recipes_router::params::PreviewForm,
};

/// Handles parsing a recipe from raw JSON.
pub async fn add_recipe_import_raw_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(form): Form<PreviewForm>,
) -> impl IntoResponse {
    let start_time = Instant::now();
    let mut items = Items::default();

    let (res, report_log) = match serde_json::from_str::<schema_org::Recipe>(&form.json_input) {
        Ok(schema) => {
            let user_settings = UserSettingDetails::get(&state.mm, user.id)
                .await
                .unwrap_or_default();
            let recipe_c = RecipeForCreate::from(&schema);
            let create_res = Recipe::create(&state.mm, user.id, &recipe_c, &user_settings).await;
            let exec_time_ms = i64::try_from(start_time.elapsed().as_millis()).unwrap_or_default();

            match create_res {
                Ok(recipe_id) => {
                    let url = format!("/recipes/{recipe_id}");
                    items.success += 1;

                    (
                        HeaderValue::from_str(&url).map_or_else(
                            |_| (StatusCode::BAD_REQUEST, "invalid redirect url").into_response(),
                            |hv| (StatusCode::OK, [(HX_REDIRECT, hv)]).into_response(),
                        ),
                        ReportLogForCreate::success(
                            1,
                            &recipe_c.name,
                            Some(recipe_id),
                            exec_time_ms,
                        ),
                    )
                }
                Err(models::Error::DuplicateEntityWithID(id)) => {
                    items.skipped += 1;
                    warn!("Recipe exists: {}", recipe_c.name);
                    broadcast_error(&state, user.id, "Recipe exists.").await;
                    (
                        Error::EntityExists { entity: "recipe" }.into_response(),
                        ReportLogForCreate::warning(
                            1,
                            &recipe_c.name,
                            Some(id),
                            "Recipe exists.",
                            exec_time_ms,
                        ),
                    )
                }
                Err(err) => {
                    items.failed += 1;
                    error!(name = recipe_c.name, ?err, "Error saving recipe");
                    broadcast_error(&state, user.id, "Failed to insert recipe.").await;
                    (
                        Error::Database.into_response(),
                        ReportLogForCreate::error(
                            1,
                            &recipe_c.name,
                            None,
                            "ImportRawFail",
                            "Failed to insert in database.",
                            exec_time_ms,
                        ),
                    )
                }
            }
        }
        Err(err) => {
            items.failed += 1;
            error!(?err, "Error parsing recipe schema JSON");
            broadcast_error(&state, user.id, "Error parsing recipe schema JSON.").await;
            (
                Error::InvalidPayload.into_response(),
                ReportLogForCreate::error(
                    1,
                    "Raw JSON",
                    None,
                    "ImportRawFail",
                    "Failed to create a recipe from the JSON.",
                    i64::try_from(start_time.elapsed().as_millis()).unwrap_or_default(),
                ),
            )
        }
    };

    let report = ReportForCreate::new(
        ReportTypeFull::raw(TertiaryReportType::json()),
        vec![report_log],
        items,
        i64::try_from(start_time.elapsed().as_millis()).unwrap_or_default(),
        user.id,
    );

    match report.insert(&state.mm).await {
        Ok(_) => state.broadcast_trigger("refreshReports", user.id).await,
        Err(err) => {
            error!(?err, "Error inserting website report into the database");
        }
    }

    res
}
