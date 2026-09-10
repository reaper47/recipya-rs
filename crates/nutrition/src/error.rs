use derive_more::From;

use support::impl_display_as_debug;

/// Result type for errors related to nutrition.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to nutrition.
#[derive(Debug, From)]
pub enum Error {
    InvalidCssSelector,
    InvalidZipArchive,
    NoFileInZip,
    NoNeedToUpdateNutrition,
    UnknownSource,

    // Modules
    Diesel(String),

    // Externals
    #[from]
    Io(std::io::Error),
    #[from]
    Reqwest(reqwest::Error),
    #[from]
    SimdJson(simd_json::Error),
    #[from]
    Run(diesel_async::pooled_connection::bb8::RunError),
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Self::Diesel(value.to_string())
    }
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
