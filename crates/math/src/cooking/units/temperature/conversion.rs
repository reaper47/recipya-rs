use crate::cooking::units::UnitType;
use crate::cooking::units::temperature::units::{Temperature, TemperatureUnit};
use crate::cooking::units::traits::{UnitConverter, UnitOperations};
use crate::cooking::units::unit::Unit;
use crate::{Error, Result};

impl UnitConverter for Temperature {
    fn convert(&self, to: UnitType) -> Result<Unit> {
        use TemperatureUnit::*;

        match self {
            Temperature::Celsius(original_value) => match to {
                UnitType::Temperature(unit) => {
                    let value = measurements::Temperature::from_celsius(*original_value);

                    match unit {
                        Celsius => Ok(Unit::Temperature(self.with_value(*original_value))),
                        Fahrenheit => Ok(Unit::Temperature(Temperature::Fahrenheit(
                            value.as_fahrenheit(),
                        ))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Temperature(self.clone()), to)),
            },
            Temperature::Fahrenheit(original_value) => match to {
                UnitType::Temperature(unit) => {
                    let value = measurements::Temperature::from_fahrenheit(*original_value);

                    match unit {
                        Celsius => Ok(Unit::Temperature(Temperature::Celsius(value.as_celsius()))),
                        Fahrenheit => Ok(Unit::Temperature(self.with_value(*original_value))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Temperature(self.clone()), to)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use TemperatureUnit::*;

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
    fn test_celsius_conversions() -> Result<()> {
        assert_eq!(
            Unit::Temperature(Temperature::Celsius(100.0))
                .convert(UnitType::Temperature(Celsius))?,
            Unit::Temperature(Temperature::Celsius(100.0)),
        );
        assert_eq!(
            Unit::Temperature(Temperature::Celsius(100.0))
                .convert(UnitType::Temperature(Fahrenheit))?,
            Unit::Temperature(Temperature::Fahrenheit(212.0)),
        );
        Ok(())
    }

    #[test]
    fn test_fahrenheit_conversions() -> Result<()> {
        assert_approx_eq(
            Unit::Temperature(Temperature::Fahrenheit(100.0))
                .convert(UnitType::Temperature(Celsius))?,
            Unit::Temperature(Temperature::Celsius(37.7777778)),
            1e-5,
        );
        assert_eq!(
            Unit::Temperature(Temperature::Fahrenheit(100.0))
                .convert(UnitType::Temperature(Fahrenheit))?,
            Unit::Temperature(Temperature::Fahrenheit(100.0)),
        );
        Ok(())
    }
}
