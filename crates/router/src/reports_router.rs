use axum::{Router, middleware::from_fn_with_state, routing::get};

use app::state::AppState;

use crate::{
    handlers::reports::{report_handler, reports_handler, reports_list_handler},
    middleware::mw_auth::mw_refresh_token,
};

/// Defines the routes for endpoints related to the reports module.
#[allow(clippy::literal_string_with_formatting_args)]
pub fn reports_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(reports_handler))
        .route("/{:report_id}", get(report_handler))
        .route("/list", get(reports_list_handler))
        .layer(from_fn_with_state(state.clone(), mw_refresh_token))
}
