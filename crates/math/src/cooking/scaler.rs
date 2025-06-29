use crate::cooking::units::traits::{UnitOperations, UnitScaler};
use crate::cooking::units::{LengthUnit, Unit, UnitType};
use crate::{Error, Result};

impl UnitScaler for Unit {
    /// Scales the unit by the given factor in the same measurement system.
    fn scale(&self, factor: f64) -> Result<Unit> {
        if factor.is_sign_negative() {
            return Err(Error::InvalidScaleFactor(factor));
        }

        match self {
            Unit::Length(unit) => unit.scale(factor),
            Unit::Mass(unit) => unit.scale(factor),
            // Unit::Temperature(unit) => unit.scale(factor),
            // Unit::Volume(unit) => unit.scale(factor),
            _ => Ok(self.clone()),
        }

        // match self.unit_type() {
        //     UnitType::Celsius | UnitType::Fahrenheit => Ok(self.with_value(scaled_value)),
        //     UnitType::Millilitre => {
        //         Ok(self.clone())
        //     }
        //     UnitType::Centilitre => {
        //         Ok(self.clone())
        //     }
        //     UnitType::Decilitre => {
        //         Ok(self.clone())
        //     }
        //     UnitType::Litre => {
        //         Ok(self.clone())
        //     }
        //     UnitType::MetricTeaspoon => {
        //         Ok(self.clone())
        //     }
        //     UnitType::MetricTablespoon => {
        //         Ok(self.clone())
        //     }
        //     UnitType::MetricDessertSpoon => {
        //         Ok(self.clone())
        //     }
        //     UnitType::MetricCup => {
        //         Ok(self.clone())
        //     }
        //     UnitType::AustralianTablespoon => {
        //         Ok(self.clone())
        //     }
        //     UnitType::ImperialTeaspoon => {
        //         Ok(self.clone())
        //     }
        //     UnitType::ImperialDessertspoon => {
        //         Ok(self.clone())
        //     }
        //     UnitType::ImperialTablespoon => {
        //         Ok(self.clone())
        //     }
        //     UnitType::ImperialFluidOunce => {
        //         Ok(self.clone())
        //     }
        //     UnitType::ImperialGill => {
        //         Ok(self.clone())
        //     }
        //     UnitType::ImperialCup => {
        //         Ok(self.clone())
        //     }
        //     UnitType::ImperialPint => {
        //         Ok(self.clone())
        //     }
        //     UnitType::ImperialQuart => {
        //         Ok(self.clone())
        //     }
        //     UnitType::ImperialGallon => {
        //         Ok(self.clone())
        //     }
        //     UnitType::USLegalCup => {
        //         Ok(self.clone())
        //     }
        //     UnitType::USTeaspoon => {
        //         Ok(self.clone())
        //     }
        //     UnitType::USTablespoon => {
        //         Ok(self.clone())
        //     }
        //     UnitType::USFluidOunce => {
        //         Ok(self.clone())
        //     }
        //     UnitType::USCup => {
        //         Ok(self.clone())
        //     }
        //     UnitType::USPint => {
        //         Ok(self.clone())
        //     }
        //     UnitType::USQuart => {
        //         Ok(self.clone())
        //     }
        //     UnitType::USGallon => {
        //         Ok(self.clone())
        //     }
        //     UnitType::Jigger => {
        //         Ok(self.clone())
        //     },
        // }
    }
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
//
//     fn assert_approx_eq(actual: Unit, expected: Unit, threshold: f64) {
//         let actual_val = actual.value();
//         let expected_val = expected.value();
//
//         assert!(
//             (actual_val - expected_val).abs() < threshold,
//             "Expected {actual:?}, got {expected:?}, difference: {}",
//             (actual_val - expected_val).abs()
//         );
//     }
//
//     mod test_scale_mass {
//         use super::*;
//

//     }
//
//     mod test_scale_temperature {
//         use super::*;
//
//         #[test]
//         fn test_celsius() -> Result<()> {
//             assert_eq!(Unit::Celsius(100.0).scale(2.0)?, Unit::Celsius(200.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_fahrenheit() -> Result<()> {
//             assert_eq!(Unit::Fahrenheit(100.0).scale(2.0)?, Unit::Fahrenheit(200.0));
//             Ok(())
//         }
//     }
//
//     mod test_scale_volume {
//         use super::*;
//     }
// }
