use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::{Router, middleware};

use crate::AppState;
use crate::handlers::recipes as h;
use crate::middleware::mw_auth::mw_refresh_token;

const FIFTY_MB: usize = 50 * 1024 * 1024;

/// Defines the routes for endpoints related to recipes.
pub fn recipes_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(h::view::recipes_handler))
        .route("/schema", get(h::schema::recipe_schema_handler))
        .route(
            "/{recipe_id}",
            get(h::view::view_recipe_handler).delete(h::delete::delete_recipe_handler),
        )
        .route(
            "/{recipe_id}/duplicate",
            get(h::duplicate::duplicate_recipe_handler),
        )
        .route(
            "/{recipe_id}/edit",
            get(h::edit::edit_recipe_handler)
                .put(h::edit::edit_recipe_put_handler)
                .layer(DefaultBodyLimit::max(FIFTY_MB)),
        )
        .route(
            "/{recipe_id}/favourite",
            post(h::favourite::toggle_favourite_handler),
        )
        .route(
            "/{recipe_id}/rescrape",
            get(h::rescrape::recrape_recipe_handler).put(h::rescrape::recrape_recipe_put_handler),
        )
        .route("/{recipe_id}/scale", get(h::scale::scale_recipe_handler))
        .route(
            "/{recipe_id}/share",
            post(h::share::share_recipe_post_handler),
        )
        .route(
            "/{recipe_id}/timeline",
            get(h::timeline::timeline_get_handler)
                .post(h::timeline::timeline_post_handler)
                .layer(DefaultBodyLimit::max(FIFTY_MB)),
        )
        .route(
            "/{recipe_id}/timelines/{timeline_id}",
            get(h::timeline::timeline_event_get_handler).put(h::timeline::timeline_put_handler),
        )
        .route(
            "/{recipe_id}/timelines/{timeline_id}/edit",
            get(h::timeline::timeline_event_get_edit_handler),
        )
        .layer(DefaultBodyLimit::max(FIFTY_MB))
        .route("/add", get(h::add::add_recipes_handler))
        .route(
            "/add/import/api",
            post(h::add::add_recipe_import_api_handler),
        )
        .route(
            "/add/import/app",
            post(h::add::add_recipe_import_app_handler).layer(DefaultBodyLimit::max(2 * FIFTY_MB)),
        )
        .route(
            "/add/import/preview",
            post(h::add::add_recipe_import_preview_handler),
        )
        .route(
            "/add/import/raw-json",
            post(h::add::add_recipe_import_raw_handler),
        )
        .route(
            "/add/manual",
            get(h::add::add_manual_recipe_handler)
                .post(h::add::add_manual_recipe_post_handler)
                .layer(DefaultBodyLimit::max(FIFTY_MB)),
        )
        .route("/add/website", post(h::add::add_website_post_handler))
        .route("/bulk", post(h::delete::bulk_delete_recipes_handler))
        .route("/bulk/categorise", post(h::categories::bulk_update_handler))
        .route(
            "/categories",
            get(h::categories::get_all_handler)
                .post(h::categories::post_recipe_categories_handler)
                .delete(h::categories::delete_recipe_categories_handler),
        )
        .route("/search", get(h::search::search_recipes_handler))
        .route(
            "/supported-applications",
            get(h::supported::supported_applications_handler),
        )
        .route(
            "/supported-websites",
            get(h::supported::supported_websites_handler),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_refresh_token,
        ))
}
