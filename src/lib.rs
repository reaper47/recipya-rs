//! Recipya is a clean, simple and powerful recipe manager your whole family will enjoy.

mod app;

use axum::Router;
use axum::routing::get;
use tokio::signal;
use crate::app::App;

/// Starts the web server.
pub async fn run_server() {
    let app = App::new().await;

    let router = Router::new().route("/", get(|| async { "Hello, world!" }));
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
