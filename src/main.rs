use clap::{Parser, Subcommand};
use recipya::run_server;

#[derive(Parser)]
#[command(name = "Recipya")]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
   #[command(about = "Starts the web server")]
    Serve,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Serve) => {
            run_server().await;
        }
        None => {}
    }
}
