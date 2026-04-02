use axum::extract::State;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use tokio::fs;
use tokio::time::Instant;
use tracing::{error, warn};
use uuid::Uuid;

use app::state::AppState;
use models::Error::DuplicateEntityWithID;
use models::Recipe;
use models::reports::report::{Items, ReportForCreate};
use models::reports::report_log::ReportLogForCreate;
use models::reports::report_types::{
    Import, PrimaryReportType, ReportTypeFull, TertiaryReportType,
};

use crate::handlers::message::{broadcast_error, broadcast_warning};
use crate::handlers::recipes::common::{broadcast_import_done_toast, schema_to_recipe_for_create};
use crate::recipes_router::params::ImportFromAppForm;
use crate::{Error, Result, middleware::mw_auth::RequireAuth};

/// Handles the importing recipes from an application endpoint.
pub async fn add_recipe_import_app_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    form: ImportFromAppForm,
) -> impl IntoResponse {
    save_parsed_recipes(state, form, user.id);

    (StatusCode::ACCEPTED, "").into_response()
}

fn save_parsed_recipes(state: AppState, form: ImportFromAppForm, user_id: Uuid) {
    tokio::spawn(async move {
        let app = form.app.to_string();
        let state = state.clone();
        let start_time = Instant::now();

        let recipes = match parse_recipes(&state, form.clone(), user_id).await {
            Ok(r) => r,
            Err(Error::NoRecipe) => {
                state.hide_broadcast(user_id).await;
                broadcast_warning(&state, user_id, "No recipes found.").await;
                return;
            }
            Err(_) => {
                state.hide_broadcast(user_id).await;
                broadcast_error(
                    &state,
                    user_id,
                    "An error occurred while parsing the recipes. Please check the logs.",
                )
                .await;
                return;
            }
        };

        let num_recipes = recipes
            .len()
            .try_into()
            .inspect_err(|err| error!("Failed to cast recipes length '{}': {err}", recipes.len()))
            .unwrap_or(i64::MAX);

        let (recipe_ids, report_logs) =
            push_recipes_to_db(&state, recipes, &start_time, user_id).await;

        let report_type = match form.app {
            integrations::App::AccuChef => ReportTypeFull::app(TertiaryReportType::accuchef()),
            integrations::App::BigOven => ReportTypeFull::app(TertiaryReportType::bigoven()),
            integrations::App::ChefTap => ReportTypeFull::app(TertiaryReportType::cheftap()),
            integrations::App::Cooklang => ReportTypeFull::app(TertiaryReportType::cooklang()),
            integrations::App::CookMate => ReportTypeFull::app(TertiaryReportType::cookmate()),
            integrations::App::Crouton => ReportTypeFull::app(TertiaryReportType::crouton()),
            integrations::App::Kalorio => ReportTypeFull::app(TertiaryReportType::kalorio()),
            integrations::App::MasterCook => ReportTypeFull::app(TertiaryReportType::mastercook()),
            integrations::App::MealMaster => ReportTypeFull::app(TertiaryReportType::mealmaster()),
            integrations::App::Paprika => ReportTypeFull::app(TertiaryReportType::paprika()),
            integrations::App::RecipeMD => ReportTypeFull::app(TertiaryReportType::recipe_md()),
            integrations::App::RecipeSage => ReportTypeFull::app(TertiaryReportType::recipe_sage()),
            integrations::App::Rezkonv => ReportTypeFull::app(TertiaryReportType::rezkonv()),
            integrations::App::Saffron => ReportTypeFull::app(TertiaryReportType::saffron()),
            integrations::App::Unknown => ReportTypeFull {
                primary: PrimaryReportType::<Import>::new(),
                secondary: None,
                tertiary: None,
            },
        };

        let items = Items::from(report_logs.as_slice());
        let report = ReportForCreate::new(
            report_type,
            report_logs,
            items,
            i64::try_from(start_time.elapsed().as_millis()).unwrap_or_default(),
            user_id,
        );
        broadcast_import_done_toast(&state, recipe_ids, num_recipes, report, app, user_id).await;
    });
}

async fn parse_recipes(
    state: &AppState,
    form: ImportFromAppForm,
    user_id: Uuid,
) -> Result<Vec<schema_org::Recipe>> {
    state
        .broadcast_progress("Parsing recipes...", 1, 100, true, user_id)
        .await;

    let recipes = match form.parse_recipes() {
        Ok(r) => r,
        Err(err) => {
            error!("Failed to parse recipes: {err}");

            let saved_file =
                state
                    .data_dir
                    .debug
                    .join(format!("{}_{}", Uuid::new_v4(), form.file_name));
            fs::write(saved_file.clone(), &form.file_data).await?;
            error!("Saved file to '{:?}' for debugging purposes", saved_file);

            return Err(err);
        }
    };

    if recipes.is_empty() {
        warn!("No recipes found in file");
        return Err(Error::NoRecipe);
    }

    Ok(recipes)
}

async fn push_recipes_to_db(
    state: &AppState,
    recipes: Vec<schema_org::Recipe>,
    start_time: &Instant,
    user_id: Uuid,
) -> (Vec<i64>, Vec<ReportLogForCreate>) {
    let mut curr = 0;
    let mut recipe_ids = Vec::new();
    let mut report_logs = Vec::new();

    let num_recipes = recipes
        .len()
        .try_into()
        .inspect_err(|err| error!("Failed to cast recipes length '{}': {err}", recipes.len()))
        .unwrap_or(i64::MAX);

    for (idx, schema) in recipes.into_iter().enumerate() {
        let seq_num = i32::try_from(idx + 1).unwrap_or(1);

        curr += 1;
        state
            .broadcast_progress("Saving recipes", curr, num_recipes, true, user_id)
            .await;

        let recipe = schema_to_recipe_for_create(state, schema).await;
        let res_create = Recipe::create(&state.mm, user_id, &recipe).await;
        let exec_time_ms = i64::try_from(start_time.elapsed().as_millis()).unwrap_or(0);

        match res_create {
            Ok(recipe_id) => {
                report_logs.push(ReportLogForCreate::success(
                    seq_num,
                    &recipe.name,
                    Some(recipe_id),
                    exec_time_ms,
                ));
                recipe_ids.push(recipe_id);
            }
            Err(DuplicateEntityWithID(id)) => {
                warn!("Recipe exists: {}", recipe.name);
                report_logs.push(ReportLogForCreate::warning(
                    seq_num,
                    &recipe.name,
                    Some(id),
                    "Recipe exists",
                    exec_time_ms,
                ));
            }
            Err(err) => {
                error!("Error saving recipe '{}': {err}", recipe.name);
                report_logs.push(ReportLogForCreate::error(
                    seq_num,
                    &recipe.name,
                    None,
                    "ImportFail",
                    &err.to_string(),
                    exec_time_ms,
                ));
            }
        }
    }

    (recipe_ids, report_logs)
}
