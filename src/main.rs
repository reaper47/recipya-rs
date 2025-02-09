mod cli;
mod core;
mod error;
mod server;

use clap::{Parser, Subcommand};
use dotenvy::dotenv;

use crate::cli::server::server;
use crate::cli::sponsors::generate_sponsors_image;
use crate::error::Result;

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
    dotenv().ok();

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    match Cli::parse().command {
        Commands::Server => {
            server().await?;
            Ok(())
        }
        Commands::Sponsors => {
            generate_sponsors_image()?;
            Ok(())
        }
    }
}
