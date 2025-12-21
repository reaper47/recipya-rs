use derive_more::From;

use support::impl_display_as_debug;

/// Result type for errors related to integrations.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of required fields.
#[derive(Debug)]
pub enum RequiredField {
    Title,
    Ingredients,
    Instructions,
}

/// Enumeration of errors related to integrations.
#[derive(Debug, From)]
pub enum Error {
    ApiError(String),
    MissingRequiredField(RequiredField),
    Parse(String),
    UnsupportedApi,
    UnsupportedApp,
    UnsupportedFileFormat,

    #[from]
    Cooklang(cooklang::error::SourceReport),
    #[from]
    Io(std::io::Error),
    #[from]
    Paprika(libpaprika::Error),
    #[from]
    RecipeMD(recipemd::Error),
    #[from]
    Reqwest(reqwest::Error),
    #[from]
    Serde(serde_json::Error),
    #[from]
    Zip(zip::result::ZipError),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
