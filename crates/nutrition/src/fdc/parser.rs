use std::{
    collections::{HashMap, VecDeque},
    marker::PhantomData,
};

use async_trait::async_trait;
use diesel_async::{AsyncConnection as _, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use models::nutrition::{
    FdcFoodFdcNutrientForInsert, FdcFoodPortionFdcFoodForInsert, FdcFoodPortionForInsert,
    FdcNutrientForInsert, FoundationFoodForInsert, MeasureUnitForInsert,
};
use repository::{ModelManager, schema};

use crate::{
    Error, Result,
    common::{DataFetchedState, DataNotFetchedState},
};

pub trait DataNotFetched {
    /// Fetches the following data sets from the FoodData Central API:
    /// - Foundation Food Data
    fn fetch(self) -> Result<FdcParser<DataFetchedState>>;
}

#[async_trait]
pub trait DataFetched: Send + Sync {
    /// Pushes the data sets fetched from the FoodData Central API into the database.
    async fn push_into_database<'a>(&'a self, mm: &'a ModelManager) -> Result<()>;
}

/// A parser for the [FoodData Central](https://fdc.nal.usda.gov/) API.
pub struct FdcParser<State> {
    foundation_food_data: Vec<FoundationFood>,
    _state: PhantomData<State>,
}

impl FdcParser<DataNotFetchedState> {
    /// Creates a new [FoodData Central](https://fdc.nal.usda.gov/) parser.
    pub fn new() -> Self {
        Self {
            foundation_food_data: Vec::new(),
            _state: PhantomData,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FoundationFoodRoot {
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
pub struct MeasureUnit {
    pub id: i64,
    pub name: String,
    pub abbreviation: String,
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

impl DataNotFetched for FdcParser<DataNotFetchedState> {
    fn fetch(self) -> Result<FdcParser<DataFetchedState>> {
        todo!("Add HTTP client to fetch foundation food data");

        let ff_data: FoundationFoodRoot = serde_json::from_reader(r)?;

        Ok(FdcParser {
            foundation_food_data: ff_data.foundation_foods,
            _state: PhantomData,
        })
    }
}

#[async_trait]
impl DataFetched for FdcParser<DataFetchedState> {
    async fn push_into_database<'a>(&'a self, mm: &'a ModelManager) -> Result<()> {
        use schema::fdc_foods;

        let mut conn = mm.pool.get().await?;

        conn.transaction::<_, Error, _>(|mut conn| {
            Box::pin(async move {
                let foundation_foods = &self.foundation_food_data;

                // fdc_foods
                let fdc_foods_ids: Vec<i64> = diesel::insert_into(fdc_foods::table)
                    .values(
                        foundation_foods
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
                let fdc_nutrients: Vec<(i64, String, String)> =
                    diesel::insert_into(schema::fdc_nutrients::table)
                        .values(
                            foundation_foods
                                .iter()
                                .flat_map(|food| {
                                    food.food_nutrients.iter().map(|nutrient| {
                                        FdcNutrientForInsert {
                                            name: &nutrient.nutrient.name,
                                            unit_name: &nutrient.nutrient.unit_name,
                                        }
                                    })
                                })
                                .collect::<Vec<_>>(),
                        )
                        .on_conflict((
                            schema::fdc_nutrients::name,
                            schema::fdc_nutrients::unit_name,
                        ))
                        .do_nothing()
                        .returning((
                            schema::fdc_nutrients::id,
                            schema::fdc_nutrients::name,
                            schema::fdc_nutrients::unit_name,
                        ))
                        .get_results(&mut conn)
                        .await?;

                let fdc_nutrients_map: HashMap<(String, String), i64> = fdc_nutrients
                    .into_iter()
                    .map(|(id, name, unit_name)| ((name, unit_name), id))
                    .collect();

                // fdc_foods_fdc_nutrients
                let mut ids = Vec::new();
                for (ff, fdc_food_db_id) in foundation_foods.iter().zip(fdc_foods_ids.clone()) {
                    ff.food_nutrients.iter().for_each(|food_nutrient| {
                        let food_nutrient = food_nutrient.clone();

                        let id = fdc_nutrients_map
                            .get(&(
                                food_nutrient.nutrient.name,
                                food_nutrient.nutrient.unit_name,
                            ))
                            .unwrap()
                            .clone();

                        ids.push((
                            fdc_food_db_id,
                            id,
                            food_nutrient.median.unwrap_or_default(),
                            food_nutrient.amount.unwrap_or_default(),
                        ));
                    });
                }

                diesel::insert_into(schema::fdc_foods_fdc_nutrients::table)
                    .values(
                        ids.into_iter()
                            .map(|(food_id, nutrient_id, median, amount)| {
                                FdcFoodFdcNutrientForInsert {
                                    food_id,
                                    nutrient_id,
                                    median,
                                    amount,
                                }
                            })
                            .collect::<Vec<_>>(),
                    )
                    .execute(&mut conn)
                    .await?;

                // measure_units
                let measure_units: Vec<(i64, String, String)> =
                    diesel::insert_into(schema::measure_units::table)
                        .values(
                            foundation_foods
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
                        .on_conflict((
                            schema::measure_units::name,
                            schema::measure_units::abbreviation,
                        ))
                        .do_nothing()
                        .returning((
                            schema::measure_units::id,
                            schema::measure_units::name,
                            schema::measure_units::abbreviation,
                        ))
                        .get_results(&mut conn)
                        .await?;

                let measurement_units_map: HashMap<(String, String), i64> = measure_units
                    .into_iter()
                    .map(|(id, name, abbreviation)| ((name, abbreviation), id))
                    .collect();

                // fdc_food_portions
                let portion_ids: Vec<i64> = diesel::insert_into(schema::fdc_food_portions::table)
                    .values(
                        foundation_foods
                            .iter()
                            .flat_map(|food| {
                                food.food_portions
                                    .iter()
                                    .map(|portion| FdcFoodPortionForInsert {
                                        value: portion.value,
                                        measure_unit_id: measurement_units_map
                                            .get(&(
                                                portion.measure_unit.name.clone(),
                                                portion.measure_unit.abbreviation.clone(),
                                            ))
                                            .cloned()
                                            .unwrap(),
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
                for (ff, fdc_food_db_id) in foundation_foods.iter().zip(fdc_foods_ids) {
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
}
