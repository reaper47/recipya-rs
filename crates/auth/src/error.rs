use derive_more::From;

use support::impl_display_as_debug;

use crate::pwd::scheme;

/// Result type for errors related to tokens.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to tokens.
#[derive(Debug, From)]
pub enum Error {
    // Password errors
    PwdWithSchemeFailedParse,
    FailSpawnBlockForHash,
    FailSpawnBlockForValidate,

    // Token errors
    HmacFailNewFromSlice,
    InvalidFormat,
    CannotDecodeIdent,
    CannotDecodeExp,
    SignatureNotMatching,
    ExpNotIso,
    Expired,

    ConfigFileWriteFailed,

    // Modules
    #[from]
    Fs(support::fs::Error),
    #[from]
    Json(jsonwebtoken::errors::Error),
    #[from]
    Scheme(scheme::Error),
    #[from]
    SerdeJson(serde_json::Error),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
