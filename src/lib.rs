//! Recipya is a clean, simple and powerful recipe manager your whole family will enjoy.

use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Router};
use tokio::signal;

mod app;
mod web;

/// Starts the web server.
pub async fn run_server() {
    let app = Arc::new(app::App::new().await);
    let addr = app.address(true);

    let router = Router::new()
        .merge(web::routes::general::routes(Arc::clone(&app)))
        .merge(web::routes::auth::routes(app))
        .fallback(handler_404);

    let listener = tokio::net::TcpListener::bind(addr.trim_start_matches("http://"))
        .await
        .expect("failed to start server");

    println!("\nServing HTTP server at address {addr}");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        web::templates::general::simple(
            "Page Not Found",
            "The page you requested to view is not found. Please go back to the main page.",
        ),
    )
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
