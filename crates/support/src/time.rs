use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{char, digit1, space0};
use nom::combinator::{map_res, opt};
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};
use time::{Duration, OffsetDateTime};

use crate::impl_display_as_debug;
pub use time::format_description::well_known::Rfc3339;

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

/// Extracts the hours and minutes from a duration, e.g. 2h30m.
pub fn parse_duration(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, hours) = opt(parse_hour).parse(input)?;
    let (input, minutes) = opt(parse_minute).parse(input)?;
    Ok((input, (hours.unwrap_or(0), minutes.unwrap_or(0))))
}

fn parse_hour(input: &str) -> IResult<&str, u32> {
    alt((
        preceded(
            space0,
            terminated(parse_number, preceded(space0, char('h'))),
        ),
        terminated(parse_number, delimited(space0, tag("hour"), space0)),
        terminated(parse_number, delimited(space0, tag("hours"), space0)),
    ))
    .parse(input)
}

fn parse_minute(input: &str) -> IResult<&str, u32> {
    preceded(
        take_till(|c: char| c.is_ascii_digit()),
        alt((
            terminated(parse_number, char('m')),
            terminated(parse_number, delimited(space0, tag("minute"), space0)),
            terminated(parse_number, delimited(space0, tag("minutes"), space0)),
            preceded(space0, parse_number),
        )),
    )
    .parse(input)
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse::<u32>).parse(input)
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
        fn test_parse_duration_1() -> Result<()> {
            let (_rem, got) = parse_duration("3h")?;

            pretty_assertions::assert_eq!(got, (3, 0));
            Ok(())
        }

        #[test]
        fn test_parse_duration_2() -> Result<()> {
            let (_rem, got) = parse_duration("3 hours")?;

            pretty_assertions::assert_eq!(got, (3, 0));
            Ok(())
        }

        #[test]
        fn test_parse_duration_3() -> Result<()> {
            let (_rem, got) = parse_duration("3 h")?;

            pretty_assertions::assert_eq!(got, (3, 0));
            Ok(())
        }

        #[test]
        fn test_parse_duration_4() -> Result<()> {
            let (_rem, got) = parse_duration("3h25")?;

            pretty_assertions::assert_eq!(got, (3, 25));
            Ok(())
        }

        #[test]
        fn test_parse_duration_5() -> Result<()> {
            let (_rem, got) = parse_duration("3h25m")?;

            pretty_assertions::assert_eq!(got, (3, 25));
            Ok(())
        }

        #[test]
        fn test_parse_duration_6() -> Result<()> {
            let (_rem, got) = parse_duration("3hours 25 minutes")?;

            pretty_assertions::assert_eq!(got, (3, 25));
            Ok(())
        }
    }
}
