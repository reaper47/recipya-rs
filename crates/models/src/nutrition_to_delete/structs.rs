use diesel::{
    Selectable,
    prelude::*,
    sql_types::{BigInt, Float8, Text},
};

use repository::schema;

/// Represents a nutrition database.
#[derive(Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = schema::nutrition_sources)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NutritionSource {
    id: i64,
    name: String,
    description: String,
    url: String,
    country: String,
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
pub(crate) struct FdcNutrientForInsert<'a> {
    pub name: &'a str,
    pub unit_name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = schema::fdc_foods_fdc_nutrients)]
pub(crate) struct FdcFoodFdcNutrientForInsert {
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
pub(crate) struct MeasureUnitForInsert {
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
pub(crate) struct FdcFoodPortionForInsert {
    pub value: f64,
    pub measure_unit_id: i64,
    pub modifier: Option<String>,
    pub gram_weight: f64,
    pub amount: f64,
}

#[derive(Insertable)]
#[diesel(table_name = schema::fdc_food_portions_fdc_foods)]
pub(crate) struct FdcFoodPortionFdcFoodForInsert {
    pub food_id: i64,
    pub portion_id: i64,
}
