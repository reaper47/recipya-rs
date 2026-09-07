use std::sync::Arc;

use rand::{RngExt, distr::Alphanumeric};
use tokio::net::TcpListener;
use tokio::signal;
use tokio_cron_scheduler::{Job, JobScheduler};
use tower_cookies::CookieManagerLayer;
use tracing::{error, info};

use app::jobs::clean_media;
use app::state::AppState;
use config::{AutologinState, Config, DataDir};
use models::{
    nutrition::NutritionDataSource,
    tokens::EmailVerificationToken,
    user::{User, UserForCreate},
};
use recipya_scraper::AppHttpClient;
use repository::ModelManager;
use router::copy_to_fs;
use router::router;
use support::fs::{AppFs, FsSupport, get_base_dir};

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

    copy_assets_to_fs()?;

    if config.states.autologin == AutologinState::On
        && let Err(err) = init_autologin_user(&state.mm).await
    {
        error!(?err, "Autologin enabled. Error initializing autologin user");
    }

    start_cron_jobs(
        Arc::new(state.clone().mm),
        Arc::new(state.clone().data_dir),
        Arc::clone(&state.fs_support),
    )
    .await?;

    let mm = state.mm.clone();
    tokio::spawn(async move {
        NutritionDataSource::update_all(&mm).await;
    });

    let router = router(state.clone())?
        .layer(CookieManagerLayer::new())
        .with_state(state.clone());

    let listener = TcpListener::bind("0.0.0.0:8078").await?;
    info!("Serving at http://{}", listener.local_addr()?);
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn copy_assets_to_fs() -> Result<()> {
    let placeholders = get_base_dir()?.join("Media/Images/Placeholders");
    let placeholder_recipe = "img/recipes/placeholder.webp";
    for (src, dest) in [
        (placeholder_recipe, "placeholder.recipe.webp"),
        (placeholder_recipe, "placeholder.recipe.original.webp"),
    ] {
        let dest = placeholders.join(dest);
        if let Err(err) = copy_to_fs(src, dest)
            && !matches!(err, router::Error::FileExists)
        {
            return Err(Error::Server(err.to_string()));
        }
    }

    let icon = get_base_dir()?.join("Media/Images/Icon");
    for (src, dest) in [
        (
            "img/icon/android-chrome-192x192.png",
            "android-chrome-192x192.png",
        ),
        (
            "img/icon/android-chrome-512x512.png",
            "android-chrome-512x512.png",
        ),
        ("img/icon/apple-touch-icon.png", "apple-touch-icon.png"),
        ("img/icon/favicon.ico", "favicon.ico"),
        ("img/icon/favicon-16x16.png", "favicon-16x16.png"),
        ("img/icon/favicon-32x32.png", "favicon-32x32.png"),
        ("img/icon/mstile-150x150.png", "mstile-150x150.png"),
        ("img/icon/safari-pinned-tab.svg", "safari-pinned-tab.svg"),
    ] {
        let dest = icon.join(dest);
        if let Err(err) = copy_to_fs(src, dest)
            && !matches!(err, router::Error::FileExists)
        {
            return Err(Error::Server(err.to_string()));
        }
    }

    Ok(())
}

async fn init_autologin_user(mm: &ModelManager) -> Result<()> {
    let admin_email = "admin@autologin.com".to_string();

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

            let created_msg = format!(
                "Admin user created with username '{admin_email}' and password '{password}'. Please jot the credentials down as they won't be shown again."
            );

            match User::new(
                mm,
                UserForCreate {
                    email: admin_email,
                    password_clear: password,
                },
            )
            .await
            {
                Ok(user) => {
                    info!(created_msg);
                    if let Err(err) = EmailVerificationToken::verify_user_email(mm, user.id).await {
                        error!(?err, "Error confirming autologin user");
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

    let mm_clean = Arc::clone(&mm);

    info!(
        "Scheduled cron job 'CleanMedia' every Sunday at midnight to clean dangling media resources"
    );
    sched
        .add(Job::new_async("0 0 0 * * 7", move |_uuid, _l| {
            info!("Running CleanMedia job");

            let mm = Arc::clone(&mm_clean);
            let data_dir = Arc::clone(&data_dir);
            let fs_support = Arc::clone(&fs_support);

            Box::pin(async move {
                if let Err(err) = clean_media(mm, data_dir, fs_support).await {
                    error!(?err, "CleanMedia: Failed to run job");
                }
            })
        })?)
        .await?;

    info!(
        "Scheduled cron job 'UpdateNutritionDataSources' on the first day of every month to update the nutrition data sources."
    );
    sched
        .add(Job::new_async("0 0 0 1 * *", move |_uuid, _l| {
            info!("Running UpdateNutritionDataSources job");

            let mm = Arc::clone(&mm);

            Box::pin(async move { NutritionDataSource::update_all(&mm).await })
        })?)
        .await?;

    sched.start().await?;
    Ok(())
}

#[allow(clippy::ignored_unit_patterns)]
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = async {
        cfg_select! {
            unix => {
                signal::unix::signal(signal::unix::SignalKind::terminate())
                    .expect("Failed to install signal handler.")
                    .recv()
                    .await;
            },
            _ => {
                std::future::pending::<()>().await;
            }
        }
    };

    tokio::select! {
        () = ctrl_c => {},
        _ = terminate => {}
    }
}
