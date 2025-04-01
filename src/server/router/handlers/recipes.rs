use axum::Form;
use axum::extract::ws::Message;
use axum::extract::{OriginalUri, Path, Query, State};
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{Html, IntoResponse};
use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use futures_util::future::join_all;
use reqwest::StatusCode;
use std::fmt::Write;
use std::ops::Not;
use tracing::error;
use uuid::Uuid;

use crate::core::model::Error::EntityNotFound;
use crate::core::model::Recipe;
use crate::core::model::recipe::{
    Category, Keyword, RecipeForCreate, RecipeForm, Sections, VideoForCreate,
};
use crate::core::model::share::ShareRecipe;
use crate::core::model::user::User;
use crate::core::model::website::{ToHtmlTable, Website};
use crate::core::support::fs::{calc_video_duration, is_file_exists, upload_image, upload_videos};
use crate::server::router::SearchParams;
use crate::server::router::handlers::helpers::is_hx_request;
use crate::server::router::handlers::message::{IMessage, MessageHtmx};
use crate::server::router::middleware::mw_auth::CtxW;
use crate::server::router::recipes_routes::{RecipeCategoryForm, ShareRecipeForm};
use crate::server::templates::data::{
    AboutData, Data, FormattedTimes, PaginationData, PaginationHtmxData, PaginationSearchData,
    SearchbarData, ShareData, ViewRecipe,
};
use crate::server::{AppState, templates};
use crate::server::{Error, Result};

/// Handles deleting a user's recipe.
pub async fn delete_recipe_handler(
    ctx: CtxW,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    match Recipe::delete(&state.mm, recipe_id, user_id).await {
        Ok(_) => (
            StatusCode::NO_CONTENT,
            [(axum_htmx::headers::HX_REDIRECT, "/")],
        )
            .into_response(),
        Err(err) => {
            error!("Error deleting recipe {recipe_id} for user {user_id}: {err}");

            let toast = MessageHtmx::error("Recipe could not be deleted.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json.into())).await;
            }

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
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let num_recipes = match Recipe::count(&state.mm, user_id).await {
        Ok(count) => count,
        Err(err) => {
            error!("Error counting recipes for user {user_id}: {err}");

            let toast = MessageHtmx::error("Error fetching number of recipes.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state
                    .broadcast(ctx.0.user_id(), Message::Text(json.into()))
                    .await;
            }

            return Error::Database.into_response();
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
                            let toast = MessageHtmx::error("Error formatting recipe times.");
                            if let Ok(json) = serde_json::to_string(&toast) {
                                state
                                    .broadcast(ctx.0.user_id(), Message::Text(json.into()))
                                    .await;
                            }
                            Error::Database
                        })
                })
                .collect();

            match mapped_recipes {
                Ok(mapped) => mapped,
                Err(_) => return Error::Database.into_response(),
            }
        }
        Err(err) => {
            error!(
                "Error fetching recipes for user '{user_id}' with search params '{:?}': {err}",
                search_params
            );
            let toast = MessageHtmx::error("Error fetching recipes.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state
                    .broadcast(ctx.0.user_id(), Message::Text(json.into()))
                    .await;
            }
            return Error::Database.into_response();
        }
    };

    templates::recipes::index(
        uri.path(),
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.is_autologin,
            is_hx_request: is_hx_request(&headers),
            about: AboutData {
                is_update_available: false,
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
    )
    .into_response()
}

/// Handles the duplicate recipe endpoint.
pub async fn duplicate_recipe_handler(
    ctx: CtxW,
    Path(recipe_id): Path<i64>,
    header_map: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let (recipe, categories, keywords) = match fetch_view_recipe(&state, user_id, recipe_id).await {
        Ok(res) => res,
        Err(err) => {
            error!("Error fetching view recipe '{recipe_id}' for user '{user_id}': {err}");
            let toast = MessageHtmx::error("Recipe not found.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json.into())).await;
            }
            return Error::Model(EntityNotFound {
                id: recipe_id,
                entity: "recipe",
            })
            .into_response();
        }
    };

    templates::recipes::add_recipe_manual(
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            recipes: vec![recipe],
            ..Default::default()
        },
        categories,
        keywords,
    )
    .into_response()
}

/// Handles a recipe's edit page.
pub async fn edit_recipe_handler(
    ctx: CtxW,
    header_map: HeaderMap,
    Path(recipe_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = ctx.0.user_id();

    let (recipe, categories, keywords) = match fetch_view_recipe(&state, user_id, recipe_id).await {
        Ok(res) => res,
        Err(err) => {
            error!("Error fetching view recipe '{recipe_id}' for user '{user_id}': {err}");
            let toast = MessageHtmx::error("Recipe not found.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json.into())).await;
            }
            return Err(Error::Model(EntityNotFound {
                id: recipe_id,
                entity: "recipe",
            }));
        }
    };

    templates::recipes::edit_recipe(
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            recipes: vec![recipe],
            ..Default::default()
        },
        &state.data_dir,
        categories,
        keywords,
    )
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
            let toast = MessageHtmx::error("Recipe not found.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json.into())).await;
            }
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
                Ok(name) if is_file_exists(name, &state.data_dir.images) => name,
                _ => {
                    let file_name = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned()
                        .parse::<Uuid>()
                        .unwrap_or_default();

                    upload_image(path, file_name, &state.data_dir.images);
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

            async move {
                match Uuid::parse_str(original_file_stem) {
                    Ok(name) if is_file_exists(name, &dir_videos) => VideoForCreate {
                        video: name,
                        duration: calc_video_duration(path.to_str().unwrap_or_default())
                            .await
                            .ok(),
                        content_url: None,
                        embed_url: None,
                    },
                    _ => VideoForCreate::from_path(path).await,
                }
            }
        }))
        .await;

        upload_videos(
            form.videos.values().cloned().collect(),
            &state.data_dir.videos,
        );
        videos
    };

    let mut recipe_c = RecipeForCreate::from(form);
    recipe_c.images = images.is_empty().not().then_some(images);
    recipe_c.videos = videos;

    match Recipe::update(&state.mm, user_id, recipe_id, &mut recipe_c).await {
        Ok(id) => id,
        Err(err) => {
            error!("Failed to update recipe '{recipe_id}' user '{user_id}': {err}");
            let toast = MessageHtmx::error("Failed to add recipe to collection.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json.into())).await;
            }
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
            let url = format!("{}/shared/r/{}", state.config.base_url, share.link);
            templates::general::share_link(&url).into_response()
        }
        Err(err) => {
            error!(
                "Error generating shared recipe link for recipe '{recipe_id}' and user '{user_id}': {err}"
            );
            let toast = MessageHtmx::error("Error parsing datetime.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json.into())).await;
            }
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
) -> impl IntoResponse {
    templates::recipes::add_page(
        uri.path(),
        Data {
            is_admin: ctx.0.user_id() == 1,
            is_authenticated: true,
            is_autologin: state.config.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            ..Default::default()
        },
    )
}

/// Handles rendering the form to add a recipe manually.
pub async fn add_manual_recipe_handler(
    ctx: CtxW,
    header_map: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

    let (categories, keywords) = match fetch_categories_keywords(&state, user_id).await {
        Ok(res) => res,
        Err(err) => {
            return err.into_response();
        }
    };

    templates::recipes::add_recipe_manual(
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            ..Default::default()
        },
        categories,
        keywords,
    )
    .into_response()
}

/// Handles posting a submitted recipe form.
pub async fn add_manual_recipe_post_handler(
    ctx: CtxW,
    State(state): State<AppState>,
    form: RecipeForm,
) -> impl IntoResponse {
    let user_id = ctx.0.user_id();

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

            upload_image(&path, file_name, &state.data_dir.images);
            file_name
        })
        .collect::<Vec<_>>();

    let videos = if form.videos.is_empty() {
        Vec::new()
    } else {
        let videos = join_all(
            form.videos
                .values()
                .map(|path| VideoForCreate::from_path(path)),
        )
        .await;

        upload_videos(
            form.videos.values().cloned().collect(),
            &state.data_dir.videos,
        );
        videos
    };

    let recipe_id = match Recipe::create(
        &state.mm,
        user_id,
        &RecipeForCreate {
            name: form.title,
            description: form.description,
            images: images.is_empty().not().then_some(images),
            yield_: form.yield_,
            source: form.source,
            videos,
            category: form.category.or(Some("uncategorized".into())),
            cuisine: form.cuisine,
            ingredients: Sections::from([("".into(), form.ingredients)]),
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
            let toast = MessageHtmx::error("Failed to add recipe to collection.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json.into())).await;
            }
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

async fn fetch_categories_keywords(
    state: &AppState,
    user_id: i64,
) -> Result<(Vec<Category>, Vec<Keyword>)> {
    let categories = match User::categories(&state.mm, user_id).await {
        Ok(categories) => categories,
        Err(err) => {
            error!("Error fetching recipe categories: {err}");
            let toast = MessageHtmx::error("Error fetching recipe categories.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json.into())).await;
            }
            return Err(Error::Database);
        }
    };

    let keywords = match User::keywords(&state.mm, user_id).await {
        Ok(keywords) => keywords,
        Err(err) => {
            error!("Error fetching recipe keywords: {err}");
            let toast = MessageHtmx::error("Error fetching recipe keywords.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state.broadcast(user_id, Message::Text(json.into())).await;
            }
            return Err(Error::Database);
        }
    };

    Ok((categories, keywords))
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
        let toast = MessageHtmx::error("Failed to add recipe category.");
        if let Ok(json) = serde_json::to_string(&toast) {
            state.broadcast(user_id, Message::Text(json.into())).await;
        }
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
        let toast = MessageHtmx::error("Category cannot be empty or uncategorized.");
        if let Ok(json) = serde_json::to_string(&toast) {
            state.broadcast(user_id, Message::Text(json.into())).await;
        }
        return Error::InvalidPayload.into_response();
    }

    if let Err(err) = Recipe::delete_recipe_category(&state.mm, &category, user_id).await {
        error!("Error deleting recipe category: {err}");
        let toast = MessageHtmx::error("Failed to delete recipe category.");
        if let Ok(json) = serde_json::to_string(&toast) {
            state.broadcast(user_id, Message::Text(json.into())).await;
        }
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
    let view = vec![ViewRecipe {
        recipe_details: recipe,
        formatted_times,
    }];

    templates::recipes::view_recipe(
        uri.path(),
        state.data_dir,
        Data {
            is_admin: user_id == 1,
            is_authenticated: true,
            is_autologin: state.config.is_autologin,
            is_hx_request: is_hx_request(&header_map),
            about: AboutData {
                is_update_available: false,
            },
            pagination: Some(PaginationData {
                left: vec![],
                middle: vec![],
                right: vec![],
                prev: 0,
                selected: 0,
                next: 0,
                htmx: PaginationHtmxData {
                    is_swap: false,
                    target: "".to_string(),
                },
                search: PaginationSearchData { current_page: 0 },
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
            recipes: view,
        },
    )
}

/// Handles the supported applications endpoint.
pub async fn supported_applications_handler(_ctx: CtxW) -> impl IntoResponse {
    let applications = [
        ("AccuChef", "https://www.accuchef.com"),
        ("ChefTap", "https://cheftap.com"),
        ("Crouton", "https://crouton.app"),
        (
            "Easy Recipe Deluxe",
            "https://easy-recipe-deluxe.software.informer.com",
        ),
        ("Kalorio", "https://www.kalorio.de"),
        ("MasterCook", "https://www.mastercook.com"),
        ("Paprika", "https://www.paprikaapp.com"),
        ("Recipe Keeper", "https://recipekeeperonline.com"),
        ("RecipeSage", "https://recipesage.com"),
        ("Saffron", "https://www.mysaffronapp.com"),
    ];

    let mut html = String::new();

    for (i, (name, url)) in applications.into_iter().enumerate() {
        html.push_str(r#"<tr class="text-center">"#);
        let _ = write!(html, "<td>{}</td>", i + 1);
        let _ = write!(
            html,
            r#"<td><a class="underline" href="{url}" target="_blank">{name}</a></td>"#
        );
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
            let toast = MessageHtmx::error("Error fetching supported websites.");
            if let Ok(json) = serde_json::to_string(&toast) {
                state
                    .broadcast(ctx.0.user_id(), Message::Text(json.into()))
                    .await;
            }
            Error::Database.into_response()
        }
    }
}
