use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use config::Config;
use router::static_files_router::static_files_routes;
use test_db::{db_url, default_config};
use test_harness::create_app_state;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::test]
async fn test_static_files_routes_ok() -> Result<()> {
    let config = Config {
        database_url: db_url().clone(),
        ..default_config()
    };
    let state = create_app_state(config).await;
    let app = static_files_routes(&state).with_state(state);

    for (path, want_status) in [
        (
            "/public/img/icon/android-chrome-192x192.png",
            StatusCode::OK,
        ),
        (
            "/public/img/icon/android-chrome-512x512.png",
            StatusCode::OK,
        ),
        ("/public/img/icon/apple-touch-icon.png", StatusCode::OK),
        ("/public/img/icon/favicon.ico", StatusCode::OK),
        ("/public/img/icon/favicon-16x16.png", StatusCode::OK),
        ("/public/img/icon/favicon-32x32.png", StatusCode::OK),
        ("/public/img/icon/mstile-150x150.png", StatusCode::OK),
        ("/public/img/icon/safari-pinned-tab.svg", StatusCode::OK),
        ("/public/robots.txt", StatusCode::OK),
        ("/public/site.webmanifest", StatusCode::OK),
        ("/public/browserconfig.xml", StatusCode::OK),
        ("/public/chicken-meat-pie.png", StatusCode::NOT_FOUND),
    ] {
        let res = app
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty())?)
            .await?;

        pretty_assertions::assert_eq!(res.status(), want_status, "Failed on route: {}", path);
    }

    Ok(())
}
