mod error;
mod server;
mod sponsors;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use rustls::crypto::ring;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

use repository::create_database_if_not_exists;
use support::software;

use error::Result;
use server::server;
use sponsors::generate_sponsors_image;

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
    if let Err(err) = dotenv() {
        warn!("Could not load .env file ({err:?}). This is expected when environment variables are injected by the host");
    }
    init_crypto();
    init_tracing()?;

    match Cli::parse().command {
        Commands::Server => {
            info!(
                "Recipya v{} starting in {} mode",
                env!("CARGO_PKG_VERSION"),
                if cfg!(debug_assertions) {
                    "debug"
                } else {
                    "release"
                }
            );

            create_database_if_not_exists("recipya")?;

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

                warn!("{message}");
            }

            server().await?;
        }
        Commands::Sponsors => {
            generate_sponsors_image();
        }
    }

    Ok(())
}

fn init_crypto() {
    ring::default_provider()
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
