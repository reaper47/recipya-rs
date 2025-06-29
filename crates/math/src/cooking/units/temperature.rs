use crate::Error;
use crate::cooking::units::traits::{UnitConverter, UnitOperations};
use crate::cooking::units::{Unit, UnitType};

#[derive(Debug, Clone, PartialEq)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

#[derive(Debug, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl UnitOperations for Temperature {
    fn unit_type(&self) -> UnitType {
        use TemperatureUnit::*;

        match self {
            Temperature::Celsius(_) => UnitType::Temperature(Celsius),
            Temperature::Fahrenheit(_) => UnitType::Temperature(Fahrenheit),
        }
    }

    fn value(&self) -> f64 {
        match self {
            Temperature::Celsius(v) => *v,
            Temperature::Fahrenheit(v) => *v,
        }
    }

    fn with_value(&self, value: f64) -> Self {
        match self {
            Temperature::Celsius(_) => Temperature::Celsius(value),
            Temperature::Fahrenheit(_) => Temperature::Fahrenheit(value),
        }
    }
}

impl UnitConverter for Temperature {
    fn convert(&self, to: UnitType) -> crate::Result<Unit> {
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

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_unit_operations {
        use super::*;

        fn create_temperature_variants() -> Vec<Temperature> {
            vec![Temperature::Celsius(25.0), Temperature::Fahrenheit(77.0)]
        }

        fn create_temperature_unit_variants() -> Vec<TemperatureUnit> {
            vec![TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit]
        }

        #[test]
        fn test_temperature_creation() {
            let celsius = Temperature::Celsius(20.0);
            let fahrenheit = Temperature::Fahrenheit(68.0);

            assert_eq!(celsius.value(), 20.0);
            assert_eq!(fahrenheit.value(), 68.0);
        }

        #[test]
        fn test_temperature_unit_creation() {
            let units = create_temperature_unit_variants();

            assert_eq!(units.len(), 2);
            assert_eq!(TemperatureUnit::Celsius, TemperatureUnit::Celsius);
            assert_eq!(TemperatureUnit::Fahrenheit, TemperatureUnit::Fahrenheit);
        }

        #[test]
        fn test_temperature_unit_inequality() {
            assert_ne!(TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit);
            assert_ne!(TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius);
        }

        #[test]
        fn test_unit_type_mapping() {
            let test_cases = vec![
                (
                    Temperature::Celsius(0.0),
                    UnitType::Temperature(TemperatureUnit::Celsius),
                ),
                (
                    Temperature::Fahrenheit(0.0),
                    UnitType::Temperature(TemperatureUnit::Fahrenheit),
                ),
            ];

            for (temperature, expected_unit_type) in test_cases {
                assert_eq!(temperature.unit_type(), expected_unit_type);
            }
        }

        #[test]
        fn test_value_extraction() {
            let test_values = vec![
                (Temperature::Celsius(100.0), 100.0),
                (Temperature::Fahrenheit(212.0), 212.0),
                (Temperature::Celsius(-40.0), -40.0),
                (Temperature::Fahrenheit(-40.0), -40.0),
                (Temperature::Celsius(37.5), 37.5),
                (Temperature::Fahrenheit(99.5), 99.5),
            ];

            for (temperature, expected) in test_values {
                assert_eq!(temperature.value(), expected);
            }
        }

        #[test]
        fn test_with_value_preserves_unit_type() {
            let variants = create_temperature_variants();
            let new_value = 150.0;

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
                (
                    Temperature::Celsius(20.0),
                    100.0,
                    Temperature::Celsius(100.0),
                ),
                (
                    Temperature::Fahrenheit(68.0),
                    212.0,
                    Temperature::Fahrenheit(212.0),
                ),
                (
                    Temperature::Celsius(0.0),
                    -273.15,
                    Temperature::Celsius(-273.15),
                ),
                (
                    Temperature::Fahrenheit(32.0),
                    -459.67,
                    Temperature::Fahrenheit(-459.67),
                ),
            ];

            for (original, new_value, expected) in test_cases {
                let result = original.with_value(new_value);
                assert_eq!(result, expected);
            }
        }

        #[test]
        fn test_zero_values() {
            let zero_variants = vec![Temperature::Celsius(0.0), Temperature::Fahrenheit(0.0)];

            for variant in zero_variants {
                assert_eq!(variant.value(), 0.0);
                let new_variant = variant.with_value(25.0);
                assert_eq!(new_variant.value(), 25.0);
            }
        }

        #[test]
        fn test_negative_values() {
            let negative_variants = vec![
                Temperature::Celsius(-10.0),
                Temperature::Fahrenheit(-40.0),
                Temperature::Celsius(-273.15),
                Temperature::Fahrenheit(-459.67),
            ];

            for variant in negative_variants.clone() {
                assert!(variant.value() < 0.0);
            }

            let celsius = Temperature::Celsius(20.0);
            let negative_celsius = celsius.with_value(-15.5);
            assert_eq!(negative_celsius.value(), -15.5);
            assert_eq!(
                negative_celsius.unit_type(),
                UnitType::Temperature(TemperatureUnit::Celsius)
            );
        }

        #[test]
        fn test_absolute_zero_temperatures() {
            let absolute_zero_celsius = Temperature::Celsius(-273.15);
            let absolute_zero_fahrenheit = Temperature::Fahrenheit(-459.67);

            assert_eq!(absolute_zero_celsius.value(), -273.15);
            assert_eq!(absolute_zero_fahrenheit.value(), -459.67);
        }

        #[test]
        fn test_boiling_point_temperatures() {
            let boiling_celsius = Temperature::Celsius(100.0);
            let boiling_fahrenheit = Temperature::Fahrenheit(212.0);

            assert_eq!(boiling_celsius.value(), 100.0);
            assert_eq!(boiling_fahrenheit.value(), 212.0);
        }

        #[test]
        fn test_freezing_point_temperatures() {
            let freezing_celsius = Temperature::Celsius(0.0);
            let freezing_fahrenheit = Temperature::Fahrenheit(32.0);

            assert_eq!(freezing_celsius.value(), 0.0);
            assert_eq!(freezing_fahrenheit.value(), 32.0);
        }

        #[test]
        fn test_body_temperature() {
            let body_temp_celsius = Temperature::Celsius(37.0);
            let body_temp_fahrenheit = Temperature::Fahrenheit(98.6);

            assert_eq!(body_temp_celsius.value(), 37.0);
            assert_eq!(body_temp_fahrenheit.value(), 98.6);
        }

        #[test]
        fn test_large_values() {
            let large_value = 1000.0;
            let variants = create_temperature_variants();

            for variant in variants {
                let large_variant = variant.with_value(large_value);
                assert_eq!(large_variant.value(), large_value);
            }
        }

        #[test]
        fn test_small_values() {
            let small_value = -300.0;
            let variants = create_temperature_variants();

            for variant in variants {
                let small_variant = variant.with_value(small_value);
                assert_eq!(small_variant.value(), small_value);
            }
        }

        #[test]
        fn test_special_float_values() {
            let celsius = Temperature::Celsius(25.0);

            let inf_celsius = celsius.with_value(f64::INFINITY);
            assert!(inf_celsius.value().is_infinite());
            assert_eq!(
                inf_celsius.unit_type(),
                UnitType::Temperature(TemperatureUnit::Celsius)
            );

            let neg_inf_celsius = celsius.with_value(f64::NEG_INFINITY);
            assert!(
                neg_inf_celsius.value().is_infinite() && neg_inf_celsius.value().is_sign_negative()
            );

            // Test with NaN
            let nan_celsius = celsius.with_value(f64::NAN);
            assert!(nan_celsius.value().is_nan());
        }

        #[test]
        fn test_clone_functionality() {
            let variants = create_temperature_variants();

            for original in variants {
                let cloned = original.clone();

                assert_eq!(original, cloned);
                assert_eq!(original.value(), cloned.value());
                assert_eq!(original.unit_type(), cloned.unit_type());
            }
        }

        #[test]
        fn test_debug_formatting() {
            let temperatures = create_temperature_variants();

            for temperature in temperatures {
                let debug_str = format!("{:?}", temperature);
                assert!(!debug_str.is_empty());

                match temperature {
                    Temperature::Celsius(_) => assert!(debug_str.contains("Celsius")),
                    Temperature::Fahrenheit(_) => assert!(debug_str.contains("Fahrenheit")),
                }
            }
        }

        #[test]
        fn test_temperature_unit_debug_formatting() {
            let units = create_temperature_unit_variants();

            for unit in units {
                let debug_str = format!("{:?}", unit);
                assert!(!debug_str.is_empty());

                match unit {
                    TemperatureUnit::Celsius => assert!(debug_str.contains("Celsius")),
                    TemperatureUnit::Fahrenheit => assert!(debug_str.contains("Fahrenheit")),
                }
            }
        }

        #[test]
        fn test_partial_eq() {
            assert_eq!(Temperature::Celsius(25.0), Temperature::Celsius(25.0));
            assert_eq!(Temperature::Fahrenheit(77.0), Temperature::Fahrenheit(77.0));
            assert_ne!(Temperature::Celsius(25.0), Temperature::Celsius(25.1));
            assert_ne!(Temperature::Celsius(0.0), Temperature::Fahrenheit(32.0));
            assert_ne!(Temperature::Celsius(100.0), Temperature::Fahrenheit(212.0));
            assert_eq!(
                Temperature::Celsius(f64::INFINITY),
                Temperature::Celsius(f64::INFINITY)
            );
            assert_ne!(
                Temperature::Celsius(f64::NAN),
                Temperature::Celsius(f64::NAN)
            ); // NaN != NaN
        }

        #[test]
        fn test_unit_operations_trait_consistency() {
            let variants = create_temperature_variants();

            for variant in variants {
                let original_value = variant.value();
                let original_type = variant.unit_type();

                let new_variant = variant.with_value(180.0);
                assert_eq!(new_variant.unit_type(), original_type);
                assert_eq!(new_variant.value(), 180.0);

                let restored = new_variant.with_value(original_value);
                assert_eq!(restored, variant);
            }
        }

        #[test]
        fn test_precision_edge_cases() {
            let value = 0.1 + 0.2;
            let celsius = Temperature::Celsius(value);

            assert_eq!(celsius.value(), value);

            let new_celsius = celsius.with_value(0.3);
            assert_eq!(new_celsius.value(), 0.3);
        }

        #[test]
        fn test_unit_type_exhaustiveness() {
            let all_variants = [Temperature::Celsius(1.0), Temperature::Fahrenheit(1.0)];

            let expected_types = [
                UnitType::Temperature(TemperatureUnit::Celsius),
                UnitType::Temperature(TemperatureUnit::Fahrenheit),
            ];

            for (variant, expected_type) in all_variants.iter().zip(expected_types.iter()) {
                assert_eq!(variant.unit_type(), *expected_type);
            }
        }

        #[test]
        fn test_chaining_operations() {
            let original = Temperature::Celsius(20.0);

            let result = original.with_value(30.0).with_value(40.0).with_value(50.0);

            assert_eq!(result.value(), 50.0);
            assert_eq!(
                result.unit_type(),
                UnitType::Temperature(TemperatureUnit::Celsius)
            );
        }

        #[test]
        fn test_temperature_unit_type_relationship() {
            let mappings = vec![
                (Temperature::Celsius(1.0), TemperatureUnit::Celsius),
                (Temperature::Fahrenheit(1.0), TemperatureUnit::Fahrenheit),
            ];

            for (temperature, expected_unit) in mappings {
                if let UnitType::Temperature(unit) = temperature.unit_type() {
                    assert_eq!(unit, expected_unit);
                } else {
                    panic!(
                        "Expected UnitType::Temperature, got {:?}",
                        temperature.unit_type()
                    );
                }
            }
        }

        #[test]
        fn test_cooking_temperatures() {
            let oven_temp_celsius = Temperature::Celsius(180.0); // Common baking temp
            let oven_temp_fahrenheit = Temperature::Fahrenheit(350.0); // Common baking temp
            let deep_fry_celsius = Temperature::Celsius(175.0); // Deep frying
            let deep_fry_fahrenheit = Temperature::Fahrenheit(347.0); // Deep frying
            let candy_temp_celsius = Temperature::Celsius(118.0); // Soft ball stage
            let candy_temp_fahrenheit = Temperature::Fahrenheit(244.0); // Soft ball stage

            assert_eq!(oven_temp_celsius.value(), 180.0);
            assert_eq!(oven_temp_fahrenheit.value(), 350.0);
            assert_eq!(deep_fry_celsius.value(), 175.0);
            assert_eq!(deep_fry_fahrenheit.value(), 347.0);
            assert_eq!(candy_temp_celsius.value(), 118.0);
            assert_eq!(candy_temp_fahrenheit.value(), 244.0);
        }

        #[test]
        fn test_food_safety_temperatures() {
            let danger_zone_start_c = Temperature::Celsius(4.0);
            let danger_zone_start_f = Temperature::Fahrenheit(40.0);
            let danger_zone_end_c = Temperature::Celsius(60.0);
            let danger_zone_end_f = Temperature::Fahrenheit(140.0);

            let chicken_safe_c = Temperature::Celsius(74.0);
            let chicken_safe_f = Temperature::Fahrenheit(165.0);
            let beef_medium_c = Temperature::Celsius(63.0);
            let beef_medium_f = Temperature::Fahrenheit(145.0);

            assert_eq!(danger_zone_start_c.value(), 4.0);
            assert_eq!(danger_zone_start_f.value(), 40.0);
            assert_eq!(danger_zone_end_c.value(), 60.0);
            assert_eq!(danger_zone_end_f.value(), 140.0);
            assert_eq!(chicken_safe_c.value(), 74.0);
            assert_eq!(chicken_safe_f.value(), 165.0);
            assert_eq!(beef_medium_c.value(), 63.0);
            assert_eq!(beef_medium_f.value(), 145.0);
        }

        #[test]
        fn test_extreme_cooking_temperatures() {
            let pizza_oven_c = Temperature::Celsius(480.0); // Wood-fired pizza oven
            let pizza_oven_f = Temperature::Fahrenheit(900.0); // Wood-fired pizza oven
            let liquid_nitrogen_c = Temperature::Celsius(-196.0); // Molecular gastronomy
            let liquid_nitrogen_f = Temperature::Fahrenheit(-321.0); // Molecular gastronomy

            assert_eq!(pizza_oven_c.value(), 480.0);
            assert_eq!(pizza_oven_f.value(), 900.0);
            assert_eq!(liquid_nitrogen_c.value(), -196.0);
            assert_eq!(liquid_nitrogen_f.value(), -321.0);
        }

        #[test]
        fn test_equivalent_temperatures_are_not_equal() {
            let freezing_c = Temperature::Celsius(0.0);
            let freezing_f = Temperature::Fahrenheit(32.0);
            let boiling_c = Temperature::Celsius(100.0);
            let boiling_f = Temperature::Fahrenheit(212.0);
            let equal_point_c = Temperature::Celsius(-40.0);
            let equal_point_f = Temperature::Fahrenheit(-40.0);

            assert_ne!(freezing_c, freezing_f);
            assert_ne!(boiling_c, boiling_f);
            assert_ne!(equal_point_c, equal_point_f);
        }

        #[test]
        fn test_value_roundtrip_property() {
            let test_values = vec![
                0.0,
                1.0,
                -1.0,
                25.0,
                -40.0,
                100.0,
                212.0,
                -273.15,
                -459.67,
                37.0,
                98.6,
                180.0,
                350.0,
                f64::MIN,
                f64::MAX,
                f64::EPSILON,
            ];

            let variants = create_temperature_variants();

            for variant in variants {
                for &test_value in &test_values {
                    let modified = variant.with_value(test_value);
                    assert_eq!(modified.value(), test_value);
                    assert_eq!(modified.unit_type(), variant.unit_type());
                }
            }
        }

        #[test]
        fn test_temperature_ranges() {
            let sub_zero_c = Temperature::Celsius(-50.0);
            let sub_zero_f = Temperature::Fahrenheit(-58.0);
            let room_temp_c = Temperature::Celsius(22.0);
            let room_temp_f = Temperature::Fahrenheit(72.0);
            let hot_day_c = Temperature::Celsius(40.0);
            let hot_day_f = Temperature::Fahrenheit(104.0);

            let cold_to_hot_c = sub_zero_c.with_value(40.0);
            let cold_to_hot_f = sub_zero_f.with_value(104.0);

            assert_eq!(cold_to_hot_c.value(), 40.0);
            assert_eq!(
                cold_to_hot_c.unit_type(),
                UnitType::Temperature(TemperatureUnit::Celsius)
            );
            assert_eq!(cold_to_hot_f.value(), 104.0);
            assert_eq!(
                cold_to_hot_f.unit_type(),
                UnitType::Temperature(TemperatureUnit::Fahrenheit)
            );
        }

        #[test]
        fn test_temperature_scale_distinction() {
            let celsius_temps = vec![
                Temperature::Celsius(0.0),
                Temperature::Celsius(25.0),
                Temperature::Celsius(100.0),
            ];

            let fahrenheit_temps = vec![
                Temperature::Fahrenheit(32.0),
                Temperature::Fahrenheit(77.0),
                Temperature::Fahrenheit(212.0),
            ];

            for celsius in &celsius_temps {
                for fahrenheit in &fahrenheit_temps {
                    assert_ne!(celsius.unit_type(), fahrenheit.unit_type());
                    assert_ne!(*celsius, *fahrenheit);
                }
            }
        }
    }

    mod tests_conversions {
        use super::*;
        use TemperatureUnit::*;

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
}
