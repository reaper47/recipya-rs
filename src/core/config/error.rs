use derive_more::derive::From;

use crate::impl_display_as_debug;

/// Result type for errors related to the app's configuration.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the app's configuration.
#[allow(unused)]
#[derive(Debug, From)]
pub enum Error {
    MissingEnv(&'static str),

    #[from]
    EnvVar(std::env::VarError),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
