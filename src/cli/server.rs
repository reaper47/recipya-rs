use std::net::SocketAddr;

use axum::middleware::from_fn_with_state;
use rand::Rng;
use rand::distr::Alphanumeric;
use tokio::net::TcpListener;
use tokio::signal;
use tower_cookies::CookieManagerLayer;
use tracing::{error, info};

use crate::core::config::Config;
use crate::core::model::user::{User, UserForCreate};
use crate::core::repository::ModelManager;
use crate::error::{Error, Result};
use crate::server::router::middleware::mw_auth::mw_ctx_resolver;
use crate::server::{AppState, router};

/// Initializes and starts Recipya's web server.
pub async fn server() -> Result<()> {
    let config = Config::load_from_env().unwrap();

    let state = AppState::new(config.clone())
        .await
        .map_err(|err| Error::Server(err.to_string()))?;

    if config.is_autologin {
        if let Err(err) = init_autologin_user(&state.mm).await {
            error!("Autologin enabled. Error initializing autologin user: {err}");
        }
    }

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

async fn init_autologin_user(mm: &ModelManager) -> Result<()> {
    let admin_email = String::from("admin@autologin.com");

    match User::get_user_by_email(mm, &admin_email).await {
        Ok(Some(_)) => {
            info!("Admin user with email 'admin@autologin.com' exists");
            Ok(())
        }
        Ok(None) => {
            let password = rand::rng()
                .sample_iter(&Alphanumeric)
                .take(18)
                .map(char::from)
                .collect::<String>();

            match User::new(
                mm,
                UserForCreate {
                    email: admin_email.clone(),
                    password_clear: password.clone(),
                },
            )
            .await
            {
                Ok(user) => {
                    info!(
                        "Admin user created with username '{admin_email}' and password '{password}'. Please jot the credentials down as they won't be shown again."
                    );
                    if let Err(err) = user.set_is_confirmed(mm).await {
                        error!("Error confirming autologin user: {err}");
                        return Err(Error::Server(err.to_string()));
                    }
                    Ok(())
                }
                Err(err) => Err(Error::Server(err.to_string())),
            }
        }
        Err(err) => Err(Error::Server(err.to_string())),
    }
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
