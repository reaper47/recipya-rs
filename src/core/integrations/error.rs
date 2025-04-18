use derive_more::From;

use crate::impl_display_as_debug;

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
    MissingRequiredField(RequiredField),
    Parse(String),
    UnsupportedFileFormat,

    #[from]
    Cooklang(cooklang::error::SourceReport),
    #[from]
    Io(std::io::Error),
    #[from]
    NomStr(nom::Err<nom::error::Error<&'static str>>),
    #[from]
    RecipeMD(recipemd::Error),
    #[from]
    Serde(serde_json::Error),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
