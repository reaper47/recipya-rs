use support::impl_display_as_debug;

/// Result type for errors related to the models.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the models.
#[derive(Debug)]
pub enum Error {
    NoRecipe,
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
