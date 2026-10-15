use core::fmt;

use serde::{Deserialize, Serialize};
use validator::ValidateEmail;

use crate::Error;

/// A syntactically valid email address.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(try_from = "String")]
pub struct Email(String);

/// Validates and wraps an owned string.
impl TryFrom<String> for Email {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let email = value.trim().to_owned();
        if email.validate_email() {
            Ok(Self(email))
        } else {
            Err(Error::InvalidData)
        }
    }
}

/// Validates and wraps a string slice.
impl TryFrom<&str> for Email {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let email = value.trim();
        if email.validate_email() {
            Ok(Self(email.into()))
        } else {
            Err(Error::InvalidData)
        }
    }
}

/// Borrows the address as a `&str`.
impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Formats the address as-is, without quoting.
impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Email {
    /// Consumes the `Email` and returns the underlying string.
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Returngs the reserved address of the built-in autologin user.
    pub fn autologin() -> Self {
        Self("admin@autologin.com".into())
    }

    /// Returns the reserved address of the built-in demo user.
    pub fn demo() -> Self {
        Self("demo@demo.com".into())
    }
}
