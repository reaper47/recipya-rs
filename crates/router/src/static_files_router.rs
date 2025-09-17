use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

use crate::handlers::static_files::static_files_handler;
use app::state::AppState;

/// Defines the routes for serving static files.
pub fn static_files_routes(state: AppState) -> Router<AppState> {
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
        .nest_service("/data/images", ServeDir::new(state.data_dir.images.root))
        .nest_service(
            "/data/images/thumbnails",
            ServeDir::new(state.data_dir.images.thumbnails),
        )
        .nest_service("/data/videos", ServeDir::new(state.data_dir.videos))
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use config::Config;
    use testing::utils::{create_app_state, default_config, test_database_url};
    use tower::ServiceExt;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_static_files_routes_ok() -> Result<()> {
        let config = Config {
            database_url: test_database_url(),
            ..default_config()
        };
        let state = create_app_state(config).await;
        let app = static_files_routes(state.clone()).with_state(state);
        let test_cases = vec![
            ("/android-chrome-192x192.png", StatusCode::OK),
            ("/android-chrome-512x512.png", StatusCode::OK),
            ("/apple-touch-icon.png", StatusCode::OK),
            ("/browserconfig.xml", StatusCode::OK),
            ("/favicon.ico", StatusCode::OK),
            ("/favicon-16x16.png", StatusCode::OK),
            ("/favicon-32x32.png", StatusCode::OK),
            ("/mstile-150x150.png", StatusCode::OK),
            ("/safari-pinned-tab.svg", StatusCode::OK),
            ("/site.webmanifest", StatusCode::OK),
            ("/public/chicken-meat-pie.png", StatusCode::NOT_FOUND),
        ];
        for (path, want_status) in test_cases {
            let res = app
                .clone()
                .oneshot(Request::builder().uri(path).body(Body::empty())?)
                .await?;

            pretty_assertions::assert_eq!(res.status(), want_status, "Failed on route: {}", path);
        }

        Ok(())
    }
}
