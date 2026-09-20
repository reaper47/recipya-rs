use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

use app::state::AppState;

use crate::handlers::static_files::static_files_handler;

/// Defines the routes for serving static files.
pub fn static_files_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/android-chrome-192x192.png", get(static_files_handler))
        .route("/android-chrome-512x512.png", get(static_files_handler))
        .route("/apple-touch-icon.png", get(static_files_handler))
        .route("/browserconfig.xml", get(static_files_handler))
        .route("/favicon.ico", get(static_files_handler))
        .route("/favicon-16x16.png", get(static_files_handler))
        .route("/favicon-32x32.png", get(static_files_handler))
        .route("/mstile-150x150.png", get(static_files_handler))
        .route("/safari-pinned-tab.svg", get(static_files_handler))
        .route("/site.webmanifest", get(static_files_handler))
        .route("/public/{*file}", get(static_files_handler))
        .nest_service("/data/images", ServeDir::new(&state.data_dir.images.root))
        .nest_service(
            "/data/images/thumbnails",
            ServeDir::new(&state.data_dir.images.thumbnails),
        )
        .nest_service("/data/videos", ServeDir::new(&state.data_dir.videos))
}
