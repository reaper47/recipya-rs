use std::str::FromStr;
use std::sync::OnceLock;

use regex::Regex;

use crate::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Unitless {
    pub value: f64,
}

impl Unitless {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

static UNITLESS_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn get_regex<'a>() -> &'a Regex {
    UNITLESS_REGEX.get_or_init(|| Regex::new(r"(?i)(\d(?:.?\s?/?\d+)*)").unwrap())
}

impl FromStr for Unitless {
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
                        let numerator: f64 = frac_parts[0].parse()?;
                        let denominator: f64 = frac_parts[1].parse()?;

                        Ok::<f64, std::num::ParseFloatError>(base + numerator / denominator)
                    })
                    .ok()?;

                Some(Unitless::new(value))
            })
            .ok_or(Error::NotDetected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_text(text: &str, expected: Unitless) {
        match Unitless::from_str(text) {
            Ok(got) => assert_eq!(
                got, expected,
                "got {got:?} instead of {expected:?} for text '{text}'"
            ),
            Err(err) => panic!("{err:?} for text '{text}'"),
        }
    }

    #[test]
    fn test_from_str() {
        assert_text("4 big apples", Unitless::new(4.0));
        assert_text(
            "4 2/3 cans of bamboo sticks",
            Unitless::new(4.666666666666667),
        );
        assert_text("3can of tomato paste", Unitless::new(3.0));
        assert_text(
            "1/2 fresh pineapple, cored and cut into 1 1/2-inch pieces",
            Unitless::new(0.5),
        );
        assert_text("2.5 slices of bacon", Unitless::new(2.5));
    }
}
