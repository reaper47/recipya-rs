use std::{
    collections::{HashMap, VecDeque},
    io::Cursor,
    marker::PhantomData,
};

use async_trait::async_trait;
use diesel::{
    dsl::sql,
    prelude::*,
    sql_types::{BigInt, Double, Float8, Nullable, Text},
};
use diesel_async::{AsyncConnection as _, RunQueryDsl};
use reqwest::Client;

use models::nutrition::{
    FdcFoodFdcNutrientForInsert, FdcFoodPortionFdcFoodForInsert, FdcFoodPortionForInsert,
    FdcNutrientForInsert, FoundationFoodForInsert, MeasureUnitForInsert,
};
use repository::{ModelManager, schema};

use crate::{
    Error, Result,
    fdc::foundation_food::{FoundationFood, FoundationFoodRoot, MeasureUnit},
    states::{DataFetchedState, DataNotFetchedState},
};

#[async_trait]
pub trait DataNotFetched<C: FdcFetcher>: Send + Sync {
    /// Fetches the following data sets from the FoodData Central API:
    /// - Foundation Food Data
    async fn fetch(self, client: &C) -> Result<FdcParser<DataFetchedState>>;
}

#[async_trait]
pub trait DataFetched: Send + Sync {
    /// Pushes the data sets fetched from the FoodData Central API into the database.
    async fn push_into_database<'a>(&'a self, mm: &'a ModelManager) -> Result<()>;
}

#[async_trait]
pub trait FdcFetcher: Send + Sync {
    async fn fetch_foundation_foods(&self) -> Result<Vec<u8>>;
}

/// The HTTP client for fetching data sets from Food Data Central.
pub struct FdcClient {
    client: Client,
}

#[async_trait]
impl FdcFetcher for FdcClient {
    async fn fetch_foundation_foods(&self) -> Result<Vec<u8>> {
        todo!()
    }
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

#[async_trait]
impl<C: FdcFetcher> DataNotFetched<C> for FdcParser<DataNotFetchedState> {
    async fn fetch(self, client: &C) -> Result<FdcParser<DataFetchedState>> {
        let res = client.fetch_foundation_foods().await?;
        let data: FoundationFoodRoot = serde_json::from_reader(Cursor::new(res))?;

        Ok(FdcParser {
            foundation_food_data: data.foundation_foods,
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

/// Contains all the details of a foundation food.
#[derive(Debug, PartialEq)]
pub struct FoundationFoodDetails {
    pub id: i64,
    pub fdc_id: i64,
    pub food_class: String,
    pub description: String,
    pub food_nutrients: Vec<NutrientDetails>,
    pub food_category: String,
    pub food_portions: Vec<PortionDetails>,
}

/// Represents a nutrient with its details.
#[derive(Debug, PartialEq, Queryable)]
pub struct NutrientDetails {
    pub id: i64,
    pub name: String,
    pub unit_name: String,
    pub median: Option<f64>,
    pub amount: f64,
}

/// Represents a food portion.
#[derive(Debug, PartialEq)]
pub struct PortionDetails {
    pub id: i64,
    pub value: f64,
    pub measure_unit: MeasureUnit,
    pub modifier: Option<String>,
    pub gram_weight: f64,
    pub amount: f64,
}

#[derive(Debug, PartialEq, Queryable)]
pub struct PortionDetailsRow {
    pub id: i64,
    pub value: f64,
    pub measure_unit_name: String,
    pub measure_unit_abbreviation: String,
    pub modifier: Option<String>,
    pub gram_weight: f64,
    pub amount: f64,
}

#[derive(Debug, QueryableByName)]
#[allow(unused)]
pub struct FdcFoodResult {
    #[diesel(sql_type = BigInt)]
    pub id: i64,
    #[diesel(sql_type = BigInt)]
    pub fdc_id: i64,
    #[diesel(sql_type = Text)]
    pub food_class: String,
    #[diesel(sql_type = Text)]
    pub description: String,
    #[diesel(sql_type = Text)]
    pub food_category: String,
    #[diesel(sql_type = Float8)]
    pub rank: f64,
}

impl FoundationFoodDetails {
    /// Retrieves the most relevant details of a foundation food from the database.
    pub async fn get_relevant(mm: &ModelManager, food: &str) -> Result<Vec<FoundationFoodDetails>> {
        let mut conn = mm.pool.get().await?;

        let food_results = diesel::sql_query(
            r#"
            WITH q AS (
              SELECT
                plainto_tsquery('english', $1) AS plain_q,
                phraseto_tsquery('english', $1) AS phrase_q
            )
            SELECT
              id,
              fdc_id,
              food_class,
              description,
              food_category,
              (
                -- Base rank
                ts_rank(description_tsv, q.plain_q) * 1.0 +
                -- Boost if main food type matches
                CASE WHEN to_tsvector('english', split_part(description, ',', 1)) @@ q.plain_q THEN
                  0.5
                ELSE
                  0.0
                END +
                -- Extra boost if it's an exact phrase match
                CASE WHEN description_tsv @@ q.phrase_q THEN
                  1.0
                ELSE
                  0.0
                END +
                -- Boost based on number of commas (fewer = more specific/simpler)
                (5.0 - LENGTH(description) + LENGTH(REPLACE(description, ',', ''))) / 10.0) AS rank
            FROM
              fdc_foods,
              q
            WHERE
              description_tsv @@ q.plain_q
            ORDER BY
              rank DESC
            LIMIT 10
            "#,
        )
        .bind::<Text, _>(food)
        .load::<FdcFoodResult>(&mut conn)
        .await?;

        let mut foods = Vec::with_capacity(food_results.len());

        for food in food_results {
            foods.push(FoundationFoodDetails {
                id: food.id,
                fdc_id: food.fdc_id,
                food_class: food.food_class,
                description: food.description,
                food_nutrients: schema::fdc_foods_fdc_nutrients::table
                    .inner_join(schema::fdc_nutrients::table)
                    .filter(schema::fdc_foods_fdc_nutrients::food_id.eq(food.id))
                    .select((
                        schema::fdc_nutrients::id,
                        schema::fdc_nutrients::name,
                        schema::fdc_nutrients::unit_name,
                        sql::<Nullable<Double>>("NULLIF(fdc_foods_fdc_nutrients.median, 0.0)"),
                        schema::fdc_foods_fdc_nutrients::amount,
                    ))
                    .load::<NutrientDetails>(&mut conn)
                    .await?,
                food_category: food.food_category,
                food_portions: schema::fdc_food_portions_fdc_foods::table
                    .inner_join(schema::fdc_food_portions::table)
                    .inner_join(schema::measure_units::table.on(
                        schema::measure_units::id.eq(schema::fdc_food_portions::measure_unit_id),
                    ))
                    .filter(schema::fdc_food_portions_fdc_foods::food_id.eq(food.id))
                    .select((
                        schema::fdc_food_portions::id,
                        schema::fdc_food_portions::value,
                        schema::measure_units::name,
                        schema::measure_units::abbreviation,
                        sql::<Nullable<Text>>("NULLIF(fdc_food_portions.modifier, '')"),
                        schema::fdc_food_portions::gram_weight,
                        schema::fdc_food_portions::amount,
                    ))
                    .load::<PortionDetailsRow>(&mut conn)
                    .await?
                    .into_iter()
                    .map(|row| PortionDetails {
                        id: row.id,
                        value: row.value,
                        measure_unit: MeasureUnit {
                            id: 1,
                            name: row.measure_unit_name,
                            abbreviation: row.measure_unit_abbreviation,
                        },
                        modifier: row.modifier,
                        gram_weight: row.gram_weight,
                        amount: row.amount,
                    })
                    .collect::<Vec<_>>(),
            })
        }

        Ok(foods)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use testing::utils::{TestDb, create_app_state};

    use super::*;

    type Result<T> = core::result::Result<T, Error>;
    type Error = Box<dyn std::error::Error>;

    const FDC_FF_DATASET_1: &'static str = "ff dataset 1";
    const FDC_FF_DATASET_2: &'static str = "ff dataset 2";

    struct FdcClientForTests {
        datasets: HashMap<String, String>,
        selected_dataset: String,
    }

    impl FdcClientForTests {
        fn new(selected_dataset: impl Into<String>) -> Self {
            Self {
                selected_dataset: selected_dataset.into(),
                datasets: HashMap::from([
                    (
                        FDC_FF_DATASET_1.to_string(),
                        r#"{"FoundationFoods":[{"foodClass":"FinalFood","description":"Hummus, commercial","foodNutrients":[{"type":"FoodNutrient","id":2219824,"nutrient":{"id":2026,"number":"840","name":"PUFA 20:2 c","rank":14250,"unitName":"g"},"foodNutrientDerivation":{"code":"AS","description":"Summed","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"amount":0.005},{"type":"FoodNutrient","id":2219825,"nutrient":{"id":1085,"number":"298","name":"Total fat (NLEA)","rank":900,"unitName":"g"},"foodNutrientDerivation":{"code":"AS","description":"Summed","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"amount":16.1}],"foodAttributes":[],"nutrientConversionFactors":[{"type":".CalorieConversionFactor","proteinValue":3.47,"fatValue":8.37,"carbohydrateValue":4.07},{"type":".ProteinConversionFactor","value":6.25}],"isHistoricalReference":false,"ndbNumber":16158,"dataType":"Foundation","foodCategory":{"description":"Legumes and Legume Products"},"fdcId":321358,"foodPortions":[{"id":118804,"value":2,"measureUnit":{"id":1001,"name":"tablespoon","abbreviation":"tbsp"},"modifier":"","gramWeight":33.9,"sequenceNumber":1,"amount":2,"minYearAcquired":2015}],"publicationDate":"4/1/2019","inputFoods":[{"id":10428,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319874,"publicationDate":"4/1/2019"}},{"id":10433,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319921,"publicationDate":"4/1/2019"}},{"id":10432,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319901,"publicationDate":"4/1/2019"}},{"id":10435,"foodDescription":"HUMMUS, TRIBE CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, TRIBE CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319958,"publicationDate":"4/1/2019"}},{"id":10429,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319879,"publicationDate":"4/1/2019"}},{"id":10436,"foodDescription":"HUMMUS, OTHER","inputFood":{"foodClass":"Composite","description":"HUMMUS, OTHER","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319978,"publicationDate":"4/1/2019"}},{"id":10431,"foodDescription":"HUMMUS, OTHER","inputFood":{"foodClass":"Composite","description":"HUMMUS, OTHER","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319894,"publicationDate":"4/1/2019"}},{"id":10430,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319885,"publicationDate":"4/1/2019"}},{"id":10434,"foodDescription":"HUMMUS, SABRA CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, SABRA CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319939,"publicationDate":"4/1/2019"}},{"id":10438,"foodDescription":"HUMMUS, OTHER","inputFood":{"foodClass":"Composite","description":"HUMMUS, OTHER","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":320005,"publicationDate":"4/1/2019"}},{"id":10437,"foodDescription":"HUMMUS, TRIBE CLASSIC","inputFood":{"foodClass":"Composite","description":"HUMMUS, TRIBE CLASSIC","dataType":"Sample","foodCategory":{"id":16,"code":"1600","description":"Legumes and Legume Products"},"fdcId":319984,"publicationDate":"4/1/2019"}}]},{"foodClass":"FinalFood","description":"Tomatoes, grape, raw","foodNutrients":[{"type":"FoodNutrient","id":2220032,"nutrient":{"id":1005,"number":"205","name":"Carbohydrate, by difference","rank":1110,"unitName":"g"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":5.51},{"type":"FoodNutrient","id":2220034,"nutrient":{"id":1008,"number":"208","name":"Energy","rank":300,"unitName":"kcal"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":27},{"type":"FoodNutrient","id":2220035,"nutrient":{"id":1062,"number":"268","name":"Energy","rank":400,"unitName":"kJ"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":113},{"type":"FoodNutrient","id":33291134,"nutrient":{"id":2066,"number":"333","name":"Vitamin A","rank":7420,"unitName":"mg"},"foodNutrientDerivation":{"foodNutrientSource":{}}}],"scientificName":"Solanum lycopersicum","foodAttributes":[],"nutrientConversionFactors":[{"type":".ProteinConversionFactor","value":6.25},{"type":".CalorieConversionFactor","proteinValue":2.44,"fatValue":8.37,"carbohydrateValue":3.57}],"isHistoricalReference":false,"ndbNumber":100147,"dataType":"Foundation","foodCategory":{"description":"Vegetables and Vegetable Products"},"fdcId":321360,"foodPortions":[{"id":118808,"value":5,"measureUnit":{"id":1082,"name":"tomatoes","abbreviation":"tomatoes"},"modifier":"","gramWeight":49.7,"sequenceNumber":1,"amount":5,"minYearAcquired":2016},{"id":118809,"value":1,"measureUnit":{"id":1000,"name":"cup","abbreviation":"cup"},"modifier":"","gramWeight":152,"sequenceNumber":2,"amount":1,"minYearAcquired":2016}],"publicationDate":"4/1/2019","inputFoods":[{"id":10480,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320512,"publicationDate":"4/1/2019"}},{"id":10472,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320476,"publicationDate":"4/1/2019"}},{"id":10473,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320480,"publicationDate":"4/1/2019"}},{"id":10477,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320504,"publicationDate":"4/1/2019"}},{"id":10468,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320438,"publicationDate":"4/1/2019"}},{"id":10475,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320489,"publicationDate":"4/1/2019"}},{"id":10464,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320418,"publicationDate":"4/1/2019"}},{"id":10479,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320507,"publicationDate":"4/1/2019"}},{"id":10471,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320460,"publicationDate":"4/1/2019"}},{"id":10465,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320423,"publicationDate":"4/1/2019"}},{"id":10463,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320413,"publicationDate":"4/1/2019"}},{"id":10474,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320485,"publicationDate":"4/1/2019"}},{"id":10470,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320451,"publicationDate":"4/1/2019"}},{"id":10476,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320494,"publicationDate":"4/1/2019"}},{"id":10478,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320499,"publicationDate":"4/1/2019"}},{"id":10469,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320443,"publicationDate":"4/1/2019"}},{"id":10466,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320428,"publicationDate":"4/1/2019"}},{"id":10467,"foodDescription":"TOMATOES, GRAPE","inputFood":{"foodClass":"Composite","description":"TOMATOES, GRAPE","dataType":"Sample","foodCategory":{"id":11,"code":"1100","description":"Vegetables and Vegetable Products"},"fdcId":320433,"publicationDate":"4/1/2019"}}]}]}"#.to_string(),
                    ),
                    (
                        FDC_FF_DATASET_2.into(),
                        r#"{"FoundationFoods":[{"foodClass":"FinalFood","description":"Egg, whole, raw, frozen, pasteurized","foodNutrients":[{"type":"FoodNutrient","id":2229844,"nutrient":{"id":1090,"number":"304","name":"Magnesium, Mg","rank":5500,"unitName":"mg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":12.4,"min":10.1,"median":11.2,"amount":11.2},{"type":"FoodNutrient","id":2229845,"nutrient":{"id":1091,"number":"305","name":"Phosphorus, P","rank":5600,"unitName":"mg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":208,"min":178,"median":188,"amount":189},{"type":"FoodNutrient","id":2229846,"nutrient":{"id":1101,"number":"315","name":"Manganese, Mn","rank":6100,"unitName":"mg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2229847,"nutrient":{"id":1092,"number":"306","name":"Potassium, K","rank":5700,"unitName":"mg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":125,"min":109,"median":117,"amount":117},{"type":"FoodNutrient","id":2229848,"nutrient":{"id":1095,"number":"309","name":"Zinc, Zn","rank":5900,"unitName":"mg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":1.32,"min":1.06,"median":1.21,"amount":1.2},{"type":"FoodNutrient","id":2229849,"nutrient":{"id":1093,"number":"307","name":"Sodium, Na","rank":5800,"unitName":"mg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":137,"min":100,"median":121,"amount":121},{"type":"FoodNutrient","id":2229850,"nutrient":{"id":1089,"number":"303","name":"Iron, Fe","rank":5400,"unitName":"mg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":2.01,"min":1.56,"median":1.76,"amount":1.77},{"type":"FoodNutrient","id":2229851,"nutrient":{"id":1062,"number":"268","name":"Energy","rank":400,"unitName":"kJ"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":627},{"type":"FoodNutrient","id":2229852,"nutrient":{"id":1003,"number":"203","name":"Protein","rank":600,"unitName":"g"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"max":12.6,"min":11.8,"median":12.3,"amount":12.3},{"type":"FoodNutrient","id":2229853,"nutrient":{"id":1007,"number":"207","name":"Ash","rank":1000,"unitName":"g"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":1.28,"min":1,"median":1.18,"amount":1.16},{"type":"FoodNutrient","id":2229854,"nutrient":{"id":1111,"number":"325","name":"Vitamin D2 (ergocalciferol)","rank":8710,"unitName":"µg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2229855,"nutrient":{"id":1051,"number":"255","name":"Water","rank":100,"unitName":"g"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":76.7,"min":74,"median":75.4,"amount":75.4},{"type":"FoodNutrient","id":2229856,"nutrient":{"id":1087,"number":"301","name":"Calcium, Ca","rank":5300,"unitName":"mg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":62,"min":47,"median":56,"amount":55},{"type":"FoodNutrient","id":2229857,"nutrient":{"id":1004,"number":"204","name":"Total lipid (fat)","rank":800,"unitName":"g"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":11.5,"min":9.39,"median":10.2,"amount":10.3},{"type":"FoodNutrient","id":2229858,"nutrient":{"id":1005,"number":"205","name":"Carbohydrate, by difference","rank":1110,"unitName":"g"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":0.91},{"type":"FoodNutrient","id":2229859,"nutrient":{"id":1008,"number":"208","name":"Energy","rank":300,"unitName":"kcal"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":150},{"type":"FoodNutrient","id":2229860,"nutrient":{"id":1098,"number":"312","name":"Copper, Cu","rank":6000,"unitName":"mg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2229861,"nutrient":{"id":1112,"number":"326","name":"Vitamin D3 (cholecalciferol)","rank":8720,"unitName":"µg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":5.1,"min":0.8,"median":1.9,"amount":2.3},{"type":"FoodNutrient","id":2229862,"nutrient":{"id":1114,"number":"328","name":"Vitamin D (D2 + D3)","rank":8700,"unitName":"µg"},"foodNutrientDerivation":{"code":"AS","description":"Summed","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"amount":2.3},{"type":"FoodNutrient","id":2229863,"nutrient":{"id":1110,"number":"324","name":"Vitamin D (D2 + D3), International Units","rank":8650,"unitName":"IU"},"foodNutrientDerivation":{"code":"AS","description":"Summed","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"amount":91},{"type":"FoodNutrient","id":2229864,"nutrient":{"id":1253,"number":"601","name":"Cholesterol","rank":15700,"unitName":"mg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":463,"min":385,"median":416,"amount":420},{"type":"FoodNutrient","id":2229865,"nutrient":{"id":1113,"number":"327","name":"25-hydroxycholecalciferol","rank":8730,"unitName":"µg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0.8,"min":0.4,"median":0.6,"amount":0.6},{"type":"FoodNutrient","id":2229866,"nutrient":{"id":1002,"number":"202","name":"Nitrogen","rank":500,"unitName":"g"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":2.02,"min":1.89,"median":1.98,"amount":1.97},{"type":"FoodNutrient","id":2229867,"nutrient":{"id":1100,"number":"314","name":"Iodine, I","rank":6150,"unitName":"µg"},"dataPoints":14,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":80.9,"min":45.1,"median":56.1,"amount":61.6}],"foodAttributes":[],"nutrientConversionFactors":[{"type":".ProteinConversionFactor","value":6.25},{"type":".CalorieConversionFactor","proteinValue":4.36,"fatValue":9.02,"carbohydrateValue":3.68}],"isHistoricalReference":false,"ndbNumber":1171,"dataType":"Foundation","foodCategory":{"description":"Dairy and Egg Products"},"fdcId":323604,"foodPortions":[{"id":119060,"value":1,"measureUnit":{"id":1038,"name":"oz","abbreviation":"oz"},"modifier":"","gramWeight":28.4,"sequenceNumber":1,"amount":1,"minYearAcquired":2017}],"publicationDate":"4/1/2019","inputFoods":[{"id":10657,"foodDescription":"EGG, LIQUID WHOLE, Producer F, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer F, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323576,"publicationDate":"4/1/2019"}},{"id":10659,"foodDescription":"EGG, LIQUID WHOLE, Producer B, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer B, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323590,"publicationDate":"4/1/2019"}},{"id":10655,"foodDescription":"EGG, LIQUID WHOLE, Producer F, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer F, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323562,"publicationDate":"4/1/2019"}},{"id":10651,"foodDescription":"EGG, LIQUID WHOLE, Producer C, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer C, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323534,"publicationDate":"4/1/2019"}},{"id":10654,"foodDescription":"EGG, LIQUID WHOLE, Producer A, LOT 2","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer A, LOT 2","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323555,"publicationDate":"4/1/2019"}},{"id":10652,"foodDescription":"EGG, LIQUID WHOLE, Producer D, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer D, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323541,"publicationDate":"4/1/2019"}},{"id":10660,"foodDescription":"EGG, LIQUID WHOLE, Producer B, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer B, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323597,"publicationDate":"4/1/2019"}},{"id":10656,"foodDescription":"EGG, LIQUID WHOLE, Producer B, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer B, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323569,"publicationDate":"4/1/2019"}},{"id":10658,"foodDescription":"EGG, LIQUID WHOLE, Producer F, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer F, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323583,"publicationDate":"4/1/2019"}},{"id":10647,"foodDescription":"EGG, LIQUID WHOLE, Producer C, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer C, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323506,"publicationDate":"4/1/2019"}},{"id":10650,"foodDescription":"EGG, LIQUID WHOLE, Producer C, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer C, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323527,"publicationDate":"4/1/2019"}},{"id":10649,"foodDescription":"EGG, LIQUID WHOLE, Producer D, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer D, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323520,"publicationDate":"4/1/2019"}},{"id":10653,"foodDescription":"EGG, LIQUID WHOLE, Producer A, LOT 2","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer A, LOT 2","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323548,"publicationDate":"4/1/2019"}},{"id":10648,"foodDescription":"EGG, LIQUID WHOLE, Producer D, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHOLE, Producer D, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323513,"publicationDate":"4/1/2019"}}]},{"foodClass":"FinalFood","description":"Egg, white, raw, frozen, pasteurized","foodNutrients":[{"type":"FoodNutrient","id":2230115,"nutrient":{"id":1092,"number":"306","name":"Potassium, K","rank":5700,"unitName":"mg"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":176,"min":77,"median":134,"amount":130},{"type":"FoodNutrient","id":2230116,"nutrient":{"id":1095,"number":"309","name":"Zinc, Zn","rank":5900,"unitName":"mg"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0.31,"min":0,"median":0,"amount":0.02},{"type":"FoodNutrient","id":2230117,"nutrient":{"id":1110,"number":"324","name":"Vitamin D (D2 + D3), International Units","rank":8650,"unitName":"IU"},"foodNutrientDerivation":{"code":"AS","description":"Summed","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"amount":0},{"type":"FoodNutrient","id":2230118,"nutrient":{"id":1093,"number":"307","name":"Sodium, Na","rank":5800,"unitName":"mg"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":164,"min":86,"median":150,"amount":144},{"type":"FoodNutrient","id":2230119,"nutrient":{"id":1098,"number":"312","name":"Copper, Cu","rank":6000,"unitName":"mg"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230120,"nutrient":{"id":1090,"number":"304","name":"Magnesium, Mg","rank":5500,"unitName":"mg"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":12,"min":9,"median":10.5,"amount":10.6},{"type":"FoodNutrient","id":2230121,"nutrient":{"id":1112,"number":"326","name":"Vitamin D3 (cholecalciferol)","rank":8720,"unitName":"µg"},"dataPoints":6,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230122,"nutrient":{"id":1114,"number":"328","name":"Vitamin D (D2 + D3)","rank":8700,"unitName":"µg"},"foodNutrientDerivation":{"code":"AS","description":"Summed","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"amount":0},{"type":"FoodNutrient","id":2230123,"nutrient":{"id":1051,"number":"255","name":"Water","rank":100,"unitName":"g"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":89,"min":87.9,"median":88.3,"amount":88.3},{"type":"FoodNutrient","id":2230124,"nutrient":{"id":1111,"number":"325","name":"Vitamin D2 (ergocalciferol)","rank":8710,"unitName":"µg"},"dataPoints":6,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230125,"nutrient":{"id":1087,"number":"301","name":"Calcium, Ca","rank":5300,"unitName":"mg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":20,"min":5,"median":8,"amount":9},{"type":"FoodNutrient","id":2230126,"nutrient":{"id":1004,"number":"204","name":"Total lipid (fat)","rank":800,"unitName":"g"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0.26,"min":0.1,"median":0.15,"amount":0.16},{"type":"FoodNutrient","id":2230127,"nutrient":{"id":1005,"number":"205","name":"Carbohydrate, by difference","rank":1110,"unitName":"g"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":0.74},{"type":"FoodNutrient","id":2230128,"nutrient":{"id":1008,"number":"208","name":"Energy","rank":300,"unitName":"kcal"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":48},{"type":"FoodNutrient","id":2230129,"nutrient":{"id":1062,"number":"268","name":"Energy","rank":400,"unitName":"kJ"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":201},{"type":"FoodNutrient","id":2230130,"nutrient":{"id":1003,"number":"203","name":"Protein","rank":600,"unitName":"g"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"max":10.6,"min":9.5,"median":10.1,"amount":10.1},{"type":"FoodNutrient","id":2230131,"nutrient":{"id":1007,"number":"207","name":"Ash","rank":1000,"unitName":"g"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0.77,"min":0.62,"median":0.71,"amount":0.7},{"type":"FoodNutrient","id":2230132,"nutrient":{"id":1089,"number":"303","name":"Iron, Fe","rank":5400,"unitName":"mg"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":1.49,"min":0,"median":0,"amount":0.18},{"type":"FoodNutrient","id":2230133,"nutrient":{"id":1253,"number":"601","name":"Cholesterol","rank":15700,"unitName":"mg"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":5,"min":1,"median":2,"amount":3},{"type":"FoodNutrient","id":2230134,"nutrient":{"id":1091,"number":"305","name":"Phosphorus, P","rank":5600,"unitName":"mg"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230135,"nutrient":{"id":1101,"number":"315","name":"Manganese, Mn","rank":6100,"unitName":"mg"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230136,"nutrient":{"id":1002,"number":"202","name":"Nitrogen","rank":500,"unitName":"g"},"dataPoints":16,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":1.7,"min":1.52,"median":1.62,"amount":1.61},{"type":"FoodNutrient","id":2230137,"nutrient":{"id":1100,"number":"314","name":"Iodine, I","rank":6150,"unitName":"µg"},"dataPoints":6,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230138,"nutrient":{"id":1113,"number":"327","name":"25-hydroxycholecalciferol","rank":8730,"unitName":"µg"},"dataPoints":6,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0}],"foodAttributes":[],"nutrientConversionFactors":[{"type":".ProteinConversionFactor","value":6.25},{"type":".CalorieConversionFactor","proteinValue":4.36,"fatValue":9.02,"carbohydrateValue":3.68}],"isHistoricalReference":false,"ndbNumber":1172,"dataType":"Foundation","foodCategory":{"description":"Dairy and Egg Products"},"fdcId":323697,"foodPortions":[{"id":119063,"value":1,"measureUnit":{"id":1038,"name":"oz","abbreviation":"oz"},"modifier":"","gramWeight":28.4,"sequenceNumber":1,"amount":1,"minYearAcquired":2017}],"publicationDate":"4/1/2019","inputFoods":[{"id":10663,"foodDescription":"EGG, LIQUID WHITE, Producer D, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer D, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323616,"publicationDate":"4/1/2019"}},{"id":10665,"foodDescription":"EGG, LIQUID WHITE, Producer E, LOT 1","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer E, LOT 1","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323627,"publicationDate":"4/1/2019"}},{"id":10661,"foodDescription":"EGG, LIQUID WHITE, Producer D, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer D, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323605,"publicationDate":"4/1/2019"}},{"id":10667,"foodDescription":"EGG, LIQUID WHITE, Producer C, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer C, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323641,"publicationDate":"4/1/2019"}},{"id":10673,"foodDescription":"EGG, LIQUID WHITE, Producer B, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer B, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323674,"publicationDate":"4/1/2019"}},{"id":10668,"foodDescription":"EGG, LIQUID WHITE, Producer A, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer A, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323646,"publicationDate":"4/1/2019"}},{"id":10671,"foodDescription":"EGG, LIQUID WHITE, Producer F, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer F, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323663,"publicationDate":"4/1/2019"}},{"id":10676,"foodDescription":"EGG, LIQUID WHITE, Producer B, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer B, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323692,"publicationDate":"4/1/2019"}},{"id":10664,"foodDescription":"EGG, LIQUID WHITE, Producer C, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer C, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323622,"publicationDate":"4/1/2019"}},{"id":10674,"foodDescription":"EGG, LIQUID WHITE, Producer F, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer F, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323679,"publicationDate":"4/1/2019"}},{"id":10666,"foodDescription":"EGG, LIQUID WHITE, Producer C, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer C, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323634,"publicationDate":"4/1/2019"}},{"id":10675,"foodDescription":"EGG, LIQUID WHITE, Producer B, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer B, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323685,"publicationDate":"4/1/2019"}},{"id":10670,"foodDescription":"EGG, LIQUID WHITE, Producer A, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer A, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323657,"publicationDate":"4/1/2019"}},{"id":10662,"foodDescription":"EGG, LIQUID WHITE, Producer D, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer D, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323610,"publicationDate":"4/1/2019"}},{"id":10672,"foodDescription":"EGG, LIQUID WHITE, Producer F, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer F, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323669,"publicationDate":"4/1/2019"}},{"id":10669,"foodDescription":"EGG, LIQUID WHITE, Producer A, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, LIQUID WHITE, Producer A, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323651,"publicationDate":"4/1/2019"}}]},{"foodClass":"FinalFood","description":"Egg, white, dried","foodNutrients":[{"type":"FoodNutrient","id":2230379,"nutrient":{"id":1062,"number":"268","name":"Energy","rank":400,"unitName":"kJ"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":1570},{"type":"FoodNutrient","id":2230380,"nutrient":{"id":1003,"number":"203","name":"Protein","rank":600,"unitName":"g"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"max":82.7,"min":77.9,"median":79.8,"amount":79.9},{"type":"FoodNutrient","id":2230381,"nutrient":{"id":1007,"number":"207","name":"Ash","rank":1000,"unitName":"g"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":6.13,"min":5.01,"median":5.45,"amount":5.47},{"type":"FoodNutrient","id":2230382,"nutrient":{"id":1089,"number":"303","name":"Iron, Fe","rank":5400,"unitName":"mg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230383,"nutrient":{"id":1090,"number":"304","name":"Magnesium, Mg","rank":5500,"unitName":"mg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":93.6,"min":82.7,"median":87.8,"amount":87.6},{"type":"FoodNutrient","id":2230384,"nutrient":{"id":1091,"number":"305","name":"Phosphorus, P","rank":5600,"unitName":"mg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":112,"min":97,"median":107,"amount":107},{"type":"FoodNutrient","id":2230385,"nutrient":{"id":1093,"number":"307","name":"Sodium, Na","rank":5800,"unitName":"mg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":1450,"min":1160,"median":1230,"amount":1250},{"type":"FoodNutrient","id":2230386,"nutrient":{"id":1098,"number":"312","name":"Copper, Cu","rank":6000,"unitName":"mg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230387,"nutrient":{"id":1101,"number":"315","name":"Manganese, Mn","rank":6100,"unitName":"mg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230388,"nutrient":{"id":1253,"number":"601","name":"Cholesterol","rank":15700,"unitName":"mg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":26,"min":9,"median":15,"amount":17},{"type":"FoodNutrient","id":2230389,"nutrient":{"id":1087,"number":"301","name":"Calcium, Ca","rank":5300,"unitName":"mg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":168,"min":61,"median":113,"amount":104},{"type":"FoodNutrient","id":2230390,"nutrient":{"id":1092,"number":"306","name":"Potassium, K","rank":5700,"unitName":"mg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":1070,"min":868,"median":953,"amount":959},{"type":"FoodNutrient","id":2230391,"nutrient":{"id":1095,"number":"309","name":"Zinc, Zn","rank":5900,"unitName":"mg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0.65,"min":0,"median":0.49,"amount":0.43},{"type":"FoodNutrient","id":2230392,"nutrient":{"id":1004,"number":"204","name":"Total lipid (fat)","rank":800,"unitName":"g"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0.82,"min":0.34,"median":0.63,"amount":0.65},{"type":"FoodNutrient","id":2230393,"nutrient":{"id":1005,"number":"205","name":"Carbohydrate, by difference","rank":1110,"unitName":"g"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":6.02},{"type":"FoodNutrient","id":2230394,"nutrient":{"id":1008,"number":"208","name":"Energy","rank":300,"unitName":"kcal"},"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":376},{"type":"FoodNutrient","id":2230395,"nutrient":{"id":1051,"number":"255","name":"Water","rank":100,"unitName":"g"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":8.85,"min":5.96,"median":8.26,"amount":7.98},{"type":"FoodNutrient","id":2230396,"nutrient":{"id":1110,"number":"324","name":"Vitamin D (D2 + D3), International Units","rank":8650,"unitName":"IU"},"foodNutrientDerivation":{"code":"AS","description":"Summed","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"amount":0},{"type":"FoodNutrient","id":2230397,"nutrient":{"id":1114,"number":"328","name":"Vitamin D (D2 + D3)","rank":8700,"unitName":"µg"},"foodNutrientDerivation":{"code":"AS","description":"Summed","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"amount":0},{"type":"FoodNutrient","id":2230398,"nutrient":{"id":1100,"number":"314","name":"Iodine, I","rank":6150,"unitName":"µg"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":59,"min":22.1,"median":29.7,"amount":34},{"type":"FoodNutrient","id":2230399,"nutrient":{"id":1112,"number":"326","name":"Vitamin D3 (cholecalciferol)","rank":8720,"unitName":"µg"},"dataPoints":5,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230400,"nutrient":{"id":1113,"number":"327","name":"25-hydroxycholecalciferol","rank":8730,"unitName":"µg"},"dataPoints":5,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0.1,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230401,"nutrient":{"id":1111,"number":"325","name":"Vitamin D2 (ergocalciferol)","rank":8710,"unitName":"µg"},"dataPoints":5,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0,"min":0,"median":0,"amount":0},{"type":"FoodNutrient","id":2230402,"nutrient":{"id":1002,"number":"202","name":"Nitrogen","rank":500,"unitName":"g"},"dataPoints":15,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":13.2,"min":12.5,"median":12.8,"amount":12.8}],"foodAttributes":[],"nutrientConversionFactors":[{"type":".ProteinConversionFactor","value":6.25},{"type":".CalorieConversionFactor","proteinValue":4.36,"fatValue":9.02,"carbohydrateValue":3.68}],"isHistoricalReference":false,"ndbNumber":1173,"dataType":"Foundation","foodCategory":{"description":"Dairy and Egg Products"},"fdcId":323793,"foodPortions":[{"id":119069,"value":1,"measureUnit":{"id":1001,"name":"tablespoon","abbreviation":"tbsp"},"modifier":"","gramWeight":7,"sequenceNumber":1,"amount":1,"minYearAcquired":2017},{"id":119070,"value":1,"measureUnit":{"id":1000,"name":"cup","abbreviation":"cup"},"modifier":"sifted","gramWeight":107,"sequenceNumber":2,"amount":1,"minYearAcquired":2017}],"publicationDate":"4/1/2019","inputFoods":[{"id":10679,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer E, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer E, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323711,"publicationDate":"4/1/2019"}},{"id":10684,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer D, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer D, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323742,"publicationDate":"4/1/2019"}},{"id":10680,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer E, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer E, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323718,"publicationDate":"4/1/2019"}},{"id":10683,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer D, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer D, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323736,"publicationDate":"4/1/2019"}},{"id":10682,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer C, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer C, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323730,"publicationDate":"4/1/2019"}},{"id":10681,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer C, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer C, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323724,"publicationDate":"4/1/2019"}},{"id":10685,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer B, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer B, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323748,"publicationDate":"4/1/2019"}},{"id":10689,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer A, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer A, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323774,"publicationDate":"4/1/2019"}},{"id":10691,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer A, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer A, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323787,"publicationDate":"4/1/2019"}},{"id":10686,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer B, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer B, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323754,"publicationDate":"4/1/2019"}},{"id":10687,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer D, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer D, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323760,"publicationDate":"4/1/2019"}},{"id":10690,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer A, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer A, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323780,"publicationDate":"4/1/2019"}},{"id":10678,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer E, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer E, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323705,"publicationDate":"4/1/2019"}},{"id":10688,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer B, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer B, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323767,"publicationDate":"4/1/2019"}},{"id":10677,"foodDescription":"EGG, DRIED WHITE STABILIZED, Producer C, LOT 3","inputFood":{"foodClass":"Composite","description":"EGG, DRIED WHITE STABILIZED, Producer C, LOT 3","dataType":"Sample","foodCategory":{"id":1,"code":"0100","description":"Dairy and Egg Products"},"fdcId":323698,"publicationDate":"4/1/2019"}}]}]}"#.into(),
                    ),
                ]),
            }
        }
    }

    #[async_trait]
    impl FdcFetcher for FdcClientForTests {
        async fn fetch_foundation_foods(&self) -> super::Result<Vec<u8>> {
            let dataset = self
                .datasets
                .get(&self.selected_dataset)
                .cloned()
                .expect(&format!("dataset {}", self.selected_dataset));

            Ok(dataset.into_bytes())
        }
    }

    #[tokio::test]
    async fn test_parses_correctly_simple_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config).await;
        let client = FdcClientForTests::new(FDC_FF_DATASET_1);
        let parser: FdcParser<DataFetchedState> = FdcParser::new().fetch(&client).await?;
        parser.push_into_database(&state.mm).await?;

        let foods = FoundationFoodDetails::get_relevant(&state.mm, "hummus").await?;

        pretty_assertions::assert_eq!(
            foods,
            vec![FoundationFoodDetails {
                id: 1,
                fdc_id: 321358,
                food_class: "FinalFood".into(),
                description: "Hummus, commercial".into(),
                food_nutrients: vec![
                    NutrientDetails {
                        id: 1,
                        name: "PUFA 20:2 c".into(),
                        unit_name: String::from("g"),
                        median: None,
                        amount: 0.005,
                    },
                    NutrientDetails {
                        id: 2,
                        name: "Total fat (NLEA)".into(),
                        unit_name: String::from("g"),
                        median: None,
                        amount: 16.1,
                    }
                ],
                food_category: "Legumes and Legume Products".into(),
                food_portions: vec![PortionDetails {
                    id: 1,
                    value: 2.0,
                    measure_unit: MeasureUnit {
                        id: 1,
                        name: "tablespoon".into(),
                        abbreviation: "tbsp".into()
                    },
                    modifier: None,
                    gram_weight: 33.9,
                    amount: 2.0,
                }],
            }]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_parses_correctly_complex_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config).await;
        let client = FdcClientForTests::new(FDC_FF_DATASET_2);
        let parser: FdcParser<DataFetchedState> = FdcParser::new().fetch(&client).await?;
        parser.push_into_database(&state.mm).await?;

        let foods = FoundationFoodDetails::get_relevant(&state.mm, "egg frozen").await?;

        pretty_assertions::assert_eq!(
            foods,
            vec![
                FoundationFoodDetails {
                    id: 1,
                    fdc_id: 323604,
                    food_class: "FinalFood".into(),
                    description: "Egg, whole, raw, frozen, pasteurized".into(),
                    food_nutrients: vec![
                        NutrientDetails {
                            id: 1,
                            name: "Magnesium, Mg".into(),
                            unit_name: String::from("mg"),
                            median: Some(11.2),
                            amount: 11.2,
                        },
                        NutrientDetails {
                            id: 2,
                            name: "Phosphorus, P".into(),
                            unit_name: String::from("mg"),
                            median: Some(188.0),
                            amount: 189.0,
                        },
                        NutrientDetails {
                            id: 3,
                            name: "Manganese, Mn".into(),
                            unit_name: String::from("mg"),
                            median: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 4,
                            name: "Potassium, K".into(),
                            unit_name: String::from("mg"),
                            median: Some(117.0),
                            amount: 117.0,
                        },
                        NutrientDetails {
                            id: 5,
                            name: "Zinc, Zn".into(),
                            unit_name: String::from("mg"),
                            median: Some(1.21),
                            amount: 1.2,
                        },
                        NutrientDetails {
                            id: 6,
                            name: "Sodium, Na".into(),
                            unit_name: String::from("mg"),
                            median: Some(121.0),
                            amount: 121.0,
                        },
                        NutrientDetails {
                            id: 7,
                            name: "Iron, Fe".into(),
                            unit_name: String::from("mg"),
                            median: Some(1.76),
                            amount: 1.77,
                        },
                        NutrientDetails {
                            id: 8,
                            name: "Energy".into(),
                            unit_name: String::from("kJ"),
                            median: None,
                            amount: 627.0,
                        },
                        NutrientDetails {
                            id: 9,
                            name: "Protein".into(),
                            unit_name: String::from("g"),
                            median: Some(12.3),
                            amount: 12.3,
                        },
                        NutrientDetails {
                            id: 10,
                            name: "Ash".into(),
                            unit_name: String::from("g"),
                            median: Some(1.18),
                            amount: 1.16,
                        },
                        NutrientDetails {
                            id: 11,
                            name: "Vitamin D2 (ergocalciferol)".into(),
                            unit_name: "µg".into(),
                            median: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 12,
                            name: "Water".into(),
                            unit_name: "g".into(),
                            median: Some(75.4,),
                            amount: 75.4,
                        },
                        NutrientDetails {
                            id: 13,
                            name: "Calcium, Ca".into(),
                            unit_name: "mg".into(),
                            median: Some(56.0,),
                            amount: 55.0,
                        },
                        NutrientDetails {
                            id: 14,
                            name: "Total lipid (fat)".into(),
                            unit_name: "g".into(),
                            median: Some(10.2,),
                            amount: 10.3,
                        },
                        NutrientDetails {
                            id: 15,
                            name: "Carbohydrate, by difference".into(),
                            unit_name: "g".into(),
                            median: None,
                            amount: 0.91,
                        },
                        NutrientDetails {
                            id: 16,
                            name: "Energy".into(),
                            unit_name: "kcal".into(),
                            median: None,
                            amount: 150.0,
                        },
                        NutrientDetails {
                            id: 17,
                            name: "Copper, Cu".into(),
                            unit_name: "mg".into(),
                            median: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 18,
                            name: "Vitamin D3 (cholecalciferol)".into(),
                            unit_name: "µg".into(),
                            median: Some(1.9,),
                            amount: 2.3,
                        },
                        NutrientDetails {
                            id: 19,
                            name: "Vitamin D (D2 + D3)".into(),
                            unit_name: "µg".into(),
                            median: None,
                            amount: 2.3,
                        },
                        NutrientDetails {
                            id: 20,
                            name: "Vitamin D (D2 + D3), International Units".into(),
                            unit_name: "IU".into(),
                            median: None,
                            amount: 91.0,
                        },
                        NutrientDetails {
                            id: 21,
                            name: "Cholesterol".into(),
                            unit_name: "mg".into(),
                            median: Some(416.0,),
                            amount: 420.0,
                        },
                        NutrientDetails {
                            id: 22,
                            name: "25-hydroxycholecalciferol".into(),
                            unit_name: "µg".into(),
                            median: Some(0.6,),
                            amount: 0.6,
                        },
                        NutrientDetails {
                            id: 23,
                            name: "Nitrogen".into(),
                            unit_name: "g".into(),
                            median: Some(1.98,),
                            amount: 1.97,
                        },
                        NutrientDetails {
                            id: 24,
                            name: "Iodine, I".into(),
                            unit_name: "µg".into(),
                            median: Some(56.1,),
                            amount: 61.6,
                        },
                    ],
                    food_category: "Dairy and Egg Products".into(),
                    food_portions: vec![PortionDetails {
                        id: 1,
                        value: 1.0,
                        measure_unit: MeasureUnit {
                            id: 1,
                            name: "oz".into(),
                            abbreviation: "oz".into()
                        },
                        modifier: None,
                        gram_weight: 28.4,
                        amount: 1.0,
                    }],
                },
                FoundationFoodDetails {
                    id: 2,
                    fdc_id: 323697,
                    food_class: "FinalFood".into(),
                    description: "Egg, white, raw, frozen, pasteurized".into(),
                    food_nutrients: vec![
                        NutrientDetails {
                            id: 1,
                            name: "Magnesium, Mg".into(),
                            unit_name: "mg".into(),
                            median: Some(10.5,),
                            amount: 10.6,
                        },
                        NutrientDetails {
                            id: 2,
                            name: "Phosphorus, P".into(),
                            unit_name: "mg".into(),
                            median: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 3,
                            name: "Manganese, Mn".into(),
                            unit_name: "mg".into(),
                            median: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 4,
                            name: "Potassium, K".into(),
                            unit_name: "mg".into(),
                            median: Some(134.0,),
                            amount: 130.0,
                        },
                        NutrientDetails {
                            id: 5,
                            name: "Zinc, Zn".into(),
                            unit_name: "mg".into(),
                            median: None,
                            amount: 0.02,
                        },
                        NutrientDetails {
                            id: 6,
                            name: "Sodium, Na".into(),
                            unit_name: "mg".into(),
                            median: Some(150.0,),
                            amount: 144.0,
                        },
                        NutrientDetails {
                            id: 7,
                            name: "Iron, Fe".into(),
                            unit_name: "mg".into(),
                            median: None,
                            amount: 0.18,
                        },
                        NutrientDetails {
                            id: 8,
                            name: "Energy".into(),
                            unit_name: "kJ".into(),
                            median: None,
                            amount: 201.0,
                        },
                        NutrientDetails {
                            id: 9,
                            name: "Protein".into(),
                            unit_name: "g".into(),
                            median: Some(10.1,),
                            amount: 10.1,
                        },
                        NutrientDetails {
                            id: 10,
                            name: "Ash".into(),
                            unit_name: "g".into(),
                            median: Some(0.71,),
                            amount: 0.7,
                        },
                        NutrientDetails {
                            id: 11,
                            name: "Vitamin D2 (ergocalciferol)".into(),
                            unit_name: "µg".into(),
                            median: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 12,
                            name: "Water".into(),
                            unit_name: "g".into(),
                            median: Some(88.3,),
                            amount: 88.3,
                        },
                        NutrientDetails {
                            id: 13,
                            name: "Calcium, Ca".into(),
                            unit_name: "mg".into(),
                            median: Some(8.0,),
                            amount: 9.0,
                        },
                        NutrientDetails {
                            id: 14,
                            name: "Total lipid (fat)".into(),
                            unit_name: "g".into(),
                            median: Some(0.15,),
                            amount: 0.16,
                        },
                        NutrientDetails {
                            id: 15,
                            name: "Carbohydrate, by difference".into(),
                            unit_name: "g".into(),
                            median: None,
                            amount: 0.74,
                        },
                        NutrientDetails {
                            id: 16,
                            name: "Energy".into(),
                            unit_name: "kcal".into(),
                            median: None,
                            amount: 48.0,
                        },
                        NutrientDetails {
                            id: 17,
                            name: "Copper, Cu".into(),
                            unit_name: "mg".into(),
                            median: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 18,
                            name: "Vitamin D3 (cholecalciferol)".into(),
                            unit_name: "µg".into(),
                            median: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 19,
                            name: "Vitamin D (D2 + D3)".into(),
                            unit_name: "µg".into(),
                            median: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 20,
                            name: "Vitamin D (D2 + D3), International Units".into(),
                            unit_name: "IU".into(),
                            median: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 21,
                            name: "Cholesterol".into(),
                            unit_name: "mg".into(),
                            median: Some(2.0,),
                            amount: 3.0,
                        },
                        NutrientDetails {
                            id: 22,
                            name: "25-hydroxycholecalciferol".into(),
                            unit_name: "µg".into(),
                            median: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 23,
                            name: "Nitrogen".into(),
                            unit_name: "g".into(),
                            median: Some(1.62,),
                            amount: 1.61,
                        },
                        NutrientDetails {
                            id: 24,
                            name: "Iodine, I".into(),
                            unit_name: "µg".into(),
                            median: None,
                            amount: 0.0,
                        },
                    ],
                    food_category: "Dairy and Egg Products".into(),
                    food_portions: vec![PortionDetails {
                        id: 2,
                        value: 1.0,
                        measure_unit: MeasureUnit {
                            id: 1,
                            name: "oz".into(),
                            abbreviation: "oz".into()
                        },
                        modifier: None,
                        gram_weight: 28.4,
                        amount: 1.0,
                    }],
                }
            ]
        );
        Ok(())
    }
}
