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
            // Unit::Mass(unit) => unit.scale(factor),
            // Unit::Temperature(unit) => unit.scale(factor),
            // Unit::Volume(unit) => unit.scale(factor),
            _ => Ok(self.clone()),
        }

        // match self.unit_type() {
        //     UnitType::Milligram => {
        //         if scaled_value >= 1e6 {
        //             Ok(Unit::Kilogram(scaled_value * 1e-6))
        //         } else if scaled_value >= 1e5 {
        //             Ok(Unit::Hectogram(scaled_value * 1e-5))
        //         } else if scaled_value >= 1e4 {
        //             Ok(Unit::Dekagram(scaled_value * 1e-4))
        //         } else if scaled_value >= 1e3 {
        //             Ok(Unit::Gram(scaled_value * 1e-3))
        //         } else {
        //             Ok(Unit::Milligram(scaled_value))
        //         }
        //     }
        //     UnitType::Gram => {
        //         if scaled_value >= 1e3 {
        //             Ok(Unit::Kilogram(scaled_value * 1e-3))
        //         } else if scaled_value >= 1e2 {
        //             Ok(Unit::Hectogram(scaled_value * 1e-2))
        //         } else if scaled_value >= 1e1 {
        //             Ok(Unit::Dekagram(scaled_value * 1e-1))
        //         } else if scaled_value >= 1.0 {
        //             Ok(Unit::Gram(scaled_value))
        //         } else {
        //             Ok(Unit::Milligram(scaled_value * 10.0))
        //         }
        //     }
        //     UnitType::Dekagram => {
        //         if scaled_value >= 1e2 {
        //             Ok(Unit::Kilogram(scaled_value * 1e-2))
        //         } else if scaled_value >= 1e1 {
        //             Ok(Unit::Hectogram(scaled_value * 1e-1))
        //         } else if scaled_value >= 1.0 {
        //             Ok(Unit::Dekagram(scaled_value))
        //         } else if scaled_value >= 1e-1 {
        //             Ok(Unit::Gram(scaled_value * 1e1))
        //         } else {
        //             Ok(Unit::Milligram(scaled_value * 1e4))
        //         }
        //     }
        //     UnitType::Hectogram => {
        //         if scaled_value >= 1e1 {
        //             Ok(Unit::Kilogram(scaled_value * 1e-1))
        //         } else if scaled_value >= 1.0 {
        //             Ok(Unit::Hectogram(scaled_value))
        //         } else if scaled_value >= 1e-1 {
        //             Ok(Unit::Dekagram(scaled_value*1e1))
        //         } else if scaled_value >= 1e-2 {
        //             Ok(Unit::Gram(scaled_value * 1e2))
        //         } else {
        //             Ok(Unit::Milligram(scaled_value * 1e5))
        //         }
        //     }
        //     UnitType::Kilogram => {
        //         if scaled_value >= 1.0 {
        //             Ok(Unit::Kilogram(scaled_value))
        //         } else if scaled_value >= 1e-1 {
        //             Ok(Unit::Hectogram(scaled_value*1e1))
        //         } else if scaled_value >= 1e-2 {
        //             Ok(Unit::Dekagram(scaled_value*1e2))
        //         } else if scaled_value >= 1e-3 {
        //             Ok(Unit::Gram(scaled_value * 1e3))
        //         } else {
        //             Ok(Unit::Milligram(scaled_value * 1e6))
        //         }
        //     }
        //     UnitType::Ounce => {
        //         if scaled_value >= 16.0 {
        //             Ok(Unit::Pound(scaled_value * 0.0625))
        //         } else {
        //             Ok(Unit::Ounce(scaled_value))
        //         }
        //     }
        //     UnitType::Pound => {
        //         if scaled_value >= 1.0 {
        //             Ok(Unit::Pound(scaled_value))
        //         } else {
        //             Ok(Unit::Ounce(scaled_value * 16.0))
        //         }
        //     }
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
//         // Milligram
//         #[test]
//         fn test_mg_to_mg() -> Result<()> {
//             assert_eq!(Unit::Milligram(10.0).scale(2.0)?, Unit::Milligram(20.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_mg_to_g() -> Result<()> {
//             assert_eq!(Unit::Milligram(100.0).scale(10.0)?, Unit::Gram(1.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_mg_to_dag() -> Result<()> {
//             assert_eq!(Unit::Milligram(1000.0).scale(10.0)?, Unit::Dekagram(1.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_mg_to_hg() -> Result<()> {
//             assert_eq!(Unit::Milligram(10_000.0).scale(10.0)?, Unit::Hectogram(1.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_mg_to_kg()->Result<()> {
//             assert_eq!(Unit::Milligram(100_000.0).scale(20.0)?, Unit::Kilogram(2.0));
//             Ok(())
//         }
//
//         // Gram
//         #[test]
//         fn test_g_to_mg() -> Result<()> {
//             assert_eq!(Unit::Gram(1.0).scale(0.5)?, Unit::Milligram(5.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_g_to_g() -> Result<()> {
//             assert_eq!(Unit::Gram(1.0).scale(8.0)?, Unit::Gram(8.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_g_to_dag() -> Result<()> {
//             assert_eq!(Unit::Gram(15.0).scale(2.0)?, Unit::Dekagram(3.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_g_to_hg() -> Result<()> {
//             assert_eq!(Unit::Gram(10.0).scale(10.0)?, Unit::Hectogram(1.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_g_to_kg()->Result<()> {
//             assert_eq!(Unit::Gram(100.0).scale(20.0)?, Unit::Kilogram(2.0));
//             Ok(())
//         }
//
//         // Dekagram
//         #[test]
//         fn test_dag_to_mg() -> Result<()> {
//             assert_eq!(Unit::Dekagram(1.0).scale(0.005)?, Unit::Milligram(50.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_dag_to_g() -> Result<()> {
//             assert_eq!(Unit::Dekagram(1.0).scale(0.5)?, Unit::Gram(5.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_dag_to_dag() -> Result<()> {
//             assert_eq!(Unit::Dekagram(3.0).scale(2.0)?, Unit::Dekagram(6.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_dag_to_hg() -> Result<()> {
//             assert_eq!(Unit::Dekagram(10.0).scale(3.0)?, Unit::Hectogram(3.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_dag_to_kg()->Result<()> {
//             assert_eq!(Unit::Dekagram(100.0).scale(20.0)?, Unit::Kilogram(20.0));
//             Ok(())
//         }
//
//         // Hectogram
//         #[test]
//         fn test_hg_to_mg() -> Result<()> {
//             assert_eq!(Unit::Hectogram(1.0).scale(0.0002)?, Unit::Milligram(20.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_hg_to_g() -> Result<()> {
//             assert_eq!(Unit::Hectogram(1.0).scale(0.02)?, Unit::Gram(2.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_hg_to_dag() -> Result<()> {
//             assert_eq!(Unit::Hectogram(1.0).scale(0.1)?, Unit::Dekagram(1.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_hg_to_hg() -> Result<()> {
//             assert_eq!(Unit::Hectogram(1.0).scale(2.0)?, Unit::Hectogram(2.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_hg_to_kg()->Result<()> {
//             assert_eq!(Unit::Hectogram(10.0).scale(20.0)?, Unit::Kilogram(20.0));
//             Ok(())
//         }
//
//         // Kilogram
//         #[test]
//         fn test_kg_to_mg() -> Result<()> {
//             assert_eq!(Unit::Kilogram(1.0).scale(0.000002)?, Unit::Milligram(2.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_kg_to_g() -> Result<()> {
//             assert_eq!(Unit::Kilogram(1.0).scale(0.002)?, Unit::Gram(2.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_kg_to_dag() -> Result<()> {
//             assert_eq!(Unit::Kilogram(1.0).scale(0.02)?, Unit::Dekagram(2.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_kg_to_hg() -> Result<()> {
//             assert_eq!(Unit::Kilogram(1.0).scale(0.2)?, Unit::Hectogram(2.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_kg_to_kg()->Result<()> {
//             assert_eq!(Unit::Kilogram(100.0).scale(20.0)?, Unit::Kilogram(2000.0));
//             Ok(())
//         }
//
//         // Ounce
//         #[test]
//         fn test_oz_to_oz() -> Result<()> {
//             assert_eq!(Unit::Ounce(1.0).scale(2.0)?, Unit::Ounce(2.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_oz_to_lb() -> Result<()> {
//             assert_eq!(Unit::Ounce(4.0).scale(4.0)?, Unit::Pound(1.0));
//             Ok(())
//         }
//
//         // Pound
//         #[test]
//         fn test_lb_to_oz() -> Result<()> {
//             assert_eq!(Unit::Pound(1.0).scale(0.5)?, Unit::Ounce(8.0));
//             Ok(())
//         }
//
//         #[test]
//         fn test_lb_to_lb() -> Result<()> {
//             assert_eq!(Unit::Pound(4.0).scale(4.0)?, Unit::Pound(16.0));
//             Ok(())
//         }
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
