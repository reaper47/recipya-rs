use derive_more::derive::From;

use support::impl_display_as_debug;

pub type Result<T> = core::result::Result<T, Error>;

#[allow(unused)]
#[derive(Debug, From)]
pub enum Error {
    Server(String),

    #[from]
    Fs(support::fs::Error),
    #[from]
    Io(std::io::Error),
    #[from]
    Jobs(tokio_cron_scheduler::JobSchedulerError),
    #[from]
    Repository(diesel::result::Error),
    #[from]
    SetGlobalDefault(tracing::subscriber::SetGlobalDefaultError),

    #[from]
    Config(config::Error),
    #[from]
    Router(router::Error),
    #[from]
    Token(auth::token::Error),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
