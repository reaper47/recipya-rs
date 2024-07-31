//! Recipya is a clean, simple and powerful recipe manager your whole family will enjoy.
use axum::{http::StatusCode, response::IntoResponse, Router};
use tokio::signal;

pub use config::config;
pub use error::{Error, Result};

mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod services;

pub mod model;
mod schema;
mod utils;
pub mod web;


/// Starts the web server.
pub async fn run_server() {
    let mm = model::ModelManager::new().await.unwrap();

    let router = Router::new()
        .merge(web::routes(mm.clone()))
        .fallback(handler_404);

    let addr = &config().ADDRESS;
    let listener = tokio::net::TcpListener::bind(addr.trim_start_matches("http://"))
        .await
        .unwrap();

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
