use crate::cooking::units::UnitType;
use crate::cooking::units::custom::{MassDekagramExt, MassHectogramExt};
use crate::cooking::units::mass::units::{Mass, MassUnit};
use crate::cooking::units::traits::{UnitConverter, UnitOperations};
use crate::cooking::units::unit::Unit;
use crate::{Error, Result};

impl UnitConverter for Mass {
    #[allow(clippy::too_many_lines)]
    fn convert(&self, to: UnitType) -> Result<Unit> {
        use MassUnit::{Dekagram, Gram, Hectogram, Kilogram, Milligram, Ounce, Pound};

        match self {
            Self::Milligram(original_value) => match to {
                UnitType::Mass(unit) => {
                    let value = measurements::Mass::from_milligrams(*original_value);

                    match unit {
                        Milligram => Ok(Unit::Mass(self.with_value(*original_value))),
                        Gram => Ok(Unit::Mass(Self::Gram(value.as_grams()))),
                        Dekagram => Ok(Unit::Mass(Self::Dekagram(value.as_dekagrams()))),
                        Hectogram => Ok(Unit::Mass(Self::Hectogram(value.as_hectograms()))),
                        Kilogram => Ok(Unit::Mass(Self::Kilogram(value.as_kilograms()))),
                        Ounce => Ok(Unit::Mass(Self::Ounce(value.as_ounces()))),
                        Pound => Ok(Unit::Mass(Self::Pound(value.as_pounds()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Mass(self.clone()), to)),
            },
            Self::Gram(original_value) => match to {
                UnitType::Mass(unit) => {
                    let value = measurements::Mass::from_grams(*original_value);

                    match unit {
                        Milligram => Ok(Unit::Mass(Self::Milligram(value.as_milligrams()))),
                        Gram => Ok(Unit::Mass(self.with_value(*original_value))),
                        Dekagram => Ok(Unit::Mass(Self::Dekagram(value.as_dekagrams()))),
                        Hectogram => Ok(Unit::Mass(Self::Hectogram(value.as_hectograms()))),
                        Kilogram => Ok(Unit::Mass(Self::Kilogram(value.as_kilograms()))),
                        Ounce => Ok(Unit::Mass(Self::Ounce(value.as_ounces()))),
                        Pound => Ok(Unit::Mass(Self::Pound(value.as_pounds()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Mass(self.clone()), to)),
            },
            Self::Dekagram(original_value) => match to {
                UnitType::Mass(unit) => {
                    let value = measurements::Mass::from_dekagrams(*original_value);

                    match unit {
                        Milligram => Ok(Unit::Mass(Self::Milligram(value.as_milligrams()))),
                        Gram => Ok(Unit::Mass(Self::Gram(value.as_grams()))),
                        Dekagram => Ok(Unit::Mass(self.with_value(*original_value))),
                        Hectogram => Ok(Unit::Mass(Self::Hectogram(value.as_hectograms()))),
                        Kilogram => Ok(Unit::Mass(Self::Kilogram(value.as_kilograms()))),
                        Ounce => Ok(Unit::Mass(Self::Ounce(value.as_ounces()))),
                        Pound => Ok(Unit::Mass(Self::Pound(value.as_pounds()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Mass(self.clone()), to)),
            },
            Self::Hectogram(original_value) => match to {
                UnitType::Mass(unit) => {
                    let value = measurements::Mass::from_hectograms(*original_value);

                    match unit {
                        Milligram => Ok(Unit::Mass(Self::Milligram(value.as_milligrams()))),
                        Gram => Ok(Unit::Mass(Self::Gram(value.as_grams()))),
                        Dekagram => Ok(Unit::Mass(Self::Dekagram(value.as_dekagrams()))),
                        Hectogram => Ok(Unit::Mass(self.with_value(*original_value))),
                        Kilogram => Ok(Unit::Mass(Self::Kilogram(value.as_kilograms()))),
                        Ounce => Ok(Unit::Mass(Self::Ounce(value.as_ounces()))),
                        Pound => Ok(Unit::Mass(Self::Pound(value.as_pounds()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Mass(self.clone()), to)),
            },
            Self::Kilogram(original_value) => match to {
                UnitType::Mass(unit) => {
                    let value = measurements::Mass::from_kilograms(*original_value);

                    match unit {
                        Milligram => Ok(Unit::Mass(Self::Milligram(value.as_milligrams()))),
                        Gram => Ok(Unit::Mass(Self::Gram(value.as_grams()))),
                        Dekagram => Ok(Unit::Mass(Self::Dekagram(value.as_dekagrams()))),
                        Hectogram => Ok(Unit::Mass(Self::Hectogram(value.as_hectograms()))),
                        Kilogram => Ok(Unit::Mass(self.with_value(*original_value))),
                        Ounce => Ok(Unit::Mass(Self::Ounce(value.as_ounces()))),
                        Pound => Ok(Unit::Mass(Self::Pound(value.as_pounds()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Mass(self.clone()), to)),
            },
            Self::Ounce(original_value) => match to {
                UnitType::Mass(unit) => {
                    let value = measurements::Mass::from_ounces(*original_value);

                    match unit {
                        Milligram => Ok(Unit::Mass(Self::Milligram(value.as_milligrams()))),
                        Gram => Ok(Unit::Mass(Self::Gram(value.as_grams()))),
                        Dekagram => Ok(Unit::Mass(Self::Dekagram(value.as_dekagrams()))),
                        Hectogram => Ok(Unit::Mass(Self::Hectogram(value.as_hectograms()))),
                        Kilogram => Ok(Unit::Mass(Self::Kilogram(value.as_kilograms()))),
                        Ounce => Ok(Unit::Mass(self.with_value(*original_value))),
                        Pound => Ok(Unit::Mass(Self::Pound(value.as_pounds()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Mass(self.clone()), to)),
            },
            Self::Pound(original_value) => match to {
                UnitType::Mass(unit) => {
                    let value = measurements::Mass::from_pounds(*original_value);

                    match unit {
                        Milligram => Ok(Unit::Mass(Self::Milligram(value.as_milligrams()))),
                        Gram => Ok(Unit::Mass(Self::Gram(value.as_grams()))),
                        Dekagram => Ok(Unit::Mass(Self::Dekagram(value.as_dekagrams()))),
                        Hectogram => Ok(Unit::Mass(Self::Hectogram(value.as_hectograms()))),
                        Kilogram => Ok(Unit::Mass(Self::Kilogram(value.as_kilograms()))),
                        Ounce => Ok(Unit::Mass(Self::Ounce(value.as_ounces()))),
                        Pound => Ok(Unit::Mass(self.with_value(*original_value))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Mass(self.clone()), to)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cooking::units::mass::units::MassUnit::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn assert_approx_eq(actual: Unit, expected: Unit, threshold: f64) {
        let actual = actual.value();
        let expected = expected.value();

        assert!(
            (actual - expected).abs() < threshold,
            "Expected {actual}, got {expected}, difference: {}",
            (actual - expected).abs()
        );
    }

    #[test]
    fn test_milligram_conversions() -> Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Milligram(1000.0)).convert(UnitType::Mass(Milligram))?,
            Unit::Mass(Mass::Milligram(1000.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Milligram(1000.0)).convert(UnitType::Mass(Gram))?,
            Unit::Mass(Mass::Gram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Milligram(10000.0)).convert(UnitType::Mass(Dekagram))?,
            Unit::Mass(Mass::Dekagram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Milligram(100000.0)).convert(UnitType::Mass(Hectogram))?,
            Unit::Mass(Mass::Hectogram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Milligram(1000000.0)).convert(UnitType::Mass(Kilogram))?,
            Unit::Mass(Mass::Kilogram(1.0)),
        );
        assert_approx_eq(
            Unit::Mass(Mass::Milligram(28349.5)).convert(UnitType::Mass(Ounce))?,
            Unit::Mass(Mass::Ounce(1.0)),
            0.1,
        );
        assert_approx_eq(
            Unit::Mass(Mass::Milligram(453592.0)).convert(UnitType::Mass(Pound))?,
            Unit::Mass(Mass::Pound(1.0)),
            0.1,
        );
        Ok(())
    }

    #[test]
    fn test_gram_conversions() -> Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Gram(1.0)).convert(UnitType::Mass(Milligram))?,
            Unit::Mass(Mass::Milligram(1000.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Gram(1.0)).convert(UnitType::Mass(Gram))?,
            Unit::Mass(Mass::Gram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Gram(10.0)).convert(UnitType::Mass(Dekagram))?,
            Unit::Mass(Mass::Dekagram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Gram(100.0)).convert(UnitType::Mass(Hectogram))?,
            Unit::Mass(Mass::Hectogram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Gram(1000.0)).convert(UnitType::Mass(Kilogram))?,
            Unit::Mass(Mass::Kilogram(1.0)),
        );
        assert_approx_eq(
            Unit::Mass(Mass::Gram(28.3495)).convert(UnitType::Mass(Ounce))?,
            Unit::Mass(Mass::Ounce(1.0)),
            0.01,
        );
        assert_approx_eq(
            Unit::Mass(Mass::Gram(453.592)).convert(UnitType::Mass(Pound))?,
            Unit::Mass(Mass::Pound(1.0)),
            0.01,
        );
        Ok(())
    }

    #[test]
    fn test_dekagram_conversions() -> Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Dekagram(1.0)).convert(UnitType::Mass(Milligram))?,
            Unit::Mass(Mass::Milligram(10000.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Dekagram(1.0)).convert(UnitType::Mass(Gram))?,
            Unit::Mass(Mass::Gram(10.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Dekagram(1.0)).convert(UnitType::Mass(Dekagram))?,
            Unit::Mass(Mass::Dekagram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Dekagram(10.0)).convert(UnitType::Mass(Hectogram))?,
            Unit::Mass(Mass::Hectogram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Dekagram(100.0)).convert(UnitType::Mass(Kilogram))?,
            Unit::Mass(Mass::Kilogram(1.0)),
        );

        assert_approx_eq(
            Unit::Mass(Mass::Dekagram(2.83495)).convert(UnitType::Mass(Ounce))?,
            Unit::Mass(Mass::Ounce(1.0)),
            0.01,
        );
        assert_approx_eq(
            Unit::Mass(Mass::Dekagram(45.3592)).convert(UnitType::Mass(Pound))?,
            Unit::Mass(Mass::Pound(1.0)),
            0.01,
        );
        Ok(())
    }

    #[test]
    fn test_hectogram_conversions() -> Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Hectogram(0.01)).convert(UnitType::Mass(Milligram))?,
            Unit::Mass(Mass::Milligram(1000.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Hectogram(0.01)).convert(UnitType::Mass(Gram))?,
            Unit::Mass(Mass::Gram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Hectogram(1.0)).convert(UnitType::Mass(Dekagram))?,
            Unit::Mass(Mass::Dekagram(10.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Hectogram(1.0)).convert(UnitType::Mass(Hectogram))?,
            Unit::Mass(Mass::Hectogram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Hectogram(10.0)).convert(UnitType::Mass(Kilogram))?,
            Unit::Mass(Mass::Kilogram(1.0)),
        );

        assert_approx_eq(
            Unit::Mass(Mass::Hectogram(0.283495)).convert(UnitType::Mass(Ounce))?,
            Unit::Mass(Mass::Ounce(1.0)),
            0.01,
        );
        assert_approx_eq(
            Unit::Mass(Mass::Hectogram(4.53592)).convert(UnitType::Mass(Pound))?,
            Unit::Mass(Mass::Pound(1.0)),
            0.01,
        );
        Ok(())
    }

    #[test]
    fn test_kilogram_conversions() -> Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Kilogram(0.001)).convert(UnitType::Mass(Milligram))?,
            Unit::Mass(Mass::Milligram(1000.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Kilogram(0.001)).convert(UnitType::Mass(Gram))?,
            Unit::Mass(Mass::Gram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Kilogram(0.01)).convert(UnitType::Mass(Dekagram))?,
            Unit::Mass(Mass::Dekagram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Kilogram(0.1)).convert(UnitType::Mass(Hectogram))?,
            Unit::Mass(Mass::Hectogram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Kilogram(1.0)).convert(UnitType::Mass(Kilogram))?,
            Unit::Mass(Mass::Kilogram(1.0)),
        );

        assert_approx_eq(
            Unit::Mass(Mass::Kilogram(0.0283495)).convert(UnitType::Mass(Ounce))?,
            Unit::Mass(Mass::Ounce(1.0)),
            0.001,
        );
        assert_approx_eq(
            Unit::Mass(Mass::Kilogram(0.453592)).convert(UnitType::Mass(Pound))?,
            Unit::Mass(Mass::Pound(1.0)),
            0.001,
        );
        Ok(())
    }

    #[test]
    fn test_ounce_conversions() -> Result<()> {
        assert_approx_eq(
            Unit::Mass(Mass::Ounce(1.0)).convert(UnitType::Mass(Milligram))?,
            Unit::Mass(Mass::Milligram(28349.5)),
            1.0,
        );
        assert_approx_eq(
            Unit::Mass(Mass::Ounce(1.0)).convert(UnitType::Mass(Gram))?,
            Unit::Mass(Mass::Gram(28.3495)),
            0.001,
        );
        assert_approx_eq(
            Unit::Mass(Mass::Ounce(1.0)).convert(UnitType::Mass(Dekagram))?,
            Unit::Mass(Mass::Dekagram(2.83495)),
            0.0001,
        );
        assert_eq!(
            Unit::Mass(Mass::Ounce(1.0)).convert(UnitType::Mass(Hectogram))?,
            Unit::Mass(Mass::Hectogram(0.28349523125)),
        );
        assert_approx_eq(
            Unit::Mass(Mass::Ounce(1.0)).convert(UnitType::Mass(Kilogram))?,
            Unit::Mass(Mass::Kilogram(0.0283495)),
            0.000001,
        );
        assert_eq!(
            Unit::Mass(Mass::Ounce(1.0)).convert(UnitType::Mass(Ounce))?,
            Unit::Mass(Mass::Ounce(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Ounce(16.0)).convert(UnitType::Mass(Pound))?,
            Unit::Mass(Mass::Pound(1.0)),
        );
        Ok(())
    }

    #[test]
    fn test_pound_conversions() -> Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Pound(1.0)).convert(UnitType::Mass(Milligram))?,
            Unit::Mass(Mass::Milligram(453592.37)),
        );
        assert_eq!(
            Unit::Mass(Mass::Pound(1.0)).convert(UnitType::Mass(Gram))?,
            Unit::Mass(Mass::Gram(453.59237)),
        );
        assert_eq!(
            Unit::Mass(Mass::Pound(1.0)).convert(UnitType::Mass(Dekagram))?,
            Unit::Mass(Mass::Dekagram(45.359237)),
        );
        assert_eq!(
            Unit::Mass(Mass::Pound(1.0)).convert(UnitType::Mass(Hectogram))?,
            Unit::Mass(Mass::Hectogram(4.5359237)),
        );
        assert_eq!(
            Unit::Mass(Mass::Pound(1.0)).convert(UnitType::Mass(Kilogram))?,
            Unit::Mass(Mass::Kilogram(0.45359237)),
        );

        assert_eq!(
            Unit::Mass(Mass::Pound(1.0)).convert(UnitType::Mass(Ounce))?,
            Unit::Mass(Mass::Ounce(16.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Pound(1.0)).convert(UnitType::Mass(Pound))?,
            Unit::Mass(Mass::Pound(1.0)),
        );
        Ok(())
    }

    #[test]
    fn test_edge_cases() -> Result<()> {
        assert_eq!(
            Unit::Mass(Mass::Gram(0.0)).convert(UnitType::Mass(Kilogram))?,
            Unit::Mass(Mass::Kilogram(0.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Pound(0.0)).convert(UnitType::Mass(Ounce))?,
            Unit::Mass(Mass::Ounce(0.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Gram(0.001)).convert(UnitType::Mass(Milligram))?,
            Unit::Mass(Mass::Milligram(1.0)),
        );
        assert_eq!(
            Unit::Mass(Mass::Kilogram(1000.0)).convert(UnitType::Mass(Gram))?,
            Unit::Mass(Mass::Gram(1000000.0)),
        );
        Ok(())
    }

    #[test]
    fn test_precision() -> Result<()> {
        let original = 123.456;
        let converted = Unit::Mass(Mass::Gram(original)).convert(UnitType::Mass(Kilogram))?;

        let back_converted = converted.convert(UnitType::Mass(Gram))?;

        assert_approx_eq(back_converted, Unit::Mass(Mass::Gram(original)), 0.001);
        Ok(())
    }

    #[test]
    fn test_identity_conversions() -> Result<()> {
        let test_cases = vec![
            (Unit::Mass(Mass::Gram(42.0)), UnitType::Mass(Gram)),
            (Unit::Mass(Mass::Kilogram(2.12)), UnitType::Mass(Kilogram)),
            (Unit::Mass(Mass::Pound(2.71)), UnitType::Mass(Pound)),
            (Unit::Mass(Mass::Ounce(1.41)), UnitType::Mass(Ounce)),
            (
                Unit::Mass(Mass::Milligram(999.0)),
                UnitType::Mass(Milligram),
            ),
            (Unit::Mass(Mass::Dekagram(66.0)), UnitType::Mass(Dekagram)),
            (Unit::Mass(Mass::Hectogram(55.0)), UnitType::Mass(Hectogram)),
        ];

        for (unit, unit_type) in test_cases {
            let expected = match unit {
                Unit::Mass(Mass::Gram(v)) => Unit::Mass(Mass::Gram(v)),
                Unit::Mass(Mass::Kilogram(v)) => Unit::Mass(Mass::Kilogram(v)),
                Unit::Mass(Mass::Pound(v)) => Unit::Mass(Mass::Pound(v)),
                Unit::Mass(Mass::Ounce(v)) => Unit::Mass(Mass::Ounce(v)),
                Unit::Mass(Mass::Milligram(v)) => Unit::Mass(Mass::Milligram(v)),
                Unit::Mass(Mass::Dekagram(v)) => Unit::Mass(Mass::Dekagram(v)),
                Unit::Mass(Mass::Hectogram(v)) => Unit::Mass(Mass::Hectogram(v)),
                _ => Unit::Mass(Mass::Gram(0.0)),
            };
            assert_eq!(unit, expected.convert(unit_type)?);
        }
        Ok(())
    }
}
