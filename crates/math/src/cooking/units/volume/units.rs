use std::str::FromStr;
use std::sync::OnceLock;

use regex::Regex;

use crate::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Volume {
    // Metric
    Millilitre(f64),
    Centilitre(f64),
    Decilitre(f64),
    Litre(f64),
    MetricTeaspoon(f64),
    MetricTablespoon(f64),
    MetricDessertspoon(f64),
    MetricCup(f64),

    // Australian
    AustralianTeaspoon(f64),
    AustralianDessertspoon(f64),
    AustralianTablespoon(f64),
    AustralianCup(f64),

    // Imperial
    ImperialTeaspoon(f64),
    ImperialDessertspoon(f64),
    ImperialTablespoon(f64),
    ImperialFluidOunce(f64),
    ImperialGill(f64),
    ImperialCup(f64),
    ImperialPint(f64),
    ImperialQuart(f64),
    ImperialGallon(f64),

    // US Customary
    USTeaspoon(f64),
    USTablespoon(f64),
    USFluidOunce(f64),
    USCup(f64),
    USPint(f64),
    USQuart(f64),
    USGallon(f64),

    // Other
    Jigger(f64),
}

#[derive(Debug, PartialEq)]
pub enum VolumeUnit {
    // Metric
    Millilitre,
    Centilitre,
    Decilitre,
    Litre,
    MetricTeaspoon,
    MetricTablespoon,
    MetricDessertSpoon,
    MetricCup,

    // Australian
    AustralianTeaspoon,
    AustralianDessertspoon,
    AustralianTablespoon,
    AustralianCup,

    // Imperial
    ImperialTeaspoon,
    ImperialDessertspoon,
    ImperialTablespoon,
    ImperialFluidOunce,
    ImperialGill,
    ImperialCup,
    ImperialPint,
    ImperialQuart,
    ImperialGallon,

    // US Customary
    USTeaspoon,
    USTablespoon,
    USFluidOunce,
    USCup,
    USPint,
    USQuart,
    USGallon,

    // Other
    Jigger,
}

static VOLUME_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn get_regex<'a>() -> &'a Regex {
    VOLUME_REGEX.get_or_init(|| {
        Regex::new(r"(?i)(\d(?:\s?[\d.]*/?\d+)?)\s?(fl(?:uid)?\.?\s?o(?:z\.?|unce)?|[a-z]+\b)")
            .unwrap()
    })
}

impl FromStr for Volume {
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
                    "ml" | "millilitre" | "milliliter" => Some(Volume::Millilitre(value)),
                    "cl" | "centilitre" | "centiliter" => Some(Volume::Centilitre(value)),
                    "dl" | "decilitre" | "deciliter" => Some(Volume::Decilitre(value)),
                    "l" | "litre" | "liter" => Some(Volume::Litre(value)),
                    "tsp" | "t" | "teaspoon" => Some(Volume::MetricTeaspoon(value)),
                    "tbsp" | "tb" | "tablespoon" => Some(Volume::MetricTablespoon(value)),
                    "dsp" | "d" | "dstsp" | "dessertspoon" | "dessert" => {
                        Some(Volume::MetricDessertspoon(value))
                    }
                    "c" | "cup" => Some(Volume::MetricCup(value)),
                    "floz" | "fl. oz." | "fluid ounce" | "oz fl" => {
                        Some(Volume::ImperialFluidOunce(value))
                    }
                    "gi" | "gill" => Some(Volume::ImperialGill(value)),
                    "pt" | "pint" => Some(Volume::ImperialPint(value)),
                    "qt" | "quart" => Some(Volume::ImperialQuart(value)),
                    "gal" | "gallon" => Some(Volume::ImperialGallon(value)),
                    "jig" | "jigger" => Some(Volume::Jigger(value)),
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

        fn assert_text(text: &str, expected: Volume) {
            match Volume::from_str(text) {
                Ok(got) => assert_eq!(
                    got, expected,
                    "got {got:?} instead of {expected:?} for text '{text}'"
                ),
                Err(err) => panic!("{err:?} for text '{text}'"),
            }
        }

        #[test]
        fn test_millilitre() {
            assert_text("2ml of rose water", Volume::Millilitre(2.0));
            assert_text("1.75 ml of rose water", Volume::Millilitre(1.75));
            assert_text("1.75ml of rose water", Volume::Millilitre(1.75));
            assert_text("1 3/4 millilitres of rose water", Volume::Millilitre(1.75));
            assert_text("1 3/4 milliliters of rose water", Volume::Millilitre(1.75));
        }

        #[test]
        fn test_centilitre() {
            assert_text("1.75 cl of rose water", Volume::Centilitre(1.75));
            assert_text("2cl of rose water", Volume::Centilitre(2.0));
            assert_text("1.75cl of rose water", Volume::Centilitre(1.75));
            assert_text("1 3/4 centilitres of rose water", Volume::Centilitre(1.75));
            assert_text("1 3/4 centiliters of rose water", Volume::Centilitre(1.75));
        }

        #[test]
        fn test_decilitre() {
            assert_text("2 dl of rose water", Volume::Decilitre(2.0));
            assert_text("1.75 dl of rose water", Volume::Decilitre(1.75));
            assert_text("1.75dl of rose water", Volume::Decilitre(1.75));
            assert_text("1 3/4 decilitres of rose water", Volume::Decilitre(1.75));
            assert_text("1 3/4 deciliters of rose water", Volume::Decilitre(1.75));
        }

        #[test]
        fn test_litre() {
            assert_text("1.75 l of rose water", Volume::Litre(1.75));
            assert_text("2 l of rose water", Volume::Litre(2.0));
            assert_text("1.75l of rose water", Volume::Litre(1.75));
            assert_text("1 3/4 litres of rose water", Volume::Litre(1.75));
            assert_text("1 3/4 liters of rose water", Volume::Litre(1.75));
        }

        #[test]
        fn test_metric_teaspoon() {
            assert_text("2 tsp of rose water", Volume::MetricTeaspoon(2.0));
            assert_text("1.75 tsp of rose water", Volume::MetricTeaspoon(1.75));
            assert_text("1.75ts of rose water", Volume::MetricTeaspoon(1.75));
            assert_text(
                "1 3/4 teaspoons of rose water",
                Volume::MetricTeaspoon(1.75),
            );
        }

        #[test]
        fn test_metric_tablespoon() {
            assert_text("2 tbsp of rose water", Volume::MetricTablespoon(2.0));
            assert_text("1.75 tbsp of rose water", Volume::MetricTablespoon(1.75));
            assert_text("1.75tbs of rose water", Volume::MetricTablespoon(1.75));
            assert_text(
                "1 3/4 tablespoons of rose water",
                Volume::MetricTablespoon(1.75),
            );
        }

        #[test]
        fn test_metric_dessertspoon() {
            assert_text("2 dsp of rose water", Volume::MetricDessertspoon(2.0));
            assert_text("1.75 dsp of rose water", Volume::MetricDessertspoon(1.75));
            assert_text("1.75ds of rose water", Volume::MetricDessertspoon(1.75));
            assert_text(
                "1 3/4 dessertspoons of rose water",
                Volume::MetricDessertspoon(1.75),
            );
            assert_text(
                "1 3/4 dessert spoons of rose water",
                Volume::MetricDessertspoon(1.75),
            );
        }

        #[test]
        fn test_metric_cup() {
            assert_text("2 cups of rose water", Volume::MetricCup(2.0));
            assert_text("1.75 cups of rose water", Volume::MetricCup(1.75));
            assert_text("1.75c of rose water", Volume::MetricCup(1.75));
            assert_text("1 3/4 c of rose water", Volume::MetricCup(1.75));
        }

        #[test]
        fn test_australian_teaspoon() {
            assert_text("2 tsp of rose water", Volume::MetricTeaspoon(2.0));
            assert_text("1.75 tsp of rose water", Volume::MetricTeaspoon(1.75));
            assert_text("1.75ts of rose water", Volume::MetricTeaspoon(1.75));
            assert_text(
                "1 3/4 teaspoons of rose water",
                Volume::MetricTeaspoon(1.75),
            );
        }

        #[test]
        fn test_australian_dessertspoon() {
            assert_text("2 dsp of rose water", Volume::MetricDessertspoon(2.0));
            assert_text("1.75 dsp of rose water", Volume::MetricDessertspoon(1.75));
            assert_text("1.75ds of rose water", Volume::MetricDessertspoon(1.75));
            assert_text(
                "1 3/4 dessertspoons of rose water",
                Volume::MetricDessertspoon(1.75),
            );
            assert_text(
                "1 3/4 dessert spoons of rose water",
                Volume::MetricDessertspoon(1.75),
            );
        }

        #[test]
        fn test_australian_tablespoon() {
            assert_text("2 tbsp of rose water", Volume::MetricTablespoon(2.0));
            assert_text("1.75 tbsp of rose water", Volume::MetricTablespoon(1.75));
            assert_text("1.75tbs of rose water", Volume::MetricTablespoon(1.75));
            assert_text(
                "1 3/4 tablespoons of rose water",
                Volume::MetricTablespoon(1.75),
            );
        }

        #[test]
        fn test_australian_cup() {
            assert_text("2 cups of rose water", Volume::MetricCup(2.0));
            assert_text("1.75 cups of rose water", Volume::MetricCup(1.75));
            assert_text("1.75c of rose water", Volume::MetricCup(1.75));
            assert_text("1 3/4 c of rose water", Volume::MetricCup(1.75));
        }

        #[test]
        fn test_imperial_teaspoon() {
            assert_text("2 tsp of rose water", Volume::MetricTeaspoon(2.0));
            assert_text("1.75 tsp of rose water", Volume::MetricTeaspoon(1.75));
            assert_text("1.75ts of rose water", Volume::MetricTeaspoon(1.75));
            assert_text(
                "1 3/4 teaspoons of rose water",
                Volume::MetricTeaspoon(1.75),
            );
        }

        #[test]
        fn test_imperial_dessertspoon() {
            assert_text("2 dsp of rose water", Volume::MetricDessertspoon(2.0));
            assert_text("1.75 dsp of rose water", Volume::MetricDessertspoon(1.75));
            assert_text("1.75ds of rose water", Volume::MetricDessertspoon(1.75));
            assert_text(
                "1 3/4 dessertspoons of rose water",
                Volume::MetricDessertspoon(1.75),
            );
            assert_text(
                "1 3/4 dessert spoons of rose water",
                Volume::MetricDessertspoon(1.75),
            );
        }

        #[test]
        fn test_imperial_tablespoon() {
            assert_text("1/2 tbsp cinnamon", Volume::MetricTablespoon(0.5));
            assert_text("2 tbsp of rose water", Volume::MetricTablespoon(2.0));
            assert_text("1.75 tbsp of rose water", Volume::MetricTablespoon(1.75));
            assert_text("1.75tbs of rose water", Volume::MetricTablespoon(1.75));
            assert_text(
                "1 3/4 tablespoons of rose water",
                Volume::MetricTablespoon(1.75),
            );
        }

        #[test]
        fn test_imperial_fluid_ounce() {
            assert_text("1.75 floz of rose water", Volume::ImperialFluidOunce(1.75));
            assert_text("2 floz of rose water", Volume::ImperialFluidOunce(2.0));
            assert_text(
                "1.75fl. oz. of rose water",
                Volume::ImperialFluidOunce(1.75),
            );
            assert_text(
                "1 3/4 fluid ounces of rose water",
                Volume::ImperialFluidOunce(1.75),
            );
        }

        #[test]
        fn test_imperial_gill() {
            assert_text("2gi of rose water", Volume::ImperialGill(2.0));
            assert_text("1.75gi of rose water", Volume::ImperialGill(1.75));
            assert_text("1.75 gills of rose water", Volume::ImperialGill(1.75));
        }

        #[test]
        fn test_imperial_cup() {
            assert_text("2 cups of rose water", Volume::MetricCup(2.0));
            assert_text("1.75 cups of rose water", Volume::MetricCup(1.75));
            assert_text("1.75c of rose water", Volume::MetricCup(1.75));
            assert_text("1 3/4 c of rose water", Volume::MetricCup(1.75));
        }

        #[test]
        fn test_imperial_pint() {
            assert_text("2pt of rose water", Volume::ImperialPint(2.0));
            assert_text("1.75pt of rose water", Volume::ImperialPint(1.75));
            assert_text("1.75 pt of rose water", Volume::ImperialPint(1.75));
            assert_text("1 3/4 pints of rose water", Volume::ImperialPint(1.75));
        }

        #[test]
        fn test_imperial_quart() {
            assert_text("2qt of rose water", Volume::ImperialQuart(2.0));
            assert_text("1.75qt of rose water", Volume::ImperialQuart(1.75));
            assert_text("1.75 qt of rose water", Volume::ImperialQuart(1.75));
            assert_text("1 3/4 quarts of rose water", Volume::ImperialQuart(1.75));
        }

        #[test]
        fn test_imperial_gallon() {
            assert_text("2gal of rose water", Volume::ImperialGallon(2.0));
            assert_text("1.75gal of rose water", Volume::ImperialGallon(1.75));
            assert_text("1.75 gal of rose water", Volume::ImperialGallon(1.75));
            assert_text("1 3/4 gallons of rose water", Volume::ImperialGallon(1.75));
        }

        #[test]
        fn test_us_teaspoon() {
            assert_text("2 tsp of rose water", Volume::MetricTeaspoon(2.0));
            assert_text("1.75 tsp of rose water", Volume::MetricTeaspoon(1.75));
            assert_text("1.75ts of rose water", Volume::MetricTeaspoon(1.75));
            assert_text(
                "1 3/4 teaspoons of rose water",
                Volume::MetricTeaspoon(1.75),
            );
        }

        #[test]
        fn test_us_tablespoon() {
            assert_text("2 tbsp of rose water", Volume::MetricTablespoon(2.0));
            assert_text("1.75 tbsp of rose water", Volume::MetricTablespoon(1.75));
            assert_text("2 tbsp of rose water", Volume::MetricTablespoon(2.0));
            assert_text("1.75tbs of rose water", Volume::MetricTablespoon(1.75));
            assert_text(
                "1 3/4 tablespoons of rose water",
                Volume::MetricTablespoon(1.75),
            );
        }

        #[test]
        fn test_us_fluid_ounce() {
            assert_text("2 floz of rose water", Volume::ImperialFluidOunce(2.0));
            assert_text("1.75 floz of rose water", Volume::ImperialFluidOunce(1.75));
            assert_text(
                "1.75fl. oz. of rose water",
                Volume::ImperialFluidOunce(1.75),
            );
            assert_text(
                "1 3/4 fluid ounces of rose water",
                Volume::ImperialFluidOunce(1.75),
            );
        }

        #[test]
        fn test_us_cup() {
            assert_text("2 cups of rose water", Volume::MetricCup(2.0));
            assert_text("1.75 cups of rose water", Volume::MetricCup(1.75));
            assert_text("1.75c of rose water", Volume::MetricCup(1.75));
            assert_text("1 3/4 c of rose water", Volume::MetricCup(1.75));
        }

        #[test]
        fn test_us_pint() {
            assert_text("2pt of rose water", Volume::ImperialPint(2.0));
            assert_text("1.75pt of rose water", Volume::ImperialPint(1.75));
            assert_text("1.75 pt of rose water", Volume::ImperialPint(1.75));
            assert_text("1 3/4 pints of rose water", Volume::ImperialPint(1.75));
        }

        #[test]
        fn test_us_quart() {
            assert_text("2qt of rose water", Volume::ImperialQuart(2.0));
            assert_text("1.75qt of rose water", Volume::ImperialQuart(1.75));
            assert_text("1.75 qt of rose water", Volume::ImperialQuart(1.75));
            assert_text("1 3/4 quarts of rose water", Volume::ImperialQuart(1.75));
        }

        #[test]
        fn test_us_gallon() {
            assert_text("2gal of rose water", Volume::ImperialGallon(2.0));
            assert_text("1.75gal of rose water", Volume::ImperialGallon(1.75));
            assert_text("1.75 gal of rose water", Volume::ImperialGallon(1.75));
            assert_text("1 3/4 gallons of rose water", Volume::ImperialGallon(1.75));
        }

        #[test]
        fn test_jigger() {
            assert_text("2jig of rose water", Volume::Jigger(2.0));
            assert_text("1.75jig of rose water", Volume::Jigger(1.75));
            assert_text("1.75 jig of rose water", Volume::Jigger(1.75));
            assert_text("1 3/4 jiggers of rose water", Volume::Jigger(1.75));
        }
    }
}
