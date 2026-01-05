use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use ingredient::IngredientParser;
use strum::{EnumIter, EnumString, IntoEnumIterator as _};
use tracing::{error, info, warn};

use repository::{ModelManager, schema};

use crate::{
    Error, Result,
    nutrition::{
        NutritionComponents,
        fdc::parser::{DataFetched as _, DataNotFetched as _, FdcClient, FdcParser},
    },
};

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
    /// Calculate the nutritional information for a recipe based on its ingredients.
    pub fn calculate_nutrition_per_100g(
        &self,
        ingredients: &[&str],
    ) -> Result<NutritionComponents> {
        todo!()
        // if self == &NutritionDataSource::Unknown {
        //     return Err(Error::UnknownSource);
        // }

        // let ingredients = ingredients
        //     .iter()
        //     .map(|ing| IngredientParser::new(false).from_str(ing))
        //     .collect::<Vec<_>>();

        // Ok(nutrition)
    }

    /// Calculate the nutritional information for a recipe based on its ingredients.
    pub fn calculate_nutrition_per_serving(
        &self,
        ingredients: &[&str],
    ) -> Result<NutritionComponents> {
        todo!()
    }

    /// Updates the nutrition data for all sources.
    pub async fn update_all(mm: &ModelManager) {
        info!("Updating nutrition data sources");
        for source in all_nutrition_sources() {
            let _ = source.update_data(mm).await;
        }
    }

    /// Updates the user's preferred nutrition data source.
    pub async fn save(&self, mm: &ModelManager, user_id: i64) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        let _ = diesel::update(
            schema::user_settings::table.filter(schema::user_settings::user_id.eq(user_id)),
        )
        .set(schema::user_settings::nutrition_source_id.eq(self.id()))
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    fn id(&self) -> i16 {
        match self {
            NutritionDataSource::USDAFoodDataCentral => 1,
            NutritionDataSource::Unknown => 0,
        }
    }

    /// Updates the nutrition source's data. If the data is already up to date, it will not be updated.
    pub async fn update_data(&self, mm: &ModelManager) -> Result<()> {
        match self {
            NutritionDataSource::USDAFoodDataCentral => {
                match FdcParser::new().fetch(&FdcClient::new(mm)).await {
                    Ok(f) => {
                        f.push_into_database(mm).await?;
                        info!("Updated nutrition data for '{self}'");
                        Ok(())
                    }
                    Err(Error::NoNeedToUpdateNutrition) => {
                        warn!("Nutrition data '{self}' already up to date.");
                        Ok(())
                    }
                    Err(err) => {
                        error!("Failed to update nutrition data '{self}': {err}");
                        Err(err)
                    }
                }
            }
            NutritionDataSource::Unknown => Err(Error::NoNeedToUpdateNutrition),
        }
    }
}

impl From<i16> for NutritionDataSource {
    fn from(id: i16) -> Self {
        match id {
            1 => NutritionDataSource::USDAFoodDataCentral,
            _ => NutritionDataSource::Unknown,
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
    fn test_nutrition_source_parse() {
        pretty_assertions::assert_eq!(
            "USDA FoodData Central"
                .parse::<NutritionDataSource>()
                .unwrap(),
            NutritionDataSource::USDAFoodDataCentral
        );
        pretty_assertions::assert_eq!(
            "unknown".parse::<NutritionDataSource>().unwrap(),
            NutritionDataSource::Unknown
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            NutritionDataSource::USDAFoodDataCentral.to_string(),
            "USDA FoodData Central"
        );
        assert_eq!(NutritionDataSource::Unknown.to_string(), "unknown");
    }

    #[test]
    fn from_id() {
        pretty_assertions::assert_eq!(
            NutritionDataSource::from(1),
            NutritionDataSource::USDAFoodDataCentral
        );
        pretty_assertions::assert_eq!(NutritionDataSource::from(2), NutritionDataSource::Unknown);
    }

    mod tests_calculate_nutrition {
        use super::*;

        #[tokio::test]
        async fn test_calculate_fdc_nutrition_simple_ok() -> Result<()> {
            todo!()
            // let ingredients = vec![];

            // let got = NutritionDataSource::USDAFoodDataCentral.calculate_nutrition(ingredients)

            // Ok(())
        }
    }
}
