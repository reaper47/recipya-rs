use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use rand::Rng;
use rand::distr::Alphanumeric;
use tokio::net::TcpListener;
use tokio::signal;
use tokio_cron_scheduler::{Job, JobScheduler};
use tower_cookies::CookieManagerLayer;
use tracing::{error, info};

use app::jobs::clean_media;
use app::state::AppState;
use config::{Config, DataDir};
use models::{
    nutrition::all_nutrition_sources,
    user::{User, UserForCreate},
};
use recipya_scraper::AppHttpClient;
use repository::ModelManager;
use router::middleware::mw_auth::mw_ctx_resolver;
use router::router;
use support::fs::{AppFs, FsSupport};

use crate::error::{Error, Result};

/// Initializes and starts Recipya's web server.
pub async fn server() -> Result<()> {
    let config = Config::load_from_env()?;

    let state = AppState::new(
        config.clone(),
        Arc::new(AppHttpClient::default()),
        Arc::new(AppFs),
    )
    .await
    .map_err(|err| Error::Server(err.to_string()))?;

    if config.is_autologin
        && let Err(err) = init_autologin_user(&state.mm).await
    {
        error!("Autologin enabled. Error initializing autologin user: {err}");
    }

    start_cron_jobs(
        Arc::new(state.clone().mm),
        Arc::new(state.clone().data_dir),
        Arc::clone(&state.fs_support),
    )
    .await?;

    info!("Updating nutrition data sources");
    for source in all_nutrition_sources() {
        let _ = source.update_data(&state.mm).await;
    }

    let router = router(state.clone())
        .await?
        .layer(from_fn_with_state(state.clone(), mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .with_state(state.clone());

    let listener = TcpListener::bind("0.0.0.0:8078").await?;
    info!("Serving at http://{}", listener.local_addr()?);
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn init_autologin_user(mm: &ModelManager) -> Result<()> {
    let admin_email: String = "admin@autologin.com".into();

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

async fn start_cron_jobs(
    mm: Arc<ModelManager>,
    data_dir: Arc<DataDir>,
    fs_support: Arc<dyn FsSupport + Sync + Send>,
) -> Result<()> {
    let sched = JobScheduler::new().await?;

    info!(
        "Scheduled cron job 'CleanMedia' every Sunday at midnight to clean dangling media resources"
    );
    sched
        .add(Job::new_async("0 0 0 * * 7", move |_uuid, _l| {
            info!("Running CleanMedia job");

            let mm = Arc::clone(&mm);
            let data_dir = Arc::clone(&data_dir);
            let fs_support = Arc::clone(&fs_support);

            Box::pin(async move {
                if let Err(err) = clean_media(mm, data_dir, fs_support).await {
                    error!("CleanMedia: Failed to run job: {err}");
                }
            })
        })?)
        .await?;

    sched.start().await?;
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
