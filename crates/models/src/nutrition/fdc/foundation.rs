use std::{collections::VecDeque, io::Read};

use diesel_async::{AsyncConnection, RunQueryDsl as _};
use repository::{ModelManager, schema};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Error, Result,
    nutrition::structs::{
        FdcFoodFdcNutrientForInsert, FdcFoodPortionFdcFoodForInsert, FdcFoodPortionForInsert,
        FdcNutrientForInsert, FoundationFoodForInsert, MeasureUnitForInsert,
    },
};

/// Contains all the details of a foundation food.
pub struct FoundationFoodDetails {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    #[serde(rename = "FoundationFoods")]
    foundation_foods: Vec<FoundationFood>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FoundationFood {
    food_class: String,
    description: String,
    food_nutrients: Vec<FoodNutrient>,
    food_attributes: Vec<Value>,
    #[serde(default)]
    nutrient_conversion_factors: Vec<NutrientConversionFactor>,
    is_historical_reference: bool,
    ndb_number: i64,
    data_type: String,
    food_category: FoodCategory,
    fdc_id: i64,
    #[serde(default)]
    food_portions: Vec<FoodPortion>,
    publication_date: String,
    #[serde(default)]
    input_foods: Vec<InputFood>,
    scientific_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FoodNutrient {
    #[serde(rename = "type")]
    type_field: String,
    id: i64,
    nutrient: Nutrient,
    data_points: Option<i64>,
    food_nutrient_derivation: FoodNutrientDerivation,
    median: Option<f64>,
    amount: Option<f64>,
    max: Option<f64>,
    min: Option<f64>,
    footnote: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Nutrient {
    id: i64,
    number: String,
    name: String,
    rank: i64,
    unit_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FoodNutrientDerivation {
    code: Option<String>,
    description: Option<String>,
    food_nutrient_source: FoodNutrientSource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FoodNutrientSource {
    id: Option<i64>,
    code: Option<String>,
    description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NutrientConversionFactor {
    #[serde(rename = "type")]
    type_field: String,
    protein_value: Option<f64>,
    fat_value: Option<f64>,
    carbohydrate_value: Option<f64>,
    value: Option<f64>,
    nitrogen_value: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FoodCategory {
    description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FoodPortion {
    id: i64,
    value: f64,
    measure_unit: MeasureUnit,
    modifier: Option<String>,
    gram_weight: f64,
    sequence_number: i64,
    amount: f64,
    min_year_acquired: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MeasureUnit {
    id: i64,
    name: String,
    abbreviation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InputFood {
    id: i64,
    food_description: String,
    input_food: InputFood2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InputFood2 {
    food_class: String,
    description: String,
    data_type: String,
    food_category: FoodCategory2,
    fdc_id: i64,
    publication_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FoodCategory2 {
    id: i64,
    code: String,
    description: String,
}

/// Populates the nutrition database tables using the **Foundation Food dataset (FNDDS)**
/// from the USDA FoodData Central (FDC).
pub async fn populate_db(mm: &ModelManager, r: impl Read) -> Result<()> {
    let root: Root = serde_json::from_reader(r)?;

    let mut conn = mm.pool.get().await?;

    conn.transaction::<_, Error, _>(|mut conn| {
        Box::pin(async move {
            // fdc_foods
            let fdc_foods_ids: Vec<i64> = diesel::insert_into(schema::fdc_foods::table)
                .values(
                    root.foundation_foods
                        .iter()
                        .map(|food| FoundationFoodForInsert {
                            food_class: &food.food_class,
                            description: &food.description,
                            food_category: &food.food_category.description,
                            fdc_id: food.fdc_id,
                        })
                        .collect::<Vec<_>>(),
                )
                .returning(schema::fdc_foods::id)
                .get_results(&mut conn)
                .await?;

            // fdc_nutrients
            let fdc_nutrients_ids: Vec<i64> = diesel::insert_into(schema::fdc_nutrients::table)
                .values(
                    root.foundation_foods
                        .iter()
                        .flat_map(|food| {
                            food.food_nutrients
                                .iter()
                                .map(|nutrient| FdcNutrientForInsert {
                                    name: &nutrient.nutrient.name,
                                    unit_name: &nutrient.nutrient.unit_name,
                                })
                        })
                        .collect::<Vec<_>>(),
                )
                .returning(schema::fdc_nutrients::id)
                .get_results(&mut conn)
                .await?;

            // fdc_foods_fdc_nutrients
            let mut fdc_nutrients_ids = VecDeque::from(fdc_nutrients_ids);

            let mut ids = Vec::new();
            for (ff, fdc_food_db_id) in root.foundation_foods.iter().zip(fdc_foods_ids.clone()) {
                ff.food_nutrients.iter().for_each(|nutrient| {
                    if let Some(id) = fdc_nutrients_ids.pop_front() {
                        ids.push((
                            fdc_food_db_id,
                            id,
                            nutrient.median.unwrap_or_default(),
                            nutrient.amount.unwrap_or_default(),
                        ));
                    }
                });
            }

            diesel::insert_into(schema::fdc_foods_fdc_nutrients::table)
                .values(
                    ids.into_iter()
                        .map(
                            |(food_id, nutrient_id, median, amount)| FdcFoodFdcNutrientForInsert {
                                food_id,
                                nutrient_id,
                                median,
                                amount,
                            },
                        )
                        .collect::<Vec<_>>(),
                )
                .execute(&mut conn)
                .await?;

            // measure_units
            let measure_unit_ids: Vec<i64> = diesel::insert_into(schema::measure_units::table)
                .values(
                    root.foundation_foods
                        .iter()
                        .flat_map(|food| {
                            food.food_portions
                                .iter()
                                .map(|portion| MeasureUnitForInsert {
                                    name: portion.measure_unit.name.clone(),
                                    abbreviation: portion.measure_unit.abbreviation.clone(),
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>(),
                )
                .returning(schema::measure_units::id)
                .get_results(&mut conn)
                .await?;

            // fdc_food_portions
            let mut measure_unit_ids = VecDeque::from(measure_unit_ids);

            let portion_ids: Vec<i64> = diesel::insert_into(schema::fdc_food_portions::table)
                .values(
                    root.foundation_foods
                        .iter()
                        .flat_map(|food| {
                            food.food_portions
                                .iter()
                                .map(|portion| FdcFoodPortionForInsert {
                                    value: portion.value,
                                    nutrition_measure_unit_id: measure_unit_ids
                                        .pop_front()
                                        .unwrap_or_default(),
                                    modifier: portion.modifier.clone(),
                                    gram_weight: portion.gram_weight,
                                    amount: portion.amount,
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>(),
                )
                .returning(schema::fdc_food_portions::id)
                .get_results(&mut conn)
                .await?;

            // fdc_food_portions_fdc_foods
            let mut portion_ids = VecDeque::from(portion_ids);

            let mut ids = Vec::new();
            for (ff, fdc_food_db_id) in root.foundation_foods.iter().zip(fdc_foods_ids) {
                ff.food_portions.iter().for_each(|_| {
                    if let Some(id) = portion_ids.pop_front() {
                        ids.push((fdc_food_db_id, id));
                    }
                });
            }

            diesel::insert_into(schema::fdc_food_portions_fdc_foods::table)
                .values(
                    ids.into_iter()
                        .map(|(food_id, portion_id)| FdcFoodPortionFdcFoodForInsert {
                            food_id,
                            portion_id,
                        })
                        .collect::<Vec<_>>(),
                )
                .execute(&mut conn)
                .await?;

            Ok(())
        })
    })
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use testing::utils::{TestDb, create_app_state};

    use super::*;

    type Result<T> = core::result::Result<T, Error>;
    type Error = Box<dyn std::error::Error>;

    #[tokio::test]
    async fn test_parses_correctly_ok() -> Result<()> {
        let example_dataset = r#"{"FoundationFoods":[{"foodClass":"FinalFood","description":"Hummus, commercial","foodNutrients":[{"type":"FoodNutrient","id":2219824,"nutrient":{"id":2026,"number":"840","name":"PUFA 20:2 c","rank":14250,"unitName":"g"},"foodNutrientDerivation":{"code":"AS","description":"Summed","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"amount":0.005},{"type":"FoodNutrient","id":2219825,"nutrient":{"id":1085,"number":"298","name":"Total fat (NLEA)","rank":900,"unitName":"g"},"foodNutrientDerivation":{"code":"AS","description":"Summed","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"amount":16.1}],"foodAttributes":[],"nutrientConversionFactors":[{"type":".CalorieConversionFactor","proteinValue":3.47,"fatValue":8.37,"carbohydrateValue":4.07},{"type":".ProteinConversionFactor","value":6.25}],"isHistoricalReference":false,"ndbNumber":16158,"dataType":"Foundation","foodCategory":{"description":"Legumes and Legume Products"},"fdcId":321358,"foodPortions":[{"id":118804,"value":2,"measureUnit":{"id":1001,"name":"tablespoon","abbreviation":"tbsp"},"modifier":"","gramWeight":33.9,"sequenceNumber":1,"amount":2,"minYearAcquired":2015}],"publicationDate":"4/1/2019","inputFoods":[{"id":10428,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319874,"publicationDate":"4/1/2019"}},{"id":10433,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319921,"publicationDate":"4/1/2019"}},{"id":10432,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319901,"publicationDate":"4/1/2019"}},{"id":10435,"foodDescription":"HUMMUS, TRIBE CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, TRIBE CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319958,"publicationDate":"4/1/2019"}},{"id":10429,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319879,"publicationDate":"4/1/2019"}},{"id":10436,"foodDescription":"HUMMUS, OTHER","inputFood":{"foodClass":"Composite","description":"HUMMUS, OTHER","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319978,"publicationDate":"4/1/2019"}},{"id":10431,"foodDescription":"HUMMUS, OTHER","inputFood":{"foodClass":"Composite","description":"HUMMUS, OTHER","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319894,"publicationDate":"4/1/2019"}},{"id":10430,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319885,"publicationDate":"4/1/2019"}},{"id":10434,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319939,"publicationDate":"4/1/2019"}},{"id":10438,"foodDescription":"HUMMUS, OTHER","inputFood":{"foodClass":"Composite","description":"HUMMUS, OTHER","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":320005,"publicationDate":"4/1/2019"}},{"id":10437,"foodDescription":"HUMMUS, TRIBE CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, TRIBE CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319984,"publicationDate":"4/1/2019"}}]},{"foodClass":"FinalFood","description":"Tomatoes, grape, raw","foodNutrients":[{"type":"FoodNutrient","id":2220032,"nutrient":{"id":1005,"number":"205","name":"Carbohydrate, by difference","rank":1110,"unitName":"g"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":5.51},{"type":"FoodNutrient","id":2220034,"nutrient":{"id":1008,"number":"208","name":"Energy","rank":300,"unitName":"kcal"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":27},{"type":"FoodNutrient","id":2220035,"nutrient":{"id":1062,"number":"268","name":"Energy","rank":400,"unitName":"kJ"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":113},{"type":"FoodNutrient","id":33291134,"nutrient":{"id":2066,"number":"333","name":"Vitamin A","rank":7420,"unitName":"mg"},"foodNutrientDerivation":{"foodNutrientSource":{}}}],"scientificName":"Solanum lycopersicum","foodAttributes":[],"nutrientConversionFactors":[{"type":".ProteinConversionFactor","value":6.25},{"type":".CalorieConversionFactor","proteinValue":2.44,"fatValue":8.37,"carbohydrateValue":3.57}],"isHistoricalReference":false,"ndbNumber":100147,"dataType":"Foundation","foodCategory":{"description":"Vegetables and Vegetable Products"},"fdcId":321360,"foodPortions":[{"id":118808,"value":5,"measureUnit":{"id":1082,"name":"tomatoes","abbreviation":"tomatoes"},"modifier":"","gramWeight":49.7,"sequenceNumber":1,"amount":5,"minYearAcquired":2016},{"id":118809,"value":1,"measureUnit":{"id":1000,"name":"cup","abbreviation":"cup"},"modifier":"","gramWeight":152,"sequenceNumber":2,"amount":1,"minYearAcquired":2016}],"publicationDate":"4/1/2019","inputFoods":[{"id":10480,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320512,"publicationDate":"4/1/2019"}},{"id":10472,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320476,"publicationDate":"4/1/2019"}},{"id":10473,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320480,"publicationDate":"4/1/2019"}},{"id":10477,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320504,"publicationDate":"4/1/2019"}},{"id":10468,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320438,"publicationDate":"4/1/2019"}},{"id":10475,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320489,"publicationDate":"4/1/2019"}},{"id":10464,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320418,"publicationDate":"4/1/2019"}},{"id":10479,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320507,"publicationDate":"4/1/2019"}},{"id":10471,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320460,"publicationDate":"4/1/2019"}},{"id":10465,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320423,"publicationDate":"4/1/2019"}},{"id":10463,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320413,"publicationDate":"4/1/2019"}},{"id":10474,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320485,"publicationDate":"4/1/2019"}},{"id":10470,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320451,"publicationDate":"4/1/2019"}},{"id":10476,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320494,"publicationDate":"4/1/2019"}},{"id":10478,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320499,"publicationDate":"4/1/2019"}},{"id":10469,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320443,"publicationDate":"4/1/2019"}},{"id":10466,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320428,"publicationDate":"4/1/2019"}},{"id":10467,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320433,"publicationDate":"4/1/2019"}}]}]}"#;
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config).await;

        populate_db(&state.mm, Cursor::new(example_dataset)).await?;

        Ok(())
    }
}
