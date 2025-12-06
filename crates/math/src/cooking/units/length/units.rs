use std::str::FromStr;
use std::sync::OnceLock;

use regex::Regex;

use crate::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Length {
    Millimetre(f64),
    Centimetre(f64),
    Metre(f64),
    Kilometre(f64),
    Inch(f64),
    Foot(f64),
}

#[derive(Debug, PartialEq)]
pub enum LengthUnit {
    Millimetre,
    Centimetre,
    Metre,
    Kilometre,
    Inch,
    Foot,
}

static LENGTH_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn get_regex<'a>() -> &'a Regex {
    LENGTH_REGEX.get_or_init(|| Regex::new(r#"(?i)(\d(?:\s?[\d.]/?\d+)?)\s?([a-z"']+)"#).unwrap())
}

impl FromStr for Length {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::NotDetected);
        }

        get_regex()
            .captures(s)
            .and_then(|caps| {
                let value = caps.get(1)?.as_str();

                let value = value
                    .parse::<f64>()
                    .or_else(|_| {
                        let parts = value.split_whitespace().collect::<Vec<&str>>();
                        let base = parts
                            .first()
                            .unwrap_or(&"")
                            .parse::<f64>()
                            .unwrap_or_default();

                        let frac_parts: Vec<&str> =
                            parts.last().unwrap_or(&"").split('/').collect();

                        let numerator: f64 = frac_parts
                            .first()
                            .map(|n| n.parse::<f64>().ok().unwrap_or_default())
                            .unwrap_or_default();

                        let denominator: f64 = frac_parts
                            .get(1)
                            .map(|n| n.parse::<f64>().ok().unwrap_or_default())
                            .unwrap_or_default();

                        Ok::<f64, std::num::ParseFloatError>(base + numerator / denominator)
                    })
                    .ok()?;
                let unit = caps.get(2)?.as_str().to_lowercase();

                match unit.trim_end_matches('s') {
                    "mm" | "millimetre" | "millimeter" => Some(Length::Millimetre(value)),
                    "cm" | "centimetre" | "centimeter" => Some(Length::Centimetre(value)),
                    "m" | "metre" | "meter" => Some(Length::Metre(value)),
                    "\"" | "inch" | "inche" | "in" => Some(Length::Inch(value)),
                    "'" | "ft" | "foot" | "feet" => Some(Length::Foot(value)),
                    _ => None,
                }
            })
            .ok_or(Error::NotDetected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_from_str {
        use super::*;

        fn assert_text(text: &str, expected: Length) {
            match Length::from_str(text) {
                Ok(got) => assert_eq!(
                    got, expected,
                    "got {got:?} instead of {expected:?} for text '{text}'"
                ),
                Err(err) => panic!("{err:?} for text '{text}'"),
            }
        }

        #[test]
        fn test_millimetre() {
            assert_text("2 mm of rope", Length::Millimetre(2.0));
            assert_text("1.75 mm of rope", Length::Millimetre(1.75));
            assert_text("1.75mm of rope", Length::Millimetre(1.75));
            assert_text("1 3/4 millimetres of rope", Length::Millimetre(1.75));
            assert_text("1 3/4 millimeters of rope", Length::Millimetre(1.75));
        }

        #[test]
        fn test_centimetre() {
            assert_text("2 cm of bacon", Length::Centimetre(2.0));
            assert_text("1.75 cm of bacon", Length::Centimetre(1.75));
            assert_text("1.75cm of bacon", Length::Centimetre(1.75));
            assert_text("1 3/4 centimetres of bacon", Length::Centimetre(1.75));
            assert_text("1 3/4 centimeters of bacon", Length::Centimetre(1.75));
        }

        #[test]
        fn test_metre() {
            assert_text("2 m of bacon", Length::Metre(2.0));
            assert_text("1.75 m of bacon", Length::Metre(1.75));
            assert_text("1.75m of bacon", Length::Metre(1.75));
            assert_text("1 3/4 metres of bacon", Length::Metre(1.75));
            assert_text("1 3/4 meters of bacon", Length::Metre(1.75));
        }

        #[test]
        fn test_inch() {
            assert_text("2\" of bacon", Length::Inch(2.0));
            assert_text("1.75 \" of bacon", Length::Inch(1.75));
            assert_text("1.75\" of bacon", Length::Inch(1.75));
            assert_text("1 3/4 inches of bacon", Length::Inch(1.75));
            assert_text("1 3/4 inch of bacon", Length::Inch(1.75));
        }

        #[test]
        fn test_foot() {
            assert_text("2' of bacon", Length::Foot(2.0));
            assert_text("1.75 ' of bacon", Length::Foot(1.75));
            assert_text("1.75' of bacon", Length::Foot(1.75));
            assert_text("1 3/4 feet of bacon", Length::Foot(1.75));
            assert_text("1 3/4 ft of bacon", Length::Foot(1.75));
            assert_text("1 3/4ft of bacon", Length::Foot(1.75));
            assert_text("1 3/4 foot of bacon", Length::Foot(1.75));
        }
    }
}
