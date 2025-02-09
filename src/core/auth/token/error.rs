use serde::Serialize;

use crate::impl_display_as_debug;

/// Result type for errors related to tokens.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to tokens.
#[derive(Debug, Serialize)]
pub enum Error {
    HmacFailNewFromSlice,

    InvalidFormat,
    CannotDecodeIdent,
    CannotDecodeExp,
    SignatureNotMatching,
    ExpNotIso,
    Expired,
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
