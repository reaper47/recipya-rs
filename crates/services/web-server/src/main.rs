use std::net::SocketAddr;

use axum::{http::StatusCode, middleware, response::IntoResponse, Router};
use tokio::{net::TcpListener, signal};
use tower_cookies::CookieManagerLayer;
use tracing::info;

use lib_core::model::ModelManager;
use web::routes_rpc::RpcState;

use crate::web::{
    mw_auth::{mw_ctx_require, mw_ctx_resolve},
    mw_res_map::mw_reponse_map,
    {routes_auth, routes_general},
};

pub use self::error::{Error, Result};

mod error;
mod log;
pub mod web;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_target(false).init();

    let mm = ModelManager::new().await?;

    let rpc_state = RpcState { mm: mm.clone() };
    let routes_rpc =
        web::routes_rpc::routes(rpc_state).route_layer(middleware::from_fn(mw_ctx_require));

    let routes_all = Router::new()
        .nest("/api", routes_rpc)
        .nest("/auth", routes_auth::routes(mm.clone()))
        .merge(routes_general::routes())
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback(handler_404);

    let addr = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 7324)))
        .await
        .unwrap();
    info!(
        "Serving HTTP server at address http://{}",
        addr.local_addr().unwrap().to_string()
    );
    axum::serve(addr, routes_all)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
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
