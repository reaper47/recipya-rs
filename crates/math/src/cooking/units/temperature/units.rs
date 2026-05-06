use std::str::FromStr;
use std::sync::OnceLock;

use regex::Regex;

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

#[derive(Debug, Eq, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

static TEMPERATURE_REGEX: OnceLock<Regex> = OnceLock::new();

/// Returns the compiled temperature regex.
///
/// # Panics
///
/// Panics if the hard-coded regex literal is invalid.
pub fn get_regex<'a>() -> &'a Regex {
    TEMPERATURE_REGEX.get_or_init(|| {
        Regex::new(r"(?i)([\d.]+)\s?(?:deg|degrees?|\u{00B0})?\s?(celsius|fahrenheit|[cf])\b")
            .unwrap()
    })
}

impl FromStr for Temperature {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::NotDetected);
        }

        get_regex()
            .captures(s)
            .and_then(|caps| {
                let value = caps.get(1)?.as_str().parse::<f64>().ok()?;
                let unit = caps.get(2)?.as_str().to_lowercase();

                match unit.chars().next().unwrap_or_default() {
                    'f' => Some(Self::Fahrenheit(value)),
                    'c' => Some(Self::Celsius(value)),
                    _ => None,
                }
            })
            .ok_or(Error::NotDetected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_from_str {
        use super::*;

        fn assert_text(text: &str, expected: Temperature) {
            match Temperature::from_str(text) {
                Ok(got) => {
                    assert_eq!(
                        got, expected,
                        "got {got:?} instead of {expected:?} for text '{text}'"
                    );
                }
                Err(err) => {
                    panic!("{err} for text '{text}'");
                }
            }
        }

        #[test]
        fn test_celsius() {
            assert_text(
                "Rope with 200 degrees celsius of rice.",
                Temperature::Celsius(200.0),
            );
            assert_text("Heat the oven to 100.56C", Temperature::Celsius(100.56));
            assert_text("Heat the oven to 100C", Temperature::Celsius(100.0));
            assert_text("Heat the oven to 100 C", Temperature::Celsius(100.0));
            assert_text("Heat the oven to 375°C", Temperature::Celsius(375.0));
            assert_text("Heat the oven to 375 celsius", Temperature::Celsius(375.0));
        }

        #[test]
        fn test_fahrenheit() -> Result<()> {
            assert_eq!(
                Temperature::from_str("Rope with 475 degrees fahrenheit of rice.")?,
                Temperature::Fahrenheit(475.0)
            );
            assert_text("Heat the oven to 100.56F", Temperature::Fahrenheit(100.56));
            assert_text("Heat the oven to 100F", Temperature::Fahrenheit(100.0));
            assert_text("Heat the oven to 100 F", Temperature::Fahrenheit(100.0));
            assert_text("Heat the oven to 375°F", Temperature::Fahrenheit(375.0));
            assert_text(
                "Heat the oven to 375 fahrenheit",
                Temperature::Fahrenheit(375.0),
            );
            Ok(())
        }

        #[test]
        fn test_celsius_and_fahrenheit_invalid() {
            assert!(
                Temperature::from_str("1 cup of flour").is_err(),
                "1 cup of flour is not temperature"
            );
        }
    }
}
