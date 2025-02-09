use time::{Duration, OffsetDateTime};

pub use time::format_description::well_known::Rfc3339;

use crate::impl_display_as_debug;

/// Formats a given `OffsetDateTime` into a string in RFC3339 format.
pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap() // TODO: need to check if safe
}

/// Returns the current UTC time plus the given number of seconds as a formatted string.
pub fn now_utc_plus_sec_str(sec: f64) -> String {
    let new_time = OffsetDateTime::now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

/// Parses a string into an `OffsetDateTime` in UTC.
pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::FailToDateParse(moment.to_string()))
}

/// Result type for errors related to time.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to time.
#[derive(Debug)]
pub enum Error {
    FailToDateParse(String),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}
