use std::sync::Arc;
use std::sync::atomic::AtomicI64;

use axum::extract::State;
use axum::response::IntoResponse;
use base64::Engine;
use base64::engine::general_purpose;
use futures::StreamExt;
use reqwest::StatusCode;
use tokio::fs;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::Instant;
use tokio_util::bytes;
use tracing::{error, warn};
use uuid::Uuid;

use app::state::AppState;
use models::Error::DuplicateEntityWithID;
use models::Recipe;
use models::recipe::structs::recipe::RecipeForCreate;
use models::reports::report::{Items, ReportForCreate};
use models::reports::report_log::ReportLogForCreate;
use models::reports::report_types::{
    Import, PrimaryReportType, ReportTypeFull, TertiaryReportType,
};
use models::settings::UserSettingDetails;

use crate::handlers::message::{broadcast_error, broadcast_warning};
use crate::handlers::recipes::common::{broadcast_import_done_toast, schema_to_recipe_for_create};
use crate::recipes_router::params::ImportFromAppForm;
use crate::{Error, Result, middleware::mw_auth::RequireAuth};

struct RecipeResult {
    recipe_id: Option<i64>,
    log: ReportLogForCreate,
}

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
        let app = form.app.clone();
        let start_time = Arc::new(Instant::now());

        let mut recipes = match parse_recipes(&state, form, user_id).await {
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

        upload_images(&state, &mut recipes).await;

        let num_recipes = recipes
            .len()
            .try_into()
            .inspect_err(|err| error!(len = recipes.len(), ?err, "Failed to cast recipes length"))
            .unwrap_or(i64::MAX);

        let state = Arc::new(state);
        let (recipe_ids, report_logs) = push_recipes_to_db(
            Arc::clone(&state),
            recipes,
            Arc::clone(&start_time),
            user_id,
        )
        .await;

        let report_type = match app {
            integrations::App::AccuChef => ReportTypeFull::app(TertiaryReportType::accuchef()),
            integrations::App::BigOven => ReportTypeFull::app(TertiaryReportType::bigoven()),
            integrations::App::ChefTap => ReportTypeFull::app(TertiaryReportType::cheftap()),
            integrations::App::ComputerCuisineDeluxe => {
                ReportTypeFull::app(TertiaryReportType::computer_cuisine_deluxe())
            }
            integrations::App::CookBook => ReportTypeFull::app(TertiaryReportType::cookbook()),
            integrations::App::Cooklang => ReportTypeFull::app(TertiaryReportType::cooklang()),
            integrations::App::CookMate => ReportTypeFull::app(TertiaryReportType::cookmate()),
            integrations::App::Crouton => ReportTypeFull::app(TertiaryReportType::crouton()),
            integrations::App::Cookn => ReportTypeFull::app(TertiaryReportType::cookn()),
            integrations::App::CopyMeThat => ReportTypeFull::app(TertiaryReportType::copymethat()),
            integrations::App::HomeCookin => ReportTypeFull::app(TertiaryReportType::home_cookin()),
            integrations::App::Kalorio => ReportTypeFull::app(TertiaryReportType::kalorio()),
            integrations::App::LeCollectionneurDeRecettes => {
                ReportTypeFull::app(TertiaryReportType::lecollectionneurderecettes())
            }
            integrations::App::MasterCook => ReportTypeFull::app(TertiaryReportType::mastercook()),
            integrations::App::MealMaster => ReportTypeFull::app(TertiaryReportType::mealmaster()),
            integrations::App::MrCook => ReportTypeFull::app(TertiaryReportType::mrcook()),
            integrations::App::MyRecipeBox => {
                ReportTypeFull::app(TertiaryReportType::myrecipebox())
            }
            integrations::App::Paprika => ReportTypeFull::app(TertiaryReportType::paprika()),
            integrations::App::Pepperplate => {
                ReportTypeFull::app(TertiaryReportType::pepperplate())
            }
            integrations::App::RecipeKeeper => {
                ReportTypeFull::app(TertiaryReportType::recipekeeper())
            }
            integrations::App::RecipeMD => ReportTypeFull::app(TertiaryReportType::recipe_md()),
            integrations::App::RecipeSage => ReportTypeFull::app(TertiaryReportType::recipe_sage()),
            integrations::App::Recipya => ReportTypeFull::app(TertiaryReportType::recipya()),
            integrations::App::Rezkonv => ReportTypeFull::app(TertiaryReportType::rezkonv()),
            integrations::App::Saffron => ReportTypeFull::app(TertiaryReportType::saffron()),
            integrations::App::ShopNCook => ReportTypeFull::app(TertiaryReportType::shopncook()),
            integrations::App::Umami => ReportTypeFull::app(TertiaryReportType::umami()),
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
        broadcast_import_done_toast(
            &state,
            recipe_ids,
            num_recipes,
            report,
            app.to_string(),
            user_id,
        )
        .await;
    });
}

async fn upload_images(state: &AppState, recipes: &mut [schema_org::Recipe]) {
    let mut set = JoinSet::new();

    for (idx_recipe, recipe) in recipes.iter().enumerate() {
        for (idx_img, image) in recipe.image.iter().enumerate() {
            match image {
                schema_org::field::FieldEnum22::URL(s) if s.starts_with('/') => {
                    let cleaned_s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
                    match general_purpose::STANDARD.decode(cleaned_s.as_bytes()) {
                        Ok(bytes) => {
                            let fs_support = state.fs_support.clone();
                            set.spawn(async move {
                                (
                                    idx_recipe,
                                    idx_img,
                                    fs_support.upload_to_temp(bytes::Bytes::from(bytes)).await,
                                )
                            });
                        }
                        Err(err) => warn!("Failed to decode image bytes: {err}"),
                    }
                }
                _ => (),
            }
        }
    }

    while let Some(res) = set.join_next().await {
        if let Ok((idx_recipe, idx_image, Ok(path))) = res
            && let schema_org::field::FieldEnum22::URL(s) =
                &mut recipes[idx_recipe].image[idx_image]
        {
            *s = path.to_string_lossy().to_string();
        }
    }
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
            error!(?err, "Failed to parse recipes");

            let saved_file =
                state
                    .data_dir
                    .debug
                    .join(format!("{}_{}", Uuid::new_v4(), form.file_name));
            fs::write(&saved_file, &form.file_data).await?;
            error!(?saved_file, "Saved file to for debugging purposes");

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
    state: Arc<AppState>,
    recipes: Vec<schema_org::Recipe>,
    start_time: Arc<Instant>,
    user_id: Uuid,
) -> (Vec<i64>, Vec<ReportLogForCreate>) {
    let num_recipes_usize = recipes.len();
    let num_recipes = num_recipes_usize
        .try_into()
        .inspect_err(|err| error!(len = recipes.len(), ?err, "Failed to cast recipes length"))
        .unwrap_or(i64::MAX);
    let user_settings = Arc::new(
        UserSettingDetails::get(&state.mm, user_id)
            .await
            .unwrap_or_default(),
    );
    let semaphore = Arc::new(Semaphore::new(16));

    state
        .broadcast_progress("Saving media", 1, num_recipes, true, user_id)
        .await;

    let curr = Arc::new(AtomicI64::new(0));
    let mut set = JoinSet::new();
    for recipe in recipes {
        let curr = Arc::clone(&curr);
        let state = Arc::clone(&state);

        set.spawn(async move {
            let recipe_c = schema_to_recipe_for_create(state.as_ref(), recipe).await;

            let num_completed = curr.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            if num_completed % 12 == 0 || num_completed == num_recipes {
                state
                    .broadcast_progress("Saving media", num_completed, num_recipes, true, user_id)
                    .await;
            }
            recipe_c
        });
    }
    let recipes_for_create: Vec<RecipeForCreate> = set.join_all().await;

    let curr = Arc::new(AtomicI64::new(0));
    let results: Vec<_> = futures::stream::iter(recipes_for_create.into_iter().enumerate())
        .map(|(idx, schema)| {
            let state = Arc::clone(&state);
            let curr = Arc::clone(&curr);
            let start_time = Arc::clone(&start_time);
            let semaphore = Arc::clone(&semaphore);
            let user_settings = Arc::clone(&user_settings);

            async move {
                let _permit = semaphore.acquire().await.unwrap();
                push_recipe(
                    idx,
                    curr,
                    state,
                    schema,
                    num_recipes,
                    start_time,
                    user_id,
                    &user_settings,
                )
                .await
            }
        })
        .buffer_unordered(12)
        .collect()
        .await;

    let mut recipe_ids = Vec::with_capacity(num_recipes_usize);
    let mut report_logs = Vec::with_capacity(num_recipes_usize);

    for res in results {
        match res {
            Ok(result) => {
                if let Some(id) = result.recipe_id {
                    recipe_ids.push(id);
                }
                report_logs.push(result.log);
            }
            Err(err) => error!(?err, "push_recipe failed"),
        }
    }

    (recipe_ids, report_logs)
}

#[allow(clippy::too_many_arguments)]
async fn push_recipe(
    idx: usize,
    curr: Arc<AtomicI64>,
    state: Arc<AppState>,
    recipe_c: RecipeForCreate,
    num_recipes: i64,
    start_time: Arc<Instant>,
    user_id: Uuid,
    user_settings: &UserSettingDetails,
) -> Result<RecipeResult> {
    let seq_num = i32::try_from(idx + 1).unwrap_or(1);
    let exec_time_ms = i64::try_from(start_time.elapsed().as_millis()).unwrap_or(0);

    let (recipe_id, log) = match Recipe::create(&state.mm, user_id, &recipe_c, user_settings).await
    {
        Ok(recipe_id) => (
            Some(recipe_id),
            ReportLogForCreate::success(seq_num, &recipe_c.name, Some(recipe_id), exec_time_ms),
        ),
        Err(DuplicateEntityWithID(id)) => {
            warn!("Recipe exists: {}", recipe_c.name);
            (
                None,
                ReportLogForCreate::warning(
                    seq_num,
                    &recipe_c.name,
                    Some(id),
                    "Recipe exists",
                    exec_time_ms,
                ),
            )
        }
        Err(err) => {
            error!(name = recipe_c.name, ?err, "Error saving recipe");
            (
                None,
                ReportLogForCreate::error(
                    seq_num,
                    &recipe_c.name,
                    None,
                    "ImportFail",
                    &err.to_string(),
                    exec_time_ms,
                ),
            )
        }
    };

    let num_completed = curr.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    if num_completed % 12 == 0 || num_completed == num_recipes {
        state
            .broadcast_progress("Saving recipes", num_completed, num_recipes, true, user_id)
            .await;
    }

    Ok(RecipeResult { recipe_id, log })
}
