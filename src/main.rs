mod cli;
mod core;
mod error;
mod server;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

use crate::cli::server::server;
use crate::cli::sponsors::generate_sponsors_image;
use crate::core::config::get_base_dir;
use crate::core::repository::create_database_if_not_exists;
use crate::core::support::software;
use crate::error::Result;
use crate::server::router::copy_to_fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts the Recipya web server
    Server,
    /// Generate the GitHub sponsors image
    Sponsors,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_crypto();
    dotenv().ok();
    init_tracing()?;

    tracing::info!(
        "Recipya v{} starting in {} mode",
        env!("CARGO_PKG_VERSION"),
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    );

    create_database_if_not_exists("recipya")?;
    copy_assets_to_fs()?;

    if software::is_ffmpeg_installed() {
        info!("FFmpeg is installed");
    } else {
        let mut message = String::from("FFmpeg is not installed. ");

        if cfg!(target_os = "macos") {
            message.push_str("Please execute: brew install ffmpeg");
        } else if cfg!(target_os = "linux") {
            message.push_str("Please consult your package manager to install it.");
        } else if cfg!(target_os = "windows") {
            message.push_str("Please install from https://www.gyan.dev/ffmpeg/builds");
        }

        warn!(message);
    }

    match Cli::parse().command {
        Commands::Server => {
            server().await?;
        }
        Commands::Sponsors => {
            generate_sponsors_image()?;
        }
    }

    Ok(())
}

fn init_crypto() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install crypto provider");
}

fn init_tracing() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(false)
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

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
        if let Err(err) = copy_to_fs(src, dest) {
            if !matches!(err, crate::server::Error::FileExists) {
                return Err(crate::error::Error::Server(err.to_string()));
            }
        }
    }

    Ok(())
}
