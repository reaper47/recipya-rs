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
    pub fn id(&self) -> i16 {
        match self {
            MeasurementSystem::ImperialUK => 1,
            MeasurementSystem::Metric => 2,
            MeasurementSystem::MetricAustralia => 3,
            MeasurementSystem::UsCustomary => 4,
        }
    }

    /// Creates a MeasurementSystem from its numeric ID
    pub fn from_id(id: i16) -> Result<Self> {
        match id {
            1 => Ok(MeasurementSystem::ImperialUK),
            2 => Ok(MeasurementSystem::Metric),
            3 => Ok(MeasurementSystem::MetricAustralia),
            4 => Ok(MeasurementSystem::UsCustomary),
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
                            MeasurementSystem::ImperialUK => {
                                Unit::Volume(Volume::ImperialTeaspoon(v))
                            }
                            MeasurementSystem::Metric => Unit::Volume(Volume::MetricTeaspoon(v)),
                            MeasurementSystem::MetricAustralia => {
                                Unit::Volume(Volume::AustralianTeaspoon(v))
                            }
                            MeasurementSystem::UsCustomary => Unit::Volume(Volume::USTeaspoon(v)),
                        },
                        Volume::MetricTablespoon(v) => match self {
                            MeasurementSystem::ImperialUK => {
                                Unit::Volume(Volume::ImperialTablespoon(v))
                            }
                            MeasurementSystem::Metric => Unit::Volume(Volume::MetricTablespoon(v)),
                            MeasurementSystem::MetricAustralia => {
                                Unit::Volume(Volume::AustralianTablespoon(v))
                            }
                            MeasurementSystem::UsCustomary => Unit::Volume(Volume::USTablespoon(v)),
                        },
                        Volume::MetricDessertspoon(v) => match self {
                            MeasurementSystem::ImperialUK => {
                                Unit::Volume(Volume::ImperialDessertspoon(v))
                            }
                            MeasurementSystem::Metric => {
                                Unit::Volume(Volume::MetricDessertspoon(v))
                            }
                            MeasurementSystem::MetricAustralia => {
                                Unit::Volume(Volume::AustralianDessertspoon(v))
                            }
                            MeasurementSystem::UsCustomary => {
                                Unit::Volume(Volume::ImperialDessertspoon(v))
                            }
                        },
                        Volume::MetricCup(v) => match self {
                            MeasurementSystem::ImperialUK => Unit::Volume(Volume::ImperialCup(v)),
                            MeasurementSystem::Metric => Unit::Volume(Volume::MetricCup(v)),
                            MeasurementSystem::MetricAustralia => {
                                Unit::Volume(Volume::AustralianCup(v))
                            }
                            MeasurementSystem::UsCustomary => Unit::Volume(Volume::USCup(v)),
                        },
                        Volume::ImperialFluidOunce(v) => match self {
                            MeasurementSystem::UsCustomary => Unit::Volume(Volume::USFluidOunce(v)),
                            _ => Unit::Volume(Volume::ImperialFluidOunce(v)),
                        },
                        Volume::ImperialPint(v) => match self {
                            MeasurementSystem::UsCustomary => Unit::Volume(Volume::USPint(v)),
                            _ => Unit::Volume(Volume::ImperialPint(v)),
                        },
                        Volume::ImperialQuart(v) => match self {
                            MeasurementSystem::UsCustomary => Unit::Volume(Volume::USQuart(v)),
                            _ => Unit::Volume(Volume::ImperialQuart(v)),
                        },
                        Volume::ImperialGallon(v) => match self {
                            MeasurementSystem::UsCustomary => Unit::Volume(Volume::USGallon(v)),
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
                Length::Millimetre(_) => MeasurementSystem::Metric,
                Length::Centimetre(_) => MeasurementSystem::Metric,
                Length::Metre(_) => MeasurementSystem::Metric,
                Length::Kilometre(_) => MeasurementSystem::Metric,
                Length::Inch(_) => MeasurementSystem::UsCustomary,
                Length::Foot(_) => MeasurementSystem::UsCustomary,
            },
            Unit::Mass(v) => match v {
                Mass::Milligram(_) => MeasurementSystem::Metric,
                Mass::Gram(_) => MeasurementSystem::Metric,
                Mass::Dekagram(_) => MeasurementSystem::Metric,
                Mass::Hectogram(_) => MeasurementSystem::Metric,
                Mass::Kilogram(_) => MeasurementSystem::Metric,
                Mass::Ounce(_) => MeasurementSystem::UsCustomary,
                Mass::Pound(_) => MeasurementSystem::UsCustomary,
            },
            Unit::Temperature(v) => match v {
                Temperature::Celsius(_) => MeasurementSystem::Metric,
                Temperature::Fahrenheit(_) => MeasurementSystem::UsCustomary,
            },
            Unit::Volume(v) => match v {
                Volume::Millilitre(_) => MeasurementSystem::Metric,
                Volume::Centilitre(_) => MeasurementSystem::Metric,
                Volume::Decilitre(_) => MeasurementSystem::Metric,
                Volume::Litre(_) => MeasurementSystem::Metric,
                Volume::MetricTeaspoon(_) => MeasurementSystem::Metric,
                Volume::MetricTablespoon(_) => MeasurementSystem::Metric,
                Volume::MetricDessertspoon(_) => MeasurementSystem::Metric,
                Volume::MetricCup(_) => MeasurementSystem::Metric,
                Volume::AustralianTeaspoon(_) => MeasurementSystem::MetricAustralia,
                Volume::AustralianDessertspoon(_) => MeasurementSystem::MetricAustralia,
                Volume::AustralianTablespoon(_) => MeasurementSystem::MetricAustralia,
                Volume::AustralianCup(_) => MeasurementSystem::MetricAustralia,
                Volume::ImperialTeaspoon(_) => MeasurementSystem::ImperialUK,
                Volume::ImperialDessertspoon(_) => MeasurementSystem::ImperialUK,
                Volume::ImperialTablespoon(_) => MeasurementSystem::ImperialUK,
                Volume::ImperialFluidOunce(_) => MeasurementSystem::ImperialUK,
                Volume::ImperialGill(_) => MeasurementSystem::ImperialUK,
                Volume::ImperialCup(_) => MeasurementSystem::ImperialUK,
                Volume::ImperialPint(_) => MeasurementSystem::ImperialUK,
                Volume::ImperialQuart(_) => MeasurementSystem::ImperialUK,
                Volume::ImperialGallon(_) => MeasurementSystem::ImperialUK,
                Volume::USTeaspoon(_) => MeasurementSystem::UsCustomary,
                Volume::USTablespoon(_) => MeasurementSystem::UsCustomary,
                Volume::USFluidOunce(_) => MeasurementSystem::UsCustomary,
                Volume::USCup(_) => MeasurementSystem::UsCustomary,
                Volume::USPint(_) => MeasurementSystem::UsCustomary,
                Volume::USQuart(_) => MeasurementSystem::UsCustomary,
                Volume::USGallon(_) => MeasurementSystem::UsCustomary,
                Volume::Jigger(_) => MeasurementSystem::UsCustomary,
            },
            Unit::Unitless(_) => MeasurementSystem::default(),
        }
    }
}

impl From<Vec<String>> for MeasurementSystem {
    fn from(elements: Vec<String>) -> Self {
        let mut counter = HashMap::new();

        for element in &elements {
            if let Ok(unit) = Unit::from_str(element) {
                *counter
                    .entry(MeasurementSystem::from_unit(&unit))
                    .or_insert(0) += 1;
            }
        }

        counter
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(system, _)| system)
            .unwrap_or(MeasurementSystem::Metric)
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
                    "2 lb beef chuck roast".to_string(),
                    "4 lb beef chuck roast".to_string(),
                    "1 stone potatoes".to_string(),
                    "8 fl oz beef stock".to_string(),
                    "4 oz carrots".to_string(),
                    "2 oz butter".to_string(),
                    "1 pint milk".to_string(),
                    "6 oz plain flour".to_string(),
                    "3 tbsp olive oil".to_string(),
                    "1 tsp salt".to_string(),
                    "½ tsp black pepper".to_string(),
                ]),
                MeasurementSystem::UsCustomary
            );
        }

        #[test]
        fn test_metric() {
            assert_eq!(
                MeasurementSystem::from(vec![
                    "1 kg chicken breast".to_string(),
                    "500 g pasta".to_string(),
                    "250 ml cream".to_string(),
                    "200 g mushrooms".to_string(),
                    "100 g parmesan cheese".to_string(),
                    "2 l vegetable broth".to_string(),
                    "30 ml olive oil".to_string(),
                    "15 g fresh basil".to_string(),
                    "5 g salt".to_string(),
                    "2 g black pepper".to_string(),
                ]),
                MeasurementSystem::Metric
            );
        }

        #[test]
        fn test_metric_australia() {
            assert_eq!(
                MeasurementSystem::from(vec![
                    "800 g lamb leg".to_string(),
                    "400 g sweet potato".to_string(),
                    "375 ml coconut milk".to_string(),
                    "250 g green beans".to_string(),
                    "125 ml fish sauce".to_string(),
                    "60 ml lime juice".to_string(),
                    "20 ml sesame oil".to_string(),
                    "1 tbsp brown sugar".to_string(),
                    "1 tsp ground coriander".to_string(),
                    "½ tsp chili flakes".to_string(),
                ]),
                MeasurementSystem::Metric
            );
        }

        #[test]
        fn test_us_customary() {
            assert_eq!(
                MeasurementSystem::from(vec![
                    "2 lbs ground turkey".to_string(),
                    "1 lb spaghetti".to_string(),
                    "8 oz cream cheese".to_string(),
                    "6 pt blue cheese".to_string(),
                    "6 floz water".to_string(),
                    "6gallons of gas".to_string(),
                    "1 foot bread".to_string(),
                    "½ cup grated cheddar".to_string(),
                    "¼ cup olive oil".to_string(),
                    "2 tbsp garlic powder".to_string(),
                    "1 tsp dried oregano".to_string(),
                    "½ tsp paprika".to_string(),
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
