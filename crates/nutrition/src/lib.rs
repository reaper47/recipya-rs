mod error;
mod fdc;
mod states;

pub use error::{Error, Result};

use strum::{EnumIter, EnumString, IntoEnumIterator};

/// Nutrition data sources supported by the application.
#[derive(Debug, Default, PartialEq, strum_macros::Display, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum NutritionDataSource {
    USDAFoodDataCentral,
    #[default]
    Unknown,
}

/// Returns a list of all nutrition data sources Recipya supports.
pub fn all_nutrition_sources() -> Vec<NutritionDataSource> {
    NutritionDataSource::iter()
        .filter(|a| !matches!(a, NutritionDataSource::Unknown))
        .collect()
}

impl NutritionDataSource {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_nutrition_sources() {
        let got = all_nutrition_sources();

        pretty_assertions::assert_eq!(got, vec![NutritionDataSource::USDAFoodDataCentral])
    }
}
