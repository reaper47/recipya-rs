use super::{Mass, MassUnit};
use crate::cooking::units::UnitType;
use crate::cooking::units::traits::UnitOperations;

impl UnitOperations for Mass {
    fn abbrev<'a>(&self) -> &'a str {
        use Mass::*;

        match self {
            Milligram(_) => "mg",
            Gram(_) => "g",
            Dekagram(_) => "dag",
            Hectogram(_) => "hg",
            Kilogram(_) => "kg",
            Ounce(_) => "oz",
            Pound(_) => "lb",
        }
    }

    fn unit_type(&self) -> UnitType {
        use MassUnit::*;

        match self {
            Mass::Milligram(_) => UnitType::Mass(Milligram),
            Mass::Gram(_) => UnitType::Mass(Gram),
            Mass::Dekagram(_) => UnitType::Mass(Dekagram),
            Mass::Hectogram(_) => UnitType::Mass(Hectogram),
            Mass::Kilogram(_) => UnitType::Mass(Kilogram),
            Mass::Ounce(_) => UnitType::Mass(Ounce),
            Mass::Pound(_) => UnitType::Mass(Pound),
        }
    }

    fn value(&self) -> f64 {
        match self {
            Mass::Milligram(v) => *v,
            Mass::Gram(v) => *v,
            Mass::Dekagram(v) => *v,
            Mass::Hectogram(v) => *v,
            Mass::Kilogram(v) => *v,
            Mass::Ounce(v) => *v,
            Mass::Pound(v) => *v,
        }
    }

    fn with_value(&self, value: f64) -> Self {
        match self {
            Mass::Milligram(_) => Mass::Milligram(value),
            Mass::Gram(_) => Mass::Gram(value),
            Mass::Dekagram(_) => Mass::Dekagram(value),
            Mass::Hectogram(_) => Mass::Hectogram(value),
            Mass::Kilogram(_) => Mass::Kilogram(value),
            Mass::Ounce(_) => Mass::Ounce(value),
            Mass::Pound(_) => Mass::Pound(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mass_variants() -> Vec<Mass> {
        vec![
            Mass::Milligram(500.0),
            Mass::Gram(25.5),
            Mass::Dekagram(3.2),
            Mass::Hectogram(1.8),
            Mass::Kilogram(2.5),
            Mass::Ounce(8.0),
            Mass::Pound(1.5),
        ]
    }

    fn create_mass_unit_variants() -> Vec<MassUnit> {
        vec![
            MassUnit::Milligram,
            MassUnit::Gram,
            MassUnit::Dekagram,
            MassUnit::Hectogram,
            MassUnit::Kilogram,
            MassUnit::Ounce,
            MassUnit::Pound,
        ]
    }

    #[test]
    fn test_abbrev() {
        use Mass::*;

        let test_cases = vec![
            (Milligram(1.0), "mg"),
            (Gram(1.0), "g"),
            (Dekagram(1.0), "dag"),
            (Hectogram(1.0), "hg"),
            (Kilogram(1.0), "kg"),
            (Ounce(1.0), "oz"),
            (Pound(1.0), "lb"),
        ];

        for (mass, expected) in test_cases {
            assert_eq!(
                mass.abbrev(),
                expected,
                "Failed for mass variant: {:?}",
                mass
            );
        }
    }

    #[test]
    fn test_mass_creation() {
        let mg = Mass::Milligram(1000.0);
        let g = Mass::Gram(100.0);
        let dag = Mass::Dekagram(10.0);
        let hg = Mass::Hectogram(5.0);
        let kg = Mass::Kilogram(2.5);
        let oz = Mass::Ounce(16.0);
        let lb = Mass::Pound(2.2);

        assert_eq!(mg.value(), 1000.0);
        assert_eq!(g.value(), 100.0);
        assert_eq!(dag.value(), 10.0);
        assert_eq!(hg.value(), 5.0);
        assert_eq!(kg.value(), 2.5);
        assert_eq!(oz.value(), 16.0);
        assert_eq!(lb.value(), 2.2);
    }

    #[test]
    fn test_mass_unit_creation() {
        let units = create_mass_unit_variants();

        // Test that all variants can be created
        assert_eq!(units.len(), 7);

        // Test specific variants
        assert_eq!(MassUnit::Milligram, MassUnit::Milligram);
        assert_eq!(MassUnit::Gram, MassUnit::Gram);
        assert_eq!(MassUnit::Dekagram, MassUnit::Dekagram);
        assert_eq!(MassUnit::Hectogram, MassUnit::Hectogram);
        assert_eq!(MassUnit::Kilogram, MassUnit::Kilogram);
        assert_eq!(MassUnit::Ounce, MassUnit::Ounce);
        assert_eq!(MassUnit::Pound, MassUnit::Pound);
    }

    #[test]
    fn test_mass_unit_inequality() {
        assert_ne!(MassUnit::Milligram, MassUnit::Gram);
        assert_ne!(MassUnit::Gram, MassUnit::Kilogram);
        assert_ne!(MassUnit::Ounce, MassUnit::Pound);
        assert_ne!(MassUnit::Dekagram, MassUnit::Hectogram);
    }

    #[test]
    fn test_unit_type_mapping() {
        let test_cases = vec![
            (Mass::Milligram(0.0), UnitType::Mass(MassUnit::Milligram)),
            (Mass::Gram(0.0), UnitType::Mass(MassUnit::Gram)),
            (Mass::Dekagram(0.0), UnitType::Mass(MassUnit::Dekagram)),
            (Mass::Hectogram(0.0), UnitType::Mass(MassUnit::Hectogram)),
            (Mass::Kilogram(0.0), UnitType::Mass(MassUnit::Kilogram)),
            (Mass::Ounce(0.0), UnitType::Mass(MassUnit::Ounce)),
            (Mass::Pound(0.0), UnitType::Mass(MassUnit::Pound)),
        ];

        for (mass, expected_unit_type) in test_cases {
            assert_eq!(mass.unit_type(), expected_unit_type);
        }
    }

    #[test]
    fn test_value_extraction() {
        let test_values = vec![
            (Mass::Milligram(1250.5), 1250.5),
            (Mass::Gram(45.7), 45.7),
            (Mass::Dekagram(8.9), 8.9),
            (Mass::Hectogram(3.17), 3.17),
            (Mass::Kilogram(0.75), 0.75),
            (Mass::Ounce(12.25), 12.25),
            (Mass::Pound(4.4), 4.4),
        ];

        for (mass, expected) in test_values {
            assert_eq!(mass.value(), expected);
        }
    }

    #[test]
    fn test_with_value_preserves_unit_type() {
        let variants = create_mass_variants();
        let new_value = 42.42;

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
            (Mass::Milligram(100.0), 750.0, Mass::Milligram(750.0)),
            (Mass::Gram(50.0), 125.5, Mass::Gram(125.5)),
            (Mass::Dekagram(10.0), 25.25, Mass::Dekagram(25.25)),
            (Mass::Hectogram(5.0), 12.75, Mass::Hectogram(12.75)),
            (Mass::Kilogram(2.0), 3.8, Mass::Kilogram(3.8)),
            (Mass::Ounce(16.0), 24.5, Mass::Ounce(24.5)),
            (Mass::Pound(1.0), 2.2, Mass::Pound(2.2)),
        ];

        for (original, new_value, expected) in test_cases {
            let result = original.with_value(new_value);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_zero_values() {
        let zero_variants = vec![
            Mass::Milligram(0.0),
            Mass::Gram(0.0),
            Mass::Dekagram(0.0),
            Mass::Hectogram(0.0),
            Mass::Kilogram(0.0),
            Mass::Ounce(0.0),
            Mass::Pound(0.0),
        ];

        for variant in zero_variants {
            assert_eq!(variant.value(), 0.0);
            let new_variant = variant.with_value(15.5);
            assert_eq!(new_variant.value(), 15.5);
        }
    }

    #[test]
    fn test_negative_values() {
        let negative_variants = vec![
            Mass::Milligram(-250.0),
            Mass::Gram(-15.5),
            Mass::Dekagram(-5.2),
            Mass::Hectogram(-2.8),
            Mass::Kilogram(-1.5),
            Mass::Ounce(-8.0),
            Mass::Pound(-0.75),
        ];

        for variant in negative_variants.clone() {
            assert!(variant.value() < 0.0);
        }

        // Test with_value with negative values
        let g = Mass::Gram(10.0);
        let negative_g = g.with_value(-25.7);
        assert_eq!(negative_g.value(), -25.7);
        assert_eq!(negative_g.unit_type(), UnitType::Mass(MassUnit::Gram));
    }

    #[test]
    fn test_large_values() {
        let large_value = 1e15;
        let variants = create_mass_variants();

        for variant in variants {
            let large_variant = variant.with_value(large_value);
            assert_eq!(large_variant.value(), large_value);
        }
    }

    #[test]
    fn test_small_values() {
        let small_value = 1e-15;
        let variants = create_mass_variants();

        for variant in variants {
            let small_variant = variant.with_value(small_value);
            assert_eq!(small_variant.value(), small_value);
        }
    }

    #[test]
    fn test_special_float_values() {
        let kg = Mass::Kilogram(1.0);

        // Test with infinity
        let inf_kg = kg.with_value(f64::INFINITY);
        assert!(inf_kg.value().is_infinite());
        assert_eq!(inf_kg.unit_type(), UnitType::Mass(MassUnit::Kilogram));

        // Test with negative infinity
        let neg_inf_kg = kg.with_value(f64::NEG_INFINITY);
        assert!(neg_inf_kg.value().is_infinite() && neg_inf_kg.value().is_sign_negative());

        // Test with NaN
        let nan_kg = kg.with_value(f64::NAN);
        assert!(nan_kg.value().is_nan());
    }

    #[test]
    fn test_clone_functionality() {
        let variants = create_mass_variants();

        for original in variants {
            let cloned = original.clone();

            assert_eq!(original, cloned);
            assert_eq!(original.value(), cloned.value());
            assert_eq!(original.unit_type(), cloned.unit_type());
        }
    }

    #[test]
    fn test_debug_formatting() {
        let masses = create_mass_variants();

        for mass in masses {
            let debug_str = format!("{:?}", mass);
            assert!(!debug_str.is_empty());

            // Verify the debug string contains the variant name
            match mass {
                Mass::Milligram(_) => assert!(debug_str.contains("Milligram")),
                Mass::Gram(_) => assert!(debug_str.contains("Gram")),
                Mass::Dekagram(_) => assert!(debug_str.contains("Dekagram")),
                Mass::Hectogram(_) => assert!(debug_str.contains("Hectogram")),
                Mass::Kilogram(_) => assert!(debug_str.contains("Kilogram")),
                Mass::Ounce(_) => assert!(debug_str.contains("Ounce")),
                Mass::Pound(_) => assert!(debug_str.contains("Pound")),
            }
        }
    }

    #[test]
    fn test_mass_unit_debug_formatting() {
        let units = create_mass_unit_variants();

        for unit in units {
            let debug_str = format!("{:?}", unit);
            assert!(!debug_str.is_empty());

            // Verify the debug string contains the unit name
            match unit {
                MassUnit::Milligram => assert!(debug_str.contains("Milligram")),
                MassUnit::Gram => assert!(debug_str.contains("Gram")),
                MassUnit::Dekagram => assert!(debug_str.contains("Dekagram")),
                MassUnit::Hectogram => assert!(debug_str.contains("Hectogram")),
                MassUnit::Kilogram => assert!(debug_str.contains("Kilogram")),
                MassUnit::Ounce => assert!(debug_str.contains("Ounce")),
                MassUnit::Pound => assert!(debug_str.contains("Pound")),
            }
        }
    }

    #[test]
    fn test_partial_eq() {
        // Test equality for Mass
        assert_eq!(Mass::Gram(10.0), Mass::Gram(10.0));
        assert_eq!(Mass::Kilogram(2.5), Mass::Kilogram(2.5));

        // Test inequality - different values
        assert_ne!(Mass::Gram(10.0), Mass::Gram(10.1));

        // Test inequality - different units
        assert_ne!(Mass::Gram(10.0), Mass::Kilogram(10.0));
        assert_ne!(Mass::Ounce(16.0), Mass::Pound(1.0)); // Even though these are equivalent

        // Test with special values
        assert_eq!(Mass::Kilogram(f64::INFINITY), Mass::Kilogram(f64::INFINITY));
        assert_ne!(Mass::Kilogram(f64::NAN), Mass::Kilogram(f64::NAN)); // NaN != NaN
    }

    #[test]
    fn test_unit_operations_trait_consistency() {
        let variants = create_mass_variants();

        for variant in variants {
            let original_value = variant.value();
            let original_type = variant.unit_type();

            // Test that with_value preserves type and sets value correctly
            let new_variant = variant.with_value(999.99);
            assert_eq!(new_variant.unit_type(), original_type);
            assert_eq!(new_variant.value(), 999.99);

            // Test that we can restore the original
            let restored = new_variant.with_value(original_value);
            assert_eq!(restored, variant);
        }
    }

    #[test]
    fn test_precision_edge_cases() {
        let value = 0.1 + 0.2; // Known floating point precision issue
        let g = Mass::Gram(value);

        assert_eq!(g.value(), value);

        let new_g = g.with_value(0.3);
        assert_eq!(new_g.value(), 0.3);
    }

    #[test]
    fn test_unit_type_exhaustiveness() {
        // This test ensures all Mass variants have corresponding UnitType mappings
        let all_variants = [
            Mass::Milligram(1.0),
            Mass::Gram(1.0),
            Mass::Dekagram(1.0),
            Mass::Hectogram(1.0),
            Mass::Kilogram(1.0),
            Mass::Ounce(1.0),
            Mass::Pound(1.0),
        ];

        let expected_types = [
            UnitType::Mass(MassUnit::Milligram),
            UnitType::Mass(MassUnit::Gram),
            UnitType::Mass(MassUnit::Dekagram),
            UnitType::Mass(MassUnit::Hectogram),
            UnitType::Mass(MassUnit::Kilogram),
            UnitType::Mass(MassUnit::Ounce),
            UnitType::Mass(MassUnit::Pound),
        ];

        for (variant, expected_type) in all_variants.iter().zip(expected_types.iter()) {
            assert_eq!(variant.unit_type(), *expected_type);
        }
    }

    #[test]
    fn test_chaining_operations() {
        let original = Mass::Kilogram(1.0);

        let result = original.with_value(2.0).with_value(3.0).with_value(4.5);

        assert_eq!(result.value(), 4.5);
        assert_eq!(result.unit_type(), UnitType::Mass(MassUnit::Kilogram));
    }

    #[test]
    fn test_mass_unit_type_relationship() {
        // Test that Mass variants map to the correct MassUnit in UnitType
        let mappings = vec![
            (Mass::Milligram(1.0), MassUnit::Milligram),
            (Mass::Gram(1.0), MassUnit::Gram),
            (Mass::Dekagram(1.0), MassUnit::Dekagram),
            (Mass::Hectogram(1.0), MassUnit::Hectogram),
            (Mass::Kilogram(1.0), MassUnit::Kilogram),
            (Mass::Ounce(1.0), MassUnit::Ounce),
            (Mass::Pound(1.0), MassUnit::Pound),
        ];

        for (mass, expected_unit) in mappings {
            if let UnitType::Mass(unit) = mass.unit_type() {
                assert_eq!(unit, expected_unit);
            } else {
                panic!("Expected UnitType::Mass, got {:?}", mass.unit_type());
            }
        }
    }

    // Property-based test simulation
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
            // Some cooking-relevant values
            28.35,   // 1 ounce in grams
            453.592, // 1 pound in grams
            1000.0,  // 1 kg in grams
        ];

        let variants = create_mass_variants();

        for variant in variants {
            for &test_value in &test_values {
                let modified = variant.with_value(test_value);
                assert_eq!(modified.value(), test_value);
                assert_eq!(modified.unit_type(), variant.unit_type());
            }
        }
    }

    #[test]
    fn test_cooking_relevant_values() {
        // Test with typical cooking measurements
        let flour = Mass::Gram(250.0); // Cup of flour
        let butter = Mass::Gram(113.0); // Stick of butter
        let sugar = Mass::Kilogram(1.0); // Bag of sugar
        let spice = Mass::Milligram(500.0); // Pinch of spice
        let meat = Mass::Pound(2.5); // Roast

        assert_eq!(flour.value(), 250.0);
        assert_eq!(butter.value(), 113.0);
        assert_eq!(sugar.value(), 1.0);
        assert_eq!(spice.value(), 500.0);
        assert_eq!(meat.value(), 2.5);

        // Test conversions maintain unit type
        let heavy_spice = spice.with_value(2000.0);
        assert_eq!(heavy_spice.unit_type(), UnitType::Mass(MassUnit::Milligram));
    }

    #[test]
    fn test_metric_vs_imperial_units() {
        // Test that metric and imperial units are properly distinguished
        let metric_units = vec![
            Mass::Milligram(1.0),
            Mass::Gram(1.0),
            Mass::Dekagram(1.0),
            Mass::Hectogram(1.0),
            Mass::Kilogram(1.0),
        ];

        let imperial_units = vec![Mass::Ounce(1.0), Mass::Pound(1.0)];

        // All metric units should be different from all imperial units
        for metric in &metric_units {
            for imperial in &imperial_units {
                assert_ne!(metric.unit_type(), imperial.unit_type());
                assert_ne!(*metric, *imperial);
            }
        }
    }
}
