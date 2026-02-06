use measurements::length::{
    METER_CENTIMETER_FACTOR, METER_FEET_FACTOR, METER_INCH_FACTOR, METER_KILOMETER_FACTOR,
    METER_MILLIMETER_FACTOR,
};

use crate::Result;
use crate::cooking::units::length::units::Length;
use crate::cooking::units::traits::{UnitOperations, UnitScaler};
use crate::cooking::units::unit::Unit;

impl UnitScaler for Length {
    fn scale(&self, factor: f64) -> Result<Unit> {
        let scaled_value = self.value() * factor;

        match self {
            Self::Millimetre(_) | Self::Centimetre(_) | Self::Metre(_) | Self::Kilometre(_) => {
                let metres_base = match self {
                    Self::Millimetre(_) => scaled_value / METER_MILLIMETER_FACTOR,
                    Self::Centimetre(_) => scaled_value / METER_CENTIMETER_FACTOR,
                    Self::Metre(_) => scaled_value,
                    Self::Kilometre(_) => scaled_value / METER_KILOMETER_FACTOR,
                    Self::Inch(_) => scaled_value / METER_INCH_FACTOR,
                    Self::Foot(_) => scaled_value / METER_FEET_FACTOR,
                };

                if metres_base >= 1e3 {
                    Ok(Unit::Length(Self::Kilometre(metres_base * 1e-3)))
                } else if metres_base >= 1.0 {
                    Ok(Unit::Length(Self::Metre(metres_base)))
                } else if metres_base >= 1e-2 {
                    Ok(Unit::Length(Self::Centimetre(metres_base * 1e2)))
                } else {
                    Ok(Unit::Length(Self::Millimetre(metres_base * 1e3)))
                }
            }
            Self::Inch(_) | Self::Foot(_) => {
                let inches_base = match self {
                    Self::Inch(_) => scaled_value,
                    Self::Foot(_) => scaled_value * 12.0,
                    _ => unreachable!(),
                };

                if inches_base >= 12.0 {
                    Ok(Unit::Length(Self::Foot(inches_base / 12.0)))
                } else {
                    Ok(Unit::Length(Self::Inch(inches_base)))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // Millimetre
    #[test]
    fn test_mm_to_km() -> Result<()> {
        assert_eq!(
            Length::Millimetre(100.0).scale(1e4)?,
            Unit::Length(Length::Kilometre(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_mm_to_m() -> Result<()> {
        assert_eq!(
            Length::Millimetre(15.0).scale(70.0)?,
            Unit::Length(Length::Metre(1.05))
        );
        Ok(())
    }

    #[test]
    fn test_mm_to_cm() -> Result<()> {
        assert_eq!(
            Length::Millimetre(15.0).scale(10.0)?,
            Unit::Length(Length::Centimetre(15.0))
        );
        Ok(())
    }

    #[test]
    fn test_mm_to_mm() -> Result<()> {
        assert_eq!(
            Length::Millimetre(3.0).scale(2.0)?,
            Unit::Length(Length::Millimetre(6.0))
        );
        Ok(())
    }

    // Centimetre
    #[test]
    fn test_cm_to_km() -> Result<()> {
        assert_eq!(
            Length::Centimetre(100.0).scale(1e4)?,
            Unit::Length(Length::Kilometre(10.0))
        );
        Ok(())
    }

    #[test]
    fn test_cm_to_m() -> Result<()> {
        assert_eq!(
            Length::Centimetre(15.0).scale(60.0)?,
            Unit::Length(Length::Metre(9.00))
        );
        Ok(())
    }

    #[test]
    fn test_cm_to_cm() -> Result<()> {
        assert_eq!(
            Length::Centimetre(15.0).scale(2.0)?,
            Unit::Length(Length::Centimetre(30.0))
        );
        Ok(())
    }

    #[test]
    fn test_cm_to_mm() -> Result<()> {
        assert_approx_eq(
            Length::Centimetre(3.0).scale(0.2)?,
            Unit::Length(Length::Millimetre(6.0)),
            1e-10,
        );
        Ok(())
    }

    // Metre
    #[test]
    fn test_m_to_km() -> Result<()> {
        assert_eq!(
            Length::Metre(100.0).scale(10.0)?,
            Unit::Length(Length::Kilometre(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_m_to_m() -> Result<()> {
        assert_eq!(
            Length::Metre(15.0).scale(60.0)?,
            Unit::Length(Length::Metre(900.0))
        );
        Ok(())
    }

    #[test]
    fn test_m_to_cm() -> Result<()> {
        assert_eq!(
            Length::Metre(1.0).scale(0.5)?,
            Unit::Length(Length::Centimetre(50.0))
        );
        Ok(())
    }

    #[test]
    fn test_m_to_mm() -> Result<()> {
        assert_approx_eq(
            Length::Metre(1.0).scale(0.002)?,
            Unit::Length(Length::Millimetre(2.0)),
            1e-10,
        );
        Ok(())
    }

    // Kilometre
    #[test]
    fn test_km_to_km() -> Result<()> {
        assert_eq!(
            Length::Kilometre(1.0).scale(10.0)?,
            Unit::Length(Length::Kilometre(10.0))
        );
        Ok(())
    }

    #[test]
    fn test_km_to_m() -> Result<()> {
        assert_eq!(
            Length::Kilometre(1.0).scale(0.5)?,
            Unit::Length(Length::Metre(500.0))
        );
        Ok(())
    }

    #[test]
    fn test_km_to_cm() -> Result<()> {
        assert_eq!(
            Length::Kilometre(1.0).scale(1e-5)?,
            Unit::Length(Length::Centimetre(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_km_to_mm() -> Result<()> {
        assert_approx_eq(
            Length::Kilometre(1.0).scale(0.000005)?,
            Unit::Length(Length::Millimetre(5.0)),
            1e-10,
        );
        Ok(())
    }

    // Inch
    #[test]
    fn test_in_to_in() -> Result<()> {
        assert_eq!(
            Length::Inch(2.0).scale(4.0)?,
            Unit::Length(Length::Inch(8.0))
        );
        Ok(())
    }

    #[test]
    fn test_in_to_ft() -> Result<()> {
        assert_eq!(
            Length::Inch(3.0).scale(4.0)?,
            Unit::Length(Length::Foot(1.0))
        );
        Ok(())
    }

    // Feet
    #[test]
    fn test_ft_to_in() -> Result<()> {
        assert_eq!(
            Length::Foot(1.0).scale(0.5)?,
            Unit::Length(Length::Inch(6.0))
        );
        Ok(())
    }

    #[test]
    fn test_ft_to_ft() -> Result<()> {
        assert_eq!(
            Length::Foot(1.0).scale(5.0)?,
            Unit::Length(Length::Foot(5.0))
        );
        Ok(())
    }
}
