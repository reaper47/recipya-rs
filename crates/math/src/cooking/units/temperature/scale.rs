use super::Temperature;
use crate::Result;
use crate::cooking::units::Unit;
use crate::cooking::units::traits::{UnitOperations, UnitScaler};

impl UnitScaler for Temperature {
    fn scale(&self, factor: f64) -> Result<Unit> {
        let scaled_value = self.value() * factor;

        Ok(Unit::Temperature(self.with_value(scaled_value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_celsius() -> crate::Result<()> {
        assert_eq!(
            Unit::Temperature(Temperature::Celsius(100.0)).scale(2.0)?,
            Unit::Temperature(Temperature::Celsius(200.0))
        );
        Ok(())
    }

    #[test]
    fn test_fahrenheit() -> crate::Result<()> {
        assert_eq!(
            Unit::Temperature(Temperature::Fahrenheit(100.0)).scale(2.0)?,
            Unit::Temperature(Temperature::Fahrenheit(200.0))
        );
        Ok(())
    }
}
