mod error;
mod fdc;
mod states;

pub use error::{Error, Result};

use strum::{EnumIter, EnumString, IntoEnumIterator};
use tracing::warn;

use repository::ModelManager;

use crate::fdc::parser::{DataFetched as _, DataNotFetched as _, FdcClient, FdcParser};

/// Nutrition data sources supported by the application.
#[derive(Debug, Default, PartialEq, strum_macros::Display, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum NutritionDataSource {
    #[strum(serialize = "USDA FoodData Central")]
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

impl NutritionDataSource {
    /// Updates the nutrition source's data. If the data is already up to date, it will not be updated.
    pub async fn update_data(&self, mm: &ModelManager) -> Result<()> {
        match self {
            NutritionDataSource::USDAFoodDataCentral => {
                let fetched = match FdcParser::new().fetch(&FdcClient::new(mm)).await {
                    Ok(f) => f,
                    Err(Error::NoNeedToUpdateNutrition) => {
                        warn!("Nutrition data '{self}' already up to date.");
                        return Ok(());
                    }
                    Err(_) => todo!(),
                };
                fetched.push_into_database(&mm).await?;
                Ok(())
            }
            NutritionDataSource::Unknown => Err(Error::NoNeedToUpdateNutrition),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_nutrition_sources() {
        let got = all_nutrition_sources();

        pretty_assertions::assert_eq!(got, vec![NutritionDataSource::USDAFoodDataCentral])
    }

    #[test]
    fn to_string() {
        assert_eq!(
            NutritionDataSource::USDAFoodDataCentral.to_string(),
            "USDA FoodData Central"
        );
        assert_eq!(NutritionDataSource::Unknown.to_string(), "unknown");
    }
}
