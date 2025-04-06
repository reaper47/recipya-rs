use derive_more::From;

use crate::impl_display_as_debug;

/// Result type for errors related to the scraper.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the scraper.
#[derive(Debug, From)]
pub enum Error {
    DomainNotImplemented,
    LdJsonNotFound,
    Filesystem,
    NoHost,
    UnknownWebsite,

    // Externals
    Parse(String),
    Select(String),

    #[from(serde_json::Error)]
    Deserialize,
    #[from(reqwest::Error)]
    Request,
}

impl_display_as_debug!(Error);

impl From<scraper::error::SelectorErrorKind<'_>> for Error {
    fn from(value: scraper::error::SelectorErrorKind) -> Self {
        Error::Select(value.to_string())
    }
}

impl std::error::Error for Error {}
