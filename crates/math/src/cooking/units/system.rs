use std::{collections::HashMap, str::FromStr};

use strum::{Display, EnumIter};

use crate::{
    Error, Result,
    cooking::units::{
        length::units::Length, mass::units::Mass, temperature::units::Temperature,
        traits::UnitScaler, unit::Unit, volume::units::Volume,
    },
};

#[derive(Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd, Display, EnumIter)]
pub enum MeasurementSystem {
    #[strum(serialize = "Imperial (UK)")]
    ImperialUK,
    #[default]
    #[strum(serialize = "Metric")]
    Metric,
    #[strum(serialize = "Metric (Australia)")]
    MetricAustralia,
    #[strum(serialize = "US Customary")]
    UsCustomary,
}

impl MeasurementSystem {
    /// Maps each measurement system to a numeric ID:
    /// - Imperial UK => 1
    /// - Metric => 2
    /// - Metric (Australia) => 3
    /// - US Customary => 4
    pub const fn id(&self) -> i16 {
        match self {
            Self::ImperialUK => 1,
            Self::Metric => 2,
            Self::MetricAustralia => 3,
            Self::UsCustomary => 4,
        }
    }

    /// Creates a `MeasurementSystem` from its numeric ID
    pub fn from_id(id: i16) -> Result<Self> {
        match id {
            1 => Ok(Self::ImperialUK),
            2 => Ok(Self::Metric),
            3 => Ok(Self::MetricAustralia),
            4 => Ok(Self::UsCustomary),
            _ => Err(Error::InvalidMeasurementSystem(id)),
        }
    }

    pub fn scale(&self, elements: Vec<String>, factor: f64) -> Vec<String> {
        elements
            .into_iter()
            .map(|s| {
                let Ok(unit) = Unit::from_str(&s) else {
                    return s;
                };

                let Ok(unit) = match unit {
                    Unit::Volume(u) => match u {
                        Volume::MetricTeaspoon(v) => match self {
                            Self::ImperialUK => Unit::Volume(Volume::ImperialTeaspoon(v)),
                            Self::Metric => Unit::Volume(Volume::MetricTeaspoon(v)),
                            Self::MetricAustralia => Unit::Volume(Volume::AustralianTeaspoon(v)),
                            Self::UsCustomary => Unit::Volume(Volume::USTeaspoon(v)),
                        },
                        Volume::MetricTablespoon(v) => match self {
                            Self::ImperialUK => Unit::Volume(Volume::ImperialTablespoon(v)),
                            Self::Metric => Unit::Volume(Volume::MetricTablespoon(v)),
                            Self::MetricAustralia => Unit::Volume(Volume::AustralianTablespoon(v)),
                            Self::UsCustomary => Unit::Volume(Volume::USTablespoon(v)),
                        },
                        Volume::MetricDessertspoon(v) => match self {
                            Self::ImperialUK => Unit::Volume(Volume::ImperialDessertspoon(v)),
                            Self::Metric => Unit::Volume(Volume::MetricDessertspoon(v)),
                            Self::MetricAustralia => {
                                Unit::Volume(Volume::AustralianDessertspoon(v))
                            }
                            Self::UsCustomary => Unit::Volume(Volume::ImperialDessertspoon(v)),
                        },
                        Volume::MetricCup(v) => match self {
                            Self::ImperialUK => Unit::Volume(Volume::ImperialCup(v)),
                            Self::Metric => Unit::Volume(Volume::MetricCup(v)),
                            Self::MetricAustralia => Unit::Volume(Volume::AustralianCup(v)),
                            Self::UsCustomary => Unit::Volume(Volume::USCup(v)),
                        },
                        Volume::ImperialFluidOunce(v) => match self {
                            Self::UsCustomary => Unit::Volume(Volume::USFluidOunce(v)),
                            _ => Unit::Volume(Volume::ImperialFluidOunce(v)),
                        },
                        Volume::ImperialPint(v) => match self {
                            Self::UsCustomary => Unit::Volume(Volume::USPint(v)),
                            _ => Unit::Volume(Volume::ImperialPint(v)),
                        },
                        Volume::ImperialQuart(v) => match self {
                            Self::UsCustomary => Unit::Volume(Volume::USQuart(v)),
                            _ => Unit::Volume(Volume::ImperialQuart(v)),
                        },
                        Volume::ImperialGallon(v) => match self {
                            Self::UsCustomary => Unit::Volume(Volume::USGallon(v)),
                            _ => Unit::Volume(Volume::ImperialGallon(v)),
                        },
                        _ => Unit::Volume(u),
                    },
                    _ => unit,
                }
                .scale(factor) else {
                    return s;
                };

                unit.replace(s)
            })
            .collect()
    }

    /// Determines the measurement system from the unit.
    fn from_unit(unit: &Unit) -> Self {
        match unit {
            Unit::Length(v) => match v {
                Length::Millimetre(_) => Self::Metric,
                Length::Centimetre(_) => Self::Metric,
                Length::Metre(_) => Self::Metric,
                Length::Kilometre(_) => Self::Metric,
                Length::Inch(_) => Self::UsCustomary,
                Length::Foot(_) => Self::UsCustomary,
            },
            Unit::Mass(v) => match v {
                Mass::Milligram(_) => Self::Metric,
                Mass::Gram(_) => Self::Metric,
                Mass::Dekagram(_) => Self::Metric,
                Mass::Hectogram(_) => Self::Metric,
                Mass::Kilogram(_) => Self::Metric,
                Mass::Ounce(_) => Self::UsCustomary,
                Mass::Pound(_) => Self::UsCustomary,
            },
            Unit::Temperature(v) => match v {
                Temperature::Celsius(_) => Self::Metric,
                Temperature::Fahrenheit(_) => Self::UsCustomary,
            },
            Unit::Volume(v) => match v {
                Volume::Millilitre(_) => Self::Metric,
                Volume::Centilitre(_) => Self::Metric,
                Volume::Decilitre(_) => Self::Metric,
                Volume::Litre(_) => Self::Metric,
                Volume::MetricTeaspoon(_) => Self::Metric,
                Volume::MetricTablespoon(_) => Self::Metric,
                Volume::MetricDessertspoon(_) => Self::Metric,
                Volume::MetricCup(_) => Self::Metric,
                Volume::AustralianTeaspoon(_) => Self::MetricAustralia,
                Volume::AustralianDessertspoon(_) => Self::MetricAustralia,
                Volume::AustralianTablespoon(_) => Self::MetricAustralia,
                Volume::AustralianCup(_) => Self::MetricAustralia,
                Volume::ImperialTeaspoon(_) => Self::ImperialUK,
                Volume::ImperialDessertspoon(_) => Self::ImperialUK,
                Volume::ImperialTablespoon(_) => Self::ImperialUK,
                Volume::ImperialFluidOunce(_) => Self::ImperialUK,
                Volume::ImperialGill(_) => Self::ImperialUK,
                Volume::ImperialCup(_) => Self::ImperialUK,
                Volume::ImperialPint(_) => Self::ImperialUK,
                Volume::ImperialQuart(_) => Self::ImperialUK,
                Volume::ImperialGallon(_) => Self::ImperialUK,
                Volume::USTeaspoon(_) => Self::UsCustomary,
                Volume::USTablespoon(_) => Self::UsCustomary,
                Volume::USFluidOunce(_) => Self::UsCustomary,
                Volume::USCup(_) => Self::UsCustomary,
                Volume::USPint(_) => Self::UsCustomary,
                Volume::USQuart(_) => Self::UsCustomary,
                Volume::USGallon(_) => Self::UsCustomary,
                Volume::Jigger(_) => Self::UsCustomary,
            },
            Unit::Unitless(_) => Self::default(),
        }
    }
}

impl From<Vec<&str>> for MeasurementSystem {
    fn from(elements: Vec<&str>) -> Self {
        let mut counter = HashMap::new();

        for element in &elements {
            if let Ok(unit) = Unit::from_str(element) {
                *counter.entry(Self::from_unit(&unit)).or_insert(0) += 1;
            }
        }

        counter
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(system, _)| system)
            .unwrap_or(Self::Metric)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", MeasurementSystem::ImperialUK),
            "Imperial (UK)"
        );
        assert_eq!(format!("{}", MeasurementSystem::Metric), "Metric");
        assert_eq!(
            format!("{}", MeasurementSystem::MetricAustralia),
            "Metric (Australia)"
        );
        assert_eq!(
            format!("{}", MeasurementSystem::UsCustomary),
            "US Customary"
        );
    }

    mod tests_from_str_vec {
        use super::*;

        #[test]
        fn test_imperial_uk() {
            assert_eq!(
                MeasurementSystem::from(vec![
                    "2 lb beef chuck roast",
                    "4 lb beef chuck roast",
                    "1 stone potatoes",
                    "8 fl oz beef stock",
                    "4 oz carrots",
                    "2 oz butter",
                    "1 pint milk",
                    "6 oz plain flour",
                    "3 tbsp olive oil",
                    "1 tsp salt",
                    "½ tsp black pepper",
                ]),
                MeasurementSystem::UsCustomary
            );
        }

        #[test]
        fn test_metric() {
            assert_eq!(
                MeasurementSystem::from(vec![
                    "1 kg chicken breast",
                    "500 g pasta",
                    "250 ml cream",
                    "200 g mushrooms",
                    "100 g parmesan cheese",
                    "2 l vegetable broth",
                    "30 ml olive oil",
                    "15 g fresh basil",
                    "5 g salt",
                    "2 g black pepper",
                ]),
                MeasurementSystem::Metric
            );
        }

        #[test]
        fn test_metric_australia() {
            assert_eq!(
                MeasurementSystem::from(vec![
                    "800 g lamb leg",
                    "400 g sweet potato",
                    "375 ml coconut milk",
                    "250 g green beans",
                    "125 ml fish sauce",
                    "60 ml lime juice",
                    "20 ml sesame oil",
                    "1 tbsp brown sugar",
                    "1 tsp ground coriander",
                    "½ tsp chili flakes",
                ]),
                MeasurementSystem::Metric
            );
        }

        #[test]
        fn test_us_customary() {
            assert_eq!(
                MeasurementSystem::from(vec![
                    "2 lbs ground turkey",
                    "1 lb spaghetti",
                    "8 oz cream cheese",
                    "6 pt blue cheese",
                    "6 floz water",
                    "6gallons of gas",
                    "1 foot bread",
                    "½ cup grated cheddar",
                    "¼ cup olive oil",
                    "2 tbsp garlic powder",
                    "1 tsp dried oregano",
                    "½ tsp paprika",
                ]),
                MeasurementSystem::UsCustomary
            );
        }
    }

    #[test]
    fn test_id() {
        assert_eq!(MeasurementSystem::ImperialUK.id(), 1);
        assert_eq!(MeasurementSystem::Metric.id(), 2);
        assert_eq!(MeasurementSystem::MetricAustralia.id(), 3);
        assert_eq!(MeasurementSystem::UsCustomary.id(), 4);
    }

    mod tests_from_unit {
        use super::*;

        #[test]
        fn test_length() {
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Length(Length::Millimetre(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Length(Length::Centimetre(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Length(Length::Metre(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Length(Length::Kilometre(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Length(Length::Inch(1.0))),
                MeasurementSystem::UsCustomary
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Length(Length::Foot(1.0))),
                MeasurementSystem::UsCustomary
            );
        }

        #[test]
        fn test_mass() {
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Mass(Mass::Milligram(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Mass(Mass::Gram(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Mass(Mass::Dekagram(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Mass(Mass::Hectogram(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Mass(Mass::Kilogram(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Mass(Mass::Ounce(1.0))),
                MeasurementSystem::UsCustomary
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Mass(Mass::Pound(1.0))),
                MeasurementSystem::UsCustomary
            );
        }

        #[test]
        fn test_temperature() {
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Temperature(Temperature::Celsius(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Temperature(Temperature::Fahrenheit(1.0))),
                MeasurementSystem::UsCustomary
            );
        }

        #[test]
        fn test_us_customary() {
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::Millilitre(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::Centilitre(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::Decilitre(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::Litre(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::MetricTeaspoon(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::MetricDessertspoon(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::MetricTablespoon(1.0))),
                MeasurementSystem::Metric
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::MetricCup(1.0))),
                MeasurementSystem::Metric
            );

            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::AustralianTeaspoon(1.0))),
                MeasurementSystem::MetricAustralia
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::AustralianDessertspoon(1.0))),
                MeasurementSystem::MetricAustralia
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::AustralianTablespoon(1.0))),
                MeasurementSystem::MetricAustralia
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::AustralianCup(1.0))),
                MeasurementSystem::MetricAustralia
            );

            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::ImperialTeaspoon(1.0))),
                MeasurementSystem::ImperialUK
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::ImperialDessertspoon(1.0))),
                MeasurementSystem::ImperialUK
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::ImperialTablespoon(1.0))),
                MeasurementSystem::ImperialUK
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::ImperialFluidOunce(1.0))),
                MeasurementSystem::ImperialUK
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::ImperialGill(1.0))),
                MeasurementSystem::ImperialUK
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::ImperialCup(1.0))),
                MeasurementSystem::ImperialUK
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::ImperialPint(1.0))),
                MeasurementSystem::ImperialUK
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::ImperialQuart(1.0))),
                MeasurementSystem::ImperialUK
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::ImperialGallon(1.0))),
                MeasurementSystem::ImperialUK
            );

            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::USTeaspoon(1.0))),
                MeasurementSystem::UsCustomary
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::USTablespoon(1.0))),
                MeasurementSystem::UsCustomary
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::USFluidOunce(1.0))),
                MeasurementSystem::UsCustomary
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::USCup(1.0))),
                MeasurementSystem::UsCustomary
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::USPint(1.0))),
                MeasurementSystem::UsCustomary
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::USQuart(1.0))),
                MeasurementSystem::UsCustomary
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::USGallon(1.0))),
                MeasurementSystem::UsCustomary
            );
            assert_eq!(
                MeasurementSystem::from_unit(&Unit::Volume(Volume::Jigger(1.0))),
                MeasurementSystem::UsCustomary
            );
        }
    }
}
