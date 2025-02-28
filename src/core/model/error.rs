use derive_more::From;

use diesel_async::pooled_connection::bb8::RunError;

use crate::core::auth::pwd;
use crate::impl_display_as_debug;

/// Result type for errors related to the models.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the models.
#[derive(Debug, From)]
pub enum Error {
    DuplicateEntity,
    EntityNotFound {
        entity: &'static str,
        id: i64,
    },

    // Modules
    #[from]
    Pwd(pwd::Error),

    // Externals
    Diesel(String),
    #[from]
    Run(RunError),
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Error::Diesel(value.to_string())
    }
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
