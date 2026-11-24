use derive_more::From;

use support::impl_display_as_debug;

/// Result type for errors related to secrets.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to secrets.
#[derive(Debug, From)]
pub enum Error {
    InvalidKeyLength,
    InvalidNonce,
    InvalidUtf8,
    KeyError(String),
    KeyNotFound,
    MasterKeyLockPoisoned,

    // Modules
    #[from]
    Aes(aes_gcm::Error),
    #[from]
    Io(std::io::Error),
    #[from]
    SerdeJson(serde_json::Error),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
