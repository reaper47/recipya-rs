use std::sync::Arc;

use axum::{extract::State, http::HeaderMap, response::IntoResponse};
use axum_htmx::HX_REDIRECT;
use config::States;
use futures_util::future::join_all;
use reqwest::StatusCode;
use tracing::error;
use uuid::Uuid;

use app::state::AppState;
use math::cooking::units::system;
use models::{
    Recipe,
    data::Data,
    recipe::{
        RecipeForm,
        structs::{media::VideoForCreate, recipe::RecipeForCreate, types::Source},
    },
    settings::UserSettingDetails,
};

use crate::{
    Error, Result,
    handlers::{
        get_settings, helpers::is_hx_request, message::broadcast_error,
        recipes::common::fetch_categories_keywords,
    },
    middleware::mw_auth::RequireAuth,
};

/// Handles rendering the form to add a recipe manually.
pub async fn add_manual_recipe_handler(
    header_map: HeaderMap,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let settings = get_settings(&state, user.id).await?;

    let (categories, keywords) = match fetch_categories_keywords(&state, user.id).await {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(templates::recipes::add_recipe_manual(
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            states: States {
                autologin: state.config.read().await.states.autologin.clone(),
                ..Default::default()
            },
            is_hx_request: is_hx_request(&header_map),
            ..Default::default()
        },
        &settings,
        categories,
        keywords,
    )
    .into_response())
}

/// Handles posting a submitted recipe form.
pub async fn add_manual_recipe_post_handler(
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
    form: RecipeForm,
) -> impl IntoResponse {
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
    let measurement_system_id = system::MeasurementSystem::from(ingredients.items_as_text()).id();

    let recipe_id = match Recipe::create(
        &state.mm,
        user.id,
        &RecipeForCreate {
            name: form.title,
            description: form.description,
            images,
            measurement_system_id,
            r#yield: form.r#yield,
            source: Source::from(form.source),
            is_favourite: false,
            rating: form.rating,
            videos,
            category: form.category.or_else(|| Some("uncategorized".into())),
            cuisine: form.cuisine,
            ingredients,
            instructions: form.instructions,
            keywords: form.keywords,
            notes: form.notes,
            nutrition: form.nutrition,
            times: form.times,
            tools: form.tools,
        },
        &UserSettingDetails::get(&state.mm, user.id)
            .await
            .unwrap_or_default(),
    )
    .await
    {
        Ok(id) => id,
        Err(err) => {
            error!(
                "Failed to add recipe to collection for user '{}': {err}",
                user.id
            );
            broadcast_error(&state, user.id, "Failed to add recipe to collection.").await;
            return Error::Database.into_response();
        }
    };

    (
        StatusCode::CREATED,
        [(HX_REDIRECT, format!("/recipes/{recipe_id}"))],
    )
        .into_response()
}
