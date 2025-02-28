use std::net::SocketAddr;

use axum::middleware::from_fn_with_state;
use tokio::net::TcpListener;
use tokio::signal;
use tower_cookies::CookieManagerLayer;
use tracing::info;

use crate::core::config::Config;
use crate::error::{Error, Result};
use crate::server::router::middleware::mw_auth::mw_ctx_resolver;
use crate::server::{AppState, router};

/// Initializes and starts Recipya's web server.
pub async fn server() -> Result<()> {
    let config = Config::load_from_env().unwrap();

    let state = AppState::new(config)
        .await
        .map_err(|err| Error::Server(err.to_string()))?;

    let router = router(state.clone())
        .await?
        .layer(from_fn_with_state(state.clone(), mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .with_state(state.clone());

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 8078))).await?;
    info!("Serving at http://{}", listener.local_addr()?);
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler.")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }
}
