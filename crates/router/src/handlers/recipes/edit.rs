use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
use axum_htmx::HX_REDIRECT;
use config::States;
use futures_util::future::join_all;
use reqwest::StatusCode;
use tracing::error;
use uuid::Uuid;

use app::state::AppState;
use models::{
    Error::EntityNotFound,
    Recipe,
    recipe::{RecipeForm, structs::recipe::RecipeForCreate},
};
use models::{data::Data, recipe::structs::media::VideoForCreate};

use crate::{
    Error, Result,
    handlers::{
        get_settings, helpers::is_hx_request, message::broadcast_error,
        recipes::common::fetch_view_recipe,
    },
    middleware::mw_auth::RequireAuth,
};

/// Handles a recipe's edit page.
pub async fn edit_recipe_handler(
    header_map: HeaderMap,
    RequireAuth(user): RequireAuth,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let settings = get_settings(&state, user.id).await?;

    let (recipe, categories, keywords) = match fetch_view_recipe(&state, user.id, recipe_id).await {
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

    let recipe_id = recipe.recipe_details.recipe.id;
    let autologin = state.config.read().await.states.autologin;

    match templates::recipes::edit_recipe(
        &state.fs_support,
        Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            states: States {
                autologin,
                ..Default::default()
            },
            is_hx_request: is_hx_request(&header_map),
            recipes: vec![recipe],
            ..Default::default()
        },
        &state.data_dir,
        &settings,
        categories,
        keywords,
    ) {
        Ok(res) => Ok(res),
        Err(err) => {
            error!(?recipe_id, user = ?user.id, ?err, "Error rendering edit recipe page");
            Err(Error::Templates)
        }
    }
}

/// Handles updating a recipe.
pub async fn edit_recipe_put_handler(
    RequireAuth(user): RequireAuth,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
    form: RecipeForm,
) -> impl IntoResponse {
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

        Arc::clone(&state.fs_support)
            .upload_videos(form.videos.into_values().collect(), &state.data_dir.videos);
        videos
    };

    match Recipe::update(&state.mm, user.id, recipe_id, &mut recipe_c).await {
        Ok(()) => {
            state.remove_cached_recipe((user.id, recipe_id)).await;
        }
        Err(err) => {
            error!(?recipe_id, user = ?user.id, ?err, "Failed to update recipe user");
            broadcast_error(&state, user.id, "Failed to add recipe to collection.").await;
            return Error::Database.into_response();
        }
    }

    let mut res = (StatusCode::SEE_OTHER, "").into_response();
    if let Ok(value) = HeaderValue::from_str(&format!("/recipes/{recipe_id}")) {
        res.headers_mut().insert(HX_REDIRECT, value);
    }
    res
}
