use derive_more::From;
use support::impl_display_as_debug;

/// Result type for errors related to the scraper.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the scraper.
#[derive(Debug, From)]
pub enum Error {
    DomainNotImplemented,
    LdJsonNotFound,
    Fetch(String),
    Filesystem,
    MissingElement(String),
    NoHost,
    NoRecipeFound,
    UnknownWebsite,

    // Externals
    Parse(String),
    Select(String),

    #[from(serde_json::Error)]
    Deserialize,
    #[from(reqwest::Error)]
    Request,
    #[from(wreq::Error)]
    Wreq,
}

impl_display_as_debug!(Error);

impl From<scraper::error::SelectorErrorKind<'_>> for Error {
    fn from(value: scraper::error::SelectorErrorKind) -> Self {
        Self::Select(value.to_string())
    }
}

impl std::error::Error for Error {}
