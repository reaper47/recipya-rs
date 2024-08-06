use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    DomainNotImplemented,
    LdJsonNotFound,
    NoHost,
    UnknownWebsite,

    // Externals
    Parse(String),
    Select(String),

    #[from]
    Deserialize(serde_json::Error),
    #[from]
    Request(reqwest::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<scraper::error::SelectorErrorKind<'_>> for Error {
    fn from(value: scraper::error::SelectorErrorKind) -> Self {
        Error::Select(value.to_string())
    }
}

impl std::error::Error for Error {}
