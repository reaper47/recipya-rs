use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};

use app::state::AppState;

use crate::handlers::general::{
    download_handler, fetch_handler, health_live_handler, health_ready_handler, index_handler,
    paper_sizes_handler, search_suggestions_handler, sse_handler, upload_note_image,
    user_initials_handler,
};
use crate::middleware::mw_auth::mw_refresh_token;

/// Defines the routes for general endpoints of the web application.
pub fn general_routes(state: &AppState) -> Router<AppState> {
    let protected = Router::new()
        .route("/download", get(download_handler))
        .route("/fetch", get(fetch_handler))
        .route("/paper-sizes", get(paper_sizes_handler))
        .route("/search-suggestions", get(search_suggestions_handler))
        .route("/sse", get(sse_handler))
        .route(
            "/upload/note-image",
            post(upload_note_image).layer(DefaultBodyLimit::max(10 * 1024 * 1024)),
        )
        .route("/user-initials", get(user_initials_handler))
        .layer(from_fn_with_state(state.clone(), mw_refresh_token));

    Router::new()
        .route("/", get(index_handler))
        .route("/health/live", get(health_live_handler))
        .route("/health/ready", get(health_ready_handler))
        .merge(protected)
}
