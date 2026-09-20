use axum::routing::{get, post};
use axum::{Router, middleware};

use app::state::AppState;

use crate::handlers::settings::{
    export_data_handler, export_data_post_handler, set_bold_ingredients_handler,
    set_default_theme_handler, set_nutrition_source_handler, set_paper_size_handler,
    set_selected_theme_handler, set_selected_timezone_handler, settings_handler,
};
use crate::middleware::mw_auth::{mw_only_admin, mw_refresh_token};

/// Defines the routes for endpoints related to the settings module.
pub fn settings_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(settings_handler))
        .route("/bold-ingredients", post(set_bold_ingredients_handler))
        .route(
            "/export-data",
            get(export_data_handler).post(export_data_post_handler),
        )
        .route("/nutrition/source", post(set_nutrition_source_handler))
        .route("/paper-size", post(set_paper_size_handler))
        .route(
            "/theme-default",
            post(set_default_theme_handler)
                .layer(middleware::from_fn_with_state(state.clone(), mw_only_admin)),
        )
        .route("/theme-selected", post(set_selected_theme_handler))
        .route("/tz", post(set_selected_timezone_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_refresh_token,
        ))
}
