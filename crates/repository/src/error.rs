use derive_more::derive::From;
use support::impl_display_as_debug;

/// Result type for errors related to the repository.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the repository.
#[allow(unused)]
#[derive(Debug, From)]
pub enum Error {
    FailToCreatePool(String),

    #[from]
    EnvVar(std::env::VarError),
    #[from]
    DieselRun(diesel_async::pooled_connection::bb8::RunError),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
