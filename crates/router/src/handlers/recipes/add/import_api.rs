use axum::{Form, extract::State, response::IntoResponse};
use futures_util::{StreamExt, pin_mut};
use reqwest::StatusCode;
use tokio::time::Instant;
use tracing::{error, warn};
use uuid::Uuid;

use app::state::AppState;
use integrations::api::Credentials;
use models::Error::{DuplicateEntity, DuplicateEntityWithID};
use models::{
    Recipe,
    reports::{
        report::{Items, ReportForCreate},
        report_log::ReportLogForCreate,
        report_types::{Import, PrimaryReportType, ReportTypeFull, TertiaryReportType},
    },
};

use crate::{
    Error,
    handlers::recipes::common::{broadcast_import_done_toast, schema_to_recipe_for_create},
    middleware::mw_auth::RequireAuth,
    recipes_router::params::ImportFromApiForm,
};

/// Handles the importing recipes from an API endpoint.
pub async fn add_recipe_import_api_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Form(form): Form<ImportFromApiForm>,
) -> impl IntoResponse {
    fetch_recipes_from_api(state, form, user.id);

    (StatusCode::ACCEPTED, "").into_response()
}

#[allow(clippy::too_many_lines)]
fn fetch_recipes_from_api(state: AppState, form: ImportFromApiForm, user_id: Uuid) {
    tokio::spawn(async move {
        let api = form.api.to_string();
        let state = state.clone();
        let start_time = Instant::now();

        state
            .broadcast_progress("Preparing import...", 0, 100, true, user_id)
            .await;

        let api_stream = form
            .api
            .fetch_recipes_stream(&form.url, Credentials::new(form.username, form.password));

        let db_stream = api_stream
            .map(|res| {
                let state = state.clone();

                async move {
                    match res {
                        Ok((id, recipe, num_recipes)) => {
                            let recipe_name = recipe.name.first().cloned().unwrap_or_default();

                            match push_recipe_in_db(&state, recipe, user_id).await {
                                Ok(recipe_id_db) => Ok((recipe_id_db, recipe_name, num_recipes)),
                                Err((name, err)) => {
                                    error!(
                                        "Failed to push recipe with id '{id}' to database: {err}",
                                    );
                                    Err((id, Some(name), num_recipes, Error::Database))
                                }
                            }
                        }
                        Err((id, num_recipes, err)) => {
                            error!("Failed to fetch recipe with id '{id}' from API: {err}");
                            Err((id, None, num_recipes, Error::FailFetch))
                        }
                    }
                }
            })
            .buffer_unordered(16);

        pin_mut!(db_stream);

        let mut total = -1;
        let mut processed = 0;
        let mut successes = Vec::new();
        let mut report_logs = Vec::new();

        let mut last_progress_time = Instant::now();
        let progress_interval = std::time::Duration::from_millis(500);
        let batch_size = 10;

        while let Some(res) = db_stream.next().await {
            let seq_num = i32::try_from(processed + 1).unwrap_or(1);
            let exec_time_ms = i64::try_from(start_time.elapsed().as_millis()).unwrap_or(0);

            match res {
                Ok((recipe_id, recipe_name, num_recipes)) => {
                    if total == -1 {
                        total = num_recipes;
                    }

                    report_logs.push(ReportLogForCreate::success(
                        seq_num,
                        &recipe_name,
                        Some(recipe_id),
                        exec_time_ms,
                    ));

                    successes.push(recipe_id);
                }
                Err((recipe_api_id, recipe_name, num_recipes, err)) => {
                    if total == -1 {
                        total = num_recipes;
                    }

                    match err {
                        Error::Model(DuplicateEntityWithID(id)) if recipe_name.is_some() => {
                            report_logs.push(ReportLogForCreate::warning(
                                seq_num,
                                &recipe_name.unwrap_or_default(),
                                Some(id),
                                "Recipe exists",
                                exec_time_ms,
                            ));
                        }
                        Error::Model(err) if recipe_name.is_some() => {
                            report_logs.push(ReportLogForCreate::error(
                                seq_num,
                                &recipe_name.unwrap_or_default(),
                                None,
                                "ImportApiModelFail",
                                &err.to_string(),
                                exec_time_ms,
                            ));
                        }
                        _ => {
                            report_logs.push(ReportLogForCreate::error(
                                seq_num,
                                &format!("{api} - id '{recipe_api_id}'"),
                                None,
                                "Fetch Failure",
                                &err.to_string(),
                                exec_time_ms,
                            ));
                        }
                    }
                }
            }

            processed += 1;

            if processed % batch_size == 0
                || last_progress_time.elapsed() >= progress_interval
                || processed == total
            {
                state
                    .broadcast_progress("Fetching recipes...", processed, total, true, user_id)
                    .await;
                last_progress_time = Instant::now();
            }
        }

        state
            .broadcast_progress("Fetching recipes...", processed, total, true, user_id)
            .await;

        let report_type = match form.api {
            integrations::api::Api::Mealie => ReportTypeFull::api(TertiaryReportType::mealie()),
            integrations::api::Api::Nextcloud => {
                ReportTypeFull::api(TertiaryReportType::nextcloud())
            }
            integrations::api::Api::Tandoor => ReportTypeFull::api(TertiaryReportType::tandoor()),
            integrations::api::Api::Unknown => ReportTypeFull {
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
        broadcast_import_done_toast(&state, successes, processed, report, api, user_id).await;
    });
}

async fn push_recipe_in_db(
    state: &AppState,
    recipe: schema_org::Recipe,
    user_id: Uuid,
) -> std::result::Result<i64, (String, Error)> {
    let recipe = schema_to_recipe_for_create(state, recipe).await;

    match Recipe::create(&state.mm, user_id, &recipe).await {
        Ok(recipe_id) => Ok(recipe_id),
        Err(DuplicateEntityWithID(id)) => {
            warn!("Recipe exists: '{}' with id '{id}'", recipe.name);
            Err((recipe.name, Error::Model(DuplicateEntity)))
        }
        Err(err) => {
            error!("Error saving recipe '{}': {err}", recipe.name);
            Err((recipe.name, Error::Model(err)))
        }
    }
}
