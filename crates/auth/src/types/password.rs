use core::fmt;

use serde::{Deserialize, Serialize};
use validator::ValidateLength;
use zeroize::ZeroizeOnDrop;

use crate::Error;

/// A syntactically valid password.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, ZeroizeOnDrop)]
#[serde(try_from = "String")]
pub struct Password(String);

/// Validates and wraps an owned string.
impl TryFrom<String> for Password {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let password = value.trim().to_owned();
        if password.validate_length(Some(8), None, None) {
            Ok(Self(password))
        } else {
            Err(Error::InvalidData)
        }
    }
}

/// Validates and wraps a string slice.
impl TryFrom<&str> for Password {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let password = value.trim();
        if password.validate_length(Some(8), None, None) {
            Ok(Self(password.into()))
        } else {
            Err(Error::InvalidData)
        }
    }
}

/// Formats the address as-is, without quoting.
impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Password(****)")
    }
}

impl Password {
    /// Consumes the `Password` and returns the underlying string.
    pub fn into_inner(self) -> String {
        self.0.clone()
    }
}
