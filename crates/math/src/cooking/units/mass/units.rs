use std::str::FromStr;
use std::sync::OnceLock;

use regex::Regex;

use crate::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Mass {
    Milligram(f64),
    Gram(f64),
    Dekagram(f64),
    Hectogram(f64),
    Kilogram(f64),
    Ounce(f64),
    Pound(f64),
}

#[derive(Debug, PartialEq)]
pub enum MassUnit {
    Milligram,
    Gram,
    Dekagram,
    Hectogram,
    Kilogram,
    Ounce,
    Pound,
}

static MASS_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn get_regex<'a>() -> &'a Regex {
    MASS_REGEX.get_or_init(|| Regex::new(r"(?i)(\d(?:\s?[\d.]/?\d+)?)\s?([a-zμ]+)\b").unwrap())
}

impl FromStr for Mass {
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
                let unit = caps.get(2)?.as_str().to_lowercase();

                match unit.trim_end_matches('s') {
                    "g" | "gram" | "gramme" => Some(Mass::Gram(value)),
                    "mg" | "milligram" | "milligramme" => Some(Mass::Milligram(value)),
                    "kg" | "kilogram" | "kilogramme" => Some(Mass::Kilogram(value)),
                    "oz" | "ounce" => Some(Mass::Ounce(value)),
                    "lb" | "pound" => Some(Mass::Pound(value)),
                    "dag" | "dekagram" | "dekagramme" | "decagram" | "decagramme" => {
                        Some(Mass::Dekagram(value))
                    }
                    "hg" | "hectogram" | "hectogramme" => Some(Mass::Hectogram(value)),
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

        fn assert_text(text: &str, expected: Mass) {
            match Mass::from_str(text) {
                Ok(got) => assert_eq!(
                    got, expected,
                    "got {got:?} instead of {expected:?} for text '{text}'"
                ),
                Err(err) => panic!("{err:?} for text '{text}'"),
            }
        }

        #[test]
        fn test_milligram() {
            assert_text("2 mg of bacon", Mass::Milligram(2.0));
            assert_text("1.75 mg of bacon", Mass::Milligram(1.75));
            assert_text("1.75mg of smelly bacon", Mass::Milligram(1.75));
            assert_text("1 3/4 milligrams of smelly bacon", Mass::Milligram(1.75));
            assert_text("1 3/4 milligrammes of smelly bacon", Mass::Milligram(1.75));
            assert_text("1.75 milligrams of smelly bacon", Mass::Milligram(1.75));
        }

        #[test]
        fn test_gram() {
            assert_text("2 g of bacon", Mass::Gram(2.0));
            assert_text("1.75 g of bacon", Mass::Gram(1.75));
            assert_text("1.75g of smelly bacon", Mass::Gram(1.75));
            assert_text("1 3/4 grams of smelly bacon", Mass::Gram(1.75));
            assert_text("1 3/4 grammes of smelly bacon", Mass::Gram(1.75));
            assert_text("1.75 grams of smelly bacon", Mass::Gram(1.75));
        }

        #[test]
        fn test_dekagram() {
            assert_text("2 decagram of bacon", Mass::Dekagram(2.0));
            assert_text("1.75 decagram of bacon", Mass::Dekagram(1.75));
            assert_text("1.75 decagrammes of smelly bacon", Mass::Dekagram(1.75));
            assert_text("1 3/4 dekagram of smelly bacon", Mass::Dekagram(1.75));
            assert_text("1 3/4 dekagrammes of smelly bacon", Mass::Dekagram(1.75));
            assert_text("1.75dag of smelly bacon", Mass::Dekagram(1.75));
            assert_text("1.75 dag of smelly bacon", Mass::Dekagram(1.75));
            assert_text("1.75 dags of smelly bacon", Mass::Dekagram(1.75));
        }

        #[test]
        fn test_hectogram() {
            assert_text("2 hg of bacon", Mass::Hectogram(2.0));
            assert_text("1.75 hg of bacon", Mass::Hectogram(1.75));
            assert_text("1.75hg of smelly bacon", Mass::Hectogram(1.75));
            assert_text("1 3/4 hectograms of smelly bacon", Mass::Hectogram(1.75));
            assert_text("1 3/4 hectogrammes of smelly bacon", Mass::Hectogram(1.75));
        }

        #[test]
        fn test_kilogram() {
            assert_text("2 kg of bacon", Mass::Kilogram(2.0));
            assert_text("1.75 kg of bacon", Mass::Kilogram(1.75));
            assert_text("1.75kg of smelly bacon", Mass::Kilogram(1.75));
            assert_text("1 3/4 kilogram of smelly bacon", Mass::Kilogram(1.75));
            assert_text("1 3/4 kilograms of smelly bacon", Mass::Kilogram(1.75));
            assert_text("1.75 kilogramme of smelly bacon", Mass::Kilogram(1.75));
            assert_text("1.75 kilogrammes of smelly bacon", Mass::Kilogram(1.75));
        }

        #[test]
        fn test_ounce() {
            assert_text("2 oz of bacon", Mass::Ounce(2.0));
            assert_text("1.75 oz of bacon", Mass::Ounce(1.75));
            assert_text("1.75oz of smelly bacon", Mass::Ounce(1.75));
            assert_text("1 3/4 ounce of smelly bacon", Mass::Ounce(1.75));
            assert_text("1 3/4 ounces of smelly bacon", Mass::Ounce(1.75));
        }

        #[test]
        fn test_pound() {
            assert_text("2 lbs of bacon", Mass::Pound(2.0));
            assert_text("1.75 lb of bacon", Mass::Pound(1.75));
            assert_text("1.75lbs of smelly bacon", Mass::Pound(1.75));
            assert_text("1 3/4 pound of smelly bacon", Mass::Pound(1.75));
            assert_text("1 3/4 pounds of smelly bacon", Mass::Pound(1.75));
        }
    }
}
