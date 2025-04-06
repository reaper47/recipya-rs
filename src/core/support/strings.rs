use std::str::FromStr;

use derive_more::From;

use crate::impl_display_as_debug;

/// Extracts a number from a string, if it exists.
pub fn extract_number<T>(s: String) -> Result<T>
where
    T: FromStr + std::fmt::Debug,
{
    s.split_whitespace()
        .find(|&part| part.chars().all(|c| c.is_ascii_digit()))
        .ok_or(Error::NoNumberFound)?
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

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_extract_number {
        use super::*;

        #[test]
        fn test_no_number_err() -> Result<()> {
            let res = extract_number::<i16>("".to_string());

            assert!(res.is_err());
            Ok(())
        }

        #[test]
        fn test_number_in_front_ok() -> Result<()> {
            let res = extract_number::<i16>("420 years of love".to_string())?;

            pretty_assertions::assert_eq!(res, 420);
            Ok(())
        }

        #[test]
        fn test_number_middle_ok() -> Result<()> {
            let res = extract_number::<i16>("years of 420 love".to_string())?;

            pretty_assertions::assert_eq!(res, 420);
            Ok(())
        }

        #[test]
        fn test_number_end_ok() -> Result<()> {
            let res = extract_number::<i16>("years of love 420".to_string())?;

            pretty_assertions::assert_eq!(res, 420);
            Ok(())
        }
    }
}
