use serde::{Deserialize, Serialize};
use time_tz::{TimeZone, Tz, timezones};

use crate::Error;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct Timezone(&'static Tz);

/// Validates and wraps an owned string.
impl TryFrom<String> for Timezone {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match timezones::get_by_name(&value) {
            Some(tz) => Ok(Self(tz)),
            None => Err(Error::InvalidTimezone),
        }
    }
}

/// Validates and wraps a string slice.
impl TryFrom<&str> for Timezone {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match timezones::get_by_name(&value) {
            Some(tz) => Ok(Self(tz)),
            None => Err(Error::InvalidTimezone),
        }
    }
}

impl Serialize for Timezone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.name())
    }
}

/// Borrows the address as a `&str`.
impl AsRef<str> for Timezone {
    fn as_ref(&self) -> &str {
        &self.0.name()
    }
}

impl Timezone {
    /// Returns all IANA names of selected time zones, sorted alphabetically.
    /// Those under `Etc/` are excluded.
    pub fn all() -> Vec<String> {
        let mut all_tz = time_tz::timezones::iter()
            .filter_map(|tz| {
                let name = tz.name();
                (!name.to_lowercase().starts_with("etc/")).then_some(name.into())
            })
            .collect::<Vec<_>>();
        all_tz.sort_unstable();
        all_tz
    }
}
