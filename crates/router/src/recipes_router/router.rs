use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::{Router, middleware};

use crate::AppState;
use crate::handlers::recipes::{
    add_manual_recipe_handler, add_manual_recipe_post_handler, add_recipe_import_api_handler,
    add_recipe_import_app_handler, add_recipe_import_preview_handler,
    add_recipe_import_raw_handler, add_recipes_handler, add_website_post_handler,
    delete_recipe_categories_handler, delete_recipe_handler, duplicate_recipe_handler,
    edit_recipe_handler, edit_recipe_put_handler, post_recipe_categories_handler,
    recipe_schema_handler, recipes_handler, recrape_recipe_handler, recrape_recipe_put_handler,
    scale_recipe_handler, search_recipes_handler, share_recipe_post_handler,
    supported_applications_handler, supported_websites_handler, timeline_event_get_edit_handler,
    timeline_event_get_handler, timeline_get_handler, timeline_post_handler, timeline_put_handler,
    toggle_favourite_handler, view_recipe_handler,
};
use crate::middleware::mw_auth::mw_refresh_token;

const FIFTY_MB: usize = 50 * 1024 * 1024;

/// Defines the routes for endpoints related to recipes.
pub fn recipes_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(recipes_handler))
        .route("/schema", get(recipe_schema_handler))
        .route(
            "/{recipe_id}",
            get(view_recipe_handler).delete(delete_recipe_handler),
        )
        .route("/{recipe_id}/duplicate", get(duplicate_recipe_handler))
        .route(
            "/{recipe_id}/edit",
            get(edit_recipe_handler)
                .put(edit_recipe_put_handler)
                .layer(DefaultBodyLimit::max(FIFTY_MB)),
        )
        .route("/{recipe_id}/favourite", post(toggle_favourite_handler))
        .route(
            "/{recipe_id}/rescrape",
            get(recrape_recipe_handler).put(recrape_recipe_put_handler),
        )
        .route("/{recipe_id}/scale", get(scale_recipe_handler))
        .route("/{recipe_id}/share", post(share_recipe_post_handler))
        .route(
            "/{recipe_id}/timeline",
            get(timeline_get_handler)
                .post(timeline_post_handler)
                .layer(DefaultBodyLimit::max(FIFTY_MB)),
        )
        .route(
            "/{recipe_id}/timelines/{timeline_id}",
            get(timeline_event_get_handler).put(timeline_put_handler),
        )
        .route(
            "/{recipe_id}/timelines/{timeline_id}/edit",
            get(timeline_event_get_edit_handler),
        )
        .layer(DefaultBodyLimit::max(FIFTY_MB))
        .route("/add", get(add_recipes_handler))
        .route("/add/import/api", post(add_recipe_import_api_handler))
        .route(
            "/add/import/app",
            post(add_recipe_import_app_handler).layer(DefaultBodyLimit::max(FIFTY_MB)),
        )
        .route(
            "/add/import/preview",
            post(add_recipe_import_preview_handler),
        )
        .route("/add/import/raw-json", post(add_recipe_import_raw_handler))
        .route(
            "/add/manual",
            get(add_manual_recipe_handler)
                .post(add_manual_recipe_post_handler)
                .layer(DefaultBodyLimit::max(FIFTY_MB)),
        )
        .route("/add/website", post(add_website_post_handler))
        .route(
            "/categories",
            post(post_recipe_categories_handler).delete(delete_recipe_categories_handler),
        )
        .route("/search", get(search_recipes_handler))
        .route(
            "/supported-applications",
            get(supported_applications_handler),
        )
        .route("/supported-websites", get(supported_websites_handler))
        .layer(middleware::from_fn_with_state(state, mw_refresh_token))
}
