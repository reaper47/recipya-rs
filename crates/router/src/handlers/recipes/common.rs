use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::ws::Message;
use futures_util::{StreamExt, future::join_all, stream};
use support::fs::FsSupport;
use tracing::{error, info, warn};
use uuid::Uuid;

use app::state::AppState;
use models::{
    Error::EntityNotFound,
    Recipe,
    data::ViewRecipe,
    recipe::structs::{
        media::VideoForCreate,
        recipe::{Category, Keyword, RecipeForCreate},
    },
    reports::report::ReportForCreate,
    time::FormattedTimes,
    user::User,
};

use crate::{
    Error, Result,
    handlers::message::{IMessage, MessageHtmx, MessageType, broadcast_error},
};

pub async fn broadcast_import_done_toast(
    state: &AppState,
    recipe_ids: Vec<i64>,
    num_recipes: i64,
    report: ReportForCreate,
    import_source: String,
    user_id: Uuid,
) {
    state.hide_broadcast(user_id).await;

    let num_success = i64::try_from(recipe_ids.len())
        .inspect_err(|err| error!("Failed to cast num success '{}': {err}", recipe_ids.len()))
        .unwrap_or(i64::MAX);
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
        "Success",
        format!("Imported {num_success} recipes. Skipped {num_skipped}."),
    )
    .action(Some(&redirect))
    .build();

    if let Ok(json) = serde_json::to_string(&toast) {
        state.broadcast(Message::Text(json.into()), user_id).await;
    }

    if let Err(err) = report.insert(&state.mm).await {
        error!("Error inserting report into the database: {err}");
    }
}

/// Fetches the user's categories and keywords from the database.
pub async fn fetch_categories_keywords(
    state: &AppState,
    user_id: Uuid,
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

pub async fn schema_to_recipe_for_create(
    state: &AppState,
    schema: schema_org::Recipe,
) -> RecipeForCreate {
    let fs_support = state.fs_support.clone();

    let schema = Arc::new(schema);
    let mut recipe_c = RecipeForCreate::from(&*schema);

    recipe_c.images = extract_images(&schema, state, fs_support.clone()).await;
    recipe_c.videos = extract_videos(&schema, state, fs_support).await;

    recipe_c.adjust_instructions_duration();
    recipe_c
}

async fn extract_images(
    schema: &Arc<schema_org::Recipe>,
    state: &AppState,
    fs_support: Arc<dyn FsSupport>,
) -> Vec<Uuid> {
    let image_refs = schema
        .image
        .iter()
        .filter_map(|img| match img {
            schema_org::field::FieldEnum22::ImageObject(image_object) => {
                image_object.url.first().cloned()
            }
            schema_org::field::FieldEnum22::URL(u) => Some(u.clone()),
        })
        .flat_map(|url| {
            url.split(';')
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    stream::iter(image_refs)
        .filter_map(|image_ref| {
            let fs_support = fs_support.clone();

            async move {
                let path = if image_ref.starts_with("http://") || image_ref.starts_with("https://")
                {
                    state
                        .scraper
                        .fetch_and_upload_to_temp(image_ref.as_str())
                        .await
                        .ok()?
                } else {
                    let normalized = image_ref
                        .strip_prefix("file://")
                        .unwrap_or(image_ref.as_str());
                    let local_path = PathBuf::from(normalized);

                    if !local_path.exists() {
                        warn!("Skipping missing local image path: {local_path:?}");
                        return None;
                    }

                    local_path
                };

                let file_name = Uuid::new_v4();

                fs_support.upload_image(&path, file_name, &state.data_dir.images.root);

                let thumbnails_dir = state.data_dir.images.thumbnails.clone();

                let res =
                    if fs_support.is_file_exists(file_name, &state.data_dir.images.root, ".webp") {
                        Some(file_name)
                    } else {
                        None
                    };

                let fs_support = Arc::clone(&state.fs_support);
                tokio::spawn(async move {
                    fs_support.generate_thumbnail(&path, file_name, &thumbnails_dir);
                });

                res
            }
        })
        .collect::<Vec<_>>()
        .await
}

async fn extract_videos(
    schema: &Arc<schema_org::Recipe>,
    state: &AppState,
    fs_support: Arc<dyn FsSupport>,
) -> Vec<VideoForCreate> {
    use schema_org::field::FieldEnum19::{Clip, VideoObject};

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

    stream::iter(urls.into_iter().zip(results))
        .filter_map(|((_url, content_url, embed_url), res)| {
            let fs_support = fs_support.clone();

            async move {
                let path = res.ok()?;
                let file_name = Uuid::new_v4();

                fs_support
                    .clone()
                    .upload_videos(vec![path.clone()], &state.data_dir.videos);

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

pub async fn fetch_view_recipe(
    state: &AppState,
    user_id: Uuid,
    recipe_id: i64,
) -> Result<(ViewRecipe, Vec<Category>, Vec<Keyword>)> {
    let recipe = match Recipe::get(&state.mm, user_id, recipe_id).await {
        Ok(recipe) => recipe,
        Err(err) => {
            error!("Error fetching recipe '{recipe_id}' for user '{user_id}': {err}");
            broadcast_error(state, user_id, "Recipe not found.").await;
            return Err(Error::Model(EntityNotFound {
                id: recipe_id.to_string(),
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
