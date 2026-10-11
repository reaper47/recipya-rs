use lexopt::Arg::{Long, Short, Value};
use yansi::Paint;

use crate::{
    Result, l10n::run_i18n, sandbox::init_sandbox, server::run_server,
    sponsors::generate_sponsors_image,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BIN_NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const I18N_CMD: &str = "i18n";
const SERVER_CMD: &str = "server";
const SPONSORS_CMD: &str = "sponsors";
const VERSION_CMD: &str = "version";

const ADD_SUBCMD: &str = "add";
const I18N_TRANSLATE_SUBCMD: &str = "translation";

const TWO_SPACES: &str = "\x20\x20";
const FOUR_SPACES: &str = "\x20\x20\x20\x20";

#[derive(Debug, Default)]
pub struct Args {
    command: Command,
    subcommands: Vec<String>,
    options: Vec<String>,
    is_help: bool,
}

impl Args {
    fn sandbox(&self) {
        match self.command {
            Command::Server | Command::Sponsors => init_sandbox(),
            Command::I18n | Command::Help | Command::Version => {}
        }
    }

    fn show_help(self) {
        match self.command {
            Command::I18n => print_help_i18n(&self.subcommands),
            Command::Server => print_help_server(),
            Command::Sponsors => println!("You discovered a secret command!"),
            Command::Version => {}
            Command::Help => print_help_help(),
        }
    }

    pub async fn run_command(self) -> Result<()> {
        if self.is_help {
            self.show_help();
            return Ok(());
        }

        self.sandbox();

        match self.command {
            Command::I18n => {
                run_i18n(&self.subcommands, &self.options);
                Ok(())
            }
            Command::Server => run_server().await,
            Command::Sponsors => {
                generate_sponsors_image();
                Ok(())
            }
            Command::Version => {
                println!("{BIN_NAME} {VERSION}");
                Ok(())
            }
            Command::Help => Ok(()),
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
enum Command {
    I18n,
    Server,
    Sponsors,
    #[default]
    Help,
    Version,
}

pub fn parse_args() -> std::result::Result<Args, lexopt::Error> {
    let mut args = Args::default();
    let mut parser = lexopt::Parser::from_env();
    let mut is_option = false;

    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                args.is_help = true;
            }
            Short('V') | Long("version") => {
                args.command = Command::Version;
            }
            Long("id") => {
                is_option = true;
                args.options.push("id".into());
            }
            Value(ref v) => match v.to_string_lossy().as_ref() {
                "help" => {
                    args.is_help = true;
                    args.command = Command::Help;
                }
                SERVER_CMD => {
                    args.command = Command::Server;
                }
                I18N_CMD => args.command = Command::I18n,
                SPONSORS_CMD => {
                    args.command = Command::Sponsors;
                }
                I18N_TRANSLATE_SUBCMD | ADD_SUBCMD => {
                    args.subcommands.push(v.to_string_lossy().to_string());
                }
                VERSION_CMD => args.command = Command::Version,
                _ => {
                    if is_option {
                        args.options.push(v.to_string_lossy().to_string());
                        is_option = false;
                        continue;
                    }
                    return Err(arg.unexpected());
                }
            },
            _ => {
                dbg!(&arg);
                return Err(arg.unexpected());
            }
        }
    }

    Ok(args)
}

fn print_help_i18n(subcommands: &[String]) {
    match subcommands.last() {
        Some(subcmd) => match subcmd.as_str() {
            ADD_SUBCMD => print_help_i18n_translate_add(),
            _ => print_help_i18n_translate(),
        },
        None => {
            println!(
                "Manage internationalisation\n\
                 \n\
                 {}: {BIN_NAME} i18n [OPTIONS] <COMMAND>\n\
                 \n\
                 {}:\n\
                   {TWO_SPACES}{}  Manage translations\n\
                 \n\
                 {}:\n\
                   {FOUR_SPACES}-h, --help     Print help (see more with '--help')",
                "Usage".bold().underline(),
                "Commands".bold().underline(),
                "translation".bold(),
                "Options".bold().underline(),
            );
        }
    }
}

fn print_help_i18n_translate() {
    println!(
        "Manage the translations under the locales directory\n\
         \n\
         {}: {BIN_NAME} i18n translations [OPTIONS] <COMMAND>\n\
         \n\
         {}:\n\
           {TWO_SPACES}{}  Add an entry to all locale files\n\
         \n\
         {}:\n\
           {TWO_SPACES}-h, --help  Print help (see more with '--help')\n\
         \n\
         You can also run `{BIN_NAME} SUBCOMMAND -h` to get more information about that subcommand.",
        "Usage".bold().underline(),
        "Commands".bold().underline(),
        "add".bold(),
        "Options".bold().underline(),
    );
}

fn print_help_i18n_translate_add() {
    println!(
        "Manage the translations under the locales directory\n\
         \n\
         {}: {BIN_NAME} i18n translations add [OPTIONS]\n\
         \n\
         {}: {BIN_NAME} i18n translations add --id tabs-close-button --value Close
         \n\
         {}:\n\
           {FOUR_SPACES}{TWO_SPACES}{} <IDENTIFIER>  Specifies the name of the identifier (required)\n\
           {FOUR_SPACES}{TWO_SPACES}{} <VALUE>    Specifies the value of the identifier (default: '')\n\
           {TWO_SPACES}-h, --help \t\t\x20Print help (see more with '--help')\n\
         \n\
         You can also run `{BIN_NAME} SUBCOMMAND -h` to get more information about that subcommand.",
        "Usage".bold().underline(),
        "Example".bold().underline(),
        "Options".bold().underline(),
        "--id".bold(),
        "--value".bold(),
    );
}

fn print_help_server() {
    println!(
        "Start the Recipya web server\n\
         \n\
         {}: {BIN_NAME} server [OPTIONS]\n\
         \n\
         {}:\n\
           {FOUR_SPACES}-h, --help  Print help (see more with '--help')",
        "Usage".bold().underline(),
        "Options".bold().underline(),
    );
}

fn print_help_help() {
    println!(
        "{DESCRIPTION}\n\
         \n\
         {}: {BIN_NAME} [OPTIONS] <COMMAND>\n\
         \n\
         {}:\n\
           {TWO_SPACES}{}   Start the Recipya web server\n\
           {TWO_SPACES}{}     Manage internationalisation\n\
           {TWO_SPACES}{}     Print this message or the help of the given subcommand(s)\n\
           {TWO_SPACES}{}  Print version\n\
         \n\
         {}:\n\
           {FOUR_SPACES}-h, --help     Print help (see more with '--help')\n\
           {FOUR_SPACES}-V, --version  Print version\n\
         \n\
         You can also run `{BIN_NAME} SUBCOMMAND -h` to get more information about that subcommand.",
        "Usage".bold().underline(),
        "Commands".bold().underline(),
        "server".bold(),
        "i18n".bold(),
        "help".bold(),
        "version".bold(),
        "Options".bold().underline(),
    );
}
