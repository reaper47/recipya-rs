use crate::cooking::units::traits::{UnitConverter, UnitOperations, UnitScaler};
use crate::cooking::units::{Unit, UnitType};
use crate::{Error, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum Length {
    Millimetre(f64),
    Centimetre(f64),
    Metre(f64),
    Kilometre(f64),
    Inch(f64),
    Foot(f64),
}

#[derive(Debug, PartialEq)]
pub enum LengthUnit {
    Millimetre,
    Centimetre,
    Metre,
    Kilometre,
    Inch,
    Foot,
}

impl UnitOperations for Length {
    fn unit_type(&self) -> UnitType {
        use LengthUnit::*;

        match self {
            Length::Millimetre(_) => UnitType::Length(Millimetre),
            Length::Centimetre(_) => UnitType::Length(Centimetre),
            Length::Metre(_) => UnitType::Length(Metre),
            Length::Kilometre(_) => UnitType::Length(Kilometre),
            Length::Inch(_) => UnitType::Length(Inch),
            Length::Foot(_) => UnitType::Length(Foot),
        }
    }

    fn value(&self) -> f64 {
        match self {
            Length::Millimetre(v) => *v,
            Length::Centimetre(v) => *v,
            Length::Metre(v) => *v,
            Length::Kilometre(v) => *v,
            Length::Inch(v) => *v,
            Length::Foot(v) => *v,
        }
    }

    fn with_value(&self, value: f64) -> Self {
        match self {
            Length::Millimetre(_) => Length::Millimetre(value),
            Length::Centimetre(_) => Length::Centimetre(value),
            Length::Metre(_) => Length::Metre(value),
            Length::Kilometre(_) => Length::Kilometre(value),
            Length::Inch(_) => Length::Inch(value),
            Length::Foot(_) => Length::Foot(value),
        }
    }
}

impl UnitConverter for Length {
    fn convert(&self, to: UnitType) -> Result<Unit> {
        use LengthUnit::*;

        match self {
            Length::Millimetre(original_value) => match to {
                UnitType::Length(unit) => {
                    let value = measurements::Length::from_millimetres(*original_value);

                    match unit {
                        Millimetre => Ok(Unit::Length(self.with_value(*original_value))),
                        Centimetre => Ok(Unit::Length(Length::Centimetre(value.as_centimetres()))),
                        Metre => Ok(Unit::Length(Length::Metre(value.as_metres()))),
                        Kilometre => Ok(Unit::Length(Length::Kilometre(value.as_kilometres()))),
                        Inch => Ok(Unit::Length(Length::Inch(value.as_inches()))),
                        Foot => Ok(Unit::Length(Length::Foot(value.as_feet()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
            },
            Length::Centimetre(original_value) => match to {
                UnitType::Length(unit) => {
                    let value = measurements::Length::from_centimetres(*original_value);

                    match unit {
                        Millimetre => Ok(Unit::Length(Length::Millimetre(value.as_millimetres()))),
                        Centimetre => Ok(Unit::Length(self.with_value(*original_value))),
                        Metre => Ok(Unit::Length(Length::Metre(value.as_metres()))),
                        Kilometre => Ok(Unit::Length(Length::Kilometre(value.as_kilometres()))),
                        Inch => Ok(Unit::Length(Length::Inch(value.as_inches()))),
                        Foot => Ok(Unit::Length(Length::Foot(value.as_feet()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
            },
            Length::Metre(original_value) => {
                let value = measurements::Length::from_metres(*original_value);

                match to {
                    UnitType::Length(unit) => match unit {
                        Millimetre => Ok(Unit::Length(Length::Millimetre(value.as_millimetres()))),
                        Centimetre => Ok(Unit::Length(Length::Centimetre(value.as_centimetres()))),
                        Metre => Ok(Unit::Length(self.with_value(*original_value))),
                        Kilometre => Ok(Unit::Length(Length::Kilometre(value.as_kilometres()))),
                        Inch => Ok(Unit::Length(Length::Inch(value.as_inches()))),
                        Foot => Ok(Unit::Length(Length::Foot(value.as_feet()))),
                    },
                    _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
                }
            }
            Length::Kilometre(original_value) => match to {
                UnitType::Length(unit) => {
                    let value = measurements::Length::from_kilometres(*original_value);

                    match unit {
                        Millimetre => Ok(Unit::Length(Length::Millimetre(value.as_millimetres()))),
                        Centimetre => Ok(Unit::Length(Length::Centimetre(value.as_centimetres()))),
                        Metre => Ok(Unit::Length(Length::Metre(value.as_metres()))),
                        Kilometre => Ok(Unit::Length(self.with_value(*original_value))),
                        Inch => Ok(Unit::Length(Length::Inch(value.as_inches()))),
                        Foot => Ok(Unit::Length(Length::Foot(value.as_feet()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
            },
            Length::Inch(original_value) => match to {
                UnitType::Length(unit) => {
                    let value = measurements::Length::from_inches(*original_value);

                    match unit {
                        Millimetre => Ok(Unit::Length(Length::Millimetre(value.as_millimetres()))),
                        Centimetre => Ok(Unit::Length(Length::Centimetre(value.as_centimetres()))),
                        Metre => Ok(Unit::Length(Length::Metre(value.as_metres()))),
                        Kilometre => Ok(Unit::Length(Length::Kilometre(value.as_kilometres()))),
                        Inch => Ok(Unit::Length(self.with_value(*original_value))),
                        Foot => Ok(Unit::Length(Length::Foot(value.as_feet()))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
            },
            Length::Foot(original_value) => match to {
                UnitType::Length(unit) => {
                    let value = measurements::Length::from_feet(*original_value);

                    match unit {
                        Millimetre => Ok(Unit::Length(Length::Millimetre(value.as_millimetres()))),
                        Centimetre => Ok(Unit::Length(Length::Centimetre(value.as_centimetres()))),
                        Metre => Ok(Unit::Length(Length::Metre(value.as_metres()))),
                        Kilometre => Ok(Unit::Length(Length::Kilometre(value.as_kilometres()))),
                        Inch => Ok(Unit::Length(Length::Inch(value.as_inches()))),
                        Foot => Ok(Unit::Length(self.with_value(*original_value))),
                    }
                }
                _ => Err(Error::UnsupportedUnit(Unit::Length(self.clone()), to)),
            },
        }
    }
}

impl UnitScaler for Length {
    fn scale(&self, factor: f64) -> Result<Unit> {
        let scaled_value = self.value() * factor;

        match self {
            Length::Millimetre(_) => {
                if scaled_value >= 1e6 {
                    Ok(Unit::Length(Length::Kilometre(scaled_value * 1e-6)))
                } else if scaled_value >= 1e3 {
                    Ok(Unit::Length(Length::Metre(scaled_value * 1e-3)))
                } else if scaled_value >= 10.0 {
                    Ok(Unit::Length(Length::Centimetre(scaled_value * 1e-1)))
                } else {
                    Ok(Unit::Length(Length::Millimetre(scaled_value)))
                }
            }
            Length::Centimetre(_) => {
                if scaled_value >= 1e5 {
                    Ok(Unit::Length(Length::Kilometre(scaled_value * 1e-5)))
                } else if scaled_value >= 1e2 {
                    Ok(Unit::Length(Length::Metre(scaled_value * 1e-2)))
                } else if scaled_value >= 10.0 {
                    Ok(Unit::Length(Length::Centimetre(scaled_value)))
                } else {
                    Ok(Unit::Length(Length::Millimetre(scaled_value * 10.0)))
                }
            }
            Length::Metre(_) => {
                if scaled_value >= 1e3 {
                    Ok(Unit::Length(Length::Kilometre(scaled_value * 1e-3)))
                } else if scaled_value >= 1.0 {
                    Ok(Unit::Length(Length::Metre(scaled_value * 1.0)))
                } else if scaled_value >= 1e-2 {
                    Ok(Unit::Length(Length::Centimetre(scaled_value * 1e2)))
                } else {
                    Ok(Unit::Length(Length::Millimetre(scaled_value * 1e3)))
                }
            }
            Length::Kilometre(_) => {
                if scaled_value >= 1.0 {
                    Ok(Unit::Length(Length::Kilometre(scaled_value)))
                } else if scaled_value >= 1e-3 {
                    Ok(Unit::Length(Length::Metre(scaled_value * 1e3)))
                } else if scaled_value >= 1e-5 {
                    Ok(Unit::Length(Length::Centimetre(scaled_value * 1e5)))
                } else {
                    Ok(Unit::Length(Length::Millimetre(scaled_value * 1e6)))
                }
            }
            Length::Inch(_) => {
                if scaled_value >= 12.0 {
                    Ok(Unit::Length(Length::Foot(scaled_value / 12.0)))
                } else {
                    Ok(Unit::Length(Length::Inch(scaled_value)))
                }
            }
            Length::Foot(_) => {
                if scaled_value >= 1.0 {
                    Ok(Unit::Length(Length::Foot(scaled_value)))
                } else {
                    Ok(Unit::Length(Length::Inch(
                        scaled_value / 0.083_333_333_333_333_33,
                    )))
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

    mod tests_unit_operations {
        use super::*;

        fn create_length_variants() -> Vec<Length> {
            vec![
                Length::Millimetre(10.0),
                Length::Centimetre(5.0),
                Length::Metre(2.0),
                Length::Kilometre(1.5),
                Length::Inch(8.0),
                Length::Foot(3.0),
            ]
        }

        #[test]
        fn test_length_creation() {
            let mm = Length::Millimetre(100.0);
            let cm = Length::Centimetre(10.0);
            let m = Length::Metre(1.0);
            let km = Length::Kilometre(0.001);
            let inch = Length::Inch(39.37);
            let foot = Length::Foot(3.28);

            assert_eq!(mm.value(), 100.0);
            assert_eq!(cm.value(), 10.0);
            assert_eq!(m.value(), 1.0);
            assert_eq!(km.value(), 0.001);
            assert_eq!(inch.value(), 39.37);
            assert_eq!(foot.value(), 3.28);
        }

        #[test]
        fn test_unit_type_mapping() {
            use LengthUnit::*;

            assert_eq!(
                Length::Millimetre(0.0).unit_type(),
                UnitType::Length(Millimetre)
            );
            assert_eq!(
                Length::Centimetre(0.0).unit_type(),
                UnitType::Length(Centimetre)
            );
            assert_eq!(Length::Metre(0.0).unit_type(), UnitType::Length(Metre));
            assert_eq!(
                Length::Kilometre(0.0).unit_type(),
                UnitType::Length(Kilometre)
            );
            assert_eq!(Length::Inch(0.0).unit_type(), UnitType::Length(Inch));
            assert_eq!(Length::Foot(0.0).unit_type(), UnitType::Length(Foot));
        }

        #[test]
        fn test_value_extraction() {
            let test_values = vec![
                (Length::Millimetre(42.5), 42.5),
                (Length::Centimetre(13.7), 13.7),
                (Length::Metre(0.95), 0.95),
                (Length::Kilometre(2.3), 2.3),
                (Length::Inch(7.25), 7.25),
                (Length::Foot(1.83), 1.83),
            ];

            for (length, expected) in test_values {
                assert_eq!(length.value(), expected);
            }
        }

        #[test]
        fn test_with_value_preserves_unit_type() {
            let variants = create_length_variants();
            let new_value = 99.99;

            for variant in variants {
                let original_type = variant.unit_type();
                let new_variant = variant.with_value(new_value);

                assert_eq!(new_variant.unit_type(), original_type);
                assert_eq!(new_variant.value(), new_value);
            }
        }

        #[test]
        fn test_with_value_all_variants() {
            let test_cases = vec![
                (Length::Millimetre(10.0), 25.5, Length::Millimetre(25.5)),
                (Length::Centimetre(5.0), 12.3, Length::Centimetre(12.3)),
                (Length::Metre(2.0), 0.75, Length::Metre(0.75)),
                (Length::Kilometre(1.5), 42.1, Length::Kilometre(42.1)),
                (Length::Inch(8.0), 6.25, Length::Inch(6.25)),
                (Length::Foot(3.0), 15.0, Length::Foot(15.0)),
            ];

            for (original, new_value, expected) in test_cases {
                let result = original.with_value(new_value);
                assert_eq!(result, expected);
            }
        }

        #[test]
        fn test_zero_values() {
            let zero_variants = vec![
                Length::Millimetre(0.0),
                Length::Centimetre(0.0),
                Length::Metre(0.0),
                Length::Kilometre(0.0),
                Length::Inch(0.0),
                Length::Foot(0.0),
            ];

            for variant in zero_variants {
                assert_eq!(variant.value(), 0.0);
                let new_variant = variant.with_value(10.0);
                assert_eq!(new_variant.value(), 10.0);
            }
        }

        #[test]
        fn test_negative_values() {
            let negative_variants = vec![
                Length::Millimetre(-5.5),
                Length::Centimetre(-2.3),
                Length::Metre(-1.0),
                Length::Kilometre(-0.5),
                Length::Inch(-3.2),
                Length::Foot(-1.8),
            ];

            for variant in negative_variants.clone() {
                assert!(variant.value() < 0.0);
            }

            let mm = Length::Millimetre(10.0);
            let negative_mm = mm.with_value(-15.7);
            assert_eq!(negative_mm.value(), -15.7);
            assert_eq!(
                negative_mm.unit_type(),
                UnitType::Length(LengthUnit::Millimetre)
            );
        }

        #[test]
        fn test_large_values() {
            let large_value = 1e12;
            let variants = create_length_variants();

            for variant in variants {
                let large_variant = variant.with_value(large_value);
                assert_eq!(large_variant.value(), large_value);
            }
        }

        #[test]
        fn test_small_values() {
            let small_value = 1e-12;
            let variants = create_length_variants();

            for variant in variants {
                let small_variant = variant.with_value(small_value);
                assert_eq!(small_variant.value(), small_value);
            }
        }

        #[test]
        fn test_special_float_values() {
            let mm = Length::Millimetre(1.0);

            let inf_mm = mm.with_value(f64::INFINITY);
            assert!(inf_mm.value().is_infinite());

            let neg_inf_mm = mm.with_value(f64::NEG_INFINITY);
            assert!(neg_inf_mm.value().is_infinite() && neg_inf_mm.value().is_sign_negative());

            let nan_mm = mm.with_value(f64::NAN);
            assert!(nan_mm.value().is_nan());
        }

        #[test]
        fn test_clone_functionality() {
            let original = Length::Metre(42.0);
            let cloned = original.clone();

            assert_eq!(original, cloned);
            assert_eq!(original.value(), cloned.value());
            assert_eq!(original.unit_type(), cloned.unit_type());
        }

        #[test]
        fn test_debug_formatting() {
            let lengths = create_length_variants();

            for length in lengths {
                let debug_str = format!("{length:?}");
                assert!(!debug_str.is_empty());

                match length {
                    Length::Millimetre(_) => assert!(debug_str.contains("Millimetre")),
                    Length::Centimetre(_) => assert!(debug_str.contains("Centimetre")),
                    Length::Metre(_) => assert!(debug_str.contains("Metre")),
                    Length::Kilometre(_) => assert!(debug_str.contains("Kilometre")),
                    Length::Inch(_) => assert!(debug_str.contains("Inch")),
                    Length::Foot(_) => assert!(debug_str.contains("Foot")),
                }
            }
        }

        #[test]
        fn test_partial_eq() {
            // Test equality
            assert_eq!(Length::Millimetre(10.0), Length::Millimetre(10.0));
            assert_eq!(Length::Centimetre(5.5), Length::Centimetre(5.5));

            // Test inequality - different values
            assert_ne!(Length::Millimetre(10.0), Length::Millimetre(10.1));

            // Test inequality - different units
            assert_ne!(Length::Millimetre(10.0), Length::Centimetre(10.0));

            // Test with special values
            assert_eq!(Length::Metre(f64::INFINITY), Length::Metre(f64::INFINITY));
            assert_ne!(Length::Metre(f64::NAN), Length::Metre(f64::NAN)); // NaN != NaN
        }

        #[test]
        fn test_unit_operations_trait_consistency() {
            let variants = create_length_variants();

            for variant in variants {
                let original_value = variant.value();
                let original_type = variant.unit_type();

                let new_variant = variant.with_value(123.45);
                assert_eq!(new_variant.unit_type(), original_type);
                assert_eq!(new_variant.value(), 123.45);

                let restored = new_variant.with_value(original_value);
                assert_eq!(restored, variant);
            }
        }

        #[test]
        fn test_precision_edge_cases() {
            let value = 0.1 + 0.2; // Known floating point precision issue
            let mm = Length::Millimetre(value);

            assert_eq!(mm.value(), value);

            let new_mm = mm.with_value(0.3);
            assert_eq!(new_mm.value(), 0.3);
        }

        #[test]
        fn test_unit_type_exhaustiveness() {
            let all_variants = [
                Length::Millimetre(1.0),
                Length::Centimetre(1.0),
                Length::Metre(1.0),
                Length::Kilometre(1.0),
                Length::Inch(1.0),
                Length::Foot(1.0),
            ];

            let expected_types = [
                UnitType::Length(LengthUnit::Millimetre),
                UnitType::Length(LengthUnit::Centimetre),
                UnitType::Length(LengthUnit::Metre),
                UnitType::Length(LengthUnit::Kilometre),
                UnitType::Length(LengthUnit::Inch),
                UnitType::Length(LengthUnit::Foot),
            ];

            for (variant, expected_type) in all_variants.iter().zip(expected_types.iter()) {
                assert_eq!(variant.unit_type(), *expected_type);
            }
        }

        #[test]
        fn test_chaining_operations() {
            let original = Length::Metre(10.0);

            let result = original.with_value(20.0).with_value(30.0).with_value(40.0);

            assert_eq!(result.value(), 40.0);
            assert_eq!(result.unit_type(), UnitType::Length(LengthUnit::Metre));
        }

        #[test]
        fn test_value_roundtrip_property() {
            let test_values = vec![
                0.0,
                1.0,
                -1.0,
                42.42,
                -99.99,
                1e6,
                -1e6,
                f64::MIN_POSITIVE,
                f64::MAX,
                f64::EPSILON,
            ];

            let variants = create_length_variants();

            for variant in variants {
                for &test_value in &test_values {
                    let modified = variant.with_value(test_value);
                    assert_eq!(modified.value(), test_value);
                    assert_eq!(modified.unit_type(), variant.unit_type());
                }
            }
        }
    }

    mod tests_conversion {
        use super::*;

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

            let to_inches =
                Length::Centimetre(original).convert(UnitType::Length(LengthUnit::Inch))?;
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

            let result3 =
                Length::Centimetre(1.0).convert(UnitType::Length(LengthUnit::Millimetre))?;
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

    mod tests_scale {
        use super::*;

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
}
