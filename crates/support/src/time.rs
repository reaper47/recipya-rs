use time::{Duration, OffsetDateTime};
use winnow::Result as WResult;
use winnow::ascii::{digit1, space0};
use winnow::combinator::{alt, delimited, opt, preceded, terminated};
use winnow::prelude::*;
use winnow::token::{literal, take_till};

use crate::impl_display_as_debug;

pub use time::format_description::well_known::Rfc3339;

/// Formats a given `OffsetDateTime` into a string in RFC3339 format.
///
/// # Panics
///
/// Panics if the datetime cannot be formatted as RFC3339. This should never
/// happen in practice as all valid `OffsetDateTime` values can be represented
/// in RFC3339 format.
pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap()
}

/// Returns the current UTC time plus the given number of seconds as a formatted string.
pub fn now_utc_plus_sec_str(sec: f64) -> usize {
    usize::try_from((OffsetDateTime::now_utc() + Duration::seconds_f64(sec)).unix_timestamp())
        .unwrap_or_default()
}

/// Parses a string into an `OffsetDateTime` in UTC.
pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::FailToDateParse(moment.to_string()))
}

/// Extracts the hours and minutes from a duration, e.g. 2h30m.
pub fn parse_hours_minutes(input: &mut &str) -> WResult<(u32, u32)> {
    let hours = opt(parse_hour).parse_next(input)?;
    let minutes = opt(parse_minute).parse_next(input)?;
    Ok((hours.unwrap_or(0), minutes.unwrap_or(0)))
}

fn parse_hour(input: &mut &str) -> WResult<u32> {
    alt((
        preceded(space0, terminated(parse_number, preceded(space0, 'h'))),
        terminated(parse_number, delimited(space0, literal("hour"), space0)),
        terminated(parse_number, delimited(space0, literal("hours"), space0)),
    ))
    .parse_next(input)
}

fn parse_minute(input: &mut &str) -> WResult<u32> {
    preceded(
        take_till(0.., |c: char| c.is_ascii_digit()),
        alt((
            terminated(parse_number, 'm'),
            terminated(parse_number, delimited(space0, literal("minute"), space0)),
            terminated(parse_number, delimited(space0, literal("minutes"), space0)),
            preceded(space0, parse_number),
        )),
    )
    .parse_next(input)
}

fn parse_number(input: &mut &str) -> WResult<u32> {
    digit1.try_map(str::parse::<u32>).parse_next(input)
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

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_parse_duration {
        use super::*;

        #[test]
        fn test_parse_hours_minutes_1() -> Result<()> {
            let got = parse_hours_minutes(&mut "3h").map_err(|err| err.to_string())?;

            pretty_assertions::assert_eq!(got, (3, 0));
            Ok(())
        }

        #[test]
        fn test_parse_hours_minutes_2() -> Result<()> {
            let got = parse_hours_minutes(&mut "3 hours").map_err(|err| err.to_string())?;

            pretty_assertions::assert_eq!(got, (3, 0));
            Ok(())
        }

        #[test]
        fn test_parse_hours_minutes_3() -> Result<()> {
            let got = parse_hours_minutes(&mut "3 h").map_err(|err| err.to_string())?;

            pretty_assertions::assert_eq!(got, (3, 0));
            Ok(())
        }

        #[test]
        fn test_parse_hours_minutes_4() -> Result<()> {
            let got = parse_hours_minutes(&mut "3h25").map_err(|err| err.to_string())?;

            pretty_assertions::assert_eq!(got, (3, 25));
            Ok(())
        }

        #[test]
        fn test_parse_hours_minutes_5() -> Result<()> {
            let got = parse_hours_minutes(&mut "3h25m").map_err(|err| err.to_string())?;

            pretty_assertions::assert_eq!(got, (3, 25));
            Ok(())
        }

        #[test]
        fn test_parse_hours_minutes_6() -> Result<()> {
            let got =
                parse_hours_minutes(&mut "3hours 25 minutes").map_err(|err| err.to_string())?;

            pretty_assertions::assert_eq!(got, (3, 25));
            Ok(())
        }
    }
}
