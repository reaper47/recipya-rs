use crate::cooking::units::{Length, Mass};
use crate::cooking::units::temperature::Temperature;
use crate::cooking::units::traits::UnitOperations;
use crate::cooking::units::UnitType;

#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    Length(Length),
    Mass(Mass),
    Temperature(Temperature),
    
    // === VOLUME ===
    // Metric
    Millilitre(f64),
    Centilitre(f64),
    Decilitre(f64),
    Litre(f64),
    MetricTeaspoon(f64),
    MetricTablespoon(f64),
    MetricDessertSpoon(f64),
    MetricCup(f64),

    // Australian
    AustralianTablespoon(f64),

    // Imperial
    ImperialTeaspoon(f64),
    ImperialDessertspoon(f64),
    ImperialTablespoon(f64),
    ImperialFluidOunce(f64),
    ImperialGill(f64),
    ImperialCup(f64),
    ImperialPint(f64),
    ImperialQuart(f64),
    ImperialGallon(f64),
    USLegalCup(f64),
    USTeaspoon(f64),
    USTablespoon(f64),
    USFluidOunce(f64),
    USCup(f64),
    USPint(f64),
    USQuart(f64),
    USGallon(f64),

    // Other
    Jigger(f64),
}

impl UnitOperations for Unit {
    fn unit_type(&self) -> UnitType {
        match self {
            Unit::Millilitre(_) => UnitType::Millilitre,
            Unit::Centilitre(_) => UnitType::Centilitre,
            Unit::Decilitre(_) => UnitType::Decilitre,
            Unit::Litre(_) => UnitType::Litre,
            Unit::MetricTeaspoon(_) => UnitType::MetricTeaspoon,
            Unit::MetricTablespoon(_) => UnitType::MetricTablespoon,
            Unit::MetricDessertSpoon(_) => UnitType::MetricDessertSpoon,
            Unit::MetricCup(_) => UnitType::MetricCup,
            Unit::AustralianTablespoon(_) => UnitType::AustralianTablespoon,
            Unit::ImperialTeaspoon(_) => UnitType::ImperialTeaspoon,
            Unit::ImperialDessertspoon(_) => UnitType::ImperialDessertspoon,
            Unit::ImperialTablespoon(_) => UnitType::ImperialTablespoon,
            Unit::ImperialFluidOunce(_) => UnitType::ImperialFluidOunce,
            Unit::ImperialGill(_) => UnitType::ImperialGill,
            Unit::ImperialCup(_) => UnitType::ImperialCup,
            Unit::ImperialPint(_) => UnitType::ImperialPint,
            Unit::ImperialQuart(_) => UnitType::ImperialQuart,
            Unit::ImperialGallon(_) => UnitType::ImperialGallon,
            Unit::USLegalCup(_) => UnitType::USLegalCup,
            Unit::USTeaspoon(_) => UnitType::USTeaspoon,
            Unit::USTablespoon(_) => UnitType::USTablespoon,
            Unit::USFluidOunce(_) => UnitType::USFluidOunce,
            Unit::USCup(_) => UnitType::USCup,
            Unit::USPint(_) => UnitType::USPint,
            Unit::USQuart(_) => UnitType::USQuart,
            Unit::USGallon(_) => UnitType::USGallon,
            Unit::Jigger(_) => UnitType::Jigger,
            Unit::Length(unit) => unit.unit_type(),
            Unit::Mass(unit) => unit.unit_type(),
            Unit::Temperature(unit) => unit.unit_type(),
        }
    }

    fn value(&self) -> f64 {
        match self {
            Unit::Millilitre(v)
            | Unit::Centilitre(v)
            | Unit::Decilitre(v)
            | Unit::Litre(v)
            | Unit::MetricTeaspoon(v)
            | Unit::MetricTablespoon(v)
            | Unit::MetricDessertSpoon(v)
            | Unit::MetricCup(v)
            | Unit::AustralianTablespoon(v)
            | Unit::ImperialTeaspoon(v)
            | Unit::ImperialDessertspoon(v)
            | Unit::ImperialTablespoon(v)
            | Unit::ImperialFluidOunce(v)
            | Unit::ImperialGill(v)
            | Unit::ImperialCup(v)
            | Unit::ImperialPint(v)
            | Unit::ImperialQuart(v)
            | Unit::ImperialGallon(v)
            | Unit::USLegalCup(v)
            | Unit::USTeaspoon(v)
            | Unit::USTablespoon(v)
            | Unit::USFluidOunce(v)
            | Unit::USCup(v)
            | Unit::USPint(v)
            | Unit::USQuart(v)
            | Unit::USGallon(v)
            | Unit::Jigger(v) => *v,
            Unit::Length(unit) => unit.value(),
            Unit::Mass(unit) => unit.value(),
            Unit::Temperature(unit) => unit.value(),
        }
    }

    fn with_value(&self, value: f64) -> Self {
        match self {
            Unit::Millilitre(_) => Unit::Millilitre(value),
            Unit::Centilitre(_) => Unit::Centilitre(value),
            Unit::Decilitre(_) => Unit::Decilitre(value),
            Unit::Litre(_) => Unit::Litre(value),
            Unit::MetricTeaspoon(_) => Unit::MetricTeaspoon(value),
            Unit::MetricTablespoon(_) => Unit::MetricTablespoon(value),
            Unit::MetricDessertSpoon(_) => Unit::MetricDessertSpoon(value),
            Unit::MetricCup(_) => Unit::MetricCup(value),
            Unit::AustralianTablespoon(_) => Unit::AustralianTablespoon(value),
            Unit::ImperialTeaspoon(_) => Unit::ImperialTeaspoon(value),
            Unit::ImperialDessertspoon(_) => Unit::ImperialDessertspoon(value),
            Unit::ImperialTablespoon(_) => Unit::ImperialTablespoon(value),
            Unit::ImperialFluidOunce(_) => Unit::ImperialFluidOunce(value),
            Unit::ImperialGill(_) => Unit::ImperialGill(value),
            Unit::ImperialCup(_) => Unit::ImperialCup(value),
            Unit::ImperialPint(_) => Unit::ImperialPint(value),
            Unit::ImperialQuart(_) => Unit::ImperialQuart(value),
            Unit::ImperialGallon(_) => Unit::ImperialGallon(value),
            Unit::USLegalCup(_) => Unit::USLegalCup(value),
            Unit::USTeaspoon(_) => Unit::USTeaspoon(value),
            Unit::USTablespoon(_) => Unit::USTablespoon(value),
            Unit::USFluidOunce(_) => Unit::USFluidOunce(value),
            Unit::USCup(_) => Unit::USCup(value),
            Unit::USPint(_) => Unit::USPint(value),
            Unit::USQuart(_) => Unit::USQuart(value),
            Unit::USGallon(_) => Unit::USGallon(value),
            Unit::Jigger(_) => Unit::Jigger(value),
            Unit::Length(unit) => Unit::Length(unit.with_value(value)),
            Unit::Mass(unit) => Unit::Mass(unit.with_value(value)),
            Unit::Temperature(unit) => Unit::Temperature(unit.with_value(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_value {
        use super::*;

        #[test]
        fn test_length_units() {
            assert_eq!(Unit::Length(Length::Millimetre(5.5)).value(), 5.5);
            assert_eq!(Unit::Length(Length::Centimetre(10.0)).value(), 10.0);
            assert_eq!(Unit::Length(Length::Metre(2.5)).value(), 2.5);
            assert_eq!(Unit::Length(Length::Inch(12.25)).value(), 12.25);
            assert_eq!(Unit::Length(Length::Foot(3.75)).value(), 3.75);
        }

        #[test]
        fn test_weight_units() {
            assert_eq!(Unit::Mass(Mass::Milligram(100.0)).value(), 100.0);
            assert_eq!(Unit::Mass(Mass::Gram(50.5)).value(), 50.5);
            assert_eq!(Unit::Mass(Mass::Dekagram(10.25)).value(), 10.25);
            assert_eq!(Unit::Mass(Mass::Hectogram(5.75)).value(), 5.75);
            assert_eq!(Unit::Mass(Mass::Kilogram(2.3)).value(), 2.3);
            assert_eq!(Unit::Mass(Mass::Ounce(16.5)).value(), 16.5);
            assert_eq!(Unit::Mass(Mass::Pound(3.2)).value(), 3.2);
        }

        #[test]
        fn test_temperature_units() {
            assert_eq!(Unit::Temperature(Temperature::Celsius(25.0)).value(), 25.0);
            assert_eq!(Unit::Temperature(Temperature::Fahrenheit(77.0)).value(), 77.0);
            assert_eq!(Unit::Temperature(Temperature::Celsius(-10.5)).value(), -10.5);
            assert_eq!(Unit::Temperature(Temperature::Fahrenheit(32.0)).value(), 32.0);
        }

        #[test]
        fn test_metric_volume_units() {
            assert_eq!(Unit::Millilitre(250.0).value(), 250.0);
            assert_eq!(Unit::Centilitre(25.0).value(), 25.0);
            assert_eq!(Unit::Decilitre(2.5).value(), 2.5);
            assert_eq!(Unit::Litre(1.5).value(), 1.5);
            assert_eq!(Unit::MetricTeaspoon(5.0).value(), 5.0);
            assert_eq!(Unit::MetricTablespoon(15.0).value(), 15.0);
            assert_eq!(Unit::MetricDessertSpoon(10.0).value(), 10.0);
            assert_eq!(Unit::MetricCup(250.0).value(), 250.0);
        }

        #[test]
        fn test_imperial_volume_units() {
            assert_eq!(Unit::ImperialTeaspoon(1.0).value(), 1.0);
            assert_eq!(Unit::ImperialDessertspoon(2.0).value(), 2.0);
            assert_eq!(Unit::ImperialTablespoon(3.0).value(), 3.0);
            assert_eq!(Unit::ImperialFluidOunce(4.0).value(), 4.0);
            assert_eq!(Unit::ImperialGill(5.0).value(), 5.0);
            assert_eq!(Unit::ImperialCup(8.0).value(), 8.0);
            assert_eq!(Unit::ImperialPint(16.0).value(), 16.0);
            assert_eq!(Unit::ImperialQuart(32.0).value(), 32.0);
            assert_eq!(Unit::ImperialGallon(128.0).value(), 128.0);
        }

        #[test]
        fn test_us_volume_units_value() {
            assert_eq!(Unit::USLegalCup(240.0).value(), 240.0);
            assert_eq!(Unit::USTeaspoon(4.93).value(), 4.93);
            assert_eq!(Unit::USTablespoon(14.79).value(), 14.79);
            assert_eq!(Unit::USFluidOunce(29.57).value(), 29.57);
            assert_eq!(Unit::USCup(236.59).value(), 236.59);
            assert_eq!(Unit::USPint(473.18).value(), 473.18);
            assert_eq!(Unit::USQuart(946.35).value(), 946.35);
            assert_eq!(Unit::USGallon(3785.41).value(), 3785.41);
        }

        #[test]
        fn test_us_volume_units_with_value() {
            let us_units = vec![
                Unit::USLegalCup(1.0),
                Unit::USTeaspoon(1.0),
                Unit::USTablespoon(1.0),
                Unit::USFluidOunce(1.0),
                Unit::USCup(1.0),
                Unit::USPint(1.0),
                Unit::USQuart(1.0),
                Unit::USGallon(1.0),
            ];

            for unit in us_units {
                let test_value = 123.45;
                let updated = unit.with_value(test_value);
                assert_eq!(updated.value(), test_value);
            }
        }

        #[test]
        fn test_special_units() {
            // Australian Tablespoon
            let aus_tbsp = Unit::AustralianTablespoon(20.0);
            assert_eq!(aus_tbsp.value(), 20.0);
            assert_eq!(aus_tbsp.with_value(25.0).value(), 25.0);

            // Jigger (cocktail measure)
            let jigger = Unit::Jigger(44.36);
            assert_eq!(jigger.value(), 44.36);
            assert_eq!(jigger.with_value(30.0).value(), 30.0);

            match aus_tbsp.with_value(15.0) {
                Unit::AustralianTablespoon(v) => assert_eq!(v, 15.0),
                _ => panic!("Expected AustralianTablespoon unit"),
            }
            match jigger.with_value(45.0) {
                Unit::Jigger(v) => assert_eq!(v, 45.0),
                _ => panic!("Expected Jigger unit"),
            }
        }

        #[test]
        fn test_zero_values() {
            let units = vec![
                Unit::Length(Length::Metre(0.0)),
                Unit::Mass(Mass::Kilogram(0.0)),
                Unit::Temperature(Temperature::Celsius(0.0)),
                Unit::Litre(0.0),
                Unit::USCup(0.0),
            ];

            for unit in units {
                assert_eq!(unit.value(), 0.0);
                assert_eq!(unit.with_value(0.0).value(), 0.0);
            }
        }

        #[test]
        fn test_negative_values() {
            let celsius = Unit::Temperature(Temperature::Celsius(-10.0));
            let fahrenheit = Unit::Temperature(Temperature::Fahrenheit(-5.0));

            assert_eq!(celsius.value(), -10.0);
            assert_eq!(fahrenheit.value(), -5.0);
            assert_eq!(celsius.with_value(-20.0).value(), -20.0);
            assert_eq!(fahrenheit.with_value(-15.0).value(), -15.0);

            let metre = Unit::Length(Length::Metre(-1.0));
            assert_eq!(metre.value(), -1.0);
            assert_eq!(metre.with_value(-2.5).value(), -2.5);
        }

        #[test]
        fn test_large_values() {
            let large_value = 1_000_000.0;
            let units = vec![
                Unit::Length(Length::Millimetre(large_value)),
                Unit::Mass(Mass::Milligram(large_value)),
                Unit::Millilitre(large_value),
            ];

            for unit in units {
                assert_eq!(unit.value(), large_value);
                assert_eq!(
                    unit.with_value(large_value * 2.0).value(),
                    large_value * 2.0
                );
            }
        }

        #[test]
        fn test_small_decimal() {
            let small_value = 0.000001;
            let gram = Unit::Mass(Mass::Gram(small_value));
            let millilitre = Unit::Millilitre(small_value);

            assert_eq!(gram.value(), small_value);
            assert_eq!(millilitre.value(), small_value);

            let smaller_value = 0.0000005;
            assert_eq!(gram.with_value(smaller_value).value(), smaller_value);
            assert_eq!(millilitre.with_value(smaller_value).value(), smaller_value);
        }

        #[test]
        fn test_infinity_and_nan() {
            let inf = f64::INFINITY;
            let neg_inf = f64::NEG_INFINITY;
            let nan = f64::NAN;
            let metre = Unit::Length(Length::Metre(1.0));

            let inf_metre = metre.with_value(inf);
            assert_eq!(inf_metre.value(), inf);
            assert!(inf_metre.value().is_infinite());

            let neg_inf_metre = metre.with_value(neg_inf);
            assert_eq!(neg_inf_metre.value(), neg_inf);
            assert!(neg_inf_metre.value().is_infinite());

            let nan_metre = metre.with_value(nan);
            assert!(nan_metre.value().is_nan());
        }

        #[test]
        fn test_immutability() {
            let original = Unit::Litre(5.0);
            let modified = original.with_value(10.0);

            assert_eq!(original.value(), 5.0);
            assert_eq!(modified.value(), 10.0);

            let another_modification = original.with_value(15.0);
            assert_eq!(original.value(), 5.0);
            assert_eq!(modified.value(), 10.0);
            assert_eq!(another_modification.value(), 15.0);
        }
    }

    mod tests_with_value {
        use super::*;

        #[test]
        fn test_length_units() {
            let mm = Unit::Length(Length::Millimetre(1.0));
            let cm = Unit::Length(Length::Centimetre(2.0));
            let m = Unit::Length(Length::Metre(3.0));
            let inch = Unit::Length(Length::Inch(4.0));
            let foot = Unit::Length(Length::Foot(5.0));

            assert_eq!(mm.with_value(10.5).value(), 10.5);
            assert_eq!(cm.with_value(20.5).value(), 20.5);
            assert_eq!(m.with_value(30.5).value(), 30.5);
            assert_eq!(inch.with_value(40.5).value(), 40.5);
            assert_eq!(foot.with_value(50.5).value(), 50.5);
        }

        #[test]
        fn test_weight_units() {
            let units = [
                Unit::Mass(Mass::Milligram(1.0)),
                Unit::Mass(Mass::Gram(1.0)),
                Unit::Mass(Mass::Dekagram(1.0)),
                Unit::Mass(Mass::Hectogram(1.0)),
                Unit::Mass(Mass::Kilogram(1.0)),
                Unit::Mass(Mass::Ounce(1.0)),
                Unit::Mass(Mass::Pound(1.0)),
            ];

            for (i, unit) in units.iter().enumerate() {
                let new_value = 100.0 + i as f64;
                let updated_unit = unit.with_value(new_value);
                assert_eq!(updated_unit.value(), new_value);
            }

            match Unit::Mass(Mass::Kilogram(5.0).with_value(15.5)){
                Unit::Mass(Mass::Kilogram(v)) => assert_eq!(v, 15.5),
                _ => panic!("Expected Kilogram unit"),
            }
        }

        #[test]
        fn test_temperature_units() {
            let celsius = Unit::Temperature(Temperature::Celsius(20.0));
            let fahrenheit = Unit::Temperature(Temperature::Fahrenheit(68.0));

            assert_eq!(celsius.with_value(100.0).value(), 100.0);
            assert_eq!(fahrenheit.with_value(212.0).value(), 212.0);
            assert_eq!(celsius.with_value(-40.0).value(), -40.0);
            assert_eq!(fahrenheit.with_value(-40.0).value(), -40.0);

            match celsius.with_value(37.5) {
                Unit::Temperature(Temperature::Celsius(v)) => assert_eq!(v, 37.5),
                _ => panic!("Expected Celsius unit"),
            }
            match fahrenheit.with_value(99.5) {
                Unit::Temperature(Temperature::Fahrenheit(v)) => assert_eq!(v, 99.5),
                _ => panic!("Expected Fahrenheit unit"),
            }
        }

        #[test]
        fn test_chained_operations() {
            let unit = Unit::Mass(Mass::Gram(1.0));
            let result = unit.with_value(2.0).with_value(3.0).with_value(4.0);

            assert_eq!(result.value(), 4.0);
            match result {
                Unit::Mass(Mass::Gram(v)) => assert_eq!(v, 4.0),
                _ => panic!("Expected Gram unit"),
            }
        }

        #[test]
        fn test_metric_volume_units() {
            let metric_units = vec![
                (Unit::Millilitre(1.0), "Millilitre"),
                (Unit::Centilitre(1.0), "Centilitre"),
                (Unit::Decilitre(1.0), "Decilitre"),
                (Unit::Litre(1.0), "Litre"),
                (Unit::MetricTeaspoon(1.0), "MetricTeaspoon"),
                (Unit::MetricTablespoon(1.0), "MetricTablespoon"),
                (Unit::MetricDessertSpoon(1.0), "MetricDessertSpoon"),
                (Unit::MetricCup(1.0), "MetricCup"),
            ];

            for (unit, name) in metric_units {
                let new_value = 42.5;
                let updated = unit.with_value(new_value);
                assert_eq!(updated.value(), new_value, "Failed for {name}");
            }
        }

        #[test]
        fn test_imperial_volume_units() {
            let imperial_units = [
                Unit::ImperialTeaspoon(1.0),
                Unit::ImperialDessertspoon(1.0),
                Unit::ImperialTablespoon(1.0),
                Unit::ImperialFluidOunce(1.0),
                Unit::ImperialGill(1.0),
                Unit::ImperialCup(1.0),
                Unit::ImperialPint(1.0),
                Unit::ImperialQuart(1.0),
                Unit::ImperialGallon(1.0),
            ];

            for (i, unit) in imperial_units.iter().enumerate() {
                let test_value = 10.0 + i as f64 * 0.5;
                let updated = unit.with_value(test_value);
                assert_eq!(updated.value(), test_value);
            }

            // Test specific unit preservation
            match Unit::ImperialPint(20.0).with_value(568.26) {
                Unit::ImperialPint(v) => assert_eq!(v, 568.26),
                _ => panic!("Expected ImperialPint unit"),
            }
        }
    }

    mod tests_unit_type {
        use crate::cooking::units::{LengthUnit, MassUnit, TemperatureUnit};
        use super::*;

        #[test]
        fn test_length_units() {
            use crate::cooking::units::LengthUnit::*;

            assert_eq!(Unit::Length(Length::Millimetre(1.0)).unit_type(), UnitType::Length(Millimetre));
            assert_eq!(Unit::Length(Length::Centimetre(2.5)).unit_type(), UnitType::Length(Centimetre));
            assert_eq!(Unit::Length(Length::Metre(10.0)).unit_type(), UnitType::Length(Metre));
            assert_eq!(Unit::Length(Length::Kilometre(100.0)).unit_type(), UnitType::Length(Kilometre));
            assert_eq!(Unit::Length(Length::Inch(12.0)).unit_type(), UnitType::Length(Inch));
            assert_eq!(Unit::Length(Length::Foot(3.0)).unit_type(), UnitType::Length(Foot));
        }

        #[test]
        fn test_mass_units() {
            use crate::cooking::units::MassUnit::*;

            assert_eq!(Unit::Mass(Mass::Milligram(100.0)).unit_type(), UnitType::Mass(Milligram));
            assert_eq!(Unit::Mass(Mass::Gram(500.0)).unit_type(), UnitType::Mass(Gram));
            assert_eq!(Unit::Mass(Mass::Dekagram(1.5)).unit_type(), UnitType::Mass(Dekagram));
            assert_eq!(Unit::Mass(Mass::Hectogram(0.8)).unit_type(), UnitType::Mass(Hectogram));
            assert_eq!(Unit::Mass(Mass::Kilogram(2.3)).unit_type(), UnitType::Mass(Kilogram));
            assert_eq!(Unit::Mass(Mass::Ounce(16.0)).unit_type(), UnitType::Mass(Ounce));
            assert_eq!(Unit::Mass(Mass::Pound(2.2)).unit_type(), UnitType::Mass(Pound));
        }

        #[test]
        fn test_temperature_units() {
            use crate::cooking::units::TemperatureUnit::*;
            
            assert_eq!(Unit::Temperature(Temperature::Celsius(25.0)).unit_type(), UnitType::Temperature(Celsius));
            assert_eq!(Unit::Temperature(Temperature::Fahrenheit(77.0)).unit_type(), UnitType::Temperature(Fahrenheit));
        }

        #[test]
        fn test_metric_volume_units() {
            assert_eq!(Unit::Millilitre(250.0).unit_type(), UnitType::Millilitre);
            assert_eq!(Unit::Centilitre(10.0).unit_type(), UnitType::Centilitre);
            assert_eq!(Unit::Decilitre(5.0).unit_type(), UnitType::Decilitre);
            assert_eq!(Unit::Litre(1.0).unit_type(), UnitType::Litre);
        }

        #[test]
        fn test_metric_cooking_units() {
            assert_eq!(Unit::MetricTeaspoon(1.0).unit_type(), UnitType::MetricTeaspoon);
            assert_eq!(Unit::MetricTablespoon(2.0).unit_type(), UnitType::MetricTablespoon);
            assert_eq!(Unit::MetricDessertSpoon(1.5).unit_type(), UnitType::MetricDessertSpoon);
            assert_eq!(Unit::MetricCup(0.25).unit_type(), UnitType::MetricCup);
        }

        #[test]
        fn test_australian_units() {
            assert_eq!(Unit::AustralianTablespoon(3.0).unit_type(), UnitType::AustralianTablespoon);
        }

        #[test]
        fn test_imperial_volume_units() {
            assert_eq!(Unit::ImperialTeaspoon(4.0).unit_type(), UnitType::ImperialTeaspoon);
            assert_eq!(Unit::ImperialDessertspoon(2.0).unit_type(), UnitType::ImperialDessertspoon);
            assert_eq!(Unit::ImperialTablespoon(1.5).unit_type(), UnitType::ImperialTablespoon);
            assert_eq!(Unit::ImperialFluidOunce(8.0).unit_type(), UnitType::ImperialFluidOunce);
            assert_eq!(Unit::ImperialGill(0.5).unit_type(), UnitType::ImperialGill);
            assert_eq!(Unit::ImperialCup(2.0).unit_type(), UnitType::ImperialCup);
            assert_eq!(Unit::ImperialPint(1.0).unit_type(), UnitType::ImperialPint);
            assert_eq!(Unit::ImperialQuart(0.5).unit_type(), UnitType::ImperialQuart);
            assert_eq!(Unit::ImperialGallon(0.25).unit_type(), UnitType::ImperialGallon);
        }

        #[test]
        fn test_us_volume_units() {
            assert_eq!(Unit::USLegalCup(1.0).unit_type(), UnitType::USLegalCup);
            assert_eq!(Unit::USTeaspoon(6.0).unit_type(), UnitType::USTeaspoon);
            assert_eq!(Unit::USTablespoon(4.0).unit_type(), UnitType::USTablespoon);
            assert_eq!(Unit::USFluidOunce(8.0).unit_type(), UnitType::USFluidOunce);
            assert_eq!(Unit::USCup(2.0).unit_type(), UnitType::USCup);
            assert_eq!(Unit::USPint(1.0).unit_type(), UnitType::USPint);
            assert_eq!(Unit::USQuart(0.5).unit_type(), UnitType::USQuart);
            assert_eq!(Unit::USGallon(0.125).unit_type(), UnitType::USGallon);
        }

        #[test]
        fn test_bartending_units() {
            assert_eq!(Unit::Jigger(2.0).unit_type(), UnitType::Jigger);
        }

        #[test]
        fn test_zero_values() {
            assert_eq!(Unit::Length(Length::Millimetre(0.0)).unit_type(), UnitType::Length(LengthUnit::Millimetre));
            assert_eq!(Unit::Mass(Mass::Gram(0.0)).unit_type(), UnitType::Mass(MassUnit::Gram));
            assert_eq!(Unit::Temperature(Temperature::Celsius(0.0)).unit_type(), UnitType::Temperature(TemperatureUnit::Celsius));
            assert_eq!(Unit::Millilitre(0.0).unit_type(), UnitType::Millilitre);
            assert_eq!(Unit::ImperialPint(0.0).unit_type(), UnitType::ImperialPint);
            assert_eq!(Unit::USCup(0.0).unit_type(), UnitType::USCup);
        }

        #[test]
        fn test_negative_values() {
            use LengthUnit::Metre;
            
            assert_eq!(Unit::Temperature(Temperature::Celsius(-10.0)).unit_type(), UnitType::Temperature(TemperatureUnit::Celsius));
            assert_eq!(Unit::Temperature(Temperature::Fahrenheit(-5.0)).unit_type(), UnitType::Temperature(TemperatureUnit::Fahrenheit));
            assert_eq!(Unit::Length(Length::Metre(-2.5)).unit_type(), UnitType::Length(Metre));
        }

        #[test]
        fn test_large_values() {
            use crate::cooking::units::LengthUnit::Metre;
            use crate::cooking::units::MassUnit::Kilogram;
            
            assert_eq!(Unit::Mass(Mass::Kilogram(1000000.0)).unit_type(), UnitType::Mass(Kilogram));
            assert_eq!(Unit::Litre(999999.9).unit_type(), UnitType::Litre);
            assert_eq!(Unit::Length(Length::Metre(1e10)).unit_type(), UnitType::Length(Metre));
        }

        #[test]
        fn test_small_fractional_values() {
            use crate::cooking::units::LengthUnit::Millimetre;
            use crate::cooking::units::MassUnit::Milligram;
            
            assert_eq!(Unit::Mass(Mass::Milligram(0.001)).unit_type(), UnitType::Mass(Milligram));
            assert_eq!(Unit::Millilitre(0.0001).unit_type(), UnitType::Millilitre);
            assert_eq!(Unit::Length(Length::Millimetre(1e-6)).unit_type(), UnitType::Length(Millimetre));
        }

        #[test]
        fn test_special_float_values() {
            use crate::cooking::units::LengthUnit::Metre;
            use crate::cooking::units::MassUnit::Gram;

            assert_eq!(Unit::Mass(Mass::Gram(f64::INFINITY)).unit_type(), UnitType::Mass(Gram));
            assert_eq!(Unit::Litre(f64::NEG_INFINITY).unit_type(), UnitType::Litre);
            assert_eq!(Unit::Length(Length::Metre(f64::NAN)).unit_type(), UnitType::Length(Metre));
        }

        #[test]
        fn test_comprehensive_unit_coverage() {
            use crate::cooking::units::{LengthUnit::*, MassUnit::*};
            
            let test_cases: Vec<(Unit, UnitType)> = vec![
                // Length units
                (Unit::Length(Length::Millimetre(1.0)), UnitType::Length(Millimetre)),
                (Unit::Length(Length::Centimetre(1.0)), UnitType::Length(Centimetre)),
                (Unit::Length(Length::Metre(1.0)), UnitType::Length(Metre)),
                (Unit::Length(Length::Inch(1.0)), UnitType::Length(Inch)),
                (Unit::Length(Length::Foot(1.0)), UnitType::Length(Foot)),

                // Mass units
                (Unit::Mass(Mass::Milligram(1.0)), UnitType::Mass(Milligram)),
                (Unit::Mass(Mass::Gram(1.0)), UnitType::Mass(Gram)),
                (Unit::Mass(Mass::Dekagram(1.0)), UnitType::Mass(Dekagram)),
                (Unit::Mass(Mass::Hectogram(1.0)), UnitType::Mass(Hectogram)),
                (Unit::Mass(Mass::Kilogram(1.0)), UnitType::Mass(Kilogram)),
                (Unit::Mass(Mass::Ounce(1.0)), UnitType::Mass(Ounce)),
                (Unit::Mass(Mass::Pound(1.0)), UnitType::Mass(Pound)),

                // Temperature units
                (Unit::Temperature(Temperature::Celsius(1.0)), UnitType::Temperature(TemperatureUnit::Celsius)),
                (Unit::Temperature(Temperature::Fahrenheit(1.0)), UnitType::Temperature(TemperatureUnit::Fahrenheit)),

                // Metric volume units
                (Unit::Millilitre(1.0), UnitType::Millilitre),
                (Unit::Centilitre(1.0), UnitType::Centilitre),
                (Unit::Decilitre(1.0), UnitType::Decilitre),
                (Unit::Litre(1.0), UnitType::Litre),

                // Metric cooking units
                (Unit::MetricTeaspoon(1.0), UnitType::MetricTeaspoon),
                (Unit::MetricTablespoon(1.0), UnitType::MetricTablespoon),
                (Unit::MetricDessertSpoon(1.0), UnitType::MetricDessertSpoon),
                (Unit::MetricCup(1.0), UnitType::MetricCup),

                // Australian units
                (Unit::AustralianTablespoon(1.0), UnitType::AustralianTablespoon),

                // Imperial volume units
                (Unit::ImperialTeaspoon(1.0), UnitType::ImperialTeaspoon),
                (Unit::ImperialDessertspoon(1.0), UnitType::ImperialDessertspoon),
                (Unit::ImperialTablespoon(1.0), UnitType::ImperialTablespoon),
                (Unit::ImperialFluidOunce(1.0), UnitType::ImperialFluidOunce),
                (Unit::ImperialGill(1.0), UnitType::ImperialGill),
                (Unit::ImperialCup(1.0), UnitType::ImperialCup),
                (Unit::ImperialPint(1.0), UnitType::ImperialPint),
                (Unit::ImperialQuart(1.0), UnitType::ImperialQuart),
                (Unit::ImperialGallon(1.0), UnitType::ImperialGallon),

                // US volume units
                (Unit::USLegalCup(1.0), UnitType::USLegalCup),
                (Unit::USTeaspoon(1.0), UnitType::USTeaspoon),
                (Unit::USTablespoon(1.0), UnitType::USTablespoon),
                (Unit::USFluidOunce(1.0), UnitType::USFluidOunce),
                (Unit::USCup(1.0), UnitType::USCup),
                (Unit::USPint(1.0), UnitType::USPint),
                (Unit::USQuart(1.0), UnitType::USQuart),
                (Unit::USGallon(1.0), UnitType::USGallon),

                // Bartending units
                (Unit::Jigger(1.0), UnitType::Jigger),
            ];

            for (unit, expected_type) in test_cases {
                assert_eq!(unit.unit_type(), expected_type, "Failed for unit: {unit:?}");
            }
        }

        #[test]
        fn test_unit_type_consistency() {
            let gram_values = vec![0.0, 1.0, -1.0, 100.5, 0.001, 1e6, f64::INFINITY];
            for value in gram_values {
                assert_eq!(Unit::Mass(Mass::Gram(value)).unit_type(), UnitType::Mass(MassUnit::Gram));
            }

            let litre_values = vec![0.0, 0.5, 2.0, 10.25, 0.001, 999.99];
            for value in litre_values {
                assert_eq!(Unit::Litre(value).unit_type(), UnitType::Litre);
            }

            let celsius_values = vec![-273.15, 0.0, 25.0, 100.0, 1000.0];
            for value in celsius_values {
                assert_eq!(Unit::Temperature(Temperature::Celsius(value)).unit_type(), UnitType::Temperature(TemperatureUnit::Celsius));
            }
        }
    }

    #[test]
    fn test_all_unit_types_covered() {
        let all_units = vec![
            // Length
            Unit::Length(Length::Metre(1.0)),
            Unit::Length(Length::Metre(1.0)),
            Unit::Length(Length::Metre(1.0)),
            Unit::Length(Length::Metre(1.0)),
            Unit::Length(Length::Metre(1.0)),
            Unit::Length(Length::Metre(1.0)),

            // Mass
            Unit::Mass(Mass::Milligram(1.0)),
            Unit::Mass(Mass::Gram(1.0)),
            Unit::Mass(Mass::Dekagram(1.0)),
            Unit::Mass(Mass::Hectogram(1.0)),
            Unit::Mass(Mass::Kilogram(1.0)),
            Unit::Mass(Mass::Ounce(1.0)),
            Unit::Mass(Mass::Pound(1.0)),

            // Temperature
            Unit::Temperature(Temperature::Celsius(1.0)),
            Unit::Temperature(Temperature::Fahrenheit(1.0)),

            // Volume - Metric
            Unit::Millilitre(1.0),
            Unit::Centilitre(1.0),
            Unit::Decilitre(1.0),
            Unit::Litre(1.0),
            Unit::MetricTeaspoon(1.0),
            Unit::MetricTablespoon(1.0),
            Unit::MetricDessertSpoon(1.0),
            Unit::MetricCup(1.0),

            // Volume - Australian
            Unit::AustralianTablespoon(1.0),

            // Volume - Imperial
            Unit::ImperialTeaspoon(1.0),
            Unit::ImperialDessertspoon(1.0),
            Unit::ImperialTablespoon(1.0),
            Unit::ImperialFluidOunce(1.0),
            Unit::ImperialGill(1.0),
            Unit::ImperialCup(1.0),
            Unit::ImperialPint(1.0),
            Unit::ImperialQuart(1.0),
            Unit::ImperialGallon(1.0),

            // Volume - US
            Unit::USLegalCup(1.0),
            Unit::USTeaspoon(1.0),
            Unit::USTablespoon(1.0),
            Unit::USFluidOunce(1.0),
            Unit::USCup(1.0),
            Unit::USPint(1.0),
            Unit::USQuart(1.0),
            Unit::USGallon(1.0),

            // Special
            Unit::Jigger(1.0),
        ];

        for (i, unit) in all_units.iter().enumerate() {
            let test_value = i as f64 + 0.5;
            assert_eq!(unit.value(), 1.0);
            assert_eq!(unit.with_value(test_value).value(), test_value);
        }
    }
}
