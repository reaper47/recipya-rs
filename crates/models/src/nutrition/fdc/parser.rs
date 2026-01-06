use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    io::{Cursor, Read as _},
    marker::PhantomData,
};

use async_trait::async_trait;
use chrono::NaiveDate;
use diesel::{
    prelude::*,
    sql_types::{BigInt, Float8, Nullable, Text},
};
use diesel_async::{AsyncConnection as _, RunQueryDsl};
use reqwest::Client;
use scraper::{Html, Selector};
use tracing::error;
use zip::ZipArchive;

use repository::{ModelManager, schema};

use crate::{
    Error, Result,
    nutrition::{
        NutritionDataSource,
        fdc::srlegacy_food::{SrLegacyRoot, SrlegacyFood},
        states::{DataFetchedState, DataNotFetchedState},
        tables::{
            self, FdcFoodFdcNutrientForInsert, FdcFoodPortionFdcFoodForInsert,
            FdcFoodPortionForInsert, FdcNutrientForInsert, FoundationFoodForInsert,
            NutritionSource,
        },
    },
};

const FDC_DATASETS_DOWNLOAD_URL: &str = "https://fdc.nal.usda.gov/download-datasets";

const BATCH_SIZE: usize = 1000;

#[async_trait]
pub trait DataNotFetched<C: FdcFetcher>: Send + Sync {
    /// Fetches the following data sets from the USDA FoodData Central API:
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
    async fn fetch_foundation_foods(&self) -> Result<ZipArchive<Cursor<Vec<u8>>>>;
}

/// The HTTP client for fetching data sets from Food Data Central.
pub struct FdcClient<'a> {
    client: Client,
    mm: &'a ModelManager,
}

impl<'a> FdcClient<'a> {
    /// Creates a new instance of `FdcClient`.
    pub fn new(mm: &'a ModelManager) -> Self {
        Self {
            client: Client::new(),
            mm,
        }
    }
}

#[async_trait]
impl<'a> FdcFetcher for FdcClient<'a> {
    async fn fetch_foundation_foods(&self) -> Result<ZipArchive<Cursor<Vec<u8>>>> {
        let (foundation_food_release_date, json_url) = {
            let html = self
                .client
                .get(FDC_DATASETS_DOWNLOAD_URL)
                .send()
                .await
                .inspect_err(|_| error!("Failed to fetch USDA FDC download dataset page"))?
                .text()
                .await?;

            let document = Html::parse_document(&html);

            const BASE_SELECTOR: &str = "table.usa-table-results.usa-table-borderless.header-alignment.no-wrap tbody tr:nth-child(2)";

            let month_year_date = document
                .select(
                    &Selector::parse(&format!("{BASE_SELECTOR} td:nth-child(2)"))
                        .expect("valid css selector for foundation food release date"),
                )
                .next()
                .map(|element| element.text().collect::<String>().trim().to_string())
                .ok_or(Error::InvalidCssSelector)
                .inspect_err(|_| error!("Failed to parse foundation food release date"))?;

            let release_date =
                NaiveDate::parse_from_str(&format!("01/{month_year_date}"), "%d/%m/%Y")?;

            let json_url = document
                .select(
                    &Selector::parse(&format!("{BASE_SELECTOR} td:nth-child(3) > a"))
                        .expect("valid css selector for json url"),
                )
                .next()
                .map(|element| element.attr("href").unwrap_or_default().to_string())
                .ok_or(Error::InvalidCssSelector)
                .inspect_err(|_| error!("Failed to parse foundation food json url"))?;

            (release_date, format!("https://fdc.nal.usda.gov{json_url}"))
        };

        let is_data_old = NutritionSource::is_current_data_old(
            self.mm,
            &NutritionDataSource::USDAFoodDataCentral.to_string(),
            foundation_food_release_date,
        )
        .await?;

        if !is_data_old || tables::FoundationFood::count(self.mm).await?.is_positive() {
            return Err(Error::NoNeedToUpdateNutrition);
        }

        let zip_bytes = self
            .client
            .get(json_url)
            .send()
            .await
            .inspect_err(|_| error!("Failed to download foundation food JSON dataset"))?
            .bytes()
            .await?;

        let cursor = Cursor::new(zip_bytes.to_vec());
        let archive = ZipArchive::new(cursor).map_err(|_| {
            error!("Failed to zip foundation food archive from response");
            Error::InvalidZipArchive
        })?;

        Ok(archive)
    }
}

/// A parser for the [FoodData Central](https://fdc.nal.usda.gov/) API.
pub struct FdcParser<'a, State> {
    srlegacy_food_data: Vec<SrlegacyFood<'a>>,
    _state: PhantomData<State>,
}

impl<'a> FdcParser<'a, DataNotFetchedState> {
    /// Creates a new [FoodData Central](https://fdc.nal.usda.gov/) parser.
    pub fn new() -> Self {
        Self {
            srlegacy_food_data: Vec::new(),
            _state: PhantomData,
        }
    }
}

impl<'a> Default for FdcParser<'a, DataNotFetchedState> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<'a, C: FdcFetcher> DataNotFetched<C> for FdcParser<'a, DataNotFetchedState> {
    async fn fetch(self, client: &C) -> Result<FdcParser<DataFetchedState>> {
        let mut archive = client.fetch_foundation_foods().await?;
        let mut file = archive.by_index(0).map_err(|_| Error::NoFileInZip)?;
        let mut bytes = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut bytes)?;

        let data: SrLegacyRoot = simd_json::serde::from_slice(&mut bytes)?;

        Ok(FdcParser {
            srlegacy_food_data: data.srlegacy_foods,
            _state: PhantomData,
        })
    }
}

#[async_trait]
impl<'a> DataFetched for FdcParser<'a, DataFetchedState> {
    async fn push_into_database<'b>(&'b self, mm: &'b ModelManager) -> Result<()> {
        use schema::fdc_foods;

        let mut conn = mm.pool.get().await?;

        conn.transaction::<_, Error, _>(|mut conn| {
            Box::pin(async move {
                diesel::sql_query(
                    "TRUNCATE TABLE
                        fdc_foods,
                        fdc_nutrients,
                        fdc_food_portions,
                        fdc_foods_fdc_nutrients,
                        fdc_food_portions_fdc_foods
                    RESTART IDENTITY CASCADE",
                )
                .execute(conn)
                .await?;

                let foundation_foods = &self.srlegacy_food_data;

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
                let unique_nutrients: BTreeSet<FdcNutrientForInsert> = foundation_foods
                    .iter()
                    .flat_map(|food| {
                        food.food_nutrients
                            .iter()
                            .map(|nutrient| FdcNutrientForInsert {
                                name: &nutrient.nutrient.name,
                                unit_name: &nutrient.nutrient.unit_name,
                            })
                    })
                    .collect();

                for chunk in unique_nutrients
                    .into_iter()
                    .collect::<Vec<_>>()
                    .chunks(BATCH_SIZE)
                {
                    diesel::insert_into(schema::fdc_nutrients::table)
                        .values(chunk)
                        .on_conflict((
                            schema::fdc_nutrients::name,
                            schema::fdc_nutrients::unit_name,
                        ))
                        .do_nothing()
                        .execute(&mut conn)
                        .await?;
                }

                let fdc_nutrients: Vec<(i64, String, String)> = schema::fdc_nutrients::table
                    .select((
                        schema::fdc_nutrients::id,
                        schema::fdc_nutrients::name,
                        schema::fdc_nutrients::unit_name,
                    ))
                    .load(&mut conn)
                    .await?;

                let fdc_nutrients_map: HashMap<(String, String), i64> = fdc_nutrients
                    .clone()
                    .into_iter()
                    .map(|(id, name, unit_name)| ((name, unit_name), id))
                    .collect();

                // fdc_foods_fdc_nutrients
                let mut ids = Vec::new();
                for (ff, fdc_food_db_id) in foundation_foods.iter().zip(fdc_foods_ids.clone()) {
                    ff.food_nutrients.iter().for_each(|food_nutrient| {
                        let food_nutrient = food_nutrient.clone();

                        let id = *fdc_nutrients_map
                            .get(&(
                                food_nutrient.nutrient.name.into_owned(),
                                food_nutrient.nutrient.unit_name.into_owned(),
                            ))
                            .unwrap();

                        ids.push((
                            fdc_food_db_id,
                            id,
                            food_nutrient.amount,
                            food_nutrient.min.unwrap_or_default(),
                            food_nutrient.max.unwrap_or_default(),
                        ));
                    });
                }

                let records = ids
                    .into_iter()
                    .map(
                        |(food_id, nutrient_id, amount, min, max)| FdcFoodFdcNutrientForInsert {
                            food_id,
                            nutrient_id,
                            amount,
                            min,
                            max,
                        },
                    )
                    .collect::<Vec<_>>();

                for chunk in records.chunks(BATCH_SIZE) {
                    diesel::insert_into(schema::fdc_foods_fdc_nutrients::table)
                        .values(chunk)
                        .execute(&mut conn)
                        .await?;
                }

                // fdc_food_portions
                let all_portions: Vec<FdcFoodPortionForInsert> = foundation_foods
                    .iter()
                    .flat_map(|food| {
                        food.food_portions
                            .iter()
                            .map(|portion| FdcFoodPortionForInsert {
                                value: portion.value,
                                modifier: portion.modifier.clone().into_owned(),
                                gram_weight: portion.gram_weight,
                                amount: portion.amount,
                            })
                    })
                    .collect();

                let mut portion_ids = Vec::with_capacity(all_portions.len());

                for chunk in all_portions.chunks(BATCH_SIZE) {
                    let ids: Vec<i64> = diesel::insert_into(schema::fdc_food_portions::table)
                        .values(chunk)
                        .returning(schema::fdc_food_portions::id)
                        .get_results(&mut conn)
                        .await?;

                    portion_ids.extend(ids);
                }

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
        .await
        .inspect_err(|err| error!("Failed to push data into database: {err:?}"))?;

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
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub amount: f64,
}

/// Represents a food portion.
#[derive(Debug, PartialEq)]
pub struct PortionDetails {
    pub id: i64,
    pub value: f64,
    pub modifier: String,
    pub gram_weight: f64,
    pub amount: f64,
}

#[derive(Debug, PartialEq, Queryable)]
pub struct PortionDetailsRow {
    pub id: i64,
    pub value: f64,
    pub modifier: String,
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
                        phraseto_tsquery('english', $1) AS phrase_q,
                        $1 AS raw_query
                )
                SELECT
                    id,
                    fdc_id,
                    food_class,
                    description,
                    food_category,
                    (
                        ts_rank(description_tsv, q.plain_q) * 1.5
                        + (CASE WHEN lower(trim(split_part(description, ',', 1))) = q.raw_query THEN 5.0 ELSE 0.0 END)
                        + (CASE WHEN description ~* (q.raw_query || ',') THEN 2.0 ELSE 0.0 END)
                        + (CASE WHEN description ILIKE ('%' || q.raw_query || '%') THEN 1.0 ELSE 0.0 END)
                        - (CASE WHEN description ILIKE '%without%' OR description ILIKE '%replacement%' OR description ILIKE '%imitation%' THEN 2.0 ELSE 0.0 END)
                        + (1.0 / (char_length(description) + 1.0))
                    ) AS rank
                FROM
                    fdc_foods,
                    q
                WHERE
                    description_tsv @@ q.plain_q
                ORDER BY
                    rank DESC
                LIMIT 10;
            "#
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
                        diesel::dsl::sql::<Nullable<diesel::sql_types::Double>>(
                            "NULLIF(fdc_foods_fdc_nutrients.min, 0)",
                        ),
                        diesel::dsl::sql::<Nullable<diesel::sql_types::Double>>(
                            "NULLIF(fdc_foods_fdc_nutrients.max, 0)",
                        ),
                        schema::fdc_foods_fdc_nutrients::amount,
                    ))
                    .order_by((
                        schema::fdc_nutrients::name.asc(),
                        schema::fdc_nutrients::id.asc(),
                    ))
                    .load::<NutrientDetails>(&mut conn)
                    .await?,
                food_category: food.food_category,
                food_portions: schema::fdc_food_portions_fdc_foods::table
                    .inner_join(schema::fdc_food_portions::table)
                    .filter(schema::fdc_food_portions_fdc_foods::food_id.eq(food.id))
                    .select((
                        schema::fdc_food_portions::id,
                        schema::fdc_food_portions::value,
                        schema::fdc_food_portions::modifier,
                        schema::fdc_food_portions::gram_weight,
                        schema::fdc_food_portions::amount,
                    ))
                    .load::<PortionDetailsRow>(&mut conn)
                    .await?
                    .into_iter()
                    .map(|row| PortionDetails {
                        id: row.id,
                        value: row.value,
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
    use std::io::Write;

    use testing::utils::{TestDb, create_app_state};
    use zip::{ZipWriter, write::FileOptions};

    use super::*;

    type Result<T> = core::result::Result<T, Error>;
    type Error = Box<dyn std::error::Error>;

    const FDC_FF_DATASET_1: &str = "ff dataset 1";
    const FDC_FF_DATASET_2: &str = "ff dataset 2";

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
                        r#"{"SRLegacyFoods":[{"foodClass":"FinalFood","description":"Hummus, commercial","foodNutrients":[{"type":"FoodNutrient","id":1846690,"nutrient":{"id":1186,"number":"431","name":"Folic acid","rank":7000,"unitName":"µg"},"dataPoints":0,"foodNutrientDerivation":{"code":"Z","description":"Assumed zero (Insignificant amount or not naturally occurring in a food, such as fiber in meat)","foodNutrientSource":{"id":5,"code":"7","description":"Assumed zero"}},"amount":0.000},{"type":"FoodNutrient","id":1846691,"nutrient":{"id":1110,"number":"324","name":"Vitamin D (D2 + D3), International Units","rank":8650,"unitName":"IU"},"dataPoints":0,"foodNutrientDerivation":{"code":"Z","description":"Assumed zero (Insignificant amount or not naturally occurring in a food, such as fiber in meat)","foodNutrientSource":{"id":5,"code":"7","description":"Assumed zero"}},"amount":0.000},{"type":"FoodNutrient","id":1846692,"nutrient":{"id":1114,"number":"328","name":"Vitamin D (D2 + D3)","rank":8700,"unitName":"µg"},"dataPoints":0,"foodNutrientDerivation":{"code":"Z","description":"Assumed zero (Insignificant amount or not naturally occurring in a food, such as fiber in meat)","foodNutrientSource":{"id":5,"code":"7","description":"Assumed zero"}},"amount":0.000},{"type":"FoodNutrient","id":1846693,"nutrient":{"id":1178,"number":"418","name":"Vitamin B-12","rank":7300,"unitName":"µg"},"dataPoints":0,"foodNutrientDerivation":{"code":"Z","description":"Assumed zero (Insignificant amount or not naturally occurring in a food, such as fiber in meat)","foodNutrientSource":{"id":5,"code":"7","description":"Assumed zero"}},"amount":0.000},{"type":"FoodNutrient","id":1846694,"nutrient":{"id":1253,"number":"601","name":"Cholesterol","rank":15700,"unitName":"mg"},"dataPoints":0,"foodNutrientDerivation":{"code":"Z","description":"Assumed zero (Insignificant amount or not naturally occurring in a food, such as fiber in meat)","foodNutrientSource":{"id":5,"code":"7","description":"Assumed zero"}},"amount":0.000},{"type":"FoodNutrient","id":1846720,"nutrient":{"id":1301,"number":"654","name":"SFA 24:0","rank":11300,"unitName":"g"},"dataPoints":6,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":0.044,"min":0.019,"amount":0.028},{"type":"FoodNutrient","id":1846804,"nutrient":{"id":1109,"number":"323","name":"Vitamin E (alpha-tocopherol)","rank":7905,"unitName":"mg"},"dataPoints":6,"foodNutrientDerivation":{"code":"A","description":"Analytical","foodNutrientSource":{"id":1,"code":"1","description":"Analytical or derived from analytical"}},"max":2.89,"min":1.40,"amount":1.54}],"foodAttributes":[],"nutrientConversionFactors":[{"type":".CalorieConversionFactor","proteinValue":3.47,"fatValue":8.37,"carbohydrateValue":4.07},{"type":".ProteinConversionFactor","value":6.25}],"isHistoricalReference":false,"ndbNumber":16158,"fdcId":174289,"dataType":"SR Legacy","foodCategory":{"description":"Legumes and Legume Products"},"foodPortions":[{"id":94086,"value":1.0,"measureUnit":{"id":9999,"name":"undetermined","abbreviation":"undetermined"},"modifier":"tbsp","gramWeight":15.0,"sequenceNumber":1,"amount":1.0},{"id":94087,"value":1.0,"measureUnit":{"id":9999,"name":"undetermined","abbreviation":"undetermined"},"modifier":"cup","gramWeight":246.0,"sequenceNumber":2,"amount":1.0}],"publicationDate":"4/1/2019","inputFoods":[]}]}"#.to_string(),
                    ),
                    (
                        FDC_FF_DATASET_2.into(),
                        r#"{"SRLegacyFoods":[{"foodClass":"FinalFood","description":"Egg, yolk, raw, frozen, pasteurized","foodNutrients":[{"type":"FoodNutrient","id":1777480,"nutrient":{"id":1186,"number":"431","name":"Folic acid","rank":7000,"unitName":"µg"},"dataPoints":0,"foodNutrientDerivation":{"code":"Z","description":"Assumed zero (Insignificant amount or not naturally occurring in a food, such as fiber in meat)","foodNutrientSource":{"id":5,"code":"7","description":"Assumed zero"}},"amount":0.000},{"type":"FoodNutrient","id":1777481,"nutrient":{"id":1108,"number":"322","name":"Carotene, alpha","rank":7450,"unitName":"µg"},"dataPoints":0,"foodNutrientDerivation":{"code":"BFFN","description":"Based on another form of the food or similar food; Concentration adjustment; Fat; Retention factors not used","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":36.0},{"type":"FoodNutrient","id":1777482,"nutrient":{"id":1120,"number":"334","name":"Cryptoxanthin, beta","rank":7460,"unitName":"µg"},"dataPoints":0,"foodNutrientDerivation":{"code":"BFFN","description":"Based on another form of the food or similar food; Concentration adjustment; Fat; Retention factors not used","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":32.0},{"type":"FoodNutrient","id":1777483,"nutrient":{"id":1122,"number":"337","name":"Lycopene","rank":7530,"unitName":"µg"},"dataPoints":0,"foodNutrientDerivation":{"code":"BFFN","description":"Based on another form of the food or similar food; Concentration adjustment; Fat; Retention factors not used","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":0.000},{"type":"FoodNutrient","id":1777484,"nutrient":{"id":1185,"number":"430","name":"Vitamin K (phylloquinone)","rank":8800,"unitName":"µg"},"dataPoints":0,"foodNutrientDerivation":{"code":"BFFN","description":"Based on another form of the food or similar food; Concentration adjustment; Fat; Retention factors not used","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":0.700},{"type":"FoodNutrient","id":1777596,"nutrient":{"id":1079,"number":"291","name":"Fiber, total dietary","rank":1200,"unitName":"g"},"dataPoints":0,"foodNutrientDerivation":{"code":"Z","description":"Assumed zero (Insignificant amount or not naturally occurring in a food, such as fiber in meat)","foodNutrientSource":{"id":5,"code":"7","description":"Assumed zero"}},"amount":0.000}],"foodAttributes":[],"nutrientConversionFactors":[{"type":".ProteinConversionFactor","value":6.25}],"isHistoricalReference":false,"ndbNumber":1126,"fdcId":173421,"dataType":"SR Legacy","foodCategory":{"description":"Dairy and Egg Products"},"foodPortions":[{"id":92493,"value":1.0,"measureUnit":{"id":9999,"name":"undetermined","abbreviation":"undetermined"},"modifier":"oz","gramWeight":28.35,"sequenceNumber":1,"amount":1.0},{"id":92494,"value":0.5,"measureUnit":{"id":9999,"name":"undetermined","abbreviation":"undetermined"},"modifier":"lb","gramWeight":227.0,"sequenceNumber":2,"amount":0.5}],"publicationDate":"4/1/2019","inputFoods":[]},{"foodClass":"FinalFood","description":"Egg, yolk, raw, frozen, sugared, pasteurized","foodNutrients":[{"type":"FoodNutrient","id":1777704,"nutrient":{"id":1105,"number":"319","name":"Retinol","rank":7430,"unitName":"µg"},"dataPoints":0,"foodNutrientDerivation":{"code":"BFZN","description":"Based on another form of the food or similar food; Concentration adjustment; No adjustment; Retention factors not used","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":313},{"type":"FoodNutrient","id":1777705,"nutrient":{"id":1106,"number":"320","name":"Vitamin A, RAE","rank":7420,"unitName":"µg"},"dataPoints":0,"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":316},{"type":"FoodNutrient","id":1777706,"nutrient":{"id":1162,"number":"401","name":"Vitamin C, total ascorbic acid","rank":6300,"unitName":"mg"},"dataPoints":0,"foodNutrientDerivation":{"code":"Z","description":"Assumed zero (Insignificant amount or not naturally occurring in a food, such as fiber in meat)","foodNutrientSource":{"id":5,"code":"7","description":"Assumed zero"}},"amount":0.000}],"foodAttributes":[],"nutrientConversionFactors":[{"type":".CalorieConversionFactor","proteinValue":4.36,"fatValue":9.02,"carbohydrateValue":3.68},{"type":".ProteinConversionFactor","value":6.25}],"isHistoricalReference":false,"ndbNumber":1127,"fdcId":173422,"dataType":"SR Legacy","foodCategory":{"description":"Dairy and Egg Products"},"foodPortions":[{"id":92495,"value":1.0,"measureUnit":{"id":9999,"name":"undetermined","abbreviation":"undetermined"},"modifier":"oz","gramWeight":28.35,"sequenceNumber":1,"amount":1.0},{"id":92496,"value":0.5,"measureUnit":{"id":9999,"name":"undetermined","abbreviation":"undetermined"},"modifier":"lb","gramWeight":227.0,"sequenceNumber":2,"amount":0.5}],"publicationDate":"4/1/2019","inputFoods":[]},{"foodClass":"FinalFood","description":"Egg, whole, cooked, fried","foodNutrients":[{"type":"FoodNutrient","id":1777824,"nutrient":{"id":1008,"number":"208","name":"Energy","rank":300,"unitName":"kcal"},"dataPoints":0,"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":196},{"type":"FoodNutrient","id":1777825,"nutrient":{"id":1110,"number":"324","name":"Vitamin D (D2 + D3), International Units","rank":8650,"unitName":"IU"},"dataPoints":0,"foodNutrientDerivation":{"code":"RA","description":"Recipe; Approximate ingredient proportions (ex. combination of several recipes)","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":88.0},{"type":"FoodNutrient","id":1777826,"nutrient":{"id":1190,"number":"435","name":"Folate, DFE","rank":7200,"unitName":"µg"},"dataPoints":0,"foodNutrientDerivation":{"code":"NC","description":"Calculated","foodNutrientSource":{"id":2,"code":"4","description":"Calculated or imputed"}},"amount":51.0}],"foodAttributes":[],"nutrientConversionFactors":[{"type":".CalorieConversionFactor","proteinValue":4.36,"fatValue":9.02,"carbohydrateValue":3.68},{"type":".ProteinConversionFactor","value":6.25}],"isHistoricalReference":false,"ndbNumber":1128,"fdcId":173423,"dataType":"SR Legacy","foodCategory":{"description":"Dairy and Egg Products"},"foodPortions":[{"id":92497,"value":1.0,"measureUnit":{"id":9999,"name":"undetermined","abbreviation":"undetermined"},"modifier":"large","gramWeight":46.0,"sequenceNumber":1,"amount":1.0}],"publicationDate":"4/1/2019","inputFoods":[]}]}"#.into(),
                    ),
                ]),
            }
        }
    }

    #[async_trait]
    impl FdcFetcher for FdcClientForTests {
        async fn fetch_foundation_foods(&self) -> super::Result<ZipArchive<Cursor<Vec<u8>>>> {
            let dataset = self
                .datasets
                .get(&self.selected_dataset)
                .cloned()
                .unwrap_or_else(|| panic!("dataset {}", self.selected_dataset));

            let mut zip_buffer = Cursor::new(Vec::new());
            {
                let mut zip = ZipWriter::new(&mut zip_buffer);
                zip.start_file(
                    "FoodData_Central_foundation_food_csv_2024-10.csv",
                    FileOptions::<()>::default(),
                )
                .unwrap();
                zip.write_all(dataset.as_bytes()).unwrap();
            }

            let bytes = zip_buffer.into_inner();
            Ok(ZipArchive::new(Cursor::new(bytes)).unwrap())
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
                fdc_id: 174289,
                food_class: "FinalFood".into(),
                description: "Hummus, commercial".into(),
                food_nutrients: vec![
                    NutrientDetails {
                        id: 1,
                        name: "Cholesterol".into(),
                        unit_name: "mg".into(),
                        min: None,
                        max: None,
                        amount: 0.0,
                    },
                    NutrientDetails {
                        id: 2,
                        name: "Folic acid".into(),
                        unit_name: "µg".into(),
                        min: None,
                        max: None,
                        amount: 0.0,
                    },
                    NutrientDetails {
                        id: 3,
                        name: "SFA 24:0".into(),
                        unit_name: "g".into(),
                        min: Some(0.019,),
                        max: Some(0.044,),
                        amount: 0.028,
                    },
                    NutrientDetails {
                        id: 4,
                        name: "Vitamin B-12".into(),
                        unit_name: "µg".into(),
                        min: None,
                        max: None,
                        amount: 0.0,
                    },
                    NutrientDetails {
                        id: 5,
                        name: "Vitamin D (D2 + D3)".into(),
                        unit_name: "µg".into(),
                        min: None,
                        max: None,
                        amount: 0.0,
                    },
                    NutrientDetails {
                        id: 6,
                        name: "Vitamin D (D2 + D3), International Units".into(),
                        unit_name: "IU".into(),
                        min: None,
                        max: None,
                        amount: 0.0,
                    },
                    NutrientDetails {
                        id: 7,
                        name: "Vitamin E (alpha-tocopherol)".into(),
                        unit_name: "mg".into(),
                        min: Some(1.4,),
                        max: Some(2.89,),
                        amount: 1.54,
                    },
                ],
                food_category: "Legumes and Legume Products".into(),
                food_portions: vec![
                    PortionDetails {
                        id: 1,
                        value: 1.0,
                        modifier: "tbsp".into(),
                        gram_weight: 15.0,
                        amount: 1.0,
                    },
                    PortionDetails {
                        id: 2,
                        value: 1.0,
                        modifier: "cup".into(),
                        gram_weight: 246.0,
                        amount: 1.0,
                    }
                ],
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

        pretty_assertions::assert_eq!(foods.len(), 2);
        pretty_assertions::assert_eq!(
            foods,
            vec![
                FoundationFoodDetails {
                    id: 1,
                    fdc_id: 173421,
                    food_class: "FinalFood".into(),
                    description: "Egg, yolk, raw, frozen, pasteurized".into(),
                    food_nutrients: vec![
                        NutrientDetails {
                            id: 1,
                            name: "Carotene, alpha".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 36.0,
                        },
                        NutrientDetails {
                            id: 2,
                            name: "Cryptoxanthin, beta".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 32.0,
                        },
                        NutrientDetails {
                            id: 4,
                            name: "Fiber, total dietary".into(),
                            unit_name: "g".into(),
                            min: None,
                            max: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 6,
                            name: "Folic acid".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 7,
                            name: "Lycopene".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 12,
                            name: "Vitamin K (phylloquinone)".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 0.7,
                        },
                    ],
                    food_category: "Dairy and Egg Products".into(),
                    food_portions: vec![
                        PortionDetails {
                            id: 1,
                            value: 1.0,
                            modifier: "oz".into(),
                            gram_weight: 28.35,
                            amount: 1.0,
                        },
                        PortionDetails {
                            id: 2,
                            value: 0.5,
                            modifier: "lb".into(),
                            gram_weight: 227.0,
                            amount: 0.5,
                        },
                    ],
                },
                FoundationFoodDetails {
                    id: 2,
                    fdc_id: 173422,
                    food_class: "FinalFood".into(),
                    description: "Egg, yolk, raw, frozen, sugared, pasteurized".into(),
                    food_nutrients: vec![
                        NutrientDetails {
                            id: 8,
                            name: "Retinol".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 313.0,
                        },
                        NutrientDetails {
                            id: 9,
                            name: "Vitamin A, RAE".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 316.0,
                        },
                        NutrientDetails {
                            id: 10,
                            name: "Vitamin C, total ascorbic acid".into(),
                            unit_name: "mg".into(),
                            min: None,
                            max: None,
                            amount: 0.0,
                        },
                    ],
                    food_category: "Dairy and Egg Products".into(),
                    food_portions: vec![
                        PortionDetails {
                            id: 3,
                            value: 1.0,
                            modifier: "oz".into(),
                            gram_weight: 28.35,
                            amount: 1.0,
                        },
                        PortionDetails {
                            id: 4,
                            value: 0.5,
                            modifier: "lb".into(),
                            gram_weight: 227.0,
                            amount: 0.5,
                        },
                    ],
                }
            ]
        );
        Ok(())
    }
}
