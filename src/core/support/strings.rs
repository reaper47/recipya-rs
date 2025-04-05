use std::str::FromStr;
use derive_more::From;
use crate::impl_display_as_debug;

/// Extracts a number from a string, if it exists.
pub fn extract_number<T>(s: String) -> Result<T> 
where 
T: FromStr +std::fmt::Debug {
    s
        .split_whitespace()
        .find(|&part| part.chars().all(|c| c.is_digit(10)))
        .ok_or_else(|| Error::NoNumberFound)?
        .parse::<T>()
        .map_err(|_| Error::Conversion)
}

/// Result type for errors related to strings.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to time.
#[derive(Debug, From)]
pub enum Error {
    Conversion,
    NoNumberFound,
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
