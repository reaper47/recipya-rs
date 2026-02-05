use crate::cooking::units::UnitType;
use crate::cooking::units::length::units::{Length, LengthUnit};
use crate::cooking::units::traits::{UnitConverter, UnitOperations};
use crate::cooking::units::unit::Unit;
use crate::{Error, Result};

impl UnitConverter for Length {
    fn convert(&self, to: UnitType) -> Result<Unit> {
        use LengthUnit::{Centimetre, Foot, Inch, Kilometre, Metre, Millimetre};

        match self {
            Self::Millimetre(original_value) => match to {
                UnitType::Length(unit) => {
                    let value = measurements::Length::from_millimetres(*original_value);

                    match unit {
                        Millimetre => Ok(Unit::Length(self.with_value(*original_value))),
                        Centimetre => Ok(Unit::Length(Self::Centimetre(value.as_centimetres()))),
                        Metre => Ok(Unit::Length(Self::Metre(value.as_metres()))),
                        Kilometre => Ok(Unit::Length(Self::Kilometre(value.as_kilometres()))),
                        Inch => Ok(Unit::Length(Self::Inch(value.as_inches()))),
                        Foot => Ok(Unit::Length(Self::Foot(value.as_feet()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
            },
            Self::Centimetre(original_value) => match to {
                UnitType::Length(unit) => {
                    let value = measurements::Length::from_centimetres(*original_value);

                    match unit {
                        Millimetre => Ok(Unit::Length(Self::Millimetre(value.as_millimetres()))),
                        Centimetre => Ok(Unit::Length(self.with_value(*original_value))),
                        Metre => Ok(Unit::Length(Self::Metre(value.as_metres()))),
                        Kilometre => Ok(Unit::Length(Self::Kilometre(value.as_kilometres()))),
                        Inch => Ok(Unit::Length(Self::Inch(value.as_inches()))),
                        Foot => Ok(Unit::Length(Self::Foot(value.as_feet()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
            },
            Self::Metre(original_value) => {
                let value = measurements::Length::from_metres(*original_value);

                match to {
                    UnitType::Length(unit) => match unit {
                        Millimetre => Ok(Unit::Length(Self::Millimetre(value.as_millimetres()))),
                        Centimetre => Ok(Unit::Length(Self::Centimetre(value.as_centimetres()))),
                        Metre => Ok(Unit::Length(self.with_value(*original_value))),
                        Kilometre => Ok(Unit::Length(Self::Kilometre(value.as_kilometres()))),
                        Inch => Ok(Unit::Length(Self::Inch(value.as_inches()))),
                        Foot => Ok(Unit::Length(Self::Foot(value.as_feet()))),
                    },
                    _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
                }
            }
            Self::Kilometre(original_value) => match to {
                UnitType::Length(unit) => {
                    let value = measurements::Length::from_kilometres(*original_value);

                    match unit {
                        Millimetre => Ok(Unit::Length(Self::Millimetre(value.as_millimetres()))),
                        Centimetre => Ok(Unit::Length(Self::Centimetre(value.as_centimetres()))),
                        Metre => Ok(Unit::Length(Self::Metre(value.as_metres()))),
                        Kilometre => Ok(Unit::Length(self.with_value(*original_value))),
                        Inch => Ok(Unit::Length(Self::Inch(value.as_inches()))),
                        Foot => Ok(Unit::Length(Self::Foot(value.as_feet()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
            },
            Self::Inch(original_value) => match to {
                UnitType::Length(unit) => {
                    let value = measurements::Length::from_inches(*original_value);

                    match unit {
                        Millimetre => Ok(Unit::Length(Self::Millimetre(value.as_millimetres()))),
                        Centimetre => Ok(Unit::Length(Self::Centimetre(value.as_centimetres()))),
                        Metre => Ok(Unit::Length(Self::Metre(value.as_metres()))),
                        Kilometre => Ok(Unit::Length(Self::Kilometre(value.as_kilometres()))),
                        Inch => Ok(Unit::Length(self.with_value(*original_value))),
                        Foot => Ok(Unit::Length(Self::Foot(value.as_feet()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
            },
            Self::Foot(original_value) => match to {
                UnitType::Length(unit) => {
                    let value = measurements::Length::from_feet(*original_value);

                    match unit {
                        Millimetre => Ok(Unit::Length(Self::Millimetre(value.as_millimetres()))),
                        Centimetre => Ok(Unit::Length(Self::Centimetre(value.as_centimetres()))),
                        Metre => Ok(Unit::Length(Self::Metre(value.as_metres()))),
                        Kilometre => Ok(Unit::Length(Self::Kilometre(value.as_kilometres()))),
                        Inch => Ok(Unit::Length(Self::Inch(value.as_inches()))),
                        Foot => Ok(Unit::Length(self.with_value(*original_value))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
            },
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

    #[test]
    fn test_millimeter_conversions() -> Result<()> {
        assert_eq!(
            Length::Millimetre(100.0).convert(UnitType::Length(LengthUnit::Millimetre))?,
            Unit::Length(Length::Millimetre(100.0))
        );
        assert_eq!(
            Length::Millimetre(100.0).convert(UnitType::Length(LengthUnit::Centimetre))?,
            Unit::Length(Length::Centimetre(10.0)),
        );
        assert_eq!(
            Length::Millimetre(1000.0).convert(UnitType::Length(LengthUnit::Metre))?,
            Unit::Length(Length::Metre(1.0)),
        );
        assert_eq!(
            Length::Millimetre(1e6).convert(UnitType::Length(LengthUnit::Kilometre))?,
            Unit::Length(Length::Kilometre(1.0)),
        );
        assert_eq!(
            Length::Millimetre(25.4).convert(UnitType::Length(LengthUnit::Inch))?,
            Unit::Length(Length::Inch(1.0))
        );
        assert_eq!(
            Length::Millimetre(304.8).convert(UnitType::Length(LengthUnit::Foot))?,
            Unit::Length(Length::Foot(1.0)),
        );
        Ok(())
    }

    #[test]
    fn test_centimeter_conversions() -> Result<()> {
        assert_eq!(
            Length::Centimetre(5.0).convert(UnitType::Length(LengthUnit::Millimetre))?,
            Unit::Length(Length::Millimetre(50.0)),
        );
        assert_eq!(
            Length::Centimetre(150.0).convert(UnitType::Length(LengthUnit::Centimetre))?,
            Unit::Length(Length::Centimetre(150.0)),
        );
        assert_eq!(
            Length::Centimetre(150.0).convert(UnitType::Length(LengthUnit::Metre))?,
            Unit::Length(Length::Metre(1.5)),
        );
        assert_eq!(
            Length::Centimetre(1e5).convert(UnitType::Length(LengthUnit::Kilometre))?,
            Unit::Length(Length::Kilometre(1.0)),
        );
        assert_eq!(
            Length::Centimetre(2.54).convert(UnitType::Length(LengthUnit::Inch))?,
            Unit::Length(Length::Inch(1.0)),
        );
        assert_eq!(
            Length::Centimetre(30.48).convert(UnitType::Length(LengthUnit::Foot))?,
            Unit::Length(Length::Foot(1.0)),
        );
        Ok(())
    }

    #[test]
    fn test_meter_conversions() -> Result<()> {
        assert_eq!(
            Length::Metre(2.5).convert(UnitType::Length(LengthUnit::Millimetre))?,
            Unit::Length(Length::Millimetre(2500.0)),
        );
        assert_eq!(
            Length::Metre(0.75).convert(UnitType::Length(LengthUnit::Centimetre))?,
            Unit::Length(Length::Centimetre(75.0)),
        );
        assert_eq!(
            Length::Metre(42.0).convert(UnitType::Length(LengthUnit::Metre))?,
            Unit::Length(Length::Metre(42.0)),
        );
        assert_eq!(
            Length::Metre(1e3).convert(UnitType::Length(LengthUnit::Kilometre))?,
            Unit::Length(Length::Kilometre(1.0)),
        );
        assert_approx_eq(
            Length::Metre(0.3048).convert(UnitType::Length(LengthUnit::Inch))?,
            Unit::Length(Length::Inch(12.0)),
            1e-6,
        );
        assert_approx_eq(
            Length::Metre(1.0).convert(UnitType::Length(LengthUnit::Foot))?,
            Unit::Length(Length::Foot(3.28084)),
            1e-6,
        );
        Ok(())
    }

    #[test]
    fn test_inch_conversions() -> Result<()> {
        assert_eq!(
            Length::Inch(1.0).convert(UnitType::Length(LengthUnit::Millimetre))?,
            Unit::Length(Length::Millimetre(25.4)),
        );
        assert_eq!(
            Length::Inch(1.0).convert(UnitType::Length(LengthUnit::Centimetre))?,
            Unit::Length(Length::Centimetre(2.54)),
        );
        assert_approx_eq(
            Length::Inch(39.3701).convert(UnitType::Length(LengthUnit::Metre))?,
            Unit::Length(Length::Metre(1.0)),
            1e-6,
        );
        assert_approx_eq(
            Length::Inch(39370.1).convert(UnitType::Length(LengthUnit::Kilometre))?,
            Unit::Length(Length::Kilometre(1.0)),
            1e-6,
        );
        assert_eq!(
            Length::Inch(5.5).convert(UnitType::Length(LengthUnit::Inch))?,
            Unit::Length(Length::Inch(5.5))
        );
        assert_eq!(
            Length::Inch(12.0).convert(UnitType::Length(LengthUnit::Foot))?,
            Unit::Length(Length::Foot(1.0))
        );
        Ok(())
    }

    #[test]
    fn test_foot_conversions() -> Result<()> {
        assert_eq!(
            Length::Foot(1.0).convert(UnitType::Length(LengthUnit::Millimetre))?,
            Unit::Length(Length::Millimetre(304.8)),
        );
        assert_eq!(
            Length::Foot(1.0).convert(UnitType::Length(LengthUnit::Centimetre))?,
            Unit::Length(Length::Centimetre(30.48)),
        );
        assert_eq!(
            Length::Foot(1.0).convert(UnitType::Length(LengthUnit::Metre))?,
            Unit::Length(Length::Metre(0.3048)),
        );
        assert_approx_eq(
            Length::Foot(3280.84).convert(UnitType::Length(LengthUnit::Kilometre))?,
            Unit::Length(Length::Kilometre(1.0)),
            1e-6,
        );
        assert_approx_eq(
            Length::Foot(2.0).convert(UnitType::Length(LengthUnit::Inch))?,
            Unit::Length(Length::Inch(24.0)),
            1e-6,
        );
        assert_eq!(
            Length::Foot(7.25).convert(UnitType::Length(LengthUnit::Foot))?,
            Unit::Length(Length::Foot(7.25))
        );
        Ok(())
    }

    #[test]
    fn test_zero_conversion() -> Result<()> {
        let got = Length::Metre(0.0).convert(UnitType::Length(LengthUnit::Inch))?;

        assert_approx_eq(got, Unit::Length(Length::Inch(0.0)), 1e-10);
        Ok(())
    }

    #[test]
    fn test_negative_values() -> Result<()> {
        let got = Length::Metre(-5.0).convert(UnitType::Length(LengthUnit::Centimetre))?;

        assert_approx_eq(got, Unit::Length(Length::Centimetre(-500.0)), 1e-10);
        Ok(())
    }

    #[test]
    fn test_very_small_values() -> Result<()> {
        let got = Length::Millimetre(0.001).convert(UnitType::Length(LengthUnit::Metre))?;

        assert_approx_eq(got, Unit::Length(Length::Metre(0.000001)), 1e-12);
        Ok(())
    }

    #[test]
    fn test_very_large_values() -> Result<()> {
        let got = Length::Metre(1000000.0).convert(UnitType::Length(LengthUnit::Millimetre))?;

        assert_approx_eq(got, Unit::Length(Length::Millimetre(1000000000.0)), 1e-6);
        Ok(())
    }

    #[test]
    fn test_fractional_values() -> Result<()> {
        let got = Length::Foot(0.5).convert(UnitType::Length(LengthUnit::Inch))?;

        assert_approx_eq(got, Unit::Length(Length::Inch(6.0)), 1e-10);
        Ok(())
    }

    #[test]
    fn test_precise_decimal() -> Result<()> {
        let got = Length::Metre(1.234567).convert(UnitType::Length(LengthUnit::Centimetre))?;

        assert_approx_eq(got, Unit::Length(Length::Centimetre(123.4567)), 1e-10);
        Ok(())
    }

    #[test]
    fn test_height_conversion_6_feet() -> Result<()> {
        let got = Length::Foot(6.0).convert(UnitType::Length(LengthUnit::Metre))?;

        assert_approx_eq(got, Unit::Length(Length::Metre(1.8288)), 1e-4);
        Ok(())
    }

    #[test]
    fn test_height_conversion_180_cm() -> Result<()> {
        let got = Length::Centimetre(180.0).convert(UnitType::Length(LengthUnit::Foot))?;

        assert_approx_eq(got, Unit::Length(Length::Foot(5.90551)), 1e-4);
        Ok(())
    }

    #[test]
    fn test_screen_size_24_inch() -> Result<()> {
        let got = Length::Inch(24.0).convert(UnitType::Length(LengthUnit::Centimetre))?;

        assert_approx_eq(got, Unit::Length(Length::Centimetre(60.96)), 1e-10);
        Ok(())
    }

    #[test]
    fn test_paper_size_a4_width() -> Result<()> {
        let got = Length::Millimetre(210.0).convert(UnitType::Length(LengthUnit::Inch))?;

        assert_approx_eq(got, Unit::Length(Length::Inch(8.26772)), 1e-4);
        Ok(())
    }

    #[test]
    fn test_round_trip_meter_inch() -> Result<()> {
        let original = 5.0;

        let to_inches = Length::Metre(original).convert(UnitType::Length(LengthUnit::Inch))?;
        let back_to_meters = to_inches.convert(UnitType::Length(LengthUnit::Metre))?;

        assert_approx_eq(back_to_meters, Unit::Length(Length::Metre(original)), 1e-10);
        Ok(())
    }

    #[test]
    fn test_round_trip_foot_millimeter() -> Result<()> {
        let original = 3.5;

        let to_mm = Length::Foot(original).convert(UnitType::Length(LengthUnit::Millimetre))?;
        let back_to_feet = to_mm.convert(UnitType::Length(LengthUnit::Foot))?;

        assert_approx_eq(back_to_feet, Unit::Length(Length::Foot(original)), 1e-10);
        Ok(())
    }

    #[test]
    fn test_round_trip_centimeter_inch() -> Result<()> {
        let original = 15.75;

        let to_inches = Length::Centimetre(original).convert(UnitType::Length(LengthUnit::Inch))?;
        let back_to_cm = to_inches.convert(UnitType::Length(LengthUnit::Centimetre))?;

        assert_approx_eq(
            back_to_cm,
            Unit::Length(Length::Centimetre(original)),
            1e-10,
        );
        Ok(())
    }

    #[test]
    fn test_international_foot_definition() -> Result<()> {
        let got = Length::Foot(1.0).convert(UnitType::Length(LengthUnit::Metre))?;

        assert_approx_eq(got, Unit::Length(Length::Metre(0.3048)), 1e-15);
        Ok(())
    }

    #[test]
    fn test_international_inch_definition() -> Result<()> {
        let got = Length::Inch(1.0).convert(UnitType::Length(LengthUnit::Millimetre))?;

        assert_approx_eq(got, Unit::Length(Length::Millimetre(25.4)), 1e-15);
        Ok(())
    }

    #[test]
    fn test_metric_definitions() -> Result<()> {
        let result1 = Length::Metre(1.0).convert(UnitType::Length(LengthUnit::Centimetre))?;
        assert_approx_eq(result1, Unit::Length(Length::Centimetre(100.0)), 1e-15);

        let result2 = Length::Metre(1.0).convert(UnitType::Length(LengthUnit::Millimetre))?;
        assert_approx_eq(result2, Unit::Length(Length::Millimetre(1000.0)), 1e-15);

        let result3 = Length::Centimetre(1.0).convert(UnitType::Length(LengthUnit::Millimetre))?;
        assert_approx_eq(result3, Unit::Length(Length::Millimetre(10.0)), 1e-15);
        Ok(())
    }

    #[test]
    fn test_complex_chain_conversion() -> Result<()> {
        let original = 1234.5; // millimeters

        let step1 =
            Length::Millimetre(original).convert(UnitType::Length(LengthUnit::Centimetre))?;
        let step2 = step1.convert(UnitType::Length(LengthUnit::Metre))?;
        let step3 = step2.convert(UnitType::Length(LengthUnit::Foot))?;
        let got = step3.convert(UnitType::Length(LengthUnit::Inch))?;

        let want = Length::Millimetre(original).convert(UnitType::Length(LengthUnit::Inch))?;
        assert_eq!(got, want);
        Ok(())
    }
}
