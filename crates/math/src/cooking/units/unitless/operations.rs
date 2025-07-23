use super::Unitless;
use crate::cooking::units::UnitType;
use crate::cooking::units::traits::UnitOperations;

impl UnitOperations for Unitless {
    fn abbrev(&self) -> &str {
        ""
    }

    fn unit_type(&self) -> UnitType {
        UnitType::Unitless
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn with_value(&self, value: f64) -> Self {
        Unitless::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abbrev() {
        let unit = Unitless::new(4.0);

        let got = unit.abbrev();

        assert_eq!(got, "");
    }

    #[test]
    fn test_unit_type() {
        let unit = Unitless::new(4.0);

        let got = unit.unit_type();

        assert_eq!(got, UnitType::Unitless);
    }

    #[test]
    fn test_value() {
        let unit = Unitless::new(4.0);

        let got = unit.value();

        assert_eq!(got, 4.0);
    }

    #[test]
    fn test_with_value() {
        let unit = Unitless::new(4.0);

        let got = unit.with_value(8.0);

        assert_eq!(got, Unitless { value: 8.0 });
    }
}
