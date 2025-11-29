use crate::Result;
use crate::cooking::units::custom::factors::volume::*;
use crate::cooking::units::traits::{UnitOperations, UnitScaler};
use crate::cooking::units::unit::Unit;
use crate::cooking::units::volume::units::Volume;

impl UnitScaler for Volume {
    fn scale(&self, factor: f64) -> Result<Unit> {
        let scaled_value = self.value() * factor;

        match self {
            Volume::Millilitre(_)
            | Volume::Centilitre(_)
            | Volume::Decilitre(_)
            | Volume::Litre(_) => {
                let litres = match self {
                    Volume::Millilitre(_) => {
                        scaled_value / measurements::volume::LITER_MILLILITERS_FACTOR
                    }
                    Volume::Centilitre(_) => scaled_value / LITRE_CENTILITRE_FACTOR,
                    Volume::Decilitre(_) => scaled_value / LITRE_DECILITRE_FACTOR,
                    Volume::Litre(_) => scaled_value,
                    _ => unreachable!(),
                };

                if litres >= 1.0 {
                    Ok(Unit::Volume(Volume::Litre(litres)))
                } else if litres >= 1e-1 {
                    Ok(Unit::Volume(Volume::Decilitre(litres * 1e1)))
                } else if litres >= 1e-2 {
                    Ok(Unit::Volume(Volume::Centilitre(litres * 1e2)))
                } else {
                    Ok(Unit::Volume(Volume::Millilitre(litres * 1e3)))
                }
            }

            Volume::MetricTeaspoon(_)
            | Volume::MetricTablespoon(_)
            | Volume::MetricDessertspoon(_)
            | Volume::MetricCup(_) => {
                let tsp = match self {
                    Volume::MetricTeaspoon(_) => scaled_value,
                    Volume::MetricTablespoon(_) => scaled_value * TABLESPOON_TEASPOON_FACTOR,
                    Volume::MetricDessertspoon(_) => scaled_value * DESERT_SPOON_TEASPOON_FACTOR,
                    Volume::MetricCup(_) => scaled_value * METRIC_CUP_TEASPOON_FACTOR,
                    _ => unreachable!(),
                };

                if tsp >= METRIC_CUP_TEASPOON_FACTOR {
                    Ok(Unit::Volume(Volume::MetricCup(
                        tsp / METRIC_CUP_TEASPOON_FACTOR,
                    )))
                } else if tsp >= TABLESPOON_TEASPOON_FACTOR {
                    Ok(Unit::Volume(Volume::MetricTablespoon(
                        tsp / TABLESPOON_TEASPOON_FACTOR,
                    )))
                } else if tsp >= DESERT_SPOON_TEASPOON_FACTOR {
                    Ok(Unit::Volume(Volume::MetricDessertspoon(
                        tsp / DESERT_SPOON_TEASPOON_FACTOR,
                    )))
                } else {
                    Ok(Unit::Volume(Volume::MetricTeaspoon(tsp)))
                }
            }

            Volume::AustralianTeaspoon(_)
            | Volume::AustralianDessertspoon(_)
            | Volume::AustralianTablespoon(_)
            | Volume::AustralianCup(_) => {
                let tsp = match self {
                    Volume::AustralianTeaspoon(_) => scaled_value,
                    Volume::AustralianDessertspoon(_) => {
                        scaled_value * DESERT_SPOON_TEASPOON_FACTOR
                    }
                    Volume::AustralianTablespoon(_) => {
                        scaled_value * AUSTRALIAN_TABLESPOON_TEASPOON_FACTOR
                    }
                    Volume::AustralianCup(_) => scaled_value * METRIC_CUP_TEASPOON_FACTOR,
                    _ => unreachable!(),
                };

                if tsp >= METRIC_CUP_TEASPOON_FACTOR {
                    Ok(Unit::Volume(Volume::AustralianCup(
                        tsp / METRIC_CUP_TEASPOON_FACTOR,
                    )))
                } else if tsp >= AUSTRALIAN_TABLESPOON_TEASPOON_FACTOR {
                    Ok(Unit::Volume(Volume::AustralianTablespoon(
                        tsp / AUSTRALIAN_TABLESPOON_TEASPOON_FACTOR,
                    )))
                } else if tsp >= DESERT_SPOON_TEASPOON_FACTOR {
                    Ok(Unit::Volume(Volume::AustralianDessertspoon(
                        tsp / DESERT_SPOON_TEASPOON_FACTOR,
                    )))
                } else {
                    Ok(Unit::Volume(Volume::AustralianTeaspoon(tsp)))
                }
            }

            Volume::ImperialTeaspoon(_)
            | Volume::ImperialDessertspoon(_)
            | Volume::ImperialTablespoon(_)
            | Volume::ImperialCup(_) => {
                let tsp = match self {
                    Volume::ImperialTeaspoon(_) => scaled_value,
                    Volume::ImperialDessertspoon(_) => scaled_value * DESERT_SPOON_TEASPOON_FACTOR,
                    Volume::ImperialTablespoon(_) => scaled_value * TABLESPOON_TEASPOON_FACTOR,
                    Volume::ImperialCup(_) => scaled_value * CUP_TEASPOON_FACTOR,
                    _ => unreachable!(),
                };

                if tsp >= CUP_TEASPOON_FACTOR {
                    Ok(Unit::Volume(Volume::ImperialCup(tsp / CUP_TEASPOON_FACTOR)))
                } else if tsp >= TABLESPOON_TEASPOON_FACTOR {
                    Ok(Unit::Volume(Volume::ImperialTablespoon(
                        tsp / TABLESPOON_TEASPOON_FACTOR,
                    )))
                } else if tsp >= DESERT_SPOON_TEASPOON_FACTOR {
                    Ok(Unit::Volume(Volume::ImperialDessertspoon(
                        tsp / DESERT_SPOON_TEASPOON_FACTOR,
                    )))
                } else {
                    Ok(Unit::Volume(Volume::ImperialTeaspoon(tsp)))
                }
            }

            Volume::ImperialFluidOunce(_)
            | Volume::ImperialGill(_)
            | Volume::ImperialPint(_)
            | Volume::ImperialQuart(_)
            | Volume::ImperialGallon(_) => {
                let floz = match self {
                    Volume::ImperialFluidOunce(_) => scaled_value,
                    Volume::ImperialGill(_) => scaled_value * IMPERIAL_GILL_FLOZ_FACTOR,
                    Volume::ImperialPint(_) => scaled_value * IMPERIAL_PINT_FLOZ_FACTOR,
                    Volume::ImperialQuart(_) => scaled_value * IMPERIAL_QUART_FLOZ_FACTOR,
                    Volume::ImperialGallon(_) => scaled_value * IMPERIAL_GALLON_FLOZ_FACTOR,
                    _ => unreachable!(),
                };

                if floz >= IMPERIAL_GALLON_FLOZ_FACTOR {
                    Ok(Unit::Volume(Volume::ImperialGallon(
                        floz / IMPERIAL_GALLON_FLOZ_FACTOR,
                    )))
                } else if floz >= IMPERIAL_QUART_FLOZ_FACTOR {
                    Ok(Unit::Volume(Volume::ImperialQuart(
                        floz / IMPERIAL_QUART_FLOZ_FACTOR,
                    )))
                } else if floz >= IMPERIAL_PINT_FLOZ_FACTOR {
                    Ok(Unit::Volume(Volume::ImperialPint(
                        floz / IMPERIAL_PINT_FLOZ_FACTOR,
                    )))
                } else if floz >= IMPERIAL_GILL_FLOZ_FACTOR {
                    Ok(Unit::Volume(Volume::ImperialGill(
                        floz / IMPERIAL_GILL_FLOZ_FACTOR,
                    )))
                } else {
                    Ok(Unit::Volume(Volume::ImperialFluidOunce(floz)))
                }
            }

            Volume::USTeaspoon(_) | Volume::USTablespoon(_) | Volume::USCup(_) => {
                let tsp = match self {
                    Volume::USTeaspoon(_) => scaled_value,
                    Volume::USTablespoon(_) => scaled_value * TABLESPOON_TEASPOON_FACTOR,
                    Volume::USCup(_) => scaled_value * CUP_TEASPOON_FACTOR,
                    _ => unreachable!(),
                };

                if tsp >= CUP_TEASPOON_FACTOR {
                    Ok(Unit::Volume(Volume::USCup(tsp / CUP_TEASPOON_FACTOR)))
                } else if tsp >= TABLESPOON_TEASPOON_FACTOR {
                    Ok(Unit::Volume(Volume::USTablespoon(
                        tsp / TABLESPOON_TEASPOON_FACTOR,
                    )))
                } else {
                    Ok(Unit::Volume(Volume::USTeaspoon(tsp)))
                }
            }

            Volume::USFluidOunce(_)
            | Volume::USPint(_)
            | Volume::USQuart(_)
            | Volume::USGallon(_) => {
                let floz = match self {
                    Volume::USFluidOunce(_) => scaled_value,
                    Volume::USPint(_) => scaled_value * US_PINT_FLOZ_FACTOR,
                    Volume::USQuart(_) => scaled_value * US_QUART_FLOZ_FACTOR,
                    Volume::USGallon(_) => scaled_value * US_GALLON_FLOZ_FACTOR,
                    _ => unreachable!(),
                };

                if floz >= US_GALLON_FLOZ_FACTOR {
                    Ok(Unit::Volume(Volume::USGallon(floz / US_GALLON_FLOZ_FACTOR)))
                } else if floz >= US_QUART_FLOZ_FACTOR {
                    Ok(Unit::Volume(Volume::USQuart(floz / US_QUART_FLOZ_FACTOR)))
                } else if floz >= US_PINT_FLOZ_FACTOR {
                    Ok(Unit::Volume(Volume::USPint(floz / US_PINT_FLOZ_FACTOR)))
                } else {
                    Ok(Unit::Volume(Volume::USFluidOunce(floz)))
                }
            }

            Volume::Jigger(_) => Ok(Unit::Volume(self.with_value(scaled_value))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cooking::units::traits::UnitOperations;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn assert_approx_eq(actual: Unit, expected: Unit, threshold: f64) {
        let actual_value = actual.value();
        let expected_value = expected.value();

        assert!(
            (actual_value - expected_value).abs() < threshold,
            "Expected {actual:?}, got {expected:?}, difference: {}",
            (actual_value - expected_value).abs()
        );
    }

    #[test]
    fn test_ml() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::Millilitre(5.0)).scale(1.5)?,
            Unit::Volume(Volume::Millilitre(7.5))
        );
        assert_eq!(
            Unit::Volume(Volume::Millilitre(5.0)).scale(2.0)?,
            Unit::Volume(Volume::Centilitre(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::Millilitre(5.0)).scale(20.0)?,
            Unit::Volume(Volume::Decilitre(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::Millilitre(5.0)).scale(200.0)?,
            Unit::Volume(Volume::Litre(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_cl() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::Centilitre(1.0)).scale(0.5)?,
            Unit::Volume(Volume::Millilitre(5.0))
        );
        assert_eq!(
            Unit::Volume(Volume::Centilitre(1.0)).scale(2.0)?,
            Unit::Volume(Volume::Centilitre(2.0))
        );
        assert_eq!(
            Unit::Volume(Volume::Centilitre(1.0)).scale(15.0)?,
            Unit::Volume(Volume::Decilitre(1.5))
        );
        assert_eq!(
            Unit::Volume(Volume::Centilitre(15.0)).scale(10.0)?,
            Unit::Volume(Volume::Litre(1.5))
        );
        Ok(())
    }

    #[test]
    fn test_dl() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::Decilitre(5.0)).scale(0.01)?,
            Unit::Volume(Volume::Millilitre(5.0))
        );
        assert_eq!(
            Unit::Volume(Volume::Decilitre(5.0)).scale(0.1)?,
            Unit::Volume(Volume::Centilitre(5.0))
        );
        assert_eq!(
            Unit::Volume(Volume::Decilitre(5.0)).scale(1.5)?,
            Unit::Volume(Volume::Decilitre(7.5))
        );
        assert_eq!(
            Unit::Volume(Volume::Decilitre(5.0)).scale(3.0)?,
            Unit::Volume(Volume::Litre(1.5))
        );
        Ok(())
    }

    #[test]
    fn test_l() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::Litre(2.0)).scale(0.0005)?,
            Unit::Volume(Volume::Millilitre(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::Litre(2.0)).scale(0.005)?,
            Unit::Volume(Volume::Centilitre(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::Litre(1.0)).scale(0.5)?,
            Unit::Volume(Volume::Decilitre(5.0))
        );
        assert_eq!(
            Unit::Volume(Volume::Litre(1.0)).scale(5.0)?,
            Unit::Volume(Volume::Litre(5.0))
        );
        Ok(())
    }

    #[test]
    fn test_tsp_uk() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::ImperialTeaspoon(1.0)).scale(1.5)?,
            Unit::Volume(Volume::ImperialTeaspoon(1.5))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTeaspoon(1.0)).scale(2.0)?,
            Unit::Volume(Volume::ImperialDessertspoon(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTeaspoon(1.5)).scale(3.0)?,
            Unit::Volume(Volume::ImperialTablespoon(1.5))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTeaspoon(10.0)).scale(4.8)?,
            Unit::Volume(Volume::ImperialCup(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_dsp_uk() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::ImperialDessertspoon(0.5)).scale(1.0)?,
            Unit::Volume(Volume::ImperialTeaspoon(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialDessertspoon(1.0)).scale(1.3)?,
            Unit::Volume(Volume::ImperialDessertspoon(1.3))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialDessertspoon(1.5)).scale(3.0)?,
            Unit::Volume(Volume::ImperialTablespoon(3.0))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialDessertspoon(10.0)).scale(4.8)?,
            Unit::Volume(Volume::ImperialCup(2.0))
        );
        Ok(())
    }

    #[test]
    fn test_tbsp_uk() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::ImperialTablespoon(0.5)).scale(0.5)?,
            Unit::Volume(Volume::ImperialTeaspoon(0.75))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTablespoon(0.5)).scale(1.5)?,
            Unit::Volume(Volume::ImperialDessertspoon(1.125))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTablespoon(1.5)).scale(3.0)?,
            Unit::Volume(Volume::ImperialTablespoon(4.5))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialTablespoon(10.0)).scale(4.8)?,
            Unit::Volume(Volume::ImperialCup(3.0))
        );
        Ok(())
    }

    #[test]
    fn test_cup_uk() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::ImperialCup(0.25)).scale(0.16)?,
            Unit::Volume(Volume::ImperialTeaspoon(1.92))
        );
        assert_approx_eq(
            Unit::Volume(Volume::ImperialCup(0.25)).scale(0.20)?,
            Unit::Volume(Volume::ImperialDessertspoon(1.2)),
            1e-6,
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialCup(0.75)).scale(0.5)?,
            Unit::Volume(Volume::ImperialTablespoon(6.0))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialCup(10.0)).scale(4.8)?,
            Unit::Volume(Volume::ImperialCup(48.0))
        );
        Ok(())
    }

    #[test]
    fn test_tsp_us() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::USTeaspoon(1.0)).scale(1.5)?,
            Unit::Volume(Volume::USTeaspoon(1.5))
        );
        assert_eq!(
            Unit::Volume(Volume::USTeaspoon(3.0)).scale(2.0)?,
            Unit::Volume(Volume::USTablespoon(2.0))
        );
        assert_eq!(
            Unit::Volume(Volume::USTeaspoon(2.0)).scale(30.0)?,
            Unit::Volume(Volume::USCup(1.25))
        );
        Ok(())
    }

    #[test]
    fn test_tbsp_us() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::USTablespoon(0.5)).scale(0.5)?,
            Unit::Volume(Volume::USTeaspoon(0.75))
        );
        assert_eq!(
            Unit::Volume(Volume::USTablespoon(1.5)).scale(3.0)?,
            Unit::Volume(Volume::USTablespoon(4.5))
        );
        assert_eq!(
            Unit::Volume(Volume::USTablespoon(10.0)).scale(4.8)?,
            Unit::Volume(Volume::USCup(3.0))
        );
        Ok(())
    }

    #[test]
    fn test_cup_us() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::USCup(0.25)).scale(0.16)?,
            Unit::Volume(Volume::USTeaspoon(1.92))
        );
        assert_eq!(
            Unit::Volume(Volume::USCup(0.75)).scale(0.5)?,
            Unit::Volume(Volume::USTablespoon(6.0))
        );
        assert_eq!(
            Unit::Volume(Volume::USCup(10.0)).scale(4.8)?,
            Unit::Volume(Volume::USCup(48.0))
        );
        Ok(())
    }

    #[test]
    fn test_tsp_metric() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::MetricTeaspoon(1.0)).scale(1.5)?,
            Unit::Volume(Volume::MetricTeaspoon(1.5))
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTeaspoon(1.0)).scale(2.0)?,
            Unit::Volume(Volume::MetricDessertspoon(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTeaspoon(1.5)).scale(3.0)?,
            Unit::Volume(Volume::MetricTablespoon(1.5))
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTeaspoon(10.0)).scale(5.0)?,
            Unit::Volume(Volume::MetricCup(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_dsp_metric() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(0.5)).scale(1.0)?,
            Unit::Volume(Volume::MetricTeaspoon(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(1.0)).scale(1.3)?,
            Unit::Volume(Volume::MetricDessertspoon(1.3))
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(1.5)).scale(3.0)?,
            Unit::Volume(Volume::MetricTablespoon(3.0))
        );
        assert_eq!(
            Unit::Volume(Volume::MetricDessertspoon(10.0)).scale(5.0)?,
            Unit::Volume(Volume::MetricCup(2.0))
        );
        Ok(())
    }

    #[test]
    fn test_tbsp_metric() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::MetricTablespoon(0.5)).scale(0.5)?,
            Unit::Volume(Volume::MetricTeaspoon(0.75))
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTablespoon(0.5)).scale(1.5)?,
            Unit::Volume(Volume::MetricDessertspoon(1.125))
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTablespoon(1.5)).scale(3.0)?,
            Unit::Volume(Volume::MetricTablespoon(4.5))
        );
        assert_eq!(
            Unit::Volume(Volume::MetricTablespoon(10.0)).scale(5.0)?,
            Unit::Volume(Volume::MetricCup(3.0))
        );
        Ok(())
    }

    #[test]
    fn test_cup_metric() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::MetricCup(0.25)).scale(0.15)?,
            Unit::Volume(Volume::MetricTeaspoon(1.875))
        );
        assert_eq!(
            Unit::Volume(Volume::MetricCup(0.25)).scale(0.20)?,
            Unit::Volume(Volume::MetricDessertspoon(1.25)),
        );
        assert_eq!(
            Unit::Volume(Volume::MetricCup(0.75)).scale(0.5)?,
            Unit::Volume(Volume::MetricTablespoon(6.25))
        );
        assert_eq!(
            Unit::Volume(Volume::MetricCup(10.0)).scale(5.0)?,
            Unit::Volume(Volume::MetricCup(50.0))
        );
        Ok(())
    }

    #[test]
    fn test_floz_us() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::USFluidOunce(1.0)).scale(2.0)?,
            Unit::Volume(Volume::USFluidOunce(2.0))
        );
        assert_eq!(
            Unit::Volume(Volume::USFluidOunce(5.0)).scale(4.0)?,
            Unit::Volume(Volume::USPint(1.25)),
        );
        assert_eq!(
            Unit::Volume(Volume::USFluidOunce(10.0)).scale(4.0)?,
            Unit::Volume(Volume::USQuart(1.25))
        );
        assert_eq!(
            Unit::Volume(Volume::USFluidOunce(60.0)).scale(100.0)?,
            Unit::Volume(Volume::USGallon(46.875))
        );
        Ok(())
    }

    #[test]
    fn test_pint_us() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::USPint(1.0)).scale(0.5)?,
            Unit::Volume(Volume::USFluidOunce(8.0))
        );
        assert_eq!(
            Unit::Volume(Volume::USPint(0.75)).scale(2.0)?,
            Unit::Volume(Volume::USPint(1.5)),
        );
        assert_eq!(
            Unit::Volume(Volume::USPint(2.0)).scale(2.0)?,
            Unit::Volume(Volume::USQuart(2.0))
        );
        assert_eq!(
            Unit::Volume(Volume::USPint(4.0)).scale(3.0)?,
            Unit::Volume(Volume::USGallon(1.5))
        );
        Ok(())
    }

    #[test]
    fn test_quart_us() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::USQuart(0.3)).scale(0.5)?,
            Unit::Volume(Volume::USFluidOunce(4.8))
        );
        assert_eq!(
            Unit::Volume(Volume::USQuart(0.4)).scale(2.0)?,
            Unit::Volume(Volume::USPint(1.6)),
        );
        assert_eq!(
            Unit::Volume(Volume::USQuart(1.0)).scale(2.0)?,
            Unit::Volume(Volume::USQuart(2.0))
        );
        assert_eq!(
            Unit::Volume(Volume::USQuart(4.0)).scale(3.0)?,
            Unit::Volume(Volume::USGallon(3.0))
        );
        Ok(())
    }

    #[test]
    fn test_gallon_us() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::USGallon(0.02)).scale(0.5)?,
            Unit::Volume(Volume::USFluidOunce(1.28))
        );
        assert_eq!(
            Unit::Volume(Volume::USGallon(0.1)).scale(2.0)?,
            Unit::Volume(Volume::USPint(1.6)),
        );
        assert_eq!(
            Unit::Volume(Volume::USGallon(0.2)).scale(2.0)?,
            Unit::Volume(Volume::USQuart(1.6))
        );
        assert_eq!(
            Unit::Volume(Volume::USGallon(4.0)).scale(3.0)?,
            Unit::Volume(Volume::USGallon(12.0))
        );
        Ok(())
    }

    #[test]
    fn test_floz_uk() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::ImperialFluidOunce(0.5)).scale(1.5)?,
            Unit::Volume(Volume::ImperialFluidOunce(0.75))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialFluidOunce(2.0)).scale(3.0)?,
            Unit::Volume(Volume::ImperialGill(1.2)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialFluidOunce(10.0)).scale(2.0)?,
            Unit::Volume(Volume::ImperialPint(1.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialFluidOunce(14.0)).scale(3.0)?,
            Unit::Volume(Volume::ImperialQuart(1.05))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialFluidOunce(20.0)).scale(100.0)?,
            Unit::Volume(Volume::ImperialGallon(12.5))
        );
        Ok(())
    }

    #[test]
    fn test_gill_uk() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::ImperialGill(0.02)).scale(0.5)?,
            Unit::Volume(Volume::ImperialFluidOunce(0.05))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialGill(0.8)).scale(2.0)?,
            Unit::Volume(Volume::ImperialGill(1.6)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialGill(1.0)).scale(4.5)?,
            Unit::Volume(Volume::ImperialPint(1.125)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialGill(5.0)).scale(2.0)?,
            Unit::Volume(Volume::ImperialQuart(1.25))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialGill(8.0)).scale(5.0)?,
            Unit::Volume(Volume::ImperialGallon(1.25))
        );
        Ok(())
    }

    #[test]
    fn test_pint_uk() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::ImperialPint(0.05)).scale(0.5)?,
            Unit::Volume(Volume::ImperialFluidOunce(0.5))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialPint(1.0)).scale(0.5)?,
            Unit::Volume(Volume::ImperialGill(2.0)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialPint(1.0)).scale(1.2)?,
            Unit::Volume(Volume::ImperialPint(1.2)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialPint(1.0)).scale(2.0)?,
            Unit::Volume(Volume::ImperialQuart(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialPint(4.0)).scale(3.0)?,
            Unit::Volume(Volume::ImperialGallon(1.5))
        );
        Ok(())
    }

    #[test]
    fn test_quart_uk() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::ImperialQuart(0.02)).scale(0.5)?,
            Unit::Volume(Volume::ImperialFluidOunce(0.4))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialQuart(0.1)).scale(1.45)?,
            Unit::Volume(Volume::ImperialGill(1.16)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialQuart(0.5)).scale(1.5)?,
            Unit::Volume(Volume::ImperialPint(1.5)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialQuart(0.8)).scale(2.0)?,
            Unit::Volume(Volume::ImperialQuart(1.6))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialQuart(4.0)).scale(3.0)?,
            Unit::Volume(Volume::ImperialGallon(3.0))
        );
        Ok(())
    }

    #[test]
    fn test_gallon_uk() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::ImperialGallon(0.02)).scale(0.5)?,
            Unit::Volume(Volume::ImperialFluidOunce(1.6))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialGallon(1.0)).scale(0.1)?,
            Unit::Volume(Volume::ImperialGill(3.2)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialGallon(0.75)).scale(0.25)?,
            Unit::Volume(Volume::ImperialPint(1.5)),
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialGallon(0.75)).scale(0.5)?,
            Unit::Volume(Volume::ImperialQuart(1.5))
        );
        assert_eq!(
            Unit::Volume(Volume::ImperialGallon(4.0)).scale(3.0)?,
            Unit::Volume(Volume::ImperialGallon(12.0))
        );
        Ok(())
    }

    #[test]
    fn test_jigger() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::Jigger(4.0)).scale(3.0)?,
            Unit::Volume(Volume::Jigger(12.0))
        );
        Ok(())
    }

    #[test]
    fn test_tsp_aus() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.0)).scale(1.5)?,
            Unit::Volume(Volume::AustralianTeaspoon(1.5))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.0)).scale(2.0)?,
            Unit::Volume(Volume::AustralianDessertspoon(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.5)).scale(3.0)?,
            Unit::Volume(Volume::AustralianTablespoon(1.125))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(10.0)).scale(5.0)?,
            Unit::Volume(Volume::AustralianCup(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_dsp_aus() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.0)).scale(1.5)?,
            Unit::Volume(Volume::AustralianTeaspoon(1.5))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.0)).scale(2.0)?,
            Unit::Volume(Volume::AustralianDessertspoon(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.5)).scale(3.0)?,
            Unit::Volume(Volume::AustralianTablespoon(1.125))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(10.0)).scale(5.0)?,
            Unit::Volume(Volume::AustralianCup(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_tbsp_aus() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.0)).scale(1.5)?,
            Unit::Volume(Volume::AustralianTeaspoon(1.5))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.0)).scale(2.0)?,
            Unit::Volume(Volume::AustralianDessertspoon(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.5)).scale(3.0)?,
            Unit::Volume(Volume::AustralianTablespoon(1.125))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(10.0)).scale(5.0)?,
            Unit::Volume(Volume::AustralianCup(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_cup_aus() -> Result<()> {
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.0)).scale(1.5)?,
            Unit::Volume(Volume::AustralianTeaspoon(1.5))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.0)).scale(2.0)?,
            Unit::Volume(Volume::AustralianDessertspoon(1.0))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(1.5)).scale(3.0)?,
            Unit::Volume(Volume::AustralianTablespoon(1.125))
        );
        assert_eq!(
            Unit::Volume(Volume::AustralianTeaspoon(10.0)).scale(5.0)?,
            Unit::Volume(Volume::AustralianCup(1.0))
        );
        Ok(())
    }
}
