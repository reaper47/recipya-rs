use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use crate::cooking::units::length::units::Length;
use crate::cooking::units::mass::units::Mass;
use crate::cooking::units::temperature::units::Temperature;
use crate::cooking::units::traits::{UnitConverter, UnitOperations, UnitScaler};
use crate::cooking::units::unitless::units::Unitless;
use crate::cooking::units::volume::units::Volume;
use crate::cooking::units::{UnitType, length, mass, temperature, unitless, volume};
use crate::{Error, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
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
            Self::Length(_) => length::units::get_regex(),
            Self::Mass(_) => mass::units::get_regex(),
            Self::Temperature(_) => temperature::units::get_regex(),
            Self::Volume(_) => volume::units::get_regex(),
            Self::Unitless(_) => unitless::units::get_regex(),
        };

        let s: String = s.into();
        re.replace(&s, self.to_string()).to_string()
    }
}

impl UnitOperations for Unit {
    fn abbrev(&self) -> &str {
        match self {
            Self::Length(u) => u.abbrev(),
            Self::Mass(u) => u.abbrev(),
            Self::Temperature(u) => u.abbrev(),
            Self::Volume(u) => u.abbrev(),
            Self::Unitless(u) => u.abbrev(),
        }
    }

    fn unit_type(&self) -> UnitType {
        match self {
            Self::Length(u) => u.unit_type(),
            Self::Mass(u) => u.unit_type(),
            Self::Temperature(u) => u.unit_type(),
            Self::Volume(u) => u.unit_type(),
            Self::Unitless(u) => u.unit_type(),
        }
    }

    fn value(&self) -> f64 {
        match self {
            Self::Length(unit) => unit.value(),
            Self::Mass(unit) => unit.value(),
            Self::Temperature(unit) => unit.value(),
            Self::Volume(unit) => unit.value(),
            Self::Unitless(u) => u.value(),
        }
    }

    fn with_value(&self, value: f64) -> Self {
        match self {
            Self::Length(unit) => Self::Length(unit.with_value(value)),
            Self::Mass(unit) => Self::Mass(unit.with_value(value)),
            Self::Temperature(unit) => Self::Temperature(unit.with_value(value)),
            Self::Volume(unit) => Self::Volume(unit.with_value(value)),
            Self::Unitless(u) => Self::Unitless(u.with_value(value)),
        }
    }
}

impl UnitConverter for Unit {
    fn convert(&self, to: UnitType) -> Result<Unit> {
        match self {
            Self::Length(u) => u.convert(to),
            Self::Mass(u) => u.convert(to),
            Self::Temperature(u) => u.convert(to),
            Self::Volume(u) => u.convert(to),
            Self::Unitless(_) => Ok(*self),
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
            Self::Length(u) => u.scale(factor),
            Self::Mass(u) => u.scale(factor),
            Self::Volume(u) => u.scale(factor),
            Self::Temperature(_) => Err(Error::InvalidScale),
            Self::Unitless(u) => u.scale(factor),
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
                Self::Length(u) => match u {
                    Length::Inch(_) | Length::Foot(_) => abbrev.to_string(),
                    _ => format!(" {abbrev}"),
                },
                Self::Temperature(_) | Self::Unitless(_) => abbrev.to_string(),
                Self::Volume(_) | Self::Mass(_) => format!(" {abbrev}"),
            }
        )
    }
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn format_fractional(value: f64) -> String {
    let whole = value.trunc();
    let frac = value.fract();

    let denominators = [2, 3, 4, 8, 16];
    let mut best = None;
    let mut best_error = f64::MAX;

    for &den in &denominators {
        let num = (frac * f64::from(den)).round();
        let approx = num / f64::from(den);
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
            Ok(Self::Volume(v))
        } else if let Ok(v) = Mass::from_str(s) {
            Ok(Self::Mass(v))
        } else if let Ok(v) = Length::from_str(s) {
            Ok(Self::Length(v))
        } else if let Ok(v) = Temperature::from_str(s) {
            Ok(Self::Temperature(v))
        } else {
            Ok(Self::Unitless(Unitless::from_str(s)?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Length::*;
    use Mass::*;
    use Temperature::*;
    use Volume::*;
    use approx::assert_relative_eq;

    mod tests_display {
        use super::*;

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
        #[allow(clippy::too_many_lines)]
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
            );
        }

        #[test]
        fn test_unitless() {
            assert_eq!(Unit::Unitless(Unitless::new(4.0)).abbrev(), "");
        }
    }

    mod tests_value {
        use std::f64;

        use super::*;

        #[test]
        fn test_length() {
            assert_relative_eq!(
                Unit::Length(Length::Millimetre(5.5)).value(),
                5.5,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Length(Length::Centimetre(10.0)).value(),
                10.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Length(Length::Metre(2.5)).value(),
                2.5,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Length(Length::Inch(12.25)).value(),
                12.25,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Length(Length::Foot(3.75)).value(),
                3.75,
                epsilon = f64::EPSILON
            );
        }

        #[test]
        fn test_mass() {
            assert_relative_eq!(
                Unit::Mass(Mass::Milligram(100.0)).value(),
                100.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Mass(Mass::Gram(50.5)).value(),
                50.5,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Mass(Mass::Dekagram(10.25)).value(),
                10.25,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Mass(Mass::Hectogram(5.75)).value(),
                5.75,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Mass(Mass::Kilogram(2.3)).value(),
                2.3,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Mass(Mass::Ounce(16.5)).value(),
                16.5,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Mass(Mass::Pound(3.2)).value(),
                3.2,
                epsilon = f64::EPSILON
            );
        }

        #[test]
        fn test_temperature() {
            assert_relative_eq!(
                Unit::Temperature(Temperature::Celsius(25.0)).value(),
                25.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Temperature(Temperature::Fahrenheit(77.0)).value(),
                77.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Temperature(Temperature::Celsius(-10.5)).value(),
                -10.5,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Temperature(Temperature::Fahrenheit(32.0)).value(),
                32.0,
                epsilon = f64::EPSILON
            );
        }

        #[test]
        #[allow(clippy::too_many_lines)]
        fn test_volume() {
            assert_relative_eq!(
                Unit::Volume(Volume::Millilitre(250.0)).value(),
                250.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::Centilitre(25.0)).value(),
                25.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::Decilitre(2.5)).value(),
                2.5,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::Litre(1.5)).value(),
                1.5,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::MetricTeaspoon(5.0)).value(),
                5.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::MetricTablespoon(15.0)).value(),
                15.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::MetricDessertspoon(10.0)).value(),
                10.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::MetricCup(250.0)).value(),
                250.0,
                epsilon = f64::EPSILON
            );

            assert_relative_eq!(
                Unit::Volume(Volume::ImperialTeaspoon(1.0)).value(),
                1.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::ImperialDessertspoon(2.0)).value(),
                2.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::ImperialTablespoon(3.0)).value(),
                3.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::ImperialFluidOunce(4.0)).value(),
                4.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::ImperialGill(5.0)).value(),
                5.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::ImperialCup(8.0)).value(),
                8.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::ImperialPint(16.0)).value(),
                16.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::ImperialQuart(32.0)).value(),
                32.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::ImperialGallon(128.0)).value(),
                128.0,
                epsilon = f64::EPSILON
            );

            assert_relative_eq!(
                Unit::Volume(Volume::USTeaspoon(4.93)).value(),
                4.93,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::USTablespoon(14.79)).value(),
                14.79,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::USFluidOunce(29.57)).value(),
                29.57,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::USCup(236.59)).value(),
                236.59,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::USPint(473.18)).value(),
                473.18,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::USQuart(946.35)).value(),
                946.35,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                Unit::Volume(Volume::USGallon(3785.41)).value(),
                3785.41,
                epsilon = f64::EPSILON
            );
        }
    }

    mod tests_with_value {
        use std::f64;

        use super::*;

        #[test]
        fn test_length() {
            let mm = Unit::Length(Length::Millimetre(1.0));
            let cm = Unit::Length(Length::Centimetre(2.0));
            let m = Unit::Length(Length::Metre(3.0));
            let inch = Unit::Length(Length::Inch(4.0));
            let foot = Unit::Length(Length::Foot(5.0));

            assert_relative_eq!(mm.with_value(10.5).value(), 10.5, epsilon = f64::EPSILON);
            assert_relative_eq!(cm.with_value(20.5).value(), 20.5, epsilon = f64::EPSILON);
            assert_relative_eq!(m.with_value(30.5).value(), 30.5, epsilon = f64::EPSILON);
            assert_relative_eq!(inch.with_value(40.5).value(), 40.5, epsilon = f64::EPSILON);
            assert_relative_eq!(foot.with_value(50.5).value(), 50.5, epsilon = f64::EPSILON);
        }

        #[test]
        #[allow(clippy::cast_precision_loss)]
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
                assert_relative_eq!(updated_unit.value(), new_value, epsilon = f64::EPSILON);
            }
        }

        #[test]
        fn test_temperature_units() {
            let celsius = Unit::Temperature(Temperature::Celsius(20.0));
            let fahrenheit = Unit::Temperature(Temperature::Fahrenheit(68.0));

            assert_relative_eq!(
                celsius.with_value(100.0).value(),
                100.0,
                epsilon = f64::EPSILON
            );
            assert_relative_eq!(
                fahrenheit.with_value(212.0).value(),
                212.0,
                epsilon = f64::EPSILON
            );
        }

        #[test]
        fn test_chained_operations() {
            let unit = Unit::Mass(Mass::Gram(1.0));
            let result = unit.with_value(2.0).with_value(3.0).with_value(4.0);

            assert_relative_eq!(result.value(), 4.0, epsilon = f64::EPSILON);
            match result {
                Unit::Mass(Mass::Gram(v)) => assert_relative_eq!(v, 4.0, epsilon = f64::EPSILON),
                _ => panic!("Expected Gram unit"),
            }
        }

        #[test]
        #[allow(clippy::cast_precision_loss)]
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
            for (unit, _) in metric_units {
                let new_value = 42.5;
                let updated = unit.with_value(new_value);
                assert_relative_eq!(updated.value(), new_value, epsilon = f64::EPSILON);
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
                let test_value = (i as f64).mul_add(0.5, 10.0);
                let updated = unit.with_value(test_value);
                assert_relative_eq!(updated.value(), test_value, epsilon = f64::EPSILON);
            }
            match Unit::Volume(Volume::ImperialPint(20.0)).with_value(568.26) {
                Unit::Volume(Volume::ImperialPint(v)) => {
                    assert_relative_eq!(v, 568.26, epsilon = f64::EPSILON);
                }
                _ => panic!("Expected ImperialPint unit"),
            }
        }

        #[test]
        fn test_unitless() {
            let unit = Unit::Unitless(Unitless::new(1.0));
            let new_value = 2.0;

            let got = unit.with_value(new_value);

            assert_relative_eq!(got.value(), new_value, epsilon = f64::EPSILON);
        }
    }

    mod tests_unit_type {
        use super::*;
        use crate::cooking::units::length::units::LengthUnit;
        use crate::cooking::units::mass::units::MassUnit;
        use crate::cooking::units::temperature::units::TemperatureUnit;
        use crate::cooking::units::volume::units::VolumeUnit;

        #[test]
        fn test_length() {
            assert_eq!(
                Unit::Length(Length::Millimetre(1.0)).unit_type(),
                UnitType::Length(LengthUnit::Millimetre)
            );
            assert_eq!(
                Unit::Length(Length::Centimetre(2.5)).unit_type(),
                UnitType::Length(LengthUnit::Centimetre)
            );
            assert_eq!(
                Unit::Length(Length::Metre(10.0)).unit_type(),
                UnitType::Length(LengthUnit::Metre)
            );
            assert_eq!(
                Unit::Length(Length::Kilometre(100.0)).unit_type(),
                UnitType::Length(LengthUnit::Kilometre)
            );
            assert_eq!(
                Unit::Length(Length::Inch(12.0)).unit_type(),
                UnitType::Length(LengthUnit::Inch)
            );
            assert_eq!(
                Unit::Length(Length::Foot(3.0)).unit_type(),
                UnitType::Length(LengthUnit::Foot)
            );
        }

        #[test]
        fn test_mass() {
            assert_eq!(
                Unit::Mass(Mass::Milligram(100.0)).unit_type(),
                UnitType::Mass(MassUnit::Milligram)
            );
            assert_eq!(
                Unit::Mass(Mass::Gram(500.0)).unit_type(),
                UnitType::Mass(MassUnit::Gram)
            );
            assert_eq!(
                Unit::Mass(Mass::Dekagram(1.5)).unit_type(),
                UnitType::Mass(MassUnit::Dekagram)
            );
            assert_eq!(
                Unit::Mass(Mass::Hectogram(0.8)).unit_type(),
                UnitType::Mass(MassUnit::Hectogram)
            );
            assert_eq!(
                Unit::Mass(Mass::Kilogram(2.3)).unit_type(),
                UnitType::Mass(MassUnit::Kilogram)
            );
            assert_eq!(
                Unit::Mass(Mass::Ounce(16.0)).unit_type(),
                UnitType::Mass(MassUnit::Ounce)
            );
            assert_eq!(
                Unit::Mass(Mass::Pound(2.2)).unit_type(),
                UnitType::Mass(MassUnit::Pound)
            );
        }

        #[test]
        fn test_temperature() {
            assert_eq!(
                Unit::Temperature(Temperature::Celsius(25.0)).unit_type(),
                UnitType::Temperature(TemperatureUnit::Celsius)
            );
            assert_eq!(
                Unit::Temperature(Temperature::Fahrenheit(77.0)).unit_type(),
                UnitType::Temperature(TemperatureUnit::Fahrenheit)
            );
        }

        #[test]
        #[allow(clippy::too_many_lines)]
        fn test_volume() {
            assert_eq!(
                Unit::Volume(Volume::Millilitre(250.0)).unit_type(),
                UnitType::Volume(VolumeUnit::Millilitre)
            );
            assert_eq!(
                Unit::Volume(Volume::Centilitre(10.0)).unit_type(),
                UnitType::Volume(VolumeUnit::Centilitre)
            );
            assert_eq!(
                Unit::Volume(Volume::Decilitre(5.0)).unit_type(),
                UnitType::Volume(VolumeUnit::Decilitre)
            );
            assert_eq!(
                Unit::Volume(Volume::Litre(1.0)).unit_type(),
                UnitType::Volume(VolumeUnit::Litre)
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTeaspoon(1.0)).unit_type(),
                UnitType::Volume(VolumeUnit::MetricTeaspoon)
            );
            assert_eq!(
                Unit::Volume(Volume::MetricTablespoon(2.0)).unit_type(),
                UnitType::Volume(VolumeUnit::MetricTablespoon)
            );
            assert_eq!(
                Unit::Volume(Volume::MetricDessertspoon(1.5)).unit_type(),
                UnitType::Volume(VolumeUnit::MetricDessertSpoon)
            );
            assert_eq!(
                Unit::Volume(Volume::MetricCup(0.25)).unit_type(),
                UnitType::Volume(VolumeUnit::MetricCup)
            );
            assert_eq!(
                Unit::Volume(Volume::AustralianTablespoon(3.0)).unit_type(),
                UnitType::Volume(VolumeUnit::AustralianTablespoon)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialTeaspoon(4.0)).unit_type(),
                UnitType::Volume(VolumeUnit::ImperialTeaspoon)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialDessertspoon(2.0)).unit_type(),
                UnitType::Volume(VolumeUnit::ImperialDessertspoon)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialTablespoon(1.5)).unit_type(),
                UnitType::Volume(VolumeUnit::ImperialTablespoon)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialFluidOunce(8.0)).unit_type(),
                UnitType::Volume(VolumeUnit::ImperialFluidOunce)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialGill(0.5)).unit_type(),
                UnitType::Volume(VolumeUnit::ImperialGill)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialCup(2.0)).unit_type(),
                UnitType::Volume(VolumeUnit::ImperialCup)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialPint(1.0)).unit_type(),
                UnitType::Volume(VolumeUnit::ImperialPint)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialQuart(0.5)).unit_type(),
                UnitType::Volume(VolumeUnit::ImperialQuart)
            );
            assert_eq!(
                Unit::Volume(Volume::ImperialGallon(0.25)).unit_type(),
                UnitType::Volume(VolumeUnit::ImperialGallon)
            );
            assert_eq!(
                Unit::Volume(Volume::USTeaspoon(6.0)).unit_type(),
                UnitType::Volume(VolumeUnit::USTeaspoon)
            );
            assert_eq!(
                Unit::Volume(Volume::USTablespoon(4.0)).unit_type(),
                UnitType::Volume(VolumeUnit::USTablespoon)
            );
            assert_eq!(
                Unit::Volume(Volume::USFluidOunce(8.0)).unit_type(),
                UnitType::Volume(VolumeUnit::USFluidOunce)
            );
            assert_eq!(
                Unit::Volume(Volume::USCup(2.0)).unit_type(),
                UnitType::Volume(VolumeUnit::USCup)
            );
            assert_eq!(
                Unit::Volume(Volume::USPint(1.0)).unit_type(),
                UnitType::Volume(VolumeUnit::USPint)
            );
            assert_eq!(
                Unit::Volume(Volume::USQuart(0.5)).unit_type(),
                UnitType::Volume(VolumeUnit::USQuart)
            );
            assert_eq!(
                Unit::Volume(Volume::USGallon(0.125)).unit_type(),
                UnitType::Volume(VolumeUnit::USGallon)
            );
            assert_eq!(
                Unit::Volume(Volume::Jigger(2.0)).unit_type(),
                UnitType::Volume(VolumeUnit::Jigger)
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
    #[allow(clippy::cast_precision_loss)]
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
            assert_relative_eq!(unit.value(), 1.0, epsilon = f64::EPSILON);
            assert_relative_eq!(
                unit.with_value(test_value).value(),
                test_value,
                epsilon = f64::EPSILON
            );
        }
    }
}
