use serde::Serialize;

use support::impl_display_as_debug;

/// Result type for errors related to the scheme.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the scheme.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum Error {
    Key,
    Salt,
    Hash,
    PwdValidate,
    SchemeNotFound(String),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
