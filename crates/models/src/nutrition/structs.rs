use diesel::{Selectable, prelude::*};

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

#[derive(Insertable)]
#[diesel(table_name = schema::fdc_foods)]
pub(crate) struct FoundationFoodForInsert<'a> {
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

/// Represents a measure unit.
#[derive(Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(table_name = schema::measure_units)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MeasureUnit {
    id: i64,
    name: String,
    abbreviation: String,
}
