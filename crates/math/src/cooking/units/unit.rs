use crate::Error;
use crate::cooking::units::UnitType;
use crate::cooking::units::traits::{UnitConverter, UnitOperations, UnitScaler};
use crate::cooking::units::{Length, Mass, Temperature, Volume};

#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    Length(Length),
    Mass(Mass),
    Temperature(Temperature),
    Volume(Volume),
}

impl UnitOperations for Unit {
    fn unit_type(&self) -> UnitType {
        match self {
            Unit::Length(unit) => unit.unit_type(),
            Unit::Mass(unit) => unit.unit_type(),
            Unit::Temperature(unit) => unit.unit_type(),
            Unit::Volume(unit) => unit.unit_type(),
        }
    }

    fn value(&self) -> f64 {
        match self {
            Unit::Length(unit) => unit.value(),
            Unit::Mass(unit) => unit.value(),
            Unit::Temperature(unit) => unit.value(),
            Unit::Volume(unit) => unit.value(),
        }
    }

    fn with_value(&self, value: f64) -> Self {
        match self {
            Unit::Length(unit) => Unit::Length(unit.with_value(value)),
            Unit::Mass(unit) => Unit::Mass(unit.with_value(value)),
            Unit::Temperature(unit) => Unit::Temperature(unit.with_value(value)),
            Unit::Volume(unit) => Unit::Volume(unit.with_value(value)),
        }
    }
}

impl UnitConverter for Unit {
    fn convert(&self, to: UnitType) -> crate::Result<Unit> {
        match self {
            Unit::Length(unit) => unit.convert(to),
            Unit::Mass(unit) => unit.convert(to),
            Unit::Temperature(unit) => unit.convert(to),
            Unit::Volume(unit) => unit.convert(to),
        }
    }
}

impl UnitScaler for Unit {
    /// Scales the unit by the given factor in the same measurement system.
    fn scale(&self, factor: f64) -> crate::Result<Unit> {
        if factor.is_sign_negative() {
            return Err(Error::InvalidScaleFactor(factor));
        }

        match self {
            Unit::Length(unit) => unit.scale(factor),
            Unit::Mass(unit) => unit.scale(factor),
            Unit::Temperature(unit) => unit.scale(factor),
            Unit::Volume(unit) => unit.scale(factor),
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
            assert_eq!(
                Unit::Temperature(Temperature::Fahrenheit(77.0)).value(),
                77.0
            );
            assert_eq!(
                Unit::Temperature(Temperature::Celsius(-10.5)).value(),
                -10.5
            );
            assert_eq!(
                Unit::Temperature(Temperature::Fahrenheit(32.0)).value(),
                32.0
            );
        }

        #[test]
        fn test_metric_volume_units() {
            assert_eq!(Unit::Volume(Volume::Millilitre(250.0)).value(), 250.0);
            assert_eq!(Unit::Volume(Volume::Centilitre(25.0)).value(), 25.0);
            assert_eq!(Unit::Volume(Volume::Decilitre(2.5)).value(), 2.5);
            assert_eq!(Unit::Volume(Volume::Litre(1.5)).value(), 1.5);
            assert_eq!(Unit::Volume(Volume::MetricTeaspoon(5.0)).value(), 5.0);
            assert_eq!(Unit::Volume(Volume::MetricTablespoon(15.0)).value(), 15.0);
            assert_eq!(Unit::Volume(Volume::MetricDessertspoon(10.0)).value(), 10.0);
            assert_eq!(Unit::Volume(Volume::MetricCup(250.0)).value(), 250.0);
        }

        #[test]
        fn test_imperial_volume_units() {
            assert_eq!(Unit::Volume(Volume::ImperialTeaspoon(1.0)).value(), 1.0);
            assert_eq!(Unit::Volume(Volume::ImperialDessertspoon(2.0)).value(), 2.0);
            assert_eq!(Unit::Volume(Volume::ImperialTablespoon(3.0)).value(), 3.0);
            assert_eq!(Unit::Volume(Volume::ImperialFluidOunce(4.0)).value(), 4.0);
            assert_eq!(Unit::Volume(Volume::ImperialGill(5.0)).value(), 5.0);
            assert_eq!(Unit::Volume(Volume::ImperialCup(8.0)).value(), 8.0);
            assert_eq!(Unit::Volume(Volume::ImperialPint(16.0)).value(), 16.0);
            assert_eq!(Unit::Volume(Volume::ImperialQuart(32.0)).value(), 32.0);
            assert_eq!(Unit::Volume(Volume::ImperialGallon(128.0)).value(), 128.0);
        }

        #[test]
        fn test_us_volume_units_value() {
            assert_eq!(Unit::Volume(Volume::USTeaspoon(4.93)).value(), 4.93);
            assert_eq!(Unit::Volume(Volume::USTablespoon(14.79)).value(), 14.79);
            assert_eq!(Unit::Volume(Volume::USFluidOunce(29.57)).value(), 29.57);
            assert_eq!(Unit::Volume(Volume::USCup(236.59)).value(), 236.59);
            assert_eq!(Unit::Volume(Volume::USPint(473.18)).value(), 473.18);
            assert_eq!(Unit::Volume(Volume::USQuart(946.35)).value(), 946.35);
            assert_eq!(Unit::Volume(Volume::USGallon(3785.41)).value(), 3785.41);
        }

        #[test]
        fn test_us_volume_units_with_value() {
            let us_units = vec![
                Unit::Volume(Volume::USTeaspoon(1.0)),
                Unit::Volume(Volume::USTablespoon(1.0)),
                Unit::Volume(Volume::USFluidOunce(1.0)),
                Unit::Volume(Volume::USCup(1.0)),
                Unit::Volume(Volume::USPint(1.0)),
                Unit::Volume(Volume::USQuart(1.0)),
                Unit::Volume(Volume::USGallon(1.0)),
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
            let aus_tbsp = Unit::Volume(Volume::AustralianTablespoon(20.0));
            assert_eq!(aus_tbsp.value(), 20.0);
            assert_eq!(aus_tbsp.with_value(25.0).value(), 25.0);

            // Jigger (cocktail measure)
            let jigger = Unit::Volume(Volume::Jigger(44.36));
            assert_eq!(jigger.value(), 44.36);
            assert_eq!(jigger.with_value(30.0).value(), 30.0);

            match aus_tbsp.with_value(15.0) {
                Unit::Volume(Volume::AustralianTablespoon(v)) => assert_eq!(v, 15.0),
                _ => panic!("Expected AustralianTablespoon unit"),
            }
            match jigger.with_value(45.0) {
                Unit::Volume(Volume::Jigger(v)) => assert_eq!(v, 45.0),
                _ => panic!("Expected Jigger unit"),
            }
        }

        #[test]
        fn test_zero_values() {
            let units = vec![
                Unit::Length(Length::Metre(0.0)),
                Unit::Mass(Mass::Kilogram(0.0)),
                Unit::Temperature(Temperature::Celsius(0.0)),
                Unit::Volume(Volume::Litre(0.0)),
                Unit::Volume(Volume::USCup(0.0)),
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
                Unit::Volume(Volume::Millilitre(large_value)),
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
            let millilitre = Unit::Volume(Volume::Millilitre(small_value));

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
            let original = Unit::Volume(Volume::Litre(5.0));
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

            match Unit::Mass(Mass::Kilogram(5.0).with_value(15.5)) {
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
                (Unit::Volume(Volume::Millilitre(1.0)), "Millilitre"),
                (Unit::Volume(Volume::Centilitre(1.0)), "Centilitre"),
                (Unit::Volume(Volume::Decilitre(1.0)), "Decilitre"),
                (Unit::Volume(Volume::Litre(1.0)), "Litre"),
                (Unit::Volume(Volume::MetricTeaspoon(1.0)), "MetricTeaspoon"),
                (
                    Unit::Volume(Volume::MetricTablespoon(1.0)),
                    "MetricTablespoon",
                ),
                (
                    Unit::Volume(Volume::MetricDessertspoon(1.0)),
                    "MetricDessertSpoon",
                ),
                (Unit::Volume(Volume::MetricCup(1.0)), "MetricCup"),
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
                Unit::Volume(Volume::ImperialTeaspoon(1.0)),
                Unit::Volume(Volume::ImperialDessertspoon(1.0)),
                Unit::Volume(Volume::ImperialTablespoon(1.0)),
                Unit::Volume(Volume::ImperialFluidOunce(1.0)),
                Unit::Volume(Volume::ImperialGill(1.0)),
                Unit::Volume(Volume::ImperialCup(1.0)),
                Unit::Volume(Volume::ImperialPint(1.0)),
                Unit::Volume(Volume::ImperialQuart(1.0)),
                Unit::Volume(Volume::ImperialGallon(1.0)),
            ];

            for (i, unit) in imperial_units.iter().enumerate() {
                let test_value = 10.0 + i as f64 * 0.5;
                let updated = unit.with_value(test_value);
                assert_eq!(updated.value(), test_value);
            }

            // Test specific unit preservation
            match Unit::Volume(Volume::ImperialPint(20.0)).with_value(568.26) {
                Unit::Volume(Volume::ImperialPint(v)) => assert_eq!(v, 568.26),
                _ => panic!("Expected ImperialPint unit"),
            }
        }
    }

    mod tests_unit_type {
        use super::*;
        use crate::cooking::units::volume::VolumeUnit::*;
        use crate::cooking::units::{LengthUnit, MassUnit, TemperatureUnit};

        #[test]
        fn test_length_units() {
            use crate::cooking::units::LengthUnit::*;

            assert_eq!(
                Unit::Length(Length::Millimetre(1.0)).unit_type(),
                UnitType::Length(Millimetre)
            );
            assert_eq!(
                Unit::Length(Length::Centimetre(2.5)).unit_type(),
                UnitType::Length(Centimetre)
            );
            assert_eq!(
                Unit::Length(Length::Metre(10.0)).unit_type(),
                UnitType::Length(Metre)
            );
            assert_eq!(
                Unit::Length(Length::Kilometre(100.0)).unit_type(),
                UnitType::Length(Kilometre)
            );
            assert_eq!(
                Unit::Length(Length::Inch(12.0)).unit_type(),
                UnitType::Length(Inch)
            );
            assert_eq!(
                Unit::Length(Length::Foot(3.0)).unit_type(),
                UnitType::Length(Foot)
            );
        }

        #[test]
        fn test_mass_units() {
            use crate::cooking::units::MassUnit::*;

            assert_eq!(
                Unit::Mass(Mass::Milligram(100.0)).unit_type(),
                UnitType::Mass(Milligram)
            );
            assert_eq!(
                Unit::Mass(Mass::Gram(500.0)).unit_type(),
                UnitType::Mass(Gram)
            );
            assert_eq!(
                Unit::Mass(Mass::Dekagram(1.5)).unit_type(),
                UnitType::Mass(Dekagram)
            );
            assert_eq!(
                Unit::Mass(Mass::Hectogram(0.8)).unit_type(),
                UnitType::Mass(Hectogram)
            );
            assert_eq!(
                Unit::Mass(Mass::Kilogram(2.3)).unit_type(),
                UnitType::Mass(Kilogram)
            );
            assert_eq!(
                Unit::Mass(Mass::Ounce(16.0)).unit_type(),
                UnitType::Mass(Ounce)
            );
            assert_eq!(
                Unit::Mass(Mass::Pound(2.2)).unit_type(),
                UnitType::Mass(Pound)
            );
        }

        #[test]
        fn test_temperature_units() {
            use crate::cooking::units::TemperatureUnit::*;

            assert_eq!(
                Unit::Temperature(Temperature::Celsius(25.0)).unit_type(),
                UnitType::Temperature(Celsius)
            );
            assert_eq!(
                Unit::Temperature(Temperature::Fahrenheit(77.0)).unit_type(),
                UnitType::Temperature(Fahrenheit)
            );
        }

        #[test]
        fn test_metric_volume_units() {
            assert_eq!(
                Unit::Volume(Volume::Millilitre(250.0)).unit_type(),
                UnitType::Volume(Millilitre)
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(10.0)).unit_type(),
                UnitType::Volume(Centilitre)
            );
            assert_eq!(
                Unit::Volume(Volume::Decilitre(5.0)).unit_type(),
                UnitType::Volume(Decilitre)
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(1.0)).unit_type(),
                UnitType::Volume(Litre)
            );
        }

        #[test]
        fn test_metric_cooking_units() {
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(1.0)).unit_type(),
                UnitType::Volume(MetricTeaspoon)
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(2.0)).unit_type(),
                UnitType::Volume(MetricTablespoon)
            );
            assert_eq!(
                Unit::Volume(Volume::MetricDessertspoon(1.5)).unit_type(),
                UnitType::Volume(MetricDessertSpoon)
            );
            assert_eq!(
                Unit::Volume(Volume::MetricCup(0.25)).unit_type(),
                UnitType::Volume(MetricCup)
            );
        }

        #[test]
        fn test_australian_units() {
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(3.0)).unit_type(),
                UnitType::Volume(AustralianTablespoon)
            );
        }

        #[test]
        fn test_imperial_volume_units() {
            assert_eq!(
                Unit::Volume(Volume::ImperialTeaspoon(4.0)).unit_type(),
                UnitType::Volume(ImperialTeaspoon)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialDessertspoon(2.0)).unit_type(),
                UnitType::Volume(ImperialDessertspoon)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialTablespoon(1.5)).unit_type(),
                UnitType::Volume(ImperialTablespoon)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialFluidOunce(8.0)).unit_type(),
                UnitType::Volume(ImperialFluidOunce)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialGill(0.5)).unit_type(),
                UnitType::Volume(ImperialGill)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialCup(2.0)).unit_type(),
                UnitType::Volume(ImperialCup)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialPint(1.0)).unit_type(),
                UnitType::Volume(ImperialPint)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialQuart(0.5)).unit_type(),
                UnitType::Volume(ImperialQuart)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialGallon(0.25)).unit_type(),
                UnitType::Volume(ImperialGallon)
            );
        }

        #[test]
        fn test_us_volume_units() {
            assert_eq!(
                Unit::Volume(Volume::USTeaspoon(6.0)).unit_type(),
                UnitType::Volume(USTeaspoon)
            );
            assert_eq!(
                Unit::Volume(Volume::USTablespoon(4.0)).unit_type(),
                UnitType::Volume(USTablespoon)
            );
            assert_eq!(
                Unit::Volume(Volume::USFluidOunce(8.0)).unit_type(),
                UnitType::Volume(USFluidOunce)
            );
            assert_eq!(
                Unit::Volume(Volume::USCup(2.0)).unit_type(),
                UnitType::Volume(USCup)
            );
            assert_eq!(
                Unit::Volume(Volume::USPint(1.0)).unit_type(),
                UnitType::Volume(USPint)
            );
            assert_eq!(
                Unit::Volume(Volume::USQuart(0.5)).unit_type(),
                UnitType::Volume(USQuart)
            );
            assert_eq!(
                Unit::Volume(Volume::USGallon(0.125)).unit_type(),
                UnitType::Volume(USGallon)
            );
        }

        #[test]
        fn test_bartending_units() {
            assert_eq!(
                Unit::Volume(Volume::Jigger(2.0)).unit_type(),
                UnitType::Volume(Jigger)
            );
        }

        #[test]
        fn test_zero_values() {
            assert_eq!(
                Unit::Length(Length::Millimetre(0.0)).unit_type(),
                UnitType::Length(LengthUnit::Millimetre)
            );
            assert_eq!(
                Unit::Mass(Mass::Gram(0.0)).unit_type(),
                UnitType::Mass(MassUnit::Gram)
            );
            assert_eq!(
                Unit::Temperature(Temperature::Celsius(0.0)).unit_type(),
                UnitType::Temperature(TemperatureUnit::Celsius)
            );
            assert_eq!(
                Unit::Volume(Volume::Millilitre(0.0)).unit_type(),
                UnitType::Volume(Millilitre)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialPint(0.0)).unit_type(),
                UnitType::Volume(ImperialPint)
            );
            assert_eq!(
                Unit::Volume(Volume::USCup(0.0)).unit_type(),
                UnitType::Volume(USCup)
            );
        }

        #[test]
        fn test_negative_values() {
            use LengthUnit::Metre;

            assert_eq!(
                Unit::Temperature(Temperature::Celsius(-10.0)).unit_type(),
                UnitType::Temperature(TemperatureUnit::Celsius)
            );
            assert_eq!(
                Unit::Temperature(Temperature::Fahrenheit(-5.0)).unit_type(),
                UnitType::Temperature(TemperatureUnit::Fahrenheit)
            );
            assert_eq!(
                Unit::Length(Length::Metre(-2.5)).unit_type(),
                UnitType::Length(Metre)
            );
        }

        #[test]
        fn test_large_values() {
            use crate::cooking::units::LengthUnit::Metre;
            use crate::cooking::units::MassUnit::Kilogram;

            assert_eq!(
                Unit::Mass(Mass::Kilogram(1000000.0)).unit_type(),
                UnitType::Mass(Kilogram)
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(999999.9)).unit_type(),
                UnitType::Volume(Litre)
            );
            assert_eq!(
                Unit::Length(Length::Metre(1e10)).unit_type(),
                UnitType::Length(Metre)
            );
        }

        #[test]
        fn test_small_fractional_values() {
            use crate::cooking::units::LengthUnit::Millimetre;
            use crate::cooking::units::MassUnit::Milligram;

            assert_eq!(
                Unit::Mass(Mass::Milligram(0.001)).unit_type(),
                UnitType::Mass(Milligram)
            );
            assert_eq!(
                Unit::Volume(Volume::Millilitre(0.0001)).unit_type(),
                UnitType::Volume(Millilitre)
            );
            assert_eq!(
                Unit::Length(Length::Millimetre(1e-6)).unit_type(),
                UnitType::Length(Millimetre)
            );
        }

        #[test]
        fn test_special_float_values() {
            use crate::cooking::units::LengthUnit::Metre;
            use crate::cooking::units::MassUnit::Gram;

            assert_eq!(
                Unit::Mass(Mass::Gram(f64::INFINITY)).unit_type(),
                UnitType::Mass(Gram)
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(f64::NEG_INFINITY)).unit_type(),
                UnitType::Volume(Litre)
            );
            assert_eq!(
                Unit::Length(Length::Metre(f64::NAN)).unit_type(),
                UnitType::Length(Metre)
            );
        }

        #[test]
        fn test_comprehensive_unit_coverage() {
            use crate::cooking::units::{LengthUnit::*, MassUnit::*};

            let test_cases: Vec<(Unit, UnitType)> = vec![
                // Length units
                (
                    Unit::Length(Length::Millimetre(1.0)),
                    UnitType::Length(Millimetre),
                ),
                (
                    Unit::Length(Length::Centimetre(1.0)),
                    UnitType::Length(Centimetre),
                ),
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
                (
                    Unit::Temperature(Temperature::Celsius(1.0)),
                    UnitType::Temperature(TemperatureUnit::Celsius),
                ),
                (
                    Unit::Temperature(Temperature::Fahrenheit(1.0)),
                    UnitType::Temperature(TemperatureUnit::Fahrenheit),
                ),
                // Metric volume units
                (
                    Unit::Volume(Volume::Millilitre(1.0)),
                    UnitType::Volume(Millilitre),
                ),
                (
                    Unit::Volume(Volume::Centilitre(1.0)),
                    UnitType::Volume(Centilitre),
                ),
                (
                    Unit::Volume(Volume::Decilitre(1.0)),
                    UnitType::Volume(Decilitre),
                ),
                (Unit::Volume(Volume::Litre(1.0)), UnitType::Volume(Litre)),
                // Metric cooking units
                (
                    Unit::Volume(Volume::MetricTeaspoon(1.0)),
                    UnitType::Volume(MetricTeaspoon),
                ),
                (
                    Unit::Volume(Volume::MetricTablespoon(1.0)),
                    UnitType::Volume(MetricTablespoon),
                ),
                (
                    Unit::Volume(Volume::MetricDessertspoon(1.0)),
                    UnitType::Volume(MetricDessertSpoon),
                ),
                (
                    Unit::Volume(Volume::MetricCup(1.0)),
                    UnitType::Volume(MetricCup),
                ),
                // Australian units
                (
                    Unit::Volume(Volume::AustralianTablespoon(1.0)),
                    UnitType::Volume(AustralianTablespoon),
                ),
                // Imperial volume units
                (
                    Unit::Volume(Volume::ImperialTeaspoon(1.0)),
                    UnitType::Volume(ImperialTeaspoon),
                ),
                (
                    Unit::Volume(Volume::ImperialDessertspoon(1.0)),
                    UnitType::Volume(ImperialDessertspoon),
                ),
                (
                    Unit::Volume(Volume::ImperialTablespoon(1.0)),
                    UnitType::Volume(ImperialTablespoon),
                ),
                (
                    Unit::Volume(Volume::ImperialFluidOunce(1.0)),
                    UnitType::Volume(ImperialFluidOunce),
                ),
                (
                    Unit::Volume(Volume::ImperialGill(1.0)),
                    UnitType::Volume(ImperialGill),
                ),
                (
                    Unit::Volume(Volume::ImperialCup(1.0)),
                    UnitType::Volume(ImperialCup),
                ),
                (
                    Unit::Volume(Volume::ImperialPint(1.0)),
                    UnitType::Volume(ImperialPint),
                ),
                (
                    Unit::Volume(Volume::ImperialQuart(1.0)),
                    UnitType::Volume(ImperialQuart),
                ),
                (
                    Unit::Volume(Volume::ImperialGallon(1.0)),
                    UnitType::Volume(ImperialGallon),
                ),
                // US volume units
                (
                    Unit::Volume(Volume::USTeaspoon(1.0)),
                    UnitType::Volume(USTeaspoon),
                ),
                (
                    Unit::Volume(Volume::USTablespoon(1.0)),
                    UnitType::Volume(USTablespoon),
                ),
                (
                    Unit::Volume(Volume::USFluidOunce(1.0)),
                    UnitType::Volume(USFluidOunce),
                ),
                (Unit::Volume(Volume::USCup(1.0)), UnitType::Volume(USCup)),
                (Unit::Volume(Volume::USPint(1.0)), UnitType::Volume(USPint)),
                (
                    Unit::Volume(Volume::USQuart(1.0)),
                    UnitType::Volume(USQuart),
                ),
                (
                    Unit::Volume(Volume::USGallon(1.0)),
                    UnitType::Volume(USGallon),
                ),
                // Bartending units
                (Unit::Volume(Volume::Jigger(1.0)), UnitType::Volume(Jigger)),
            ];

            for (unit, expected_type) in test_cases {
                assert_eq!(unit.unit_type(), expected_type, "Failed for unit: {unit:?}");
            }
        }

        #[test]
        fn test_unit_type_consistency() {
            let gram_values = vec![0.0, 1.0, -1.0, 100.5, 0.001, 1e6, f64::INFINITY];
            for value in gram_values {
                assert_eq!(
                    Unit::Mass(Mass::Gram(value)).unit_type(),
                    UnitType::Mass(MassUnit::Gram)
                );
            }

            let litre_values = vec![0.0, 0.5, 2.0, 10.25, 0.001, 999.99];
            for value in litre_values {
                assert_eq!(
                    Unit::Volume(Volume::Litre(value)).unit_type(),
                    UnitType::Volume(Litre)
                );
            }

            let celsius_values = vec![-273.15, 0.0, 25.0, 100.0, 1000.0];
            for value in celsius_values {
                assert_eq!(
                    Unit::Temperature(Temperature::Celsius(value)).unit_type(),
                    UnitType::Temperature(TemperatureUnit::Celsius)
                );
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
            Unit::Volume(Volume::Millilitre(1.0)),
            Unit::Volume(Volume::Centilitre(1.0)),
            Unit::Volume(Volume::Decilitre(1.0)),
            Unit::Volume(Volume::Litre(1.0)),
            Unit::Volume(Volume::MetricTeaspoon(1.0)),
            Unit::Volume(Volume::MetricTablespoon(1.0)),
            Unit::Volume(Volume::MetricDessertspoon(1.0)),
            Unit::Volume(Volume::MetricCup(1.0)),
            // Volume - Australian
            Unit::Volume(Volume::AustralianTablespoon(1.0)),
            // Volume - Imperial
            Unit::Volume(Volume::ImperialTeaspoon(1.0)),
            Unit::Volume(Volume::ImperialDessertspoon(1.0)),
            Unit::Volume(Volume::ImperialTablespoon(1.0)),
            Unit::Volume(Volume::ImperialFluidOunce(1.0)),
            Unit::Volume(Volume::ImperialGill(1.0)),
            Unit::Volume(Volume::ImperialCup(1.0)),
            Unit::Volume(Volume::ImperialPint(1.0)),
            Unit::Volume(Volume::ImperialQuart(1.0)),
            Unit::Volume(Volume::ImperialGallon(1.0)),
            // Volume - US
            Unit::Volume(Volume::USTeaspoon(1.0)),
            Unit::Volume(Volume::USTablespoon(1.0)),
            Unit::Volume(Volume::USFluidOunce(1.0)),
            Unit::Volume(Volume::USCup(1.0)),
            Unit::Volume(Volume::USPint(1.0)),
            Unit::Volume(Volume::USQuart(1.0)),
            Unit::Volume(Volume::USGallon(1.0)),
            // Special
            Unit::Volume(Volume::Jigger(1.0)),
        ];

        for (i, unit) in all_units.iter().enumerate() {
            let test_value = i as f64 + 0.5;
            assert_eq!(unit.value(), 1.0);
            assert_eq!(unit.with_value(test_value).value(), test_value);
        }
    }
}
