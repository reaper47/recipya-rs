use std::{collections::HashMap, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use time::{PrimitiveDateTime, macros::format_description};
use tracing::error;
use uuid::Uuid;

use app::state::AppState;
use config::DataDir;
use models::recipe::timeline::RecipeTimeline;
use models::{Error::EntityNotFound, Recipe, recipe::timeline::RecipeTimelineForCreate};
use support::fs::FsSupport;
use templates::recipes::timeline::Event;

use crate::{
    Error,
    handlers::message::{broadcast_error, broadcast_success, broadcast_warning},
    middleware::mw_auth::RequireAuth,
    recipes_router::params::{OrderParams, TimelineEventForm},
};

/// Handles getting a timeline event for edit.
pub async fn timeline_event_get_edit_handler(
    Path((recipe_id, timeline_id)): Path<(i64, i64)>,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Query(q): Query<OrderParams>,
) -> impl IntoResponse {
    let event = match RecipeTimeline::get(&state.mm, timeline_id, recipe_id, user.id).await {
        Ok(t) => t,
        Err(EntityNotFound { entity, .. }) => {
            broadcast_warning(&state, user.id, "Timeline event does not exist.").await;
            return Error::EntityNotFound { entity }.into_response();
        }
        Err(err) => {
            error!(?timeline_id, ?recipe_id, user = ?user.id, ?err, "Error fetching timeline event");
            broadcast_error(&state, user.id, "Could not fetch timeline event.").await;
            return Error::Database.into_response();
        }
    };

    templates::recipes::timeline::render_edit(event, q.index, q.max_index, recipe_id)
        .into_response()
}

/// Handles updating a timeline component of the recipe.
pub async fn timeline_put_handler(
    Path((recipe_id, timeline_id)): Path<(i64, i64)>,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    form: TimelineEventForm,
) -> impl IntoResponse {
    let original_event = match RecipeTimeline::get(&state.mm, timeline_id, recipe_id, user.id).await
    {
        Ok(t) => t,
        Err(EntityNotFound { entity, .. }) => {
            broadcast_warning(&state, user.id, "Timeline event does not exist.").await;
            return Error::EntityNotFound { entity }.into_response();
        }
        Err(err) => {
            error!(?timeline_id, ?recipe_id, user = ?user.id, ?err, "Error fetching timeline event");
            broadcast_error(&state, user.id, "Could not fetch timeline event.").await;
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
        upload_image(form.image, &Arc::clone(&state.fs_support), &state.data_dir)
    };

    let new_event_params = RecipeTimeline {
        id: original_event.id,
        recipe_id: original_event.recipe_id,
        user_id: original_event.user_id,
        title: form.title,
        comment: form.comment,
        rating: form.rating,
        image,
        created_at: form.date.unwrap_or(PrimitiveDateTime::MIN),
    };

    let new_event = match RecipeTimeline::edit(&state.mm, user.id, &new_event_params).await {
        Ok(t) => t,
        Err(err) => {
            error!(?new_event_params, ?err, "Failed to edit timeline event");
            broadcast_error(&state, user.id, "Failed to edit timeline event.").await;
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

/// Handles getting a recipe's timeline.
pub async fn timeline_get_handler(
    RequireAuth(user): RequireAuth,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let recipe = match Recipe::get_recipe_only(&state.mm, user.id, recipe_id).await {
        Ok(recipe) => recipe,
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

    let events = match RecipeTimeline::all(&state.mm, recipe_id, user.id).await {
        Ok(components) => components.into_iter().map(Event::from).collect::<Vec<_>>(),
        Err(err) => {
            error!(?recipe_id, user = ?user.id, ?err, "Error fetching timeline components");
            broadcast_error(&state, user.id, "Failed to fetch timeline components.").await;
            return Error::Model(EntityNotFound {
                id: recipe_id.to_string(),
                entity: "timeline",
            })
            .into_response();
        }
    };

    let static_events = vec![Event {
        date: recipe
            .created_at
            .date()
            .format(format_description!("[year]-[month]-[day]"))
            .unwrap(),
        title: "Recipe created".into(),
        ..Default::default()
    }];

    let mut all_events = static_events;
    all_events.extend(events);

    templates::recipes::timeline::render_events(recipe_id, &all_events).into_response()
}

/// Handles adding a timeline component to the recipe.
pub async fn timeline_post_handler(
    RequireAuth(user): RequireAuth,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
    form: TimelineEventForm,
) -> impl IntoResponse {
    if let Err(err) = RecipeTimeline::create(
        &state.mm,
        recipe_id,
        user.id,
        &RecipeTimelineForCreate {
            title: form.title,
            comment: form.comment,
            rating: form.rating,
            image: upload_image(form.image, &Arc::clone(&state.fs_support), &state.data_dir),
            created_at: form.date,
        },
    )
    .await
    {
        error!(?recipe_id, user = ?user.id, ?err, "Error creating timeline event");
        broadcast_error(&state, user.id, "Could not create timeline event.").await;
        return Error::Database.into_response();
    }

    broadcast_success(&state, user.id, "Timeline event created.").await;
    (StatusCode::CREATED, "").into_response()
}

/// Handles getting a timeline event for view.
pub async fn timeline_event_get_handler(
    Path((recipe_id, timeline_id)): Path<(i64, i64)>,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    Query(q): Query<OrderParams>,
) -> impl IntoResponse {
    let event = match RecipeTimeline::get(&state.mm, timeline_id, recipe_id, user.id).await {
        Ok(t) => Event::from(t),
        Err(EntityNotFound { entity, .. }) => {
            broadcast_warning(&state, user.id, "Timeline event does not exist.").await;
            return Error::EntityNotFound { entity }.into_response();
        }
        Err(err) => {
            error!(?recipe_id, user = ?user.id, ?err, "Error fetching timeline event");
            broadcast_error(&state, user.id, "Could not fetch timeline event.").await;
            return Error::Database.into_response();
        }
    };

    templates::recipes::timeline::render_event(&event, q.index, q.max_index, recipe_id)
        .into_response()
}

fn upload_image(
    map: HashMap<String, PathBuf>,
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
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
        .copied()
}
