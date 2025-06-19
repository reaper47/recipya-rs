use std::str::{FromStr, from_utf8};

use crate::impl_display_as_debug;
use derive_more::From;
use encoding_rs::{ISO_8859_15, WINDOWS_1252};

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

/// Attempts to convert a buffer to UTF-8 if not already in this encoding.
pub fn auto_convert_to_utf8(buffer: &[u8]) -> String {
    if let Ok(content) = from_utf8(buffer) {
        return content.to_string();
    }

    let encodings = [WINDOWS_1252, ISO_8859_15];

    for encoding in &encodings {
        let (content, _encoding, had_errors) = encoding.decode(buffer);
        if !had_errors {
            return content.to_string();
        }
    }

    String::from_utf8_lossy(buffer).to_string()
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
