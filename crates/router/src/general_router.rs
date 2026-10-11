use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};

use app::state::AppState;

use crate::handlers::general as h;
use crate::middleware::mw_auth::mw_refresh_token;

/// Defines the routes for general endpoints of the web application.
pub fn general_routes(state: &AppState) -> Router<AppState> {
    let protected = Router::new()
        .route("/download", get(h::download_handler))
        .route("/fetch", get(h::fetch_handler))
        .route("/paper-sizes", get(h::paper_sizes_handler))
        .route("/search-suggestions", get(h::search_suggestions_handler))
        .route("/sse", get(h::sse_handler))
        .route(
            "/upload/note-image",
            post(h::upload_note_image).layer(DefaultBodyLimit::max(10 * 1024 * 1024)),
        )
        .route("/user-initials", get(h::user_initials_handler))
        .layer(from_fn_with_state(state.clone(), mw_refresh_token));

    Router::new()
        .route("/", get(h::index_handler))
        .route("/health/live", get(h::health_live_handler))
        .route("/health/ready", get(h::health_ready_handler))
        .merge(protected)
}
