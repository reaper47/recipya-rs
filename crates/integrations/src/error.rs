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

/// A shortened version of nom parsing errors that excludes verbose input data.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ShortNomError {
    kind: nom::error::ErrorKind,
    context: String,
}

/// Enumeration of errors related to integrations.
#[derive(Debug, From)]
pub enum Error {
    ApiError(String),
    MissingRequiredField(RequiredField),
    NomStr(ShortNomError),
    Parse(String),
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

impl<I> From<nom::Err<nom::error::Error<I>>> for Error
where
    I: std::fmt::Display,
{
    fn from(err: nom::Err<nom::error::Error<I>>) -> Self {
        let max_chars = 30;

        match err {
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                let input_preview = format!("{}", e.input)
                    .chars()
                    .take(max_chars)
                    .collect::<String>();

                Error::NomStr(ShortNomError {
                    kind: e.code,
                    context: if input_preview.len() == max_chars {
                        format!("{}...", input_preview)
                    } else {
                        input_preview
                    },
                })
            }
            nom::Err::Incomplete(_) => Error::NomStr(ShortNomError {
                kind: nom::error::ErrorKind::Complete,
                context: "incomplete input".to_string(),
            }),
        }
    }
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
