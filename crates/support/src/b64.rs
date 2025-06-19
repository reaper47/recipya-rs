use crate::impl_display_as_debug;
use base64::engine::{Engine, general_purpose};

/// Encodes the given byte slice to a URL-safe base64 string without padding.
pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(content)
}

/// Decodes a URL-safe base64 string without padding to a byte vector.
pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>> {
    general_purpose::URL_SAFE_NO_PAD
        .decode(b64u)
        .map_err(|_| Error::FailToB64uDecode)
}

/// Decodes a URL-safe base64 string without padding and converts it to a UTF-8 string.
pub fn b64u_decode_to_string(b64u: &str) -> Result<String> {
    b64u_decode(b64u)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(Error::FailToB64uDecode)
}

/// Result type for errors related to base64.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to base64.
#[derive(Debug)]
pub enum Error {
    FailToB64uDecode,
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_b64u_encode_ok() -> Result<()> {
        let got = b64u_encode("hello world");

        assert_eq!(got, "aGVsbG8gd29ybGQ");
        Ok(())
    }

    #[test]
    fn test_b64u_decode_ok() -> Result<()> {
        let got = b64u_decode("aGVsbG8gd29ybGQ")?;

        assert_eq!(got, b"hello world");
        Ok(())
    }

    #[test]
    fn test_b64u_decode_err() -> Result<()> {
        match b64u_decode("1") {
            Ok(_) => panic!("Should not have parsed"),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn test_b64u_decode_to_string_ok() -> Result<()> {
        let got = b64u_decode_to_string("aGVsbG8gd29ybGQ")?;

        assert_eq!(got, "hello world");
        Ok(())
    }

    #[test]
    fn test_b64u_decode_to_string_err() -> Result<()> {
        match b64u_decode("1") {
            Ok(_) => panic!("Should not have parsed"),
            Err(_) => Ok(()),
        }
    }
}
