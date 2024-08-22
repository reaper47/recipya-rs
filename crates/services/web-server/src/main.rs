mod config;
mod error;
mod web;

pub use self::error::{Error, Result};

use std::net::SocketAddr;

use lib_web::AppState;
use tokio::{net::TcpListener, signal};
use tracing::info;

use recipya::routes_all;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 7324)))
        .await
        .unwrap();
    info!(
        "Serving HTTP server at address http://{}",
        listener.local_addr().unwrap().to_string()
    );

    let state = AppState::new().await.unwrap();
    axum::serve(listener, routes_all(state).await.unwrap())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
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
