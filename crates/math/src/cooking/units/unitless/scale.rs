use crate::Result;
use crate::cooking::units::traits::UnitScaler;
use crate::cooking::units::unit::Unit;
use crate::cooking::units::unitless::units::Unitless;

impl UnitScaler for Unitless {
    fn scale(&self, factor: f64) -> Result<Unit> {
        let scaled_value = self.value * factor;

        Ok(Unit::Unitless(Unitless::new(scaled_value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_unitless() -> Result<()> {
        assert_eq!(
            Unit::Unitless(Unitless::new(1.0)).scale(2.0)?,
            Unit::Unitless(Unitless::new(2.0))
        );
        Ok(())
    }
}
