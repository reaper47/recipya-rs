use derive_more::derive::From;

use support::impl_display_as_debug;

/// Result type for errors related to the application.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the application.
#[derive(Debug, From)]
pub enum Error {
    FromStr(String),

    // Externals
    #[from]
    Io(std::io::Error),
    #[from]
    Run(diesel_async::pooled_connection::bb8::RunError),

    // Modules
    #[from]
    Config(config::Error),
    #[from]
    Repository(repository::Error),
    #[from]
    Scraper(recipya_scraper::Error),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::FromStr(s.to_string())
    }
}
