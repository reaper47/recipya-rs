use derive_more::derive::From;
use diesel_async::pooled_connection::bb8::RunError;

use crate::impl_display_as_debug;

/// Result type for errors related to jobs.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to jobs.
#[allow(unused)]
#[derive(Debug, From)]
pub enum Error {
    FromStr(String),

    #[from]
    Io(std::io::Error),
    #[from]
    Run(RunError),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::FromStr(s.to_string())
    }
}
