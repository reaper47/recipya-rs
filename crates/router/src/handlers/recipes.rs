use std::collections::HashMap;
use std::fmt::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

use axum::Form;
use axum::extract::ws::Message;
use axum::extract::{OriginalUri, Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, Uri};
use axum::response::{Html, IntoResponse};
use axum_htmx::HX_REDIRECT;
use chrono::NaiveDateTime;
use futures_util::future::join_all;
use futures_util::pin_mut;
use futures_util::stream::{self, StreamExt};
use integrations::api::Credentials;
use itertools::izip;
use reqwest::StatusCode;
use serde::Deserialize;
use tokio::fs;
use tokio::sync::{Mutex, mpsc};
use tokio::time::Instant;
use tracing::{error, info, warn};
use url::Url;
use uuid::Uuid;

use app::state::AppState;
use config::DataDir;
use math::cooking::units::system;
use models::Error::{DuplicateEntity, EntityNotFound};
use models::data::{AboutData, Data, PaginationData, SearchbarData, ShareData, ViewRecipe};
use models::params::SearchParams;
use models::recipe::RecipeForm;
use models::recipe::structs::media::VideoForCreate;
use models::recipe::structs::recipe::{Category, Keyword, RecipeForCreate};
use models::recipe::structs::section::{Item, SectionComponents};
use models::recipe::timeline::{RecipeTimeline, RecipeTimelineForCreate};
use models::report::{ReportForCreate, ReportLogForCreate, ReportTypes};
use models::settings::UserSettingDetails;
use models::share::ShareRecipe;
use models::time::FormattedTimes;
use models::user::User;
use models::website::{ToHtmlTable, Website};
use models::{Recipe, RecipeDetails};
use support::fs::FsSupport;
use templates::recipes::timeline::Event;

use crate::handlers::get_settings;
use crate::handlers::helpers::is_hx_request;
use crate::handlers::message::{
    IMessage, MessageHtmx, MessageType, broadcast_error, broadcast_success, broadcast_warning,
};
use crate::middleware::mw_auth::CtxW;
use crate::recipes_router::params::{
    FavouriteParams, ImportFromApiForm, ImportFromAppForm, OrderParams, PreviewForm,
    RecipeCategoryForm, RecipeScrapeForm, ShareRecipeForm, TimelineEventForm,
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
            (StatusCode::NO_CONTENT, [(HX_REDIRECT, "/")]).into_response()
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
            is_preview: false,
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
    let fs_support = Arc::clone(&state.fs_support);

    let mut recipe_c = RecipeForCreate::from(&form);
    recipe_c.images = form
        .images
        .into_iter()
        .map(
            |(original_file_stem, path)| match Uuid::parse_str(&original_file_stem) {
                Ok(name)
                    if fs_support.is_file_exists(name, &state.data_dir.images.root, ".webp") =>
                {
                    name
                }
                Ok(_) | Err(_) => {
                    let file_name = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned()
                        .parse::<Uuid>()
                        .unwrap_or_default();

                    fs_support.upload_image(&path, file_name, &state.data_dir.images.root);

                    let thumbnails_dir = state.data_dir.images.thumbnails.clone();
                    let fs_support = Arc::clone(&state.fs_support);
                    tokio::spawn(async move {
                        fs_support.generate_thumbnail(&path, file_name, &thumbnails_dir);
                    });

                    file_name
                }
            },
        )
        .collect::<Vec<_>>();

    recipe_c.videos = if form.videos.is_empty() {
        Vec::new()
    } else {
        let videos = join_all(form.videos.iter().map(|(original_file_stem, path)| {
            let dir_videos = state.data_dir.videos.clone();
            let fs_support = Arc::clone(&state.fs_support);

            async move {
                match Uuid::parse_str(original_file_stem) {
                    Ok(name) if fs_support.is_file_exists(name, &dir_videos, ".webm") => {
                        VideoForCreate {
                            video: name,
                            duration: fs_support
                                .calc_video_duration(path.to_str().unwrap_or_default())
                                .await
                                .ok(),
                            content_url: None,
                            embed_url: None,
                        }
                    }
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
        res.headers_mut().insert(HX_REDIRECT, value);
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
                system::MeasurementSystem::from_id(recipe.recipe.measurement_system_id)
                    .unwrap_or_default();

            let factor = params.yield_param as f64 / recipe.recipe.yield_ as f64;
            let items = recipe.ingredients.items_as_text();
            let scaled = measurement_system.scale(items, factor);

            for (x, y) in izip!(recipe.ingredients.iter_mut(), scaled) {
                x.text = y;
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

    templates::recipes::render_ingredients_instructions(&recipe).into_response()
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

/// Handles getting a recipe's timeline.
pub async fn timeline_get_handler(
    ctx: CtxW,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let recipe = match Recipe::get_recipe_only(&state.mm, user_id, recipe_id).await {
        Ok(recipe) => recipe,
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

    let events = match RecipeTimeline::all(&state.mm, recipe_id, user_id).await {
        Ok(components) => components.into_iter().map(Event::from).collect::<Vec<_>>(),
        Err(err) => {
            error!(
                "Error fetching timeline components for recipe '{recipe_id}' of user '{user_id}': {err}"
            );
            broadcast_error(&state, user_id, "Failed to fetch timeline components.").await;
            return Error::Model(EntityNotFound {
                id: recipe_id,
                entity: "timeline",
            })
            .into_response();
        }
    };

    let static_events = vec![Event {
        date: recipe.created_at.date().format("%x").to_string(),
        title: "Recipe created".into(),
        ..Default::default()
    }];

    let mut all_events = static_events;
    all_events.extend(events);

    templates::recipes::timeline::render_events(recipe_id, all_events).into_response()
}

/// Handles adding a timeline component to the recipe.
pub async fn timeline_post_handler(
    ctx: CtxW,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
    form: TimelineEventForm,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    if let Err(err) = RecipeTimeline::create(
        &state.mm,
        recipe_id,
        user_id,
        &RecipeTimelineForCreate {
            title: form.title,
            comment: form.comment,
            rating: form.rating,
            image: upload_image(form.image, Arc::clone(&state.fs_support), &state.data_dir).await,
            created_at: form.date,
        },
    )
    .await
    {
        error!(
            "Error creating timeline event for recipe with id '{recipe_id}' and user '{user_id}': {err}"
        );
        broadcast_error(&state, user_id, "Could not create timeline event.").await;
        return Error::Database.into_response();
    }

    broadcast_success(&state, user_id, "Timeline event created.").await;
    (StatusCode::CREATED, "").into_response()
}

async fn upload_image(
    map: HashMap<String, PathBuf>,
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    data_dir: &DataDir,
) -> Option<Uuid> {
    map.into_iter()
        .map(
            |(original_file_stem, path)| match Uuid::parse_str(&original_file_stem) {
                Ok(name) if fs_support.is_file_exists(name, &data_dir.images.timeline, ".webp") => {
                    name
                }
                Ok(_) | Err(_) => {
                    let file_name = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned()
                        .parse::<Uuid>()
                        .unwrap_or_default();

                    fs_support.upload_image(&path, file_name, &data_dir.images.timeline);
                    file_name
                }
            },
        )
        .collect::<Vec<_>>()
        .first()
        .cloned()
}

/// Handles getting a timeline event for view.
pub async fn timeline_event_get_handler(
    ctx: CtxW,
    Path((recipe_id, timeline_id)): Path<(i64, i64)>,
    State(state): State<AppState>,
    Query(q): Query<OrderParams>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let event = match RecipeTimeline::get(&state.mm, timeline_id, recipe_id, user_id).await {
        Ok(t) => Event::from(t),
        Err(EntityNotFound { entity, .. }) => {
            broadcast_warning(&state, user_id, "Timeline event does not exist.").await;
            return Error::EntityNotFound { entity }.into_response();
        }
        Err(err) => {
            error!(
                "Error fetching timeline event with id '{timeline_id}' for recipe with id '{recipe_id}' and user '{user_id}': {err}"
            );
            broadcast_error(&state, user_id, "Could not fetch timeline event.").await;
            return Error::Database.into_response();
        }
    };

    templates::recipes::timeline::render_event(&event, q.index, q.max_index, recipe_id)
        .into_response()
}

/// Handles getting a timeline event for edit.
pub async fn timeline_event_get_edit_handler(
    ctx: CtxW,
    Path((recipe_id, timeline_id)): Path<(i64, i64)>,
    State(state): State<AppState>,
    Query(q): Query<OrderParams>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let event = match RecipeTimeline::get(&state.mm, timeline_id, recipe_id, user_id).await {
        Ok(t) => t,
        Err(EntityNotFound { entity, .. }) => {
            broadcast_warning(&state, user_id, "Timeline event does not exist.").await;
            return Error::EntityNotFound { entity }.into_response();
        }
        Err(err) => {
            error!(
                "Error fetching timeline event with id '{timeline_id}' for recipe with id '{recipe_id}' and user '{user_id}': {err}"
            );
            broadcast_error(&state, user_id, "Could not fetch timeline event.").await;
            return Error::Database.into_response();
        }
    };

    templates::recipes::timeline::render_edit(event, q.index, q.max_index, recipe_id)
        .into_response()
}

/// Handles updating a timeline component of the recipe.
pub async fn timeline_put_handler(
    ctx: CtxW,
    Path((recipe_id, timeline_id)): Path<(i64, i64)>,
    State(state): State<AppState>,
    form: TimelineEventForm,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let original_event = match RecipeTimeline::get(&state.mm, timeline_id, recipe_id, user_id).await
    {
        Ok(t) => t,
        Err(EntityNotFound { entity, .. }) => {
            broadcast_warning(&state, user_id, "Timeline event does not exist.").await;
            return Error::EntityNotFound { entity }.into_response();
        }
        Err(err) => {
            error!(
                "Error fetching timeline event with id '{timeline_id}' for recipe with id '{recipe_id}' and user '{user_id}': {err}"
            );
            broadcast_error(&state, user_id, "Could not fetch timeline event.").await;
            return Error::Database.into_response();
        }
    };

    let image = if let Some(img) = form.original_image_filename
        && state
            .fs_support
            .is_file_exists(img, &state.data_dir.images.timeline, ".webp")
    {
        Some(img)
    } else {
        upload_image(form.image, Arc::clone(&state.fs_support), &state.data_dir).await
    };

    let new_event_params = RecipeTimeline {
        id: original_event.id,
        recipe_id: original_event.recipe_id,
        user_id: original_event.user_id,
        title: form.title,
        comment: form.comment,
        rating: form.rating,
        image,
        created_at: form.date.unwrap_or_default(),
    };

    let new_event = match RecipeTimeline::edit(&state.mm, user_id, &new_event_params).await {
        Ok(t) => t,
        Err(err) => {
            error!("Failed to edit timeline event '{new_event_params:?}': {err}");
            broadcast_error(&state, user_id, "Failed to edit timeline event.").await;
            return Error::Database.into_response();
        }
    };

    templates::recipes::timeline::render_event(
        &Event::from(new_event),
        form.index.unwrap_or_default(),
        form.max_index.unwrap_or_default(),
        recipe_id,
    )
    .into_response()
}

/// Toggles the favourite state of a recipe.
pub async fn toggle_favourite_handler(
    ctx: CtxW,
    uri: Uri,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
    Form(params): Form<FavouriteParams>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let is_favourite = match Recipe::toggle_favourite(&state.mm, user_id, recipe_id).await {
        Ok(v) => v,
        Err(err) => {
            error!(
                "Error toggling the favourite state of recipe '{recipe_id}' for user '{user_id}': {err}"
            );
            broadcast_error(&state, user_id, "Error toggling favourite.").await;
            return Error::Database.into_response();
        }
    };

    if let Ok(r) = Recipe::get(&state.mm, user_id, recipe_id).await
        && let Ok(times) = FormattedTimes::from_times(&r.times)
    {
        let cache_key = (user_id, recipe_id);
        let view_recipe = ViewRecipe {
            recipe_details: r,
            formatted_times: times,
        };

        state.remove_cached_recipe(cache_key).await;
        state.cache_recipe(cache_key, &view_recipe).await;
    };

    let path = uri.path();
    let is_deletable = path.starts_with("/recipes/search") && path.contains("fav=true");

    templates::recipes::render_favourite_button(
        recipe_id,
        is_favourite,
        is_deletable,
        params.is_view_recipe.unwrap_or_default(),
    )
    .into_response()
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
        schema_org::Recipe::schema(),
        settings,
    )
    .into_response())
}

/// Handles the importing recipes from an application endpoint.
pub async fn add_recipe_import_app_handler(
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
        let app = form.app.to_string();
        let state = state.clone();
        let start_time = Instant::now();

        let recipes = match parse_recipes(&state, form, user_id).await {
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

        let num_recipes = recipes.len() as i64;

        let (mut report, recipe_ids) = push_recipes_to_db(&state, recipes, user_id).await;
        report.exec_time_ms = start_time.elapsed().as_millis() as i64;

        broadcast_import_done_toast(&state, recipe_ids, num_recipes, report, app, user_id).await;
    });
}

async fn parse_recipes(
    state: &AppState,
    mut form: ImportFromAppForm,
    user_id: i64,
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
    user_id: i64,
) -> (ReportForCreate, Vec<i64>) {
    let mut report = ReportForCreate::new(ReportTypes::Import, user_id);
    let mut curr = 0;
    let mut recipe_ids = Vec::new();
    let num_recipes = recipes.len() as i64;

    for schema in recipes {
        curr += 1;
        state
            .broadcast_progress("Saving recipes", curr, num_recipes, true, user_id)
            .await;

        let recipe = schema_to_recipe_for_create(state, schema).await;

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

    (report, recipe_ids)
}

/// Handles the importing recipes from an API endpoint.
pub async fn add_recipe_import_api_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(form): Form<ImportFromApiForm>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();
    fetch_recipes_from_api(state, form, user_id);

    (StatusCode::ACCEPTED, "").into_response()
}

fn fetch_recipes_from_api(state: AppState, form: ImportFromApiForm, user_id: i64) {
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
        let mut failures = Vec::new();
        let mut report = ReportForCreate::new(ReportTypes::Import, user_id);

        let mut last_progress_time = Instant::now();
        let progress_interval = std::time::Duration::from_millis(500);
        let batch_size = 10;

        while let Some(res) = db_stream.next().await {
            match res {
                Ok((recipe_id, recipe_name, num_recipes)) => {
                    if total == -1 {
                        total = num_recipes;
                    }

                    report
                        .report_logs
                        .push(ReportLogForCreate::new_success(recipe_name));

                    successes.push(recipe_id);
                }
                Err((recipe_api_id, recipe_name, num_recipes, err)) => {
                    if total == -1 {
                        total = num_recipes;
                    }

                    match err {
                        Error::Model(DuplicateEntity) if recipe_name.is_some() => {
                            report.report_logs.push(ReportLogForCreate::new_warning(
                                recipe_name.unwrap_or_default(),
                                "Recipe exists".into(),
                            ));
                        }
                        Error::Model(..) if recipe_name.is_some() => {
                            report.report_logs.push(ReportLogForCreate::new_error(
                                recipe_name.unwrap_or_default(),
                                err.to_string(),
                            ));
                        }
                        _ => {
                            report.report_logs.push(ReportLogForCreate::new_error(
                                "Fetch Failure".into(),
                                err.to_string(),
                            ));
                        }
                    }

                    failures.push((recipe_api_id, err));
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

        report.exec_time_ms = start_time.elapsed().as_millis() as i64;
        broadcast_import_done_toast(&state, successes, processed, report, api, user_id).await;
    });
}

async fn push_recipe_in_db(
    state: &AppState,
    recipe: schema_org::Recipe,
    user_id: i64,
) -> std::result::Result<i64, (String, Error)> {
    let recipe = schema_to_recipe_for_create(state, recipe).await;

    match Recipe::create(&state.mm, user_id, &recipe).await {
        Ok(recipe_id) => Ok(recipe_id),
        Err(DuplicateEntity) => {
            warn!("Recipe exists: {}", recipe.name);
            Err((recipe.name, Error::Model(DuplicateEntity)))
        }
        Err(err) => {
            error!("Error saving recipe '{}': {err}", recipe.name);
            Err((recipe.name, Error::Model(err)))
        }
    }
}

async fn broadcast_import_done_toast(
    state: &AppState,
    recipe_ids: Vec<i64>,
    num_recipes: i64,
    report: ReportForCreate,
    import_source: String,
    user_id: i64,
) {
    state.hide_broadcast(user_id).await;

    let num_success = recipe_ids.len() as i64;
    let num_skipped = num_recipes - num_success;

    info!(
        "Imported recipes ({import_source}): user_id={user_id}, success={num_success}, skipped={num_skipped}, total={num_recipes}"
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
}

/// Handles generating a preview of the recipe based on the input JSON recipe schema.
pub async fn add_recipe_import_preview_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(form): Form<PreviewForm>,
) -> Result<impl IntoResponse> {
    match serde_json::from_str::<schema_org::Recipe>(&form.json_input) {
        Ok(schema) => {
            let user_id = ctx.0.user_id();

            let recipe_c = RecipeForCreate::from(&schema);
            let recipe_details = RecipeDetails::from(recipe_c);
            let formatted_times = FormattedTimes::from_times(&recipe_details.times)?;
            let view_recipe = ViewRecipe {
                recipe_details,
                formatted_times,
            };

            let fs_support = Arc::clone(&state.fs_support);
            let data_dir = state.data_dir.clone();

            match templates::recipes::view_recipe_helper(
                fs_support,
                data_dir,
                &Data {
                    is_admin: user_id == 1,
                    is_authenticated: true,
                    is_autologin: state.config.read().await.is_autologin,
                    is_hx_request: true,
                    is_preview: true,
                    about: AboutData {
                        is_update_available: false,
                        is_check_update: false,
                        last_checked_update_at: Default::default(),
                        last_updated_at: Default::default(),
                        version: "".to_string(),
                    },
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
                },
            ) {
                Ok(res) => Ok(res.into_response()),
                Err(err) => {
                    error!("Error rendering view recipe page preview for user {user_id}: {err}");
                    broadcast_error(&state, user_id, "Error rendering recipe preview.").await;
                    Err(Error::Templates)
                }
            }
        }
        Err(err) => {
            error!("Error parsing recipe schema JSON: {err}");
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

/// Handles parsing a recipe from raw JSON.
pub async fn add_recipe_import_raw_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    Form(form): Form<PreviewForm>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    match serde_json::from_str::<schema_org::Recipe>(&form.json_input) {
        Ok(schema) => {
            let recipe_c = RecipeForCreate::from(&schema);

            match Recipe::create(&state.mm, user_id, &recipe_c).await {
                Ok(recipe_id) => {
                    let url = format!("/recipes/{recipe_id}");

                    match HeaderValue::from_str(&url) {
                        Ok(hv) => (StatusCode::OK, [(HX_REDIRECT, hv)]).into_response(),
                        Err(_) => (StatusCode::BAD_REQUEST, "invalid redirect url").into_response(),
                    }
                }
                Err(DuplicateEntity) => {
                    warn!("Recipe exists: {}", recipe_c.name);
                    broadcast_error(&state, user_id, "Recipe exists.").await;
                    Error::EntityExists { entity: "recipe" }.into_response()
                }
                Err(err) => {
                    error!("Error saving recipe '{}': {err}", recipe_c.name);
                    broadcast_error(&state, user_id, "Failed to insert recipe.").await;
                    Error::Database.into_response()
                }
            }
        }
        Err(err) => {
            error!("Error parsing recipe schema JSON: {err}");
            broadcast_error(&state, user_id, "Error parsing recipe schema JSON.").await;
            Error::InvalidPayload.into_response()
        }
    }
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

            fs_support.upload_image(&path, file_name, &state.data_dir.images.root);

            let thumbnails_dir = state.data_dir.images.thumbnails.clone();
            let fs_support = Arc::clone(&state.fs_support);
            tokio::spawn(async move {
                fs_support.generate_thumbnail(&path, file_name, &thumbnails_dir);
            });

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
    let measurement_system_id = system::MeasurementSystem::from(ingredients.clone()).id();

    let recipe_id = match Recipe::create(
        &state.mm,
        user_id,
        &RecipeForCreate {
            name: form.title,
            description: form.description,
            images,
            measurement_system_id,
            r#yield: form.yield_,
            source: form.source.unwrap_or_default(),
            is_favourite: false,
            rating: form.rating,
            videos,
            category: form.category.or(Some("uncategorized".into())),
            cuisine: form.cuisine,
            ingredients: SectionComponents::Flat(ingredients.iter().map(Item::new).collect()),
            instructions: SectionComponents::Flat(
                form.instructions.iter().map(Item::new).collect(),
            ),
            keywords: form.keywords,
            notes: form.notes,
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
        res.headers_mut().insert(HX_REDIRECT, value);
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

#[allow(dead_code)]
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

async fn schema_to_recipe_for_create(
    state: &AppState,
    schema: schema_org::Recipe,
) -> RecipeForCreate {
    let fs_support = state.fs_support.clone();

    let schema = Arc::new(schema);
    let mut recipe_c = RecipeForCreate::from(&*schema);

    recipe_c.images = extract_images(&schema, state, fs_support.clone()).await;
    recipe_c.videos = extract_videos(&schema, state, fs_support).await;

    recipe_c
}

async fn extract_images(
    schema: &Arc<schema_org::Recipe>,
    state: &AppState,
    fs_support: Arc<dyn FsSupport>,
) -> Vec<Uuid> {
    let fetches: Vec<_> = schema
        .image
        .iter()
        .filter_map(|img| {
            let state = state.clone();
            let fs_support = fs_support.clone();

            let url_opt = match img {
                schema_org::field::FieldEnum22::ImageObject(image_object) => {
                    image_object.url.first().cloned()
                }
                schema_org::field::FieldEnum22::URL(u) => Some(u.clone()),
            };

            url_opt.map(|url| async move { fetch_image_async(&state, fs_support, &url).await })
        })
        .collect();

    join_all(fetches).await.into_iter().flatten().collect()
}

async fn fetch_image_async(
    state: &AppState,
    fs_support: Arc<dyn FsSupport>,
    file_path: &str,
) -> Option<Uuid> {
    let file_path = file_path.to_string();
    let state = state.clone();

    tokio::task::spawn_blocking(move || fetch_image(&state, fs_support, &file_path))
        .await
        .ok()
        .flatten()
}

fn fetch_image(state: &AppState, fs_support: Arc<dyn FsSupport>, file_path: &str) -> Option<Uuid> {
    let path = if file_path.starts_with("/tmp") {
        PathBuf::from(file_path)
    } else {
        PathBuf::new()
    };

    let file_name = Uuid::new_v4();
    fs_support.upload_image(&path, file_name, &state.data_dir.images.root);

    let thumbnails_dir = state.data_dir.images.thumbnails.clone();
    let fs_support = Arc::clone(&state.fs_support);
    tokio::spawn(async move {
        fs_support.generate_thumbnail(&path, file_name, &thumbnails_dir);
    });

    state
        .fs_support
        .is_file_exists(file_name, &state.data_dir.images.root, ".webp")
        .then_some(file_name)
}

async fn extract_videos(
    schema: &Arc<schema_org::Recipe>,
    state: &AppState,
    fs_support: Arc<dyn FsSupport>,
) -> Vec<VideoForCreate> {
    use schema_org::field::FieldEnum19::*;

    let urls = schema
        .video
        .iter()
        .filter_map(|video| match video {
            Clip(clip) => clip.video.first().and_then(|v| match v {
                VideoObject(video_object) => Some((
                    video_object.url.first()?,
                    video_object.content_url.first(),
                    video_object.embed_url.first(),
                )),
                Clip(_) => None,
            }),
            VideoObject(video_object) => Some((
                video_object.url.first()?,
                video_object.content_url.first(),
                video_object.embed_url.first(),
            )),
        })
        .collect::<Vec<_>>();

    let futures = urls.iter().map(|(url, _content_url, _embed_url)| {
        state.scraper.fetch_and_upload_to_temp(url.as_str())
    });

    let results = join_all(futures).await;

    stream::iter(urls.into_iter().zip(results.into_iter()))
        .filter_map(|((_url, content_url, embed_url), res)| {
            let fs_support = fs_support.clone();

            async move {
                let path = res.ok()?;
                let file_name = Uuid::new_v4();

                fs_support
                    .clone()
                    .upload_videos(vec![path.clone()], &state.data_dir.images.root);

                if fs_support.is_file_exists(file_name, &state.data_dir.videos, ".webp") {
                    let duration = fs_support
                        .calc_video_duration(path.to_str().unwrap_or_default())
                        .await
                        .ok();

                    Some(VideoForCreate {
                        video: file_name,
                        duration,
                        content_url: content_url.cloned(),
                        embed_url: embed_url.cloned(),
                    })
                } else {
                    None
                }
            }
        })
        .collect::<Vec<_>>()
        .await
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
            is_preview: false,
            about: AboutData {
                is_update_available: false,
                is_check_update: false,
                last_checked_update_at: Default::default(),
                last_updated_at: Default::default(),
                version: "".to_string(),
            },
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
        let is_favourites = search_params.is_favourites.unwrap_or_default();

        return Ok(templates::search::no_results(is_favourites).into_response());
    }

    let settings = get_settings(&state, user_id).await?;

    Ok(templates::recipes::search_results(
        state.fs_support,
        uri.path(),
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.read().await.is_autologin,
            is_hx_request: is_hx_request(&headers),
            // TODO: Populate AboutData with good values.
            is_preview: false,
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
                headers.get(axum_htmx::HX_REQUEST).is_some(),
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
