mod config;
mod error;
mod web;

pub use self::error::{Error, Result};

use std::net::SocketAddr;

use axum::{middleware, Router};
use config::web_config;
use lib_web::{middleware::{mw_auth::mw_ctx_resolve, mw_res_map::mw_reponse_map}, routes::routes_static};
use tokio::{net::TcpListener, signal};
use tower_cookies::CookieManagerLayer;
use tracing::info;

use lib_core::model::ModelManager;
use tracing_subscriber::EnvFilter;

use crate::web::{routes_auth, routes_general};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mm = ModelManager::new().await?;

    /* TODO: Figure out how to make this work
    let routes_rpc = web::routes_rpc::routes(mm.clone())
        .route_layer(middleware::from_fn(mw_ctx_require));*/

    let routes_all = Router::new()
        // .nest("/api", routes_rpc)
        .nest("/auth", routes_auth::routes(mm.clone()))
        .merge(routes_general::routes())
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir(&web_config().WEB_FOLDER));

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 7324)))
        .await
        .unwrap();
    info!(
        "Serving HTTP server at address http://{}",
        listener.local_addr().unwrap().to_string()
    );
    axum::serve(listener, routes_all)
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
