use std::{borrow::Cow, str::FromStr};

use diesel_async::AsyncPgConnection;
use ingredient::{Ingredient, IngredientParser};
use math::cooking::units::{traits::UnitOperations, unit::Unit, unitless::units::Unitless};
use strum::{EnumIter, EnumString, IntoEnumIterator as _};
use tracing::{error, info, warn};
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::{
    Error, Result,
    nutrition::{
        CalculatedNutrition, NutritionComponents,
        fdc::parser::{
            DataFetched as _, DataNotFetched as _, FdcClient, FdcParser, SRLegacyFoodDetails,
        },
    },
};

/// Nutrition data sources supported by the application.
#[derive(Debug, Default, Eq, PartialEq, strum_macros::Display, EnumIter, EnumString)]
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

struct IngredientForCalculation<'a> {
    original: Cow<'a, str>,
    parsed: Ingredient,
    unit: Unit,
}

#[derive(Debug, Clone)]
struct PortionUnit {
    amount: f64,
    gram_weight: f64,
    modifier: String,
    unit: Unit,
}

impl NutritionDataSource {
    /// Calculate the nutritional information for a recipe based on its ingredients.
    pub async fn calculate_nutrition(
        &self,
        conn: &mut AsyncPgConnection,
        ingredients: &[&str],
        num_servings: i16,
    ) -> Result<CalculatedNutrition> {
        if self == &Self::Unknown {
            return Err(Error::UnknownSource);
        }

        let ingredients = self
            .normalize_ingredients(ingredients)
            .into_iter()
            .map(|ing| IngredientForCalculation {
                original: ing.clone(),
                parsed: IngredientParser::new(false).from_str(&ing),
                unit: Unit::from_str(&ing).unwrap_or(Unit::Unitless(Unitless { value: 1.0 })),
            })
            .collect::<Vec<_>>();

        self.calculate_nutrition_facts(conn, ingredients, num_servings)
            .await
    }

    fn normalize_ingredients<'a>(&self, ingredients: &[&'a str]) -> Vec<Cow<'a, str>> {
        match self {
            Self::USDAFoodDataCentral => ingredients
                .iter()
                .map(|&ing| Self::normalize_ingredient(ing))
                .collect(),
            Self::Unknown => ingredients.iter().map(|&s| Cow::Borrowed(s)).collect(),
        }
    }

    fn normalize_ingredient(ing: &str) -> Cow<'_, str> {
        if ing.contains("egg") {
            Cow::Owned(
                ing.replace("eggs", "egg")
                    .replace("egg", "eggs whole fresh"),
            )
        } else if ing.contains("white sugar") {
            Cow::Owned(ing.replace("white sugar", "sugar"))
        } else if ing.contains("walnuts") {
            Cow::Owned(ing.replace("walnuts", "walnuts english"))
        } else {
            Cow::Borrowed(ing)
        }
    }

    async fn calculate_nutrition_facts(
        &self,
        conn: &mut AsyncPgConnection,
        ingredients: Vec<IngredientForCalculation<'_>>,
        num_servings: i16,
    ) -> Result<CalculatedNutrition> {
        match self {
            Self::USDAFoodDataCentral => {
                let mut nutrition_data = Vec::with_capacity(ingredients.len());

                for ing in ingredients {
                    if let Some(calc) = self.calculate_ingredient_nutrition(conn, &ing).await? {
                        nutrition_data.push(calc);
                    }
                }

                Ok(Self::aggregate_nutrition(&nutrition_data, num_servings))
            }
            Self::Unknown => Err(Error::UnknownSource),
        }
    }

    async fn calculate_ingredient_nutrition(
        &self,
        conn: &mut AsyncPgConnection,
        ing: &IngredientForCalculation<'_>,
    ) -> Result<Option<(f64, CalculatedNutrition)>> {
        let foods = SRLegacyFoodDetails::get_relevant(conn, &ing.parsed.name).await?;

        let Some(most_relevant_food) = foods.first() else {
            warn!("No food found for ingredient: {}", ing.parsed.name);
            return Ok(None);
        };

        let nutrition = Self::extract_nutrition_from_food(most_relevant_food);
        let weight = Self::calculate_ingredient_weight(most_relevant_food, ing);
        let scaled_nutrition = Self::scale_nutrition_to_weight(nutrition, weight);

        Ok(Some((weight, scaled_nutrition)))
    }

    #[allow(clippy::cast_possible_truncation)]
    fn extract_nutrition_from_food(food: &SRLegacyFoodDetails) -> NutritionComponents {
        food.food_nutrients.iter().fold(
            NutritionComponents::default(),
            |mut nutrition, nutrient| {
                let v = nutrient.amount;
                match nutrient.fdc_id {
                    1003 => nutrition.protein_g = v,
                    1004 => nutrition.total_fat_g = v,
                    1005 => nutrition.total_carbohydrates = v,
                    1008 => {
                        nutrition.calories_kcal =
                            v.clamp(f64::from(i16::MIN), f64::from(i16::MAX)) as i16;
                    }
                    1079 => nutrition.fiber_g = v,
                    1093 => nutrition.sodium_mg = v,
                    1253 => nutrition.cholesterol_mg = v,
                    1257 => nutrition.trans_fat_g = v,
                    1258 => nutrition.saturated_fat_g = v,
                    1292 | 1293 => nutrition.unsaturated_fat_g += v,
                    2000 => nutrition.sugars_g = v,
                    _ => {}
                }
                nutrition
            },
        )
    }

    fn parse_portion_units(food: &SRLegacyFoodDetails) -> Vec<PortionUnit> {
        food.food_portions
            .iter()
            .filter_map(|portion| {
                Unit::from_str(&format!("1 {}", portion.modifier))
                    .ok()
                    .map(|unit| PortionUnit {
                        amount: portion.amount,
                        gram_weight: portion.gram_weight,
                        modifier: portion.modifier.clone(),
                        unit,
                    })
            })
            .collect()
    }

    fn calculate_ingredient_weight(
        food: &SRLegacyFoodDetails,
        ing: &IngredientForCalculation<'_>,
    ) -> f64 {
        let portion_units = Self::parse_portion_units(food);

        match &ing.unit {
            Unit::Unitless(u) => Self::calculate_unitless_weight(&portion_units, ing, u.value),
            _ => Self::calculate_unit_weight(&portion_units, ing),
        }
    }

    fn calculate_unitless_weight(
        portion_units: &[PortionUnit],
        ing: &IngredientForCalculation<'_>,
        value: f64,
    ) -> f64 {
        if let Some(portion) = portion_units.iter().find(|p| {
            ing.original
                .contains(p.modifier.split_ascii_whitespace().next().unwrap_or(""))
        }) {
            return Self::scale_by_amount(value, portion.amount, portion.gram_weight);
        }

        if let Some(portion) = portion_units.iter().find(|p| p.modifier == "medium") {
            return Self::scale_by_amount(value, portion.amount, portion.gram_weight);
        }

        1.0
    }

    fn calculate_unit_weight(
        portion_units: &[PortionUnit],
        ing: &IngredientForCalculation<'_>,
    ) -> f64 {
        let ing_amount = ing.unit.value();
        let ing_unit_type = ing.unit.unit_type();

        if let Some(portion) = portion_units.iter().find(|p| {
            ing_unit_type == p.unit.unit_type()
                && ing
                    .original
                    .contains(p.modifier.rsplit(", ").next().unwrap_or_default())
        }) {
            return Self::scale_by_amount(ing_amount, portion.amount, portion.gram_weight);
        }

        if let Some(portion) = portion_units
            .iter()
            .find(|p| ing_unit_type == p.unit.unit_type())
        {
            return Self::scale_by_amount(ing_amount, portion.amount, portion.gram_weight);
        }

        2.0
    }

    fn scale_by_amount(ing_amount: f64, portion_amount: f64, gram_weight: f64) -> f64 {
        if (ing_amount - portion_amount).abs() < f64::EPSILON {
            gram_weight
        } else {
            (ing_amount / portion_amount) * gram_weight
        }
    }

    fn scale_nutrition_to_weight(
        nutrition: NutritionComponents,
        weight: f64,
    ) -> CalculatedNutrition {
        let scale = weight / 100.0;

        CalculatedNutrition {
            per_100g: nutrition * scale,
            per_serving: NutritionComponents::default(),
        }
    }

    fn aggregate_nutrition(
        nutrition_data: &[(f64, CalculatedNutrition)],
        num_servings: i16,
    ) -> CalculatedNutrition {
        let total_weight: f64 = nutrition_data.iter().map(|(w, _)| w).sum();
        let scale_per_100g = 100.0 / total_weight;

        let per_100g =
            nutrition_data
                .iter()
                .fold(NutritionComponents::default(), |mut acc, (_, n)| {
                    acc += n.per_100g;
                    acc
                })
                * scale_per_100g;

        let per_serving =
            nutrition_data
                .iter()
                .fold(NutritionComponents::default(), |mut acc, (_, n)| {
                    acc += n.per_100g;
                    acc
                })
                / num_servings;

        CalculatedNutrition {
            per_100g,
            per_serving,
        }
    }

    /// Updates the nutrition data for all sources.
    pub async fn update_all(mm: &ModelManager) {
        info!("Updating nutrition data sources");
        for source in all_nutrition_sources() {
            info!("Updating data for {source}");
            let _ = source.update_data(mm).await;
        }
    }

    /// Updates the user's preferred nutrition data source.
    pub async fn save(&self, mm: &ModelManager, user_id: Uuid) -> Result<()> {
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;

        let mut conn = mm.pool.get().await?;

        let _ = diesel::update(
            schema::user_settings::table.filter(schema::user_settings::user_id.eq(user_id)),
        )
        .set(schema::user_settings::nutrition_source_id.eq(self.id()))
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    const fn id(&self) -> i16 {
        match self {
            Self::USDAFoodDataCentral => 1,
            Self::Unknown => 0,
        }
    }

    /// Updates the nutrition source's data. If the data is already up to date, it will not be updated.
    pub async fn update_data(&self, mm: &ModelManager) -> Result<()> {
        match self {
            Self::USDAFoodDataCentral => match FdcParser::new().fetch(&FdcClient::new(mm)).await {
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
            },
            Self::Unknown => Err(Error::NoNeedToUpdateNutrition),
        }
    }
}

impl From<i16> for NutritionDataSource {
    fn from(id: i16) -> Self {
        match id {
            1 => Self::USDAFoodDataCentral,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use testing::utils::{TestDb, create_app_state};

    use crate::nutrition::{NutritionDataSource, all_nutrition_sources};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_all_nutrition_sources() {
        let got = all_nutrition_sources();

        pretty_assertions::assert_eq!(got, vec![NutritionDataSource::USDAFoodDataCentral]);
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

    mod tests_calculate_nutrition_per_100g {
        use crate::nutrition::{
            CalculatedNutrition, NutritionComponents,
            fdc::parser::{DataFetched as _, DataNotFetched as _, FdcParser},
            states::DataFetchedState,
            testdata::nutrition_data::nutrition_data_for_tests::*,
        };

        use super::*;

        #[tokio::test]
        async fn test_no_calculation_when_unknown_source_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let mut conn = state.mm.pool.get().await?;
            let source = NutritionDataSource::Unknown;

            let res = source
                .calculate_nutrition(&mut conn, Vec::new().as_slice(), 1)
                .await;

            assert!(res.is_err());
            Ok(())
        }

        #[tokio::test]
        async fn test_nutrition1_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config).await;
            let mut conn = state.mm.pool.get().await?;
            let client = FdcClientForTests::new(FDC_FF_DATASET_3);
            let parser: FdcParser<DataFetchedState> = FdcParser::new().fetch(&client).await?;
            parser.push_into_database(&state.mm).await?;
            let ingredients = vec![
                "1 cup white sugar",
                "1/2 cup butter, melted",
                "2 eggs",
                "1 teaspoon vanilla extract",
                "1 cups all-purpose flour",
                "1 teaspoon baking soda",
                "1/2 teaspoon salt",
                "1/2 cup sour cream",
                "1/2 cup chopped walnuts",
                "2 medium bananas, sliced",
            ];
            let source = NutritionDataSource::USDAFoodDataCentral;
            let num_servings = 4;

            let got = source
                .calculate_nutrition(&mut conn, ingredients.as_slice(), num_servings)
                .await?;

            pretty_assertions::assert_eq!(
                got,
                CalculatedNutrition {
                    per_100g: NutritionComponents {
                        calories_kcal: 312,
                        total_carbohydrates: 38.764_939_987_365_764,
                        sugars_g: 24.448_962_939_566_222,
                        protein_g: 4.255_598_020_635_922,
                        total_fat_g: 16.082_924_826_279_21,
                        saturated_fat_g: 7.657_184_670_456_936,
                        unsaturated_fat_g: 7.361_701_200_252_684,
                        cholesterol_mg: 64.396_188_671_299_21,
                        sodium_mg: 356.610_128_448_094_3,
                        fiber_g: 1.414_034_533_586_017_8,
                        trans_fat_g: 0.395_476_942_514_213_5,
                    },
                    per_serving: NutritionComponents {
                        calories_kcal: 740,
                        total_carbohydrates: 92.047_350_000_000_01,
                        sugars_g: 58.054_062_5,
                        protein_g: 10.104_917_499_999_999,
                        total_fat_g: 38.188_904_999_999_99,
                        saturated_fat_g: 18.181_984_999_999_997,
                        unsaturated_fat_g: 17.480_359_5,
                        cholesterol_mg: 152.90875,
                        sodium_mg: 846.77075,
                        fiber_g: 3.357_624_999_999_999_6,
                        trans_fat_g: 0.93906,
                    },
                }
            );
            Ok(())
        }
    }
}
