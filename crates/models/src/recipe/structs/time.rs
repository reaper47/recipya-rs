use diesel::prelude::*;

use repository::schema;

use crate::Recipe;

/// Represents a time components of a recipe.
#[derive(
    Clone, Debug, Default, Eq, PartialEq, Associations, Queryable, Identifiable, Selectable,
)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
    /// Creates a `TimesForCreate` from its individual components.
    pub fn from_components(
        prep: Option<iso8601::Duration>,
        cook: Option<iso8601::Duration>,
        total: Option<iso8601::Duration>,
    ) -> Self {
        let prep_seconds = i32::try_from(prep.map_or_else(
            || 15 * 60,
            |d| {
                let duration: std::time::Duration = d.into();
                duration.as_secs()
            },
        ))
        .unwrap_or_default();

        let cook_seconds = i32::try_from(cook.map_or_else(
            || 30 * 60,
            |d| {
                let duration: std::time::Duration = d.into();
                duration.as_secs()
            },
        ))
        .unwrap_or_default();

        let total_seconds = i32::try_from(total.map_or_else(
            || 15 * 60 + 30 * 60,
            |d| {
                let duration: std::time::Duration = d.into();
                duration.as_secs()
            },
        ))
        .unwrap_or_default();

        let (prep_seconds, cook_seconds) = match total {
            Some(_) => {
                if prep.is_some() && cook.is_none() {
                    (prep_seconds, total_seconds - prep_seconds)
                } else if prep.is_none() && cook.is_some() {
                    (total_seconds - cook_seconds, cook_seconds)
                } else {
                    (prep_seconds, cook_seconds)
                }
            }
            None => (prep_seconds, cook_seconds),
        };

        Self {
            prep_seconds,
            cook_seconds,
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

impl From<Times> for TimesForCreate {
    fn from(t: Times) -> Self {
        Self {
            prep_seconds: t.prep_seconds,
            cook_seconds: t.cook_seconds,
        }
    }
}

/// Represents the preparation and cooking times for a recipe stored in the database.
#[derive(AsChangeset, Associations, Insertable, Eq, PartialEq)]
#[diesel(table_name = schema::times)]
#[diesel(belongs_to(Recipe))]
pub struct TimesForInsert {
    pub recipe_id: i64,
    pub prep_seconds: i32,
    pub cook_seconds: i32,
}
