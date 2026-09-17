mod error;
mod server;
mod sponsors;

use dotenvy::dotenv;
use lexopt::Arg::{Long, Short, Value};
use rustls::crypto::ring;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

use repository::create_database_if_not_exists;
use support::software;

use error::Result;
use server::server;
use sponsors::generate_sponsors_image;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BIN_NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const SERVER: &str = "server";
const SPONSORS: &str = "sponsors";

#[derive(Default)]
struct Args {
    command: Option<Command>,
}

impl Args {
    fn sandbox(&self) {
        if let Some(cmd) = &self.command {
            match cmd {
                Command::Server | Command::Sponsors => sandbox(),
                Command::Help(_) | Command::Version => {}
            }
        }
    }
}

enum Command {
    Server,
    Sponsors,
    Help(Option<String>),
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_args()?;

    if let Err(err) = dotenv() {
        warn!(
            "Could not load .env file ({err}). This is expected when environment variables are injected by the host"
        );
    }
    init_crypto();
    init_tracing()?;

    args.sandbox();

    match args.command {
        Some(Command::Server) => run_server().await?,
        Some(Command::Sponsors) => generate_sponsors_image(),
        Some(Command::Help(sub)) => show_help(sub.as_deref()),
        Some(Command::Version) => show_version(),
        None => show_help(None),
    }

    Ok(())
}

fn parse_args() -> std::result::Result<Args, lexopt::Error> {
    let mut args = Args::default();
    let mut parser = lexopt::Parser::from_env();
    let mut is_help_mode = false;

    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                args.command = Some(Command::Help(None));
                is_help_mode = true;
            }
            Short('V' | 'v') | Long("version") => {
                args.command = Some(Command::Version);
            }
            Value(ref v) => match v.to_string_lossy().as_ref() {
                "help" => {
                    is_help_mode = true;
                    args.command = Some(Command::Help(None));
                }
                SERVER if is_help_mode => {
                    args.command = Some(Command::Help(Some(SERVER.to_owned())));
                }
                SERVER => {
                    args.command = Some(Command::Server);
                }
                SPONSORS => {
                    args.command = Some(Command::Sponsors);
                }
                _ => return Err(arg.unexpected()),
            },
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(args)
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

fn sandbox() {
    cfg_select! {
        target_os = "linux" => {
            sandbox_helper().unwrap();
        }
        _ => {
            info!("Landlock sandboxing is not supported on this platform")
        }
    }
}

#[cfg(target_os = "linux")]
fn sandbox_helper() -> Result<()> {
    use landlock::{
        ABI, Access, AccessFs, PathBeneath, PathFd, Ruleset, RulesetAttr, RulesetCreatedAttr,
    };
    use procfs::sys::kernel::Version;
    use support::fs::get_base_dir;

    let version = Version::current().unwrap();

    let abi = match (version.major, version.minor) {
        (7, 1..) => ABI::V9,
        (7, 0) => ABI::V8,
        (6, 15..) => ABI::V7,
        (6, 12..) => ABI::V6,
        (6, 10..) => ABI::V5,
        (6, 7..) => ABI::V4,
        (6, 2..) => ABI::V3,
        (5, 19..) => ABI::V2,
        (5, 13..) => ABI::V1,
        _ => {
            warn!("Kernel {version:?} does not support the required Landlock ABI");
            return Ok(());
        }
    };

    let read_only = AccessFs::from_read(abi);
    let read_write = AccessFs::from_all(abi);
    let read_exec = read_only | AccessFs::Execute;

    Ruleset::default()
        .handle_access(read_write)?
        .create()?
        .add_rule(PathBeneath::new(PathFd::new("./")?, read_only))?
        .add_rule(PathBeneath::new(PathFd::new("/dev")?, read_only))?
        .add_rule(PathBeneath::new(PathFd::new("/etc")?, read_only))?
        .add_rule(PathBeneath::new(PathFd::new("/lib64")?, read_only))?
        .add_rule(PathBeneath::new(PathFd::new("/tmp")?, read_only))?
        .add_rule(PathBeneath::new(PathFd::new("/usr/bin")?, read_exec))?
        .add_rule(PathBeneath::new(PathFd::new(get_base_dir()?)?, read_write))?
        .restrict_self()?;

    info!("Sandboxing enabled with Landlock ABI {abi:?}");
    Ok(())
}

async fn run_server() -> Result<()> {
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

    server().await
}

fn show_help(subcommand: Option<&str>) {
    match subcommand {
        Some(SERVER) => {
            println!(
                "Starts the Recipya web server\n\
                 \n\
                 Usage: {BIN_NAME} server [OPTIONS]\n\
                 \n\
                 Options:\n\
                   \t-h, --help  Print help"
            );
        }
        _ => {
            println!(
                "{DESCRIPTION}\n\
                 \n\
                 Usage: {BIN_NAME} <COMMAND>\n\
                 \n\
                 Commands:\n\
                   \tserver    Starts the Recipya web server\n\
                   \thelp      Print this message or the help of the given subcommand(s)\n\
                 \n\
                 Options:\n\
                   \t-h, --help     Print help\n\
                   \t-V, --version  Print version\n"
            );
        }
    }
}

fn show_version() {
    println!("{BIN_NAME} {VERSION}");
}
