use std::fmt::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

use app::state::AppState;
use axum::Form;
use axum::extract::ws::Message;
use axum::extract::{OriginalUri, Path, Query, State};
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{Html, IntoResponse};
use chrono::NaiveDateTime;
use futures_util::future::join_all;
use math::cooking::units;
use models::Error::{DuplicateEntity, EntityNotFound};
use models::Recipe;
use models::data::{
    AboutData, Data, PaginationData, PaginationHtmxData, PaginationSearchData, SearchbarData,
    ShareData, ViewRecipe,
};
use models::params::SearchParams;
use models::recipe::{
    Category, Keyword, RecipeForCreate, RecipeForm, RecipeSearch, VideoForCreate,
};
use models::report::{ReportForCreate, ReportLogForCreate, ReportTypes};
use models::settings::UserSettingDetails;
use models::share::ShareRecipe;
use models::time::FormattedTimes;
use models::user::User;
use models::website::{ToHtmlTable, Website};
use recipe_schema::{ClipOrVideoObject, ImageObjectOrUrl, RecipeSchema, Sections};
use reqwest::StatusCode;
use serde::Deserialize;
use support::fs::FsSupport;
use tokio::fs;
use tokio::sync::{Mutex, mpsc};
use tokio::time::Instant;
use tracing::{error, info, warn};
use url::Url;
use uuid::Uuid;

use crate::handlers::get_settings;
use crate::handlers::helpers::is_hx_request;
use crate::handlers::message::{IMessage, MessageHtmx, MessageType, broadcast_error};
use crate::middleware::mw_auth::CtxW;
use crate::recipes_routes::{
    ImportFromAppForm, RecipeCategoryForm, RecipeScrapeForm, ShareRecipeForm,
};
use crate::{Error, Result};

/// Handles deleting a user's recipe.
pub async fn delete_recipe_handler(
    ctx: CtxW,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    match Recipe::delete(&state.mm, recipe_id, user_id).await {
        Ok(_) => {
            state.remove_cached_recipe((user_id, recipe_id)).await;
            (
                StatusCode::NO_CONTENT,
                [(axum_htmx::headers::HX_REDIRECT, "/")],
            )
                .into_response()
        }
        Err(err) => {
            error!("Error deleting recipe {recipe_id} for user {user_id}: {err}");
            broadcast_error(&state, user_id, "Recipe could not be deleted.").await;
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

/// Handles viewing the recipes.
pub async fn recipes_handler(
    ctx: CtxW,
    headers: HeaderMap,
    Query(search_params): Query<SearchParams>,
    OriginalUri(uri): OriginalUri,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = ctx.0.user_id();

    let settings = get_settings(&state, user_id).await?;

    let num_recipes = match Recipe::count(&state.mm, user_id).await {
        Ok(count) => count,
        Err(err) => {
            error!("Error counting recipes for user {user_id}: {err}");
            broadcast_error(&state, user_id, "Error fetching number of recipes.").await;
            return Ok(Error::Database.into_response());
        }
    };

    let recipes = match Recipe::get_page(&state.mm, user_id, &search_params).await {
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
                            error!("Error formatting times for recipe: {err}");
                            broadcast_error(&state, user_id, "Error formatting recipe times.")
                                .await;
                            Error::Database
                        })
                })
                .collect();

            match mapped_recipes {
                Ok(mapped) => mapped,
                Err(_) => return Err(Error::Database),
            }
        }
        Err(err) => {
            error!(
                "Error fetching recipes for user '{user_id}' with search params '{search_params:?}': {err}"
            );
            broadcast_error(&state, user_id, "Error fetching recipes.").await;
            return Err(Error::Database);
        }
    };

    Ok(templates::recipes::index(
        state.fs_support,
        uri.path(),
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&headers),
            // TODO: Populate AboutData with good values.
            about: AboutData {
                is_update_available: false,
                is_check_update: false,
                last_checked_update_at: Default::default(),
                last_updated_at: Default::default(),
                version: "".to_string(),
            },
            pagination: Some(PaginationData::new_for_recipes(
                &search_params,
                num_recipes,
                false,
            )),
            searchbar: Some(SearchbarData::from_params(search_params)),
            share: None,
            recipes,
        },
        state.data_dir,
        settings,
    )
    .into_response())
}

/// Handles the duplicate recipe endpoint.
pub async fn duplicate_recipe_handler(
    ctx: CtxW,
    Path(recipe_id): Path<i64>,
    header_map: HeaderMap,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = ctx.0.user_id();

    let settings = get_settings(&state, user_id).await?;

    let (mut recipe, categories, keywords) =
        match fetch_view_recipe(&state, user_id, recipe_id).await {
            Ok(res) => res,
            Err(err) => {
                error!("Error fetching view recipe '{recipe_id}' for user '{user_id}': {err}");
                broadcast_error(&state, user_id, "Recipe not found.").await;
                return Err(Error::Model(EntityNotFound {
                    id: recipe_id,
                    entity: "recipe",
                }));
            }
        };

    recipe.recipe_details.recipe.name = format!("{} (copy)", recipe.recipe_details.recipe.name);

    Ok(templates::recipes::add_recipe_manual(
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            recipes: vec![recipe],
            ..Default::default()
        },
        settings,
        categories,
        keywords,
    )
    .into_response())
}

/// Handles a recipe's edit page.
pub async fn edit_recipe_handler(
    ctx: CtxW,
    header_map: HeaderMap,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = ctx.0.user_id();

    let settings = get_settings(&state, user_id).await?;

    let (recipe, categories, keywords) = match fetch_view_recipe(&state, user_id, recipe_id).await {
        Ok(res) => res,
        Err(err) => {
            error!("Error fetching view recipe '{recipe_id}' for user '{user_id}': {err}");
            broadcast_error(&state, user_id, "Recipe not found.").await;
            return Err(Error::Model(EntityNotFound {
                id: recipe_id,
                entity: "recipe",
            }));
        }
    };

    let recipe_id = recipe.recipe_details.recipe.id;

    match templates::recipes::edit_recipe(
        state.fs_support,
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            recipes: vec![recipe],
            ..Default::default()
        },
        &state.data_dir,
        settings,
        categories,
        keywords,
    ) {
        Ok(res) => Ok(res),
        Err(err) => {
            error!(
                "Error rendering edit recipe page for user {user_id} and recipe {recipe_id}: {err}"
            );
            Err(Error::Templates)
        }
    }
}

async fn fetch_view_recipe(
    state: &AppState,
    user_id: i64,
    recipe_id: i64,
) -> Result<(ViewRecipe, Vec<Category>, Vec<Keyword>)> {
    let recipe = match Recipe::get(&state.mm, user_id, recipe_id).await {
        Ok(recipe) => recipe,
        Err(err) => {
            error!("Error fetching recipe '{recipe_id}' for user '{user_id}': {err}");
            broadcast_error(state, user_id, "Recipe not found.").await;
            return Err(Error::Model(EntityNotFound {
                id: recipe_id,
                entity: "recipe",
            }));
        }
    };

    let (categories, keywords) = match fetch_categories_keywords(state, user_id).await {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };

    let formatted_times = match FormattedTimes::from_times(&recipe.times) {
        Ok(formatted_times) => formatted_times,
        Err(err) => {
            error!("Failed to format times for recipe '{recipe_id}': {err}");
            return Err(Error::BadTimeFormat);
        }
    };

    Ok((
        ViewRecipe {
            recipe_details: recipe,
            formatted_times,
        },
        categories,
        keywords,
    ))
}

/// Handles updating a recipe.
pub async fn edit_recipe_put_handler(
    ctx: CtxW,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
    form: RecipeForm,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let images = form
        .images
        .iter()
        .map(
            |(original_file_stem, path)| match Uuid::parse_str(original_file_stem) {
                Ok(name)
                    if state
                        .fs_support
                        .is_file_exists(name, &state.data_dir.images) =>
                {
                    name
                }
                _ => {
                    let file_name = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned()
                        .parse::<Uuid>()
                        .unwrap_or_default();

                    state
                        .fs_support
                        .upload_image(path, file_name, &state.data_dir.images);
                    file_name
                }
            },
        )
        .collect::<Vec<_>>();

    let videos = if form.videos.is_empty() {
        Vec::new()
    } else {
        let videos = join_all(form.videos.iter().map(|(original_file_stem, path)| {
            let dir_videos = state.data_dir.videos.clone();
            let fs_support = Arc::clone(&state.fs_support);

            async move {
                match Uuid::parse_str(original_file_stem) {
                    Ok(name) if fs_support.is_file_exists(name, &dir_videos) => VideoForCreate {
                        video: name,
                        duration: fs_support
                            .calc_video_duration(path.to_str().unwrap_or_default())
                            .await
                            .ok(),
                        content_url: None,
                        embed_url: None,
                    },
                    _ => VideoForCreate::from_path(fs_support, path).await,
                }
            }
        }))
        .await;

        Arc::clone(&state.fs_support).upload_videos(
            form.videos.values().cloned().collect(),
            &state.data_dir.videos,
        );
        videos
    };

    let mut recipe_c = RecipeForCreate::from(form);
    recipe_c.images = images;
    recipe_c.videos = videos;

    match Recipe::update(&state.mm, user_id, recipe_id, &mut recipe_c).await {
        Ok(_) => {
            state.remove_cached_recipe((user_id, recipe_id)).await;
        }
        Err(err) => {
            error!("Failed to update recipe '{recipe_id}' user '{user_id}': {err}");
            broadcast_error(&state, user_id, "Failed to add recipe to collection.").await;
            return Error::Database.into_response();
        }
    };

    let mut res = (StatusCode::SEE_OTHER, "").into_response();
    if let Ok(value) = HeaderValue::from_str(&format!("/recipes/{recipe_id}")) {
        res.headers_mut()
            .insert(axum_htmx::headers::HX_REDIRECT, value);
    }
    res
}

#[derive(Deserialize)]
pub struct YieldQueryParams {
    #[serde(rename = "yield")]
    pub yield_param: u16,
}

/// Handles scaling the recipe's yield.
pub async fn scale_recipe_handler(
    ctx_w: CtxW,
    Path(recipe_id): Path<i64>,
    Query(params): Query<YieldQueryParams>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = ctx_w.0.user_id();

    if params.yield_param == 0 {
        broadcast_error(&state, user_id, "Yield must be greater than zero.").await;
        return Error::InvalidQuery.into_response();
    }

    let recipe = match Recipe::get(&state.mm, user_id, recipe_id).await {
        Ok(mut recipe) => {
            let measurement_system =
                units::MeasurementSystem::from_id(recipe.recipe.measurement_system_id)
                    .unwrap_or_default();

            let factor = params.yield_param as f64 / recipe.recipe.yield_ as f64;

            for (_name, ingredients) in recipe.ingredients.iter_mut() {
                *ingredients = measurement_system.scale(ingredients.clone(), factor);
            }

            recipe
        }
        Err(err) => {
            error!("Error fetching recipe '{recipe_id}' for user '{user_id}': {err}");
            broadcast_error(&state, user_id, "Recipe not found.").await;
            return Error::Model(EntityNotFound {
                id: recipe_id,
                entity: "recipe",
            })
            .into_response();
        }
    };

    templates::recipes::ingredients_instructions(&recipe).into_response()
}

/// Handles generating a link for the recipe to share.
pub async fn share_recipe_post_handler(
    ctx: CtxW,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
    Form(form): Form<ShareRecipeForm>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let expires_at: Option<NaiveDateTime> = form.datetime.and_then(|dt| {
        match NaiveDateTime::parse_from_str(dt.as_str(), "%Y-%m-%dT%H:%M") {
            Ok(parsed) => Some(parsed),
            Err(err) => {
                error!("Invalid datetime: {}", err);
                None
            }
        }
    });

    match ShareRecipe::new(&state.mm, user_id, recipe_id, expires_at).await {
        Ok(share) => {
            let url = format!(
                "{}/shared/r/{}",
                state.config.read().await.base_url,
                share.link
            );
            templates::general::share_link(&url).into_response()
        }
        Err(err) => {
            error!(
                "Error generating shared recipe link for recipe '{recipe_id}' and user '{user_id}': {err}"
            );
            broadcast_error(&state, user_id, "Error parsing datetime.").await;
            Error::BadTimeFormat.into_response()
        }
    }
}

/// Handles the add recipe page.
pub async fn add_recipes_handler(
    ctx: CtxW,
    header_map: HeaderMap,
    OriginalUri(uri): OriginalUri,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = ctx.0.user_id();

    let settings = get_settings(&state, user_id).await?;

    Ok(templates::recipes::add_page(
        uri.path(),
        Data {
            is_admin: ctx.0.user_id() == 1,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            ..Default::default()
        },
        settings,
    )
    .into_response())
}

/// Handles the import recipes endpoint.
pub async fn add_recipe_import_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    form: ImportFromAppForm,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();
    save_parsed_recipes(state, form, user_id);

    (StatusCode::ACCEPTED, "").into_response()
}

fn save_parsed_recipes(state: AppState, form: ImportFromAppForm, user_id: i64) {
    tokio::spawn(async move {
        let state = state.clone();
        let start_time = Instant::now();

        let recipes = match parse_recipes(&state, form, user_id).await {
            Ok(r) => r,
            Err(Error::NoRecipe) => {
                state.hide_broadcast(user_id).await;
                let toast = MessageHtmx::warning("No recipes found.");
                if let Ok(json) = serde_json::to_string(&toast) {
                    state.broadcast(user_id, Message::Text(json.into())).await;
                }
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

        let mut report = ReportForCreate::new(ReportTypes::Import, user_id);
        let mut curr = 0;
        let total_recipes = recipes.len() as i64;
        let mut recipe_ids = Vec::new();

        for schema in recipes {
            curr += 1;
            state
                .broadcast_progress("Saving recipes", curr, total_recipes, true, user_id)
                .await;

            let recipe = schema_to_recipe_for_create(&state, schema).await;

            match Recipe::create(&state.mm, user_id, &recipe).await {
                Ok(recipe_id) => {
                    report
                        .report_logs
                        .push(ReportLogForCreate::new_success(recipe.name));
                    recipe_ids.push(recipe_id);
                }
                Err(DuplicateEntity) => {
                    warn!("Recipe exists: {}", recipe.name);
                    report.report_logs.push(ReportLogForCreate::new_warning(
                        recipe.name,
                        "Recipe exists".into(),
                    ));
                }
                Err(err) => {
                    error!("Error saving recipe '{}': {err}", recipe.name);
                    report
                        .report_logs
                        .push(ReportLogForCreate::new_error(recipe.name, err.to_string()));
                }
            }
        }

        report.exec_time_ms = start_time.elapsed().as_millis() as i64;
        state.hide_broadcast(user_id).await;

        let num_success = recipe_ids.len() as i64;
        let num_skipped = total_recipes - num_success;

        info!(
            "Imported recipes: user_id={user_id}, success={num_success}, skipped={num_skipped}, total={total_recipes}"
        );

        let redirect = if num_success == 1 {
            format!("View /recipes/{}", recipe_ids.first().unwrap_or(&-1))
        } else {
            "View /reports?view=latest".to_string()
        };

        let toast = MessageHtmx::builder(
            MessageType::Toast,
            "Operation Successful",
            format!("Imported {num_success} recipes. Skipped {num_skipped}."),
        )
        .action(Some(&redirect))
        .build();

        if let Ok(json) = serde_json::to_string(&toast) {
            state.broadcast(user_id, Message::Text(json.into())).await;
        }

        if let Err(err) = report.insert(&state.mm).await {
            error!("Error inserting report into the database: {err}");
        }
    });
}

async fn parse_recipes(
    state: &AppState,
    mut form: ImportFromAppForm,
    user_id: i64,
) -> Result<Vec<RecipeSchema>> {
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

/// Handles rendering the form to add a recipe manually.
pub async fn add_manual_recipe_handler(
    ctx: CtxW,
    header_map: HeaderMap,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = ctx.0.user_id();

    let settings = get_settings(&state, user_id).await?;

    let (categories, keywords) = match fetch_categories_keywords(&state, user_id).await {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(templates::recipes::add_recipe_manual(
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            ..Default::default()
        },
        settings,
        categories,
        keywords,
    )
    .into_response())
}

/// Handles posting a submitted recipe form.
pub async fn add_manual_recipe_post_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    form: RecipeForm,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();
    let fs_support = Arc::clone(&state.fs_support);

    let images = form
        .images
        .into_values()
        .map(|path| {
            let file_name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
                .parse::<Uuid>()
                .unwrap_or_default();

            fs_support.upload_image(&path, file_name, &state.data_dir.images);
            file_name
        })
        .collect::<Vec<_>>();

    let videos = if form.videos.is_empty() {
        Vec::new()
    } else {
        let videos = join_all(
            form.videos
                .values()
                .map(|path| VideoForCreate::from_path(Arc::clone(&fs_support), path)),
        )
        .await;

        fs_support.upload_videos(
            form.videos.values().cloned().collect(),
            &state.data_dir.videos,
        );
        videos
    };

    let ingredients = form.ingredients;
    let measurement_system_id = units::MeasurementSystem::from(ingredients.clone()).id();

    let recipe_id = match Recipe::create(
        &state.mm,
        user_id,
        &RecipeForCreate {
            name: form.title,
            description: form.description,
            images,
            measurement_system_id,
            yield_: form.yield_,
            source: form.source,
            videos,
            category: form.category.or(Some("uncategorized".into())),
            cuisine: form.cuisine,
            ingredients: Sections::from([("".into(), ingredients)]),
            instructions: Sections::from([("".into(), form.instructions)]),
            keywords: form.keywords,
            nutrition: form.nutrition,
            times: form.times,
            tools: form.tools,
        },
    )
    .await
    {
        Ok(id) => id,
        Err(err) => {
            error!("Failed to add recipe to collection for user '{user_id}': {err}");
            broadcast_error(&state, user_id, "Failed to add recipe to collection.").await;
            return Error::Database.into_response();
        }
    };

    let mut res = (StatusCode::SEE_OTHER, "").into_response();
    if let Ok(value) = HeaderValue::from_str(&format!("/recipes/{recipe_id}")) {
        res.headers_mut()
            .insert(axum_htmx::headers::HX_REDIRECT, value);
    }
    res
}

/// Fetches the user's categories and keywords from the database.
pub async fn fetch_categories_keywords(
    state: &AppState,
    user_id: i64,
) -> Result<(Vec<Category>, Vec<Keyword>)> {
    let categories = match User::categories(&state.mm, user_id).await {
        Ok(categories) => categories,
        Err(err) => {
            error!("Error fetching recipe categories: {err}");
            broadcast_error(state, user_id, "Error fetching recipe categories.").await;
            return Err(Error::Database);
        }
    };

    let keywords = match User::keywords(&state.mm, user_id).await {
        Ok(keywords) => keywords,
        Err(err) => {
            error!("Error fetching recipe keywords: {err}");
            broadcast_error(state, user_id, "Error fetching recipe keywords.").await;
            return Err(Error::Database);
        }
    };

    Ok((categories, keywords))
}

#[derive(Clone)]
struct FetchWebsiteContext {
    count_success: Arc<AtomicI64>,
    count_warning: Arc<AtomicI64>,
    count_error: Arc<AtomicI64>,
    recipe_ids: Arc<Mutex<Vec<i64>>>,
    report: Arc<Mutex<ReportForCreate>>,
    started_at: Instant,
    total: i64,
}

impl FetchWebsiteContext {
    fn new(num_websites: usize, user_id: i64) -> Self {
        Self {
            count_success: Arc::new(Default::default()),
            count_warning: Arc::new(Default::default()),
            count_error: Arc::new(Default::default()),
            recipe_ids: Arc::new(Mutex::new(Vec::with_capacity(num_websites))),
            report: Arc::new(Mutex::new(ReportForCreate::new(
                ReportTypes::Import,
                user_id,
            ))),
            started_at: Instant::now(),
            total: num_websites as i64,
        }
    }

    async fn send_toast_after_processing(&self, state: &AppState, user_id: i64) {
        let count_success = self.count_success.load(Ordering::SeqCst);
        let count_error = self.count_error.load(Ordering::SeqCst);
        let count_warning = self.count_warning.load(Ordering::SeqCst);

        let toast = if self.total == 1 {
            if count_warning == 1 {
                let recipe_id = self.recipe_ids.lock().await.pop().unwrap();
                let view_recipe_link = format!("View /recipes/{recipe_id}");

                MessageHtmx::builder(
                    MessageType::Toast,
                    "Operation Warning",
                    "The recipe exists.",
                )
                .action(Some(&view_recipe_link))
                .build()
            } else if count_error == 1 {
                MessageHtmx::builder(
                    MessageType::Toast,
                    "Operation Failed",
                    "Fetching the recipe failed.",
                )
                .action(Some("View /reports?view=latest"))
                .build()
            } else if count_success == 1 {
                let recipe_id = self.recipe_ids.lock().await.pop().unwrap();
                let view_recipe_link = format!("View /recipes/{recipe_id}");

                MessageHtmx::builder(
                    MessageType::Toast,
                    "Operation Successful",
                    "Recipe has been added to your collection.",
                )
                .action(Some(&view_recipe_link))
                .build()
            } else {
                MessageHtmx::error("No recipe has been scraped.")
            }
        } else {
            let num_skipped = self.total - (count_success + count_warning);

            let message = format!("Fetched: {count_success}. Skipped: {num_skipped}");
            MessageHtmx::builder(MessageType::Toast, "Operation Successful", &message)
                .action(Some("View /reports?view=latest"))
                .build()
        };

        if let Ok(json) = serde_json::to_string(&toast) {
            state.broadcast(user_id, Message::Text(json.into())).await;
        }
    }
}

/// Handles scraping recipes from websites.
pub async fn add_website_post_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(form): Form<RecipeScrapeForm>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let mut urls = form
        .urls
        .lines()
        .filter_map(|line| Url::parse(line.trim_end_matches('/')).ok())
        .collect::<Vec<_>>();

    if urls.is_empty() {
        broadcast_error(&state, user_id, "No valid URLs found.").await;
        return Error::InvalidPayload.into_response();
    }
    urls.sort();
    urls.dedup();

    scrape_recipes(state, urls, user_id);

    (StatusCode::ACCEPTED, "").into_response()
}

fn scrape_recipes(state: AppState, urls: Vec<Url>, user_id: i64) {
    tokio::spawn(async move {
        let num_websites = urls.len();
        let (tx, mut rx) = mpsc::channel::<()>(num_websites);
        let fetch_ctx = FetchWebsiteContext::new(num_websites, user_id);
        let start_time = Instant::now();

        for url in urls {
            let tx = tx.clone();
            let fetch_ctx = fetch_ctx.clone();
            let state = state.clone();

            tokio::spawn(async move {
                match state.scrape(url.clone()) {
                    Ok(schema) => {
                        let recipe_c = schema_to_recipe_for_create(&state, schema).await;

                        match Recipe::create(&state.mm, user_id, &recipe_c).await {
                            Ok(recipe_id) => {
                                fetch_ctx.count_success.fetch_add(1, Ordering::SeqCst);
                                fetch_ctx.recipe_ids.lock().await.push(recipe_id);
                                fetch_ctx
                                    .report
                                    .lock()
                                    .await
                                    .report_logs
                                    .push(ReportLogForCreate::new_success(url.into()));
                            }
                            Err(err) => {
                                fetch_ctx.count_error.fetch_add(1, Ordering::SeqCst);
                                error!("Error inserting recipe into database '{url}': {err}");
                                if matches!(err, DuplicateEntity) {
                                    fetch_ctx.report.lock().await.report_logs.push(
                                        ReportLogForCreate::new_warning(
                                            url.into(),
                                            "Recipe exists".into(),
                                        ),
                                    );
                                } else {
                                    fetch_ctx.report.lock().await.report_logs.push(
                                        ReportLogForCreate::new_error(url.into(), err.to_string()),
                                    );
                                }
                            }
                        }
                    }
                    Err(err) => {
                        fetch_ctx.count_error.fetch_add(1, Ordering::SeqCst);
                        error!("Error fetching recipe '{url}': {err}");
                        fetch_ctx
                            .report
                            .lock()
                            .await
                            .report_logs
                            .push(ReportLogForCreate::new_error(url.into(), err.to_string()));
                    }
                }

                let _ = tx.send(()).await;
            });
        }
        drop(tx);

        let mut processed = 0;
        while rx.recv().await.is_some() {
            processed += 1;
            let title = format!("Fetched {processed}/{}", fetch_ctx.total);
            state
                .broadcast_progress(&title, processed, fetch_ctx.total, true, user_id)
                .await;
        }
        state.hide_broadcast(user_id).await;

        fetch_ctx.report.lock().await.exec_time_ms = start_time.elapsed().as_millis() as i64;
        if let Err(err) = fetch_ctx.report.lock().await.insert(&state.mm).await {
            error!("Error inserting report into the database: {err}");
        }

        fetch_ctx.send_toast_after_processing(&state, user_id).await;
    });
}

async fn schema_to_recipe_for_create(state: &AppState, schema: RecipeSchema) -> RecipeForCreate {
    let fs_support = state.fs_support.clone();

    let schema = Arc::new(schema);
    let mut recipe_c = RecipeForCreate::from(&*schema);

    recipe_c.images = extract_images(&schema, state, fs_support.clone()).await;
    recipe_c.videos = extract_videos(&schema, state, fs_support).await;

    recipe_c
}

async fn extract_images(
    schema: &Arc<RecipeSchema>,
    state: &AppState,
    fs_support: Arc<dyn FsSupport>,
) -> Vec<Uuid> {
    let mut vec = Vec::new();

    for images in schema.image.iter() {
        for image in images.iter() {
            let url = match image {
                ImageObjectOrUrl::Url(url) => Some(url),
                ImageObjectOrUrl::ImageObject(obj) => obj.url.as_ref().or(obj.content_url.as_ref()),
            };

            if let Some(url) = url {
                // TODO: Fetch the image using reqwest.
                let path = PathBuf::new();

                let file_name = Uuid::new_v4();
                fs_support.upload_image(&path, file_name, &state.data_dir.images);
                vec.push(
                    state
                        .fs_support
                        .is_file_exists(file_name, &state.data_dir.images)
                        .then_some(file_name),
                );
            }
        }
    }

    vec.into_iter()
        .map(|img| img.unwrap_or_default())
        .filter(|v| v != &Uuid::nil())
        .collect()
}

async fn extract_videos(
    schema: &Arc<RecipeSchema>,
    state: &AppState,
    fs_support: Arc<dyn FsSupport>,
) -> Vec<VideoForCreate> {
    let mut videos = Vec::new();

    for v in schema.video.iter() {
        for clip in v.iter() {
            match clip {
                ClipOrVideoObject::Clip(_) => {
                    warn!("ClipType not implemented");
                }
                ClipOrVideoObject::VideoObject(obj) => {
                    if let Ok(path) = state
                        .scraper
                        .fetch_and_upload_to_temp(obj.content_url.as_str())
                        .await
                    {
                        let file_name = Uuid::new_v4();
                        fs_support
                            .clone()
                            .upload_videos(vec![path.clone()], &state.data_dir.images);

                        if fs_support.is_file_exists(file_name, &state.data_dir.videos) {
                            videos.push(VideoForCreate {
                                video: file_name,
                                duration: fs_support
                                    .calc_video_duration(path.to_str().unwrap_or_default())
                                    .await
                                    .ok(),
                                content_url: schema
                                    .extract_video_content_urls()
                                    .map(|v| v.into_iter().map(String::from).collect()),
                                embed_url: schema
                                    .extract_video_embed_urls()
                                    .map(|v| v.into_iter().map(String::from).collect()),
                            });
                        }
                    }
                }
            }
        }
    }

    videos
}

/// Handles adding a recipe category into the database.
pub async fn post_recipe_categories_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(form): Form<RecipeCategoryForm>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let category = form.category;
    if category.is_empty() {
        return Error::InvalidPayload.into_response();
    }

    if let Err(err) = Recipe::add_category(&state.mm, &category, user_id).await {
        error!("Error adding recipe category: {err}");
        broadcast_error(&state, user_id, "Failed to add recipe category.").await;
        return Error::Database.into_response();
    }

    templates::settings::new_recipe_category(&category).into_response()
}

/// Handles deleting a recipe category from the database.
pub async fn delete_recipe_categories_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(form): Form<RecipeCategoryForm>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let category = form.category;
    if category.is_empty() || category == "uncategorized" {
        broadcast_error(
            &state,
            user_id,
            "Category cannot be empty or uncategorized.",
        )
        .await;
        return Error::InvalidPayload.into_response();
    }

    if let Err(err) = Recipe::delete_recipe_category(&state.mm, &category, user_id).await {
        error!("Error deleting recipe category: {err}");
        broadcast_error(&state, user_id, "Failed to delete recipe category.").await;
        return Error::Database.into_response();
    }

    (StatusCode::NO_CONTENT, "").into_response()
}

/// Handles viewing a recipe.
pub async fn view_recipe_handler(
    ctx: CtxW,
    header_map: HeaderMap,
    Path(recipe_id): Path<i64>,
    OriginalUri(uri): OriginalUri,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = ctx.0.user_id();

    let cache_key = (user_id, recipe_id);
    let view_recipe = match state.get_cached_recipe(cache_key).await {
        Some(recipe) => recipe,
        None => {
            let recipe = match Recipe::get(&state.mm, user_id, recipe_id).await {
                Ok(recipe) => recipe,
                Err(_) => {
                    return Ok(templates::general::simple(
                        "Recipe Not Found",
                        "The recipe you requested to view is not found.",
                    ));
                }
            };

            let formatted_times = FormattedTimes::from_times(&recipe.times)?;
            let view_recipe = ViewRecipe {
                recipe_details: recipe,
                formatted_times,
            };

            state.cache_recipe(cache_key, &view_recipe).await;
            view_recipe
        }
    };

    let user_settings = UserSettingDetails::get_settings(&state.mm, user_id).await?;

    match templates::recipes::view_recipe(
        state.fs_support,
        uri.path(),
        state.data_dir,
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            about: AboutData {
                is_update_available: false,
                is_check_update: false,
                last_checked_update_at: Default::default(),
                last_updated_at: Default::default(),
                version: "".to_string(),
            },
            pagination: Some(PaginationData {
                prev: 0,
                selected: 0,
                next: 0,
                htmx: PaginationHtmxData {
                    is_swap: false,
                    target: "".to_string(),
                },
                search: PaginationSearchData { current_page: 0 },
                slots: vec![],
                is_hidden: false,
                num_pages: 0,
                num_results: 0,
                results_per_page: 0,
                url: "".to_string(),
                url_queries: "".to_string(),
            }),
            searchbar: Some(SearchbarData {
                sort: "a-z".into(),
                term: "a-z".into(),
            }),
            share: Some(ShareData {
                is_from_host: true,
                is_shared: false,
            }),
            recipes: vec![view_recipe],
        },
        user_settings,
    ) {
        Ok(res) => Ok(res),
        Err(err) => {
            error!(
                "Error rendering view recipe page for user {user_id} and recipe {recipe_id}: {err}"
            );
            Err(Error::Templates)
        }
    }
}

/// Handles searching recipes.
pub async fn search_recipes_handler(
    ctx: CtxW,
    headers: HeaderMap,
    Query(search_params): Query<SearchParams>,
    OriginalUri(uri): OriginalUri,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = ctx.0.user_id();

    let recipes = match Recipe::get_page(&state.mm, user_id, &search_params).await {
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
                            error!("Error formatting times for recipe: {err}");
                            broadcast_error(&state, user_id, "Error formatting recipe times.")
                                .await;
                            Error::Database
                        })
                })
                .collect();

            match mapped_recipes {
                Ok(mapped) => mapped,
                Err(_) => return Err(Error::Database),
            }
        }
        Err(err) => {
            error!(
                "Error fetching recipes for user '{user_id}' with search params '{search_params:?}': {err}"
            );
            broadcast_error(&state, user_id, "Error fetching recipes.").await;
            return Err(Error::Database);
        }
    };

    if recipes.is_empty() {
        return Ok(templates::search::no_results().into_response());
    }

    let hx_target_value = match search_params.q.as_ref() {
        None => "#content",
        Some(s) if s.is_empty() => "#content",
        Some(_) => "#list-recipes",
    };

    let settings = get_settings(&state, user_id).await?;

    let body = templates::recipes::search_results(
        state.fs_support,
        uri.path(),
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&headers),
            // TODO: Populate AboutData with good values.
            about: AboutData {
                is_update_available: false,
                is_check_update: false,
                last_checked_update_at: Default::default(),
                last_updated_at: Default::default(),
                version: "".to_string(),
            },
            pagination: Some(PaginationData::new_for_recipes(
                &search_params,
                recipes.len() as i64,
                false,
            )),
            searchbar: Some(SearchbarData::from_params(search_params)),
            share: None,
            recipes,
        },
        state.data_dir,
        settings,
    );

    let mut extra_headers = HeaderMap::new();
    extra_headers.insert(
        axum_htmx::HX_RETARGET,
        HeaderValue::from_static(hx_target_value),
    );

    Ok((extra_headers, body).into_response())
}

/// Handles the supported applications endpoint.
pub async fn supported_applications_handler(_ctx: CtxW) -> impl IntoResponse {
    let applications = [
        ("AccuChef", "https://www.accuchef.com", vec![]),
        ("BigOven", "https://www.bigoven.com", vec![".txt"]),
        ("ChefTap", "https://cheftap.com", vec![".txt"]),
        ("Cooklang", "https://cooklang.org/", vec![".cook"]),
        (
            "COOKmate",
            "https://cooklang.org/",
            vec![".mcb", ".mmf", ".rk", ".xml"],
        ),
        ("Crouton", "https://crouton.app", vec![".crumb"]),
        (
            "Easy Recipe Deluxe",
            "https://easy-recipe-deluxe.software.informer.com",
            vec![],
        ),
        ("Kalorio", "https://www.kalorio.de", vec![".txt", ".xml"]),
        (
            "MasterCook",
            "https://www.mastercook.com",
            vec![".mx2", ".mxp", ".mz2", ".txt"],
        ),
        (
            "Meal-Master",
            "https://web.archive.org/web/20081221021301/http://episoft.home.comcast.net/~episoft/mmdown.htm",
            vec![".mx2", ".mxp", ".mz2", ".txt"],
        ),
        (
            "Paprika",
            "https://www.paprikaapp.com",
            vec![".paprikarecipes"],
        ),
        ("Recipe Keeper", "https://recipekeeperonline.com", vec![]),
        ("RecipeMD", "https://recipemd.org/", vec![".md"]),
        (
            "RecipeSage",
            "https://recipesage.com",
            vec![".json", ".txt", ".xml"],
        ),
        ("Rezkonv", "https://www.rezkonv.de/", vec![".rk"]),
        ("Saffron", "https://www.mysaffronapp.com", vec![".txt"]),
    ];

    let mut html = String::new();

    for (i, (name, url, formats)) in applications.into_iter().enumerate() {
        html.push_str(r#"<tr class="text-center">"#);
        let _ = write!(html, "<td>{}</td>", i + 1);
        let _ = write!(
            html,
            r#"<td><a class="underline" href="{url}" target="_blank">{name}</a></td>"#
        );
        let _ = write!(html, "<td>{}</td>", formats.join(", "));
        html.push_str("</tr>");
    }

    Html(html)
}

/// Handles the supported websites endpoint.
pub async fn supported_websites_handler(
    ctx: CtxW,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match Website::supported_websites(&state.mm).await {
        Ok(websites) => Html(websites.to_html_table_rows()).into_response(),
        Err(err) => {
            error!("Error fetching supported websites: {err}");
            let user_id = ctx.0.user_id();
            broadcast_error(&state, user_id, "Error fetching supported websites.").await;
            Error::Database.into_response()
        }
    }
}
