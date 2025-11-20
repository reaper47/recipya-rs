use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};

use repository::schema;

use crate::Recipe;

/// Represents a time components of a recipe.
#[derive(Clone, Debug, Default, PartialEq, Associations, Queryable, Identifiable, Selectable)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::times)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Times {
    pub id: i64,
    pub recipe_id: i64,
    pub prep_seconds: i32,
    pub cook_seconds: i32,
    pub total_seconds: i32,
}

/// Represents the preparation and cooking times for a recipe during creation.
#[derive(Clone, Debug, PartialEq)]
pub struct TimesForCreate {
    pub prep_seconds: i32,
    pub cook_seconds: i32,
}

impl Default for TimesForCreate {
    fn default() -> Self {
        Self {
            prep_seconds: 15 * 60,
            cook_seconds: 30 * 60,
        }
    }
}

impl TimesForCreate {
    /// Creates a TimesForCreate from its individual components.
    pub fn from_components(
        prep: Option<iso8601::Duration>,
        cook: Option<iso8601::Duration>,
    ) -> Self {
        Self {
            prep_seconds: prep
                .map(|d| {
                    let duration: std::time::Duration = d.into();
                    duration.as_secs()
                })
                .unwrap_or_else(|| 15 * 60) as i32,
            cook_seconds: cook
                .map(|d| {
                    let duration: std::time::Duration = d.into();
                    duration.as_secs()
                })
                .unwrap_or_else(|| 30 * 60) as i32,
        }
    }
}

impl From<TimesForCreate> for Times {
    fn from(times: TimesForCreate) -> Self {
        Self {
            prep_seconds: times.prep_seconds,
            cook_seconds: times.cook_seconds,
            total_seconds: times.prep_seconds + times.cook_seconds,
            ..Default::default()
        }
    }
}

/// Represents the preparation and cooking times for a recipe stored in the database.
#[derive(AsChangeset, Associations, Insertable, PartialEq)]
#[diesel(table_name = schema::times)]
#[diesel(belongs_to(Recipe))]
pub struct TimesForInsert {
    pub recipe_id: i64,
    pub prep_seconds: i32,
    pub cook_seconds: i32,
}
