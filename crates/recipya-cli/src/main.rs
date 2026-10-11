mod cli;
mod error;
mod l10n;
mod sandbox;
mod server;
mod sponsors;

use dotenvy::dotenv;
use rustls::crypto::ring;
use tracing::warn;
use tracing_subscriber::EnvFilter;

use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::parse_args()?;

    if let Err(err) = dotenv() {
        warn!(
            "Could not load .env file ({err}). This is expected when environment variables are injected by the host"
        );
    }
    init_crypto();
    init_tracing()?;

    args.run_command().await
}

fn init_crypto() {
    ring::default_provider()
        .install_default()
        .expect("failed to install crypto provider");
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
