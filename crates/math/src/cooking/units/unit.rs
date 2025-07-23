use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use crate::cooking::units::traits::{UnitConverter, UnitOperations, UnitScaler};
use crate::cooking::units::{Length, Mass, Temperature, Unitless, Volume};
use crate::cooking::units::{UnitType, length, mass, temperature, unitless, volume};
use crate::{Error, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    Length(Length),
    Mass(Mass),
    Temperature(Temperature),
    Volume(Volume),
    Unitless(Unitless),
}

impl Unit {
    pub fn replace(&self, s: impl Into<String>) -> String {
        let re = match self {
            Unit::Length(_) => length::get_regex(),
            Unit::Mass(_) => mass::get_regex(),
            Unit::Temperature(_) => temperature::get_regex(),
            Unit::Volume(_) => volume::get_regex(),
            Unit::Unitless(_) => unitless::get_regex(),
        };

        let s: String = s.into();
        re.replace(&s, self.to_string()).to_string()
    }
}

impl UnitOperations for Unit {
    fn abbrev(&self) -> &str {
        match self {
            Unit::Length(u) => u.abbrev(),
            Unit::Mass(u) => u.abbrev(),
            Unit::Temperature(u) => u.abbrev(),
            Unit::Volume(u) => u.abbrev(),
            Unit::Unitless(u) => u.abbrev(),
        }
    }

    fn unit_type(&self) -> UnitType {
        match self {
            Unit::Length(u) => u.unit_type(),
            Unit::Mass(u) => u.unit_type(),
            Unit::Temperature(u) => u.unit_type(),
            Unit::Volume(u) => u.unit_type(),
            Unit::Unitless(u) => u.unit_type(),
        }
    }

    fn value(&self) -> f64 {
        match self {
            Unit::Length(unit) => unit.value(),
            Unit::Mass(unit) => unit.value(),
            Unit::Temperature(unit) => unit.value(),
            Unit::Volume(unit) => unit.value(),
            Unit::Unitless(u) => u.value(),
        }
    }

    fn with_value(&self, value: f64) -> Self {
        match self {
            Unit::Length(unit) => Unit::Length(unit.with_value(value)),
            Unit::Mass(unit) => Unit::Mass(unit.with_value(value)),
            Unit::Temperature(unit) => Unit::Temperature(unit.with_value(value)),
            Unit::Volume(unit) => Unit::Volume(unit.with_value(value)),
            Unit::Unitless(u) => Unit::Unitless(u.with_value(value)),
        }
    }
}

impl UnitConverter for Unit {
    fn convert(&self, to: UnitType) -> Result<Unit> {
        match self {
            Unit::Length(u) => u.convert(to),
            Unit::Mass(u) => u.convert(to),
            Unit::Temperature(u) => u.convert(to),
            Unit::Volume(u) => u.convert(to),
            Unit::Unitless(_) => Ok(self.clone()),
        }
    }
}

impl UnitScaler for Unit {
    /// Scales the unit by the given factor in the same measurement system.
    fn scale(&self, factor: f64) -> Result<Unit> {
        if factor.is_sign_negative() {
            return Err(Error::InvalidScaleFactor(factor));
        }

        match self {
            Unit::Length(u) => u.scale(factor),
            Unit::Mass(u) => u.scale(factor),
            Unit::Volume(u) => u.scale(factor),
            Unit::Temperature(_) => Err(Error::InvalidScale),
            Unit::Unitless(u) => u.scale(factor),
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let abbrev = self.abbrev();

        write!(
            f,
            "{}{}",
            format_fractional(self.value()),
            match self {
                Unit::Length(u) => match u {
                    Length::Inch(_) | Length::Foot(_) => abbrev.to_string(),
                    _ => format!(" {abbrev}"),
                },
                Unit::Temperature(_) => abbrev.to_string(),
                Unit::Volume(_) | Unit::Mass(_) => format!(" {abbrev}"),
                Unit::Unitless(_) => abbrev.to_string(),
            }
        )
    }
}

fn format_fractional(value: f64) -> String {
    let whole = value.trunc();
    let frac = value.fract();

    let denominators = [2, 3, 4, 8, 16];
    let mut best = None;
    let mut best_error = f64::MAX;

    for &den in denominators.iter() {
        let num = (frac * den as f64).round();
        let approx = num / den as f64;
        let error = (frac - approx).abs();

        if error < best_error {
            best = Some((num as u32, den));
            best_error = error;
        }
    }

    match best {
        Some((0, _)) => format!("{whole:.0}"),
        Some((num, den)) => {
            if whole == 0.0 {
                format!("{num}/{den}")
            } else {
                format!("{whole:.0} {num}/{den}")
            }
        }
        None => format!("{value:.2}"),
    }
}

impl FromStr for Unit {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok(v) = Volume::from_str(s) {
            Ok(Unit::Volume(v))
        } else if let Ok(v) = Mass::from_str(s) {
            Ok(Unit::Mass(v))
        } else if let Ok(v) = Length::from_str(s) {
            Ok(Unit::Length(v))
        } else if let Ok(v) = Temperature::from_str(s) {
            Ok(Unit::Temperature(v))
        } else {
            Ok(Unit::Unitless(Unitless::from_str(s)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_display {
        use super::*;
        use crate::cooking::units::Length::*;

        #[test]
        fn test_length() {
            assert_eq!(Unit::Length(Millimetre(100.0)).to_string(), "100 mm");
            assert_eq!(Unit::Length(Centimetre(50.0)).to_string(), "50 cm");
            assert_eq!(Unit::Length(Metre(3.0)).to_string(), "3 m");
            assert_eq!(Unit::Length(Kilometre(4.0)).to_string(), "4 km");
            assert_eq!(Unit::Length(Inch(1.0)).to_string(), "1\"");
            assert_eq!(Unit::Length(Foot(6.0)).to_string(), "6'");
        }

        #[test]
        fn test_temperature() {
            assert_eq!(
                Unit::Temperature(Temperature::Celsius(25.0)).to_string(),
                "25°C"
            );
            assert_eq!(
                Unit::Temperature(Temperature::Fahrenheit(77.0)).to_string(),
                "77°F"
            );
        }

        #[test]
        fn test_volume() {
            assert_eq!(
                Unit::Volume(Volume::Millilitre(250.0)).to_string(),
                "250 ml"
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(250.0)).to_string(),
                "250 cl"
            );
            assert_eq!(Unit::Volume(Volume::Decilitre(250.0)).to_string(), "250 dl");
            assert_eq!(Unit::Volume(Volume::Litre(1.0)).to_string(), "1 l");

            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(1.0)).to_string(),
                "1 tsp"
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianTeaspoon(1.0)).to_string(),
                "1 tsp"
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialTeaspoon(1.0)).to_string(),
                "1 tsp"
            );
            assert_eq!(Unit::Volume(Volume::USTeaspoon(1.0)).to_string(), "1 tsp");

            assert_eq!(
                Unit::Volume(Volume::MetricDessertspoon(1.0)).to_string(),
                "1 dsp"
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianDessertspoon(1.0)).to_string(),
                "1 dsp"
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialDessertspoon(1.0)).to_string(),
                "1 dsp"
            );

            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(1.0)).to_string(),
                "1 tbsp"
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(1.0)).to_string(),
                "1 tbsp"
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialTablespoon(1.0)).to_string(),
                "1 tbsp"
            );
            assert_eq!(
                Unit::Volume(Volume::USTablespoon(1.0)).to_string(),
                "1 tbsp"
            );

            assert_eq!(Unit::Volume(Volume::MetricCup(1.0)).to_string(), "1 cup");
            assert_eq!(
                Unit::Volume(Volume::AustralianCup(1.0)).to_string(),
                "1 cup"
            );
            assert_eq!(Unit::Volume(Volume::ImperialCup(1.0)).to_string(), "1 cup");
            assert_eq!(Unit::Volume(Volume::USCup(1.0)).to_string(), "1 cup");

            assert_eq!(
                Unit::Volume(Volume::ImperialFluidOunce(1.0)).to_string(),
                "1 fl oz"
            );
            assert_eq!(
                Unit::Volume(Volume::USFluidOunce(1.0)).to_string(),
                "1 fl oz"
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialGill(1.0)).to_string(),
                "1 gill"
            );
            assert_eq!(Unit::Volume(Volume::ImperialPint(1.0)).to_string(), "1 pt");
            assert_eq!(Unit::Volume(Volume::USPint(1.0)).to_string(), "1 pt");
            assert_eq!(Unit::Volume(Volume::ImperialQuart(1.0)).to_string(), "1 qt");
            assert_eq!(Unit::Volume(Volume::USQuart(1.0)).to_string(), "1 qt");
            assert_eq!(
                Unit::Volume(Volume::ImperialGallon(1.0)).to_string(),
                "1 gal"
            );
            assert_eq!(Unit::Volume(Volume::USGallon(1.0)).to_string(), "1 gal");
            assert_eq!(Unit::Volume(Volume::Jigger(1.0)).to_string(), "1 jig");
        }

        #[test]
        fn test_mass_display() {
            assert_eq!(Unit::Mass(Mass::Gram(500.0)).to_string(), "500 g");
            assert_eq!(Unit::Mass(Mass::Kilogram(3.0)).to_string(), "3 kg");
            assert_eq!(Unit::Mass(Mass::Pound(1.0)).to_string(), "1 lb");
            assert_eq!(Unit::Mass(Mass::Ounce(8.0)).to_string(), "8 oz");
            assert_eq!(Unit::Mass(Mass::Milligram(750.0)).to_string(), "750 mg");
        }

        #[test]
        fn test_fractions() {
            assert_eq!(Unit::Length(Millimetre(1.125)).to_string(), "1 1/8 mm");
            assert_eq!(Unit::Length(Centimetre(1.25)).to_string(), "1 1/4 cm");
            assert_eq!(Unit::Length(Metre(1.5)).to_string(), "1 1/2 m");
            assert_eq!(Unit::Length(Kilometre(1.75)).to_string(), "1 3/4 km");
        }

        #[test]
        fn test_unitless() {
            assert_eq!(Unit::Unitless(Unitless::new(4.0)).to_string(), "4");
            assert_eq!(Unit::Unitless(Unitless::new(1.5)).to_string(), "1 1/2");
        }
    }

    mod tests_replace {
        use super::*;
        use Length::*;
        use Mass::*;
        use Temperature::*;
        use Volume::*;

        fn assert_replace(s: &str, unit: Unit, expected: &str) {
            let got = unit.replace(s);
            assert_eq!(got, expected);
        }

        #[test]
        fn test_length() {
            let s = "1 mg of grandma's smoky bacon";
            assert_replace(
                s,
                Unit::Length(Millimetre(1.0)),
                "1 mm of grandma's smoky bacon",
            );
            assert_replace(
                s,
                Unit::Length(Centimetre(1.0)),
                "1 cm of grandma's smoky bacon",
            );
            assert_replace(s, Unit::Length(Metre(1.0)), "1 m of grandma's smoky bacon");
            assert_replace(
                s,
                Unit::Length(Kilometre(1.0)),
                "1 km of grandma's smoky bacon",
            );
            assert_replace(s, Unit::Length(Inch(1.0)), "1\" of grandma's smoky bacon");
            assert_replace(s, Unit::Length(Foot(1.0)), "1' of grandma's smoky bacon");
        }

        #[test]
        fn test_mass() {
            let s = "1 mg of grandma's clear water";
            assert_replace(
                s,
                Unit::Mass(Milligram(2.0)),
                "2 mg of grandma's clear water",
            );
            assert_replace(s, Unit::Mass(Gram(1.0)), "1 g of grandma's clear water");
            assert_replace(
                s,
                Unit::Mass(Dekagram(1.0)),
                "1 dag of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Mass(Hectogram(1.0)),
                "1 hg of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Mass(Kilogram(1.0)),
                "1 kg of grandma's clear water",
            );
            assert_replace(s, Unit::Mass(Ounce(1.0)), "1 oz of grandma's clear water");
            assert_replace(s, Unit::Mass(Pound(1.0)), "1 lb of grandma's clear water");
        }

        #[test]
        fn test_temperature() {
            let s = "Heat the oven to 100 degrees celsius";
            assert_replace(
                s,
                Unit::Temperature(Celsius(150.0)),
                "Heat the oven to 150°C",
            );
            assert_replace(
                s,
                Unit::Temperature(Fahrenheit(352.0)),
                "Heat the oven to 352°F",
            );
        }

        #[test]
        fn test_volume() {
            let s = "1 mg of grandma's clear water";
            assert_replace(
                s,
                Unit::Volume(Millilitre(1.0)),
                "1 ml of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(Centilitre(1.0)),
                "1 cl of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(Decilitre(1.0)),
                "1 dl of grandma's clear water",
            );
            assert_replace(s, Unit::Volume(Litre(1.0)), "1 l of grandma's clear water");
            assert_replace(
                s,
                Unit::Volume(MetricTeaspoon(1.0)),
                "1 tsp of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(MetricTablespoon(1.0)),
                "1 tbsp of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(MetricDessertspoon(1.0)),
                "1 dsp of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(MetricCup(1.0)),
                "1 cup of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(AustralianTeaspoon(1.0)),
                "1 tsp of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(AustralianDessertspoon(1.0)),
                "1 dsp of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(AustralianTablespoon(1.0)),
                "1 tbsp of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(AustralianCup(1.0)),
                "1 cup of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(ImperialTeaspoon(1.0)),
                "1 tsp of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(ImperialDessertspoon(1.0)),
                "1 dsp of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(ImperialTablespoon(1.0)),
                "1 tbsp of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(ImperialFluidOunce(1.0)),
                "1 fl oz of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(ImperialGill(1.0)),
                "1 gill of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(ImperialCup(1.0)),
                "1 cup of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(ImperialPint(1.0)),
                "1 pt of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(ImperialQuart(1.0)),
                "1 qt of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(ImperialGallon(1.0)),
                "1 gal of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(USTeaspoon(1.0)),
                "1 tsp of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(USTablespoon(1.0)),
                "1 tbsp of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(USFluidOunce(1.0)),
                "1 fl oz of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(USCup(1.0)),
                "1 cup of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(USPint(1.0)),
                "1 pt of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(USQuart(1.0)),
                "1 qt of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(USGallon(1.0)),
                "1 gal of grandma's clear water",
            );
            assert_replace(
                s,
                Unit::Volume(Jigger(1.0)),
                "1 jig of grandma's clear water",
            );
        }

        #[test]
        fn test_unitless() {
            let s = "4 big apples";
            assert_replace(s, Unit::Unitless(Unitless::new(8.0)), "8 big apples");
        }
    }

    mod tests_abbrev {
        use super::*;

        #[test]
        fn test_length() {
            assert_eq!(Unit::Length(Length::Millimetre(1.0)).abbrev(), "mm");
        }

        #[test]
        fn test_mass() {
            assert_eq!(Unit::Mass(Mass::Gram(1.0)).abbrev(), "g");
        }

        #[test]
        fn test_temperature() {
            assert_eq!(
                Unit::Temperature(Temperature::Celsius(165.0)).abbrev(),
                "°C"
            )
        }

        #[test]
        fn test_unitless() {
            assert_eq!(Unit::Unitless(Unitless::new(4.0)).abbrev(), "");
        }
    }

    mod tests_value {
        use super::*;

        #[test]
        fn test_length() {
            assert_eq!(Unit::Length(Length::Millimetre(5.5)).value(), 5.5);
            assert_eq!(Unit::Length(Length::Centimetre(10.0)).value(), 10.0);
            assert_eq!(Unit::Length(Length::Metre(2.5)).value(), 2.5);
            assert_eq!(Unit::Length(Length::Inch(12.25)).value(), 12.25);
            assert_eq!(Unit::Length(Length::Foot(3.75)).value(), 3.75);
        }

        #[test]
        fn test_mass() {
            assert_eq!(Unit::Mass(Mass::Milligram(100.0)).value(), 100.0);
            assert_eq!(Unit::Mass(Mass::Gram(50.5)).value(), 50.5);
            assert_eq!(Unit::Mass(Mass::Dekagram(10.25)).value(), 10.25);
            assert_eq!(Unit::Mass(Mass::Hectogram(5.75)).value(), 5.75);
            assert_eq!(Unit::Mass(Mass::Kilogram(2.3)).value(), 2.3);
            assert_eq!(Unit::Mass(Mass::Ounce(16.5)).value(), 16.5);
            assert_eq!(Unit::Mass(Mass::Pound(3.2)).value(), 3.2);
        }

        #[test]
        fn test_temperature() {
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
        fn test_volume() {
            assert_eq!(Unit::Volume(Volume::Millilitre(250.0)).value(), 250.0);
            assert_eq!(Unit::Volume(Volume::Centilitre(25.0)).value(), 25.0);
            assert_eq!(Unit::Volume(Volume::Decilitre(2.5)).value(), 2.5);
            assert_eq!(Unit::Volume(Volume::Litre(1.5)).value(), 1.5);
            assert_eq!(Unit::Volume(Volume::MetricTeaspoon(5.0)).value(), 5.0);
            assert_eq!(Unit::Volume(Volume::MetricTablespoon(15.0)).value(), 15.0);
            assert_eq!(Unit::Volume(Volume::MetricDessertspoon(10.0)).value(), 10.0);
            assert_eq!(Unit::Volume(Volume::MetricCup(250.0)).value(), 250.0);

            assert_eq!(Unit::Volume(Volume::ImperialTeaspoon(1.0)).value(), 1.0);
            assert_eq!(Unit::Volume(Volume::ImperialDessertspoon(2.0)).value(), 2.0);
            assert_eq!(Unit::Volume(Volume::ImperialTablespoon(3.0)).value(), 3.0);
            assert_eq!(Unit::Volume(Volume::ImperialFluidOunce(4.0)).value(), 4.0);
            assert_eq!(Unit::Volume(Volume::ImperialGill(5.0)).value(), 5.0);
            assert_eq!(Unit::Volume(Volume::ImperialCup(8.0)).value(), 8.0);
            assert_eq!(Unit::Volume(Volume::ImperialPint(16.0)).value(), 16.0);
            assert_eq!(Unit::Volume(Volume::ImperialQuart(32.0)).value(), 32.0);
            assert_eq!(Unit::Volume(Volume::ImperialGallon(128.0)).value(), 128.0);

            assert_eq!(Unit::Volume(Volume::USTeaspoon(4.93)).value(), 4.93);
            assert_eq!(Unit::Volume(Volume::USTablespoon(14.79)).value(), 14.79);
            assert_eq!(Unit::Volume(Volume::USFluidOunce(29.57)).value(), 29.57);
            assert_eq!(Unit::Volume(Volume::USCup(236.59)).value(), 236.59);
            assert_eq!(Unit::Volume(Volume::USPint(473.18)).value(), 473.18);
            assert_eq!(Unit::Volume(Volume::USQuart(946.35)).value(), 946.35);
            assert_eq!(Unit::Volume(Volume::USGallon(3785.41)).value(), 3785.41);
        }
    }

    mod tests_with_value {
        use super::*;

        #[test]
        fn test_length() {
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
        fn test_mass() {
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
        }

        #[test]
        fn test_temperature_units() {
            let celsius = Unit::Temperature(Temperature::Celsius(20.0));
            let fahrenheit = Unit::Temperature(Temperature::Fahrenheit(68.0));

            assert_eq!(celsius.with_value(100.0).value(), 100.0);
            assert_eq!(fahrenheit.with_value(212.0).value(), 212.0);
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
        fn test_volume() {
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
            match Unit::Volume(Volume::ImperialPint(20.0)).with_value(568.26) {
                Unit::Volume(Volume::ImperialPint(v)) => assert_eq!(v, 568.26),
                _ => panic!("Expected ImperialPint unit"),
            }
        }

        #[test]
        fn test_unitless() {
            let unit = Unit::Unitless(Unitless::new(1.0));
            let new_value = 2.0;

            let got = unit.with_value(new_value);

            assert_eq!(got.value(), new_value);
        }
    }

    mod tests_unit_type {
        use super::*;

        #[test]
        fn test_length() {
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
        fn test_mass() {
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
        fn test_temperature() {
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
        fn test_volume() {
            use crate::cooking::units::VolumeUnit::*;

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
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(3.0)).unit_type(),
                UnitType::Volume(AustralianTablespoon)
            );
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
            assert_eq!(
                Unit::Volume(Volume::Jigger(2.0)).unit_type(),
                UnitType::Volume(Jigger)
            );
        }

        #[test]
        fn test_unitless() {
            assert_eq!(
                Unit::Unitless(Unitless::new(25.0)).unit_type(),
                UnitType::Unitless
            );
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
