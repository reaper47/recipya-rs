use derive_more::From;

use support::impl_display_as_debug;

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
    NoSearch,
    Time,
    ThemeNotFound,

    // Modules
    #[from]
    Config(config::Error),
    #[from]
    Scraper(recipya_scraper::Error),
    #[from]
    Repository(repository::Error),
    #[from]
    Pwd(auth::pwd::Error),

    // Externals
    Diesel(String),
    #[from]
    Fmt(std::fmt::Error),
    #[from]
    Math(math::Error),
    #[from]
    HumanTime(humantime::DurationError),
    #[from]
    Run(diesel_async::pooled_connection::bb8::RunError),
    #[from]
    SerdeJson(serde_json::Error),
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Error::Diesel(value.to_string())
    }
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
