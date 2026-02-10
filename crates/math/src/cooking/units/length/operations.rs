use crate::cooking::units::UnitType;
use crate::cooking::units::length::units::{Length, LengthUnit};
use crate::cooking::units::traits::UnitOperations;

impl UnitOperations for Length {
    fn abbrev(&self) -> &str {
        use Length::{Centimetre, Foot, Inch, Kilometre, Metre, Millimetre};

        match self {
            Millimetre(_) => "mm",
            Centimetre(_) => "cm",
            Metre(_) => "m",
            Kilometre(_) => "km",
            Inch(_) => "\"",
            Foot(_) => "'",
        }
    }

    fn unit_type(&self) -> UnitType {
        use LengthUnit::{Centimetre, Foot, Inch, Kilometre, Metre, Millimetre};

        match self {
            Self::Millimetre(_) => UnitType::Length(Millimetre),
            Self::Centimetre(_) => UnitType::Length(Centimetre),
            Self::Metre(_) => UnitType::Length(Metre),
            Self::Kilometre(_) => UnitType::Length(Kilometre),
            Self::Inch(_) => UnitType::Length(Inch),
            Self::Foot(_) => UnitType::Length(Foot),
        }
    }

    fn value(&self) -> f64 {
        match self {
            Self::Millimetre(v)
            | Self::Centimetre(v)
            | Self::Metre(v)
            | Self::Kilometre(v)
            | Self::Inch(v)
            | Self::Foot(v) => *v,
        }
    }

    fn with_value(&self, value: f64) -> Self {
        match self {
            Self::Millimetre(_) => Self::Millimetre(value),
            Self::Centimetre(_) => Self::Centimetre(value),
            Self::Metre(_) => Self::Metre(value),
            Self::Kilometre(_) => Self::Kilometre(value),
            Self::Inch(_) => Self::Inch(value),
            Self::Foot(_) => Self::Foot(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64;

    use approx::assert_relative_eq;

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
    fn test_abbrev() {
        use Length::*;

        let test_cases = vec![
            (Millimetre(1.0), "mm"),
            (Centimetre(1.0), "cm"),
            (Metre(1.0), "m"),
            (Kilometre(1.0), "km"),
            (Inch(1.0), "\""),
            (Foot(1.0), "'"),
        ];

        for (length, expected) in test_cases {
            assert_eq!(
                length.abbrev(),
                expected,
                "Failed for length variant: {length:?}",
            );
        }
    }

    #[test]
    fn test_length_creation() {
        let mm = Length::Millimetre(100.0);
        let cm = Length::Centimetre(10.0);
        let m = Length::Metre(1.0);
        let km = Length::Kilometre(0.001);
        let inch = Length::Inch(39.37);
        let foot = Length::Foot(3.28);

        assert_relative_eq!(mm.value(), 100.0, epsilon = f64::EPSILON);
        assert_relative_eq!(cm.value(), 10.0, epsilon = f64::EPSILON);
        assert_relative_eq!(m.value(), 1.0, epsilon = f64::EPSILON);
        assert_relative_eq!(km.value(), 0.001, epsilon = f64::EPSILON);
        assert_relative_eq!(inch.value(), 39.37, epsilon = f64::EPSILON);
        assert_relative_eq!(foot.value(), 3.28, epsilon = f64::EPSILON);
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
            assert_relative_eq!(length.value(), expected, epsilon = f64::EPSILON);
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
            assert_relative_eq!(new_variant.value(), new_value, epsilon = f64::EPSILON);
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
            assert_relative_eq!(variant.value(), 0.0, epsilon = f64::EPSILON);
            let new_variant = variant.with_value(10.0);
            assert_relative_eq!(new_variant.value(), 10.0, epsilon = f64::EPSILON);
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

        for variant in negative_variants {
            assert!(variant.value() < 0.0);
        }

        let mm = Length::Millimetre(10.0);
        let negative_mm = mm.with_value(-15.7);
        assert_relative_eq!(negative_mm.value(), -15.7, epsilon = f64::EPSILON);
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
            assert_relative_eq!(large_variant.value(), large_value, epsilon = f64::EPSILON);
        }
    }

    #[test]
    fn test_small_values() {
        let small_value = 1e-12;
        let variants = create_length_variants();

        for variant in variants {
            let small_variant = variant.with_value(small_value);
            assert_relative_eq!(small_variant.value(), small_value, epsilon = f64::EPSILON);
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
            assert_relative_eq!(new_variant.value(), 123.45, epsilon = f64::EPSILON);

            let restored = new_variant.with_value(original_value);
            assert_eq!(restored, variant);
        }
    }

    #[test]
    fn test_precision_edge_cases() {
        let value = 0.1 + 0.2; // Known floating point precision issue
        let mm = Length::Millimetre(value);

        assert_relative_eq!(mm.value(), value, epsilon = f64::EPSILON);

        let new_mm = mm.with_value(0.3);
        assert_relative_eq!(new_mm.value(), 0.3, epsilon = f64::EPSILON);
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

        assert_relative_eq!(result.value(), 40.0, epsilon = f64::EPSILON);
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
                assert_relative_eq!(modified.value(), test_value, epsilon = f64::EPSILON);
                assert_eq!(modified.unit_type(), variant.unit_type());
            }
        }
    }
}
