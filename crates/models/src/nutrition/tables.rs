use diesel::{
    Selectable,
    dsl::{exists, select},
    prelude::*,
    sql_types::{BigInt, Float8, Text},
};
use diesel_async::RunQueryDsl;
use time::Date;

use repository::{ModelManager, schema};

use crate::Result;

/// Represents a nutrition database.
#[derive(Debug, Queryable, Identifiable, Eq, PartialEq, Selectable)]
#[diesel(table_name = schema::nutrition_sources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NutritionSource {
    pub id: i16,
    pub name: String,
    pub description: String,
    pub url: String,
    pub country: String,
    pub created_on: Date,
    pub updated_on: Option<Date>,
}

impl NutritionSource {
    /// Fetches all nutrition sources.
    pub async fn all(mm: &ModelManager) -> Result<Vec<Self>> {
        schema::nutrition_sources::table
            .select(Self::as_select())
            .load(&mut mm.pool.get().await?)
            .await
            .map_err(Into::into)
    }
}

/// Represents a foundation food.
#[derive(Queryable, Identifiable, Eq, PartialEq, Selectable)]
#[diesel(table_name = schema::fdc_foods)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FoundationFood {
    pub id: i64,
    pub food_class: String,
    pub description: String,
    pub food_category: String,
    pub fdc_id: i64,
}

impl FoundationFood {
    /// Gets the number of entries in the database.
    pub async fn is_populated(mm: &ModelManager) -> Result<bool> {
        Ok(select(exists(schema::fdc_foods::table.limit(1)))
            .get_result(&mut mm.pool.get().await?)
            .await?)
    }
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
#[derive(Insertable)]
#[diesel(table_name = schema::fdc_foods)]
pub(crate) struct FoundationFoodForInsert<'a> {
    pub food_class: &'a str,
    pub description: &'a str,
    pub food_category: &'a str,
    pub fdc_id: i64,
}

/// Represents an FDC nutrient.
#[derive(Queryable, Identifiable, Eq, PartialEq, Selectable)]
#[diesel(table_name = schema::fdc_nutrients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FdcNutrient {
    id: i64,
    fdc_id: i64,
    name: String,
    unit_name: String,
}

#[derive(Eq, Hash, PartialEq, Insertable, PartialOrd, Ord)]
#[diesel(table_name = schema::fdc_nutrients)]
pub(crate) struct FdcNutrientForInsert<'a> {
    pub fdc_id: i64,
    pub name: &'a str,
    pub unit_name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = schema::fdc_foods_fdc_nutrients)]
pub(crate) struct FdcFoodFdcNutrientForInsert {
    pub food_id: i64,
    pub nutrient_id: i64,
    pub amount: f64,
    pub min: f64,
    pub max: f64,
}

/// Represents a food portion.
#[derive(Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = schema::fdc_food_portions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FdcFoodPortion {
    pub id: i64,
    pub value: f64,
    pub modifier: String,
    pub gram_weight: f64,
    pub amount: f64,
}

#[derive(Insertable)]
#[diesel(table_name = schema::fdc_food_portions)]
pub(crate) struct FdcFoodPortionForInsert {
    pub value: f64,
    pub modifier: String,
    pub gram_weight: f64,
    pub amount: f64,
}

#[derive(Insertable)]
#[diesel(table_name = schema::fdc_food_portions_fdc_foods)]
pub(crate) struct FdcFoodPortionFdcFoodForInsert {
    pub food_id: i64,
    pub portion_id: i64,
}

impl NutritionSource {
    /// Checks whether the nutrition database can be updated based on when the data was last updated.
    pub async fn is_current_data_old(
        mm: &ModelManager,
        source_name: &str,
        date: Date,
    ) -> Result<bool> {
        schema::nutrition_sources::table
            .filter(schema::nutrition_sources::name.eq(source_name))
            .first::<Self>(&mut mm.pool.get().await?)
            .await?
            .updated_on
            .map_or_else(
                || Ok(true),
                |updated_on| {
                    Ok((updated_on.year(), updated_on.month()) < (date.year(), date.month()))
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use test_db::default_config;
    use test_utils::create_app_state;
    use time::OffsetDateTime;

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn a_food<'a>() -> FoundationFoodForInsert<'a> {
        FoundationFoodForInsert {
            food_class: "FinalFood",
            description: "kiwi, raw",
            food_category: "Fruits",
            fdc_id: 32196,
        }
    }

    #[tokio::test]
    async fn test_all_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let now = OffsetDateTime::now_utc().date();

        let got = NutritionSource::all(&state.mm).await?;

        pretty_assertions::assert_eq!(
            got,
            vec![NutritionSource {
                id: 1,
                name: "USDA FoodData Central".into(),
                description: "USDA FoodData Central".into(),
                url: "https://fdc.nal.usda.gov/".into(),
                country: "United States".into(),
                created_on: now,
                updated_on: None,
            }]
        );
        Ok(())
    }

    mod tests_foundation_food {
        use super::*;

        #[tokio::test]
        async fn test_insert_and_count() -> Result<()> {
            let state = create_app_state(default_config()).await;
            diesel::insert_into(schema::fdc_foods::table)
                .values(&a_food())
                .execute(&mut state.mm.pool.get().await?)
                .await?;

            let got = FoundationFood::is_populated(&state.mm).await?;

            assert!(got);
            Ok(())
        }
    }

    mod tests_nutrition_source {
        use super::*;

        fn usda_fdc_source_name<'a>() -> &'a str {
            "USDA FoodData Central"
        }

        mod tests_is_current_data_old {
            use time::Duration;

            use super::*;

            #[tokio::test]
            async fn test_updated_on_null_ok() -> Result<()> {
                let state = create_app_state(default_config()).await;

                let is_old = NutritionSource::is_current_data_old(
                    &state.mm,
                    usda_fdc_source_name(),
                    OffsetDateTime::now_utc().date(),
                )
                .await?;

                assert!(is_old);
                Ok(())
            }

            #[tokio::test]
            async fn test_updated_on_equal_current_date_ok() -> Result<()> {
                let state = create_app_state(default_config()).await;
                diesel::insert_into(schema::fdc_foods::table)
                    .values(&a_food())
                    .execute(&mut state.mm.pool.get().await?)
                    .await?;

                let is_old = NutritionSource::is_current_data_old(
                    &state.mm,
                    usda_fdc_source_name(),
                    OffsetDateTime::now_utc().date(),
                )
                .await?;

                assert!(!is_old);
                Ok(())
            }

            #[tokio::test]
            async fn test_updated_on_before_current_date_ok() -> Result<()> {
                let state = create_app_state(default_config()).await;
                diesel::insert_into(schema::fdc_foods::table)
                    .values(&a_food())
                    .execute(&mut state.mm.pool.get().await?)
                    .await?;

                let is_old = NutritionSource::is_current_data_old(
                    &state.mm,
                    usda_fdc_source_name(),
                    OffsetDateTime::now_utc().date() + Duration::weeks(6),
                )
                .await?;

                assert!(is_old);
                Ok(())
            }
        }
    }
}
