use chrono::{Datelike, NaiveDate};
use diesel::{
    Selectable,
    prelude::*,
    sql_types::{BigInt, Float8, Text},
};
use diesel_async::{AsyncConnection, RunQueryDsl};

use repository::{ModelManager, schema};

use crate::{Error, Result};

/// Represents a nutrition database.
#[derive(Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = schema::nutrition_sources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NutritionSource {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub url: String,
    pub country: String,
    pub created_on: NaiveDate,
    pub updated_on: Option<NaiveDate>,
}

/// Represents a foundation food.
#[derive(Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = schema::fdc_foods)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FoundationFood {
    pub id: i64,
    pub food_class: String,
    pub description: String,
    pub food_category: String,
    pub fdc_id: i64,
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
pub struct FoundationFoodForInsert<'a> {
    pub food_class: &'a str,
    pub description: &'a str,
    pub food_category: &'a str,
    pub fdc_id: i64,
}

/// Represents an FDC nutrient.
#[derive(Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = schema::fdc_nutrients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FdcNutrient {
    id: i64,
    name: String,
    unit_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::fdc_nutrients)]
pub struct FdcNutrientForInsert<'a> {
    pub name: &'a str,
    pub unit_name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = schema::fdc_foods_fdc_nutrients)]
pub struct FdcFoodFdcNutrientForInsert {
    pub food_id: i64,
    pub nutrient_id: i64,
    pub median: f64,
    pub amount: f64,
}

/// Represents a measure unit.
#[derive(Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = schema::measure_units)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MeasureUnit {
    id: i64,
    name: String,
    abbreviation: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::measure_units)]
pub struct MeasureUnitForInsert {
    pub name: String,
    pub abbreviation: String,
}

/// Represents a food portion.
#[derive(Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = schema::fdc_food_portions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FdcFoodPortion {
    pub id: i64,
    pub value: f64,
    pub measure_unit_id: i64,
    pub modifier: Option<String>,
    pub gram_weight: f64,
    pub amount: f64,
}

#[derive(Insertable)]
#[diesel(table_name = schema::fdc_food_portions)]
pub struct FdcFoodPortionForInsert {
    pub value: f64,
    pub measure_unit_id: i64,
    pub modifier: Option<String>,
    pub gram_weight: f64,
    pub amount: f64,
}

#[derive(Insertable)]
#[diesel(table_name = schema::fdc_food_portions_fdc_foods)]
pub struct FdcFoodPortionFdcFoodForInsert {
    pub food_id: i64,
    pub portion_id: i64,
}

impl NutritionSource {
    /// Checks whether the nutrition database can be updated based on when the data was last updated.
    pub async fn is_current_data_old(
        mm: &ModelManager,
        source_name: &str,
        date: NaiveDate,
    ) -> Result<bool> {
        let mut conn = mm.pool.get().await?;

        let nutrition_source = schema::nutrition_sources::table
            .filter(schema::nutrition_sources::name.eq(source_name))
            .first::<NutritionSource>(&mut conn)
            .await?;

        match nutrition_source.updated_on {
            Some(updated_on) => {
                Ok((updated_on.year(), updated_on.month()) < (date.year(), date.month()))
            }
            None => Ok(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use testing::utils::{TestDb, create_app_state};

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_nutrition_source {
        use super::*;

        fn usda_fdc_source_name<'a>() -> &'a str {
            "USDA FoodData Central"
        }

        mod tests_is_current_data_old {
            use chrono::Local;

            use super::*;

            #[tokio::test]
            async fn test_updated_on_null_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let state = create_app_state(config.clone()).await;

                let is_old = NutritionSource::is_current_data_old(
                    &state.mm,
                    usda_fdc_source_name(),
                    Local::now().date_naive(),
                )
                .await?;

                assert!(is_old);
                Ok(())
            }

            #[tokio::test]
            async fn test_updated_on_equal_current_date_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let state = create_app_state(config.clone()).await;
                let mut conn = state.mm.pool.get().await?;
                diesel::insert_into(schema::fdc_foods::table)
                    .values(&FoundationFoodForInsert {
                        food_class: "FinalFood",
                        description: "kiwi, raw",
                        food_category: "Fruits",
                        fdc_id: 32196,
                    })
                    .execute(&mut conn)
                    .await?;

                let is_old = NutritionSource::is_current_data_old(
                    &state.mm,
                    usda_fdc_source_name(),
                    Local::now().date_naive(),
                )
                .await?;

                assert!(!is_old);
                Ok(())
            }

            #[tokio::test]
            async fn test_updated_on_before_current_date_ok() -> Result<()> {
                todo!()
            }
        }
    }
}
