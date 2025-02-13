use derive_more::derive::From;
use serde::Serialize;

use super::scheme;
use crate::impl_display_as_debug;

/// Result type for errors related to passwords.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to passwords.
#[derive(Debug, From, PartialEq, Serialize)]
pub enum Error {
    PwdWithSchemeFailedParse,

    FailSpawnBlockForHash,
    FailSpawnBlockForValidate,

    // Modules
    #[from]
    Scheme(scheme::Error),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
