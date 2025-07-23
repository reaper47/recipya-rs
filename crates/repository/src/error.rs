use derive_more::derive::From;

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

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
