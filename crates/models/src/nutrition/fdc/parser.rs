use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    io::{Cursor, Read as _},
    marker::PhantomData,
};

use async_trait::async_trait;
use diesel::{
    prelude::*,
    sql_types::{BigInt, Float8, Nullable, Text},
};
use diesel_async::{AsyncConnection as _, AsyncPgConnection, RunQueryDsl};
use reqwest::Client;
use scraper::{Html, Selector};
use time::{Date, macros::format_description};
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
    /// Fetches the following data sets from the `USDA FoodData Central` API:
    /// - Foundation Food Data
    async fn fetch(self, client: &C) -> Result<FdcParser<DataFetchedState>>;
}

#[async_trait]
pub trait DataFetched: Send + Sync {
    /// Pushes the data sets fetched from the `FoodData Central API` into the database.
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
impl FdcFetcher for FdcClient<'_> {
    #[allow(clippy::items_after_statements)]
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

            let release_date = Date::parse(
                &format!("01/{month_year_date}"),
                format_description!("[day]/[month]/[year]"),
            )
            .inspect_err(|_| error!("Failed to parse foundation food release date"))
            .unwrap_or(Date::MIN);

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

        if !is_data_old || tables::FoundationFood::is_populated(self.mm).await? {
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

impl FdcParser<'_, DataNotFetchedState> {
    /// Creates a new [FoodData Central](https://fdc.nal.usda.gov/) parser.
    pub const fn new() -> Self {
        Self {
            srlegacy_food_data: Vec::new(),
            _state: PhantomData,
        }
    }
}

impl Default for FdcParser<'_, DataNotFetchedState> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<C: FdcFetcher> DataNotFetched<C> for FdcParser<'_, DataNotFetchedState> {
    async fn fetch(self, client: &C) -> Result<FdcParser<DataFetchedState>> {
        let mut archive = client.fetch_foundation_foods().await?;
        let mut file = archive.by_index(0).map_err(|_| Error::NoFileInZip)?;
        let mut bytes = Vec::with_capacity(
            usize::try_from(file.size())
                .inspect_err(|err| {
                    error!("Failed to cast file size to usize '{}': {err}", file.size());
                })
                .unwrap_or_default(),
        );
        file.read_to_end(&mut bytes)?;

        let data: SrLegacyRoot = simd_json::serde::from_slice(&mut bytes)?;

        Ok(FdcParser {
            srlegacy_food_data: data.srlegacy_foods,
            _state: PhantomData,
        })
    }
}

#[async_trait]
#[allow(clippy::too_many_lines)]
impl DataFetched for FdcParser<'_, DataFetchedState> {
    async fn push_into_database<'b>(&'b self, mm: &'b ModelManager) -> Result<()> {
        use schema::fdc_foods;

        let mut conn = mm.pool.get().await?;

        conn.transaction::<_, Error, _>(async |mut conn| {
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
                            fdc_id: nutrient.nutrient.id,
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
        .await
        .inspect_err(|err| error!("Failed to push data into database: {err}"))?;

        Ok(())
    }
}

/// Contains all the details of a foundation food.
#[derive(Debug, PartialEq)]
pub struct SRLegacyFoodDetails {
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
    pub fdc_id: i64,
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

impl SRLegacyFoodDetails {
    /// Retrieves the most relevant details of a foundation food from the database.
    pub async fn get_relevant(conn: &mut AsyncPgConnection, food: &str) -> Result<Vec<Self>> {
        let food_results = diesel::sql_query(
            r"
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
            "
        )
        .bind::<Text, _>(food)
        .load::<FdcFoodResult>(conn)
        .await?;

        let mut foods = Vec::with_capacity(food_results.len());

        for food in food_results {
            foods.push(Self {
                id: food.id,
                fdc_id: food.fdc_id,
                food_class: food.food_class,
                description: food.description,
                food_nutrients: schema::fdc_foods_fdc_nutrients::table
                    .inner_join(schema::fdc_nutrients::table)
                    .filter(schema::fdc_foods_fdc_nutrients::food_id.eq(food.id))
                    .select((
                        schema::fdc_nutrients::id,
                        schema::fdc_nutrients::fdc_id,
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
                    .load::<NutrientDetails>(conn)
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
                    .load::<PortionDetailsRow>(conn)
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
            });
        }

        Ok(foods)
    }
}

#[cfg(test)]
mod tests {
    use test_db::TestDb;
    use test_utils::create_app_state;

    use crate::nutrition::testdata::nutrition_data::nutrition_data_for_tests::*;

    use super::*;

    type Result<T> = core::result::Result<T, Error>;
    type Error = Box<dyn std::error::Error>;

    #[tokio::test]
    async fn test_parses_correctly_simple_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config).await;
        let mut conn = state.mm.pool.get().await?;
        let client = FdcClientForTests::new(FDC_FF_DATASET_1);
        let parser: FdcParser<DataFetchedState> = FdcParser::new().fetch(&client).await?;
        parser.push_into_database(&state.mm).await?;

        let foods = SRLegacyFoodDetails::get_relevant(&mut conn, "hummus").await?;

        pretty_assertions::assert_eq!(
            foods,
            vec![SRLegacyFoodDetails {
                id: 1,
                fdc_id: 174_289,
                food_class: "FinalFood".into(),
                description: "Hummus, commercial".into(),
                food_nutrients: vec![
                    NutrientDetails {
                        id: 6,
                        fdc_id: 1253,
                        name: "Cholesterol".into(),
                        unit_name: "mg".into(),
                        min: None,
                        max: None,
                        amount: 0.0,
                    },
                    NutrientDetails {
                        id: 5,
                        fdc_id: 1186,
                        name: "Folic acid".into(),
                        unit_name: "µg".into(),
                        min: None,
                        max: None,
                        amount: 0.0,
                    },
                    NutrientDetails {
                        id: 7,
                        fdc_id: 1301,
                        name: "SFA 24:0".into(),
                        unit_name: "g".into(),
                        min: Some(0.019,),
                        max: Some(0.044,),
                        amount: 0.028,
                    },
                    NutrientDetails {
                        id: 4,
                        fdc_id: 1178,
                        name: "Vitamin B-12".into(),
                        unit_name: "µg".into(),
                        min: None,
                        max: None,
                        amount: 0.0,
                    },
                    NutrientDetails {
                        id: 3,
                        fdc_id: 1114,
                        name: "Vitamin D (D2 + D3)".into(),
                        unit_name: "µg".into(),
                        min: None,
                        max: None,
                        amount: 0.0,
                    },
                    NutrientDetails {
                        id: 2,
                        fdc_id: 1110,
                        name: "Vitamin D (D2 + D3), International Units".into(),
                        unit_name: "IU".into(),
                        min: None,
                        max: None,
                        amount: 0.0,
                    },
                    NutrientDetails {
                        id: 1,
                        fdc_id: 1109,
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
    #[allow(clippy::too_many_lines)]
    async fn test_parses_correctly_complex_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config).await;
        let mut conn = state.mm.pool.get().await?;
        let client = FdcClientForTests::new(FDC_FF_DATASET_2);
        let parser: FdcParser<DataFetchedState> = FdcParser::new().fetch(&client).await?;
        parser.push_into_database(&state.mm).await?;

        let foods = SRLegacyFoodDetails::get_relevant(&mut conn, "egg frozen").await?;

        pretty_assertions::assert_eq!(foods.len(), 2);
        pretty_assertions::assert_eq!(
            foods,
            vec![
                SRLegacyFoodDetails {
                    id: 1,
                    fdc_id: 173_421,
                    food_class: "FinalFood".into(),
                    description: "Egg, yolk, raw, frozen, pasteurized".into(),
                    food_nutrients: vec![
                        NutrientDetails {
                            id: 5,
                            fdc_id: 1108,
                            name: "Carotene, alpha".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 36.0,
                        },
                        NutrientDetails {
                            id: 7,
                            fdc_id: 1120,
                            name: "Cryptoxanthin, beta".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 32.0,
                        },
                        NutrientDetails {
                            id: 2,
                            fdc_id: 1079,
                            name: "Fiber, total dietary".into(),
                            unit_name: "g".into(),
                            min: None,
                            max: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 11,
                            fdc_id: 1186,
                            name: "Folic acid".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 8,
                            fdc_id: 1122,
                            name: "Lycopene".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 0.0,
                        },
                        NutrientDetails {
                            id: 10,
                            fdc_id: 1185,
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
                SRLegacyFoodDetails {
                    id: 2,
                    fdc_id: 173_422,
                    food_class: "FinalFood".into(),
                    description: "Egg, yolk, raw, frozen, sugared, pasteurized".into(),
                    food_nutrients: vec![
                        NutrientDetails {
                            id: 3,
                            fdc_id: 1105,
                            name: "Retinol".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 313.0,
                        },
                        NutrientDetails {
                            id: 4,
                            fdc_id: 1106,
                            name: "Vitamin A, RAE".into(),
                            unit_name: "µg".into(),
                            min: None,
                            max: None,
                            amount: 316.0,
                        },
                        NutrientDetails {
                            id: 9,
                            fdc_id: 1162,
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
