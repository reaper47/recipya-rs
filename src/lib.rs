//! Recipya is a clean, simple and powerful recipe manager your whole family will enjoy.

use axum::response::{IntoResponse, Redirect};
use axum::Router;
use axum::routing::get;
use tokio::signal;
use tower_http::services::ServeDir;

use crate::app::App;

mod app;

/// Starts the web server.
pub async fn run_server() {
    let app = App::new().await;

    let router = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route(
            "/guide/auth/login",
            get(|| async { Redirect::permanent("/auth/login") }),
        )
        .nest_service("/guide", ServeDir::new("docs/public"));

    let addr = app.address(true);
    let listener = tokio::net::TcpListener::bind(addr.trim_start_matches("http://"))
        .await
        .expect("failed to start server");

    println!("\nServing HTTP server at address {addr}");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler.")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }

    println!("Signal received. Starting graceful shutdown.")
}
