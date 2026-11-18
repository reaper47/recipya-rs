use diesel::prelude::*;

use jiff::{Span, SpanRelativeTo};
use repository::schema;

use crate::{
    Recipe,
    types::{DurationSeconds, FIFTEEN_MINUTES_SEC, THIRTY_MINUTES_SEC},
};

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
    pub prep_seconds: DurationSeconds,
    pub cook_seconds: DurationSeconds,
    pub total_seconds: DurationSeconds,
}

/// Represents the preparation and cooking times for a recipe during creation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimesForCreate {
    pub prep_seconds: DurationSeconds,
    pub cook_seconds: DurationSeconds,
}

impl Default for TimesForCreate {
    fn default() -> Self {
        Self {
            prep_seconds: FIFTEEN_MINUTES_SEC.into(),
            cook_seconds: THIRTY_MINUTES_SEC.into(),
        }
    }
}

impl TimesForCreate {
    /// Creates a `TimesForCreate` from its individual components.
    pub fn from_components(prep: Option<Span>, cook: Option<Span>, total: Option<Span>) -> Self {
        let prep_seconds = prep
            .and_then(span_to_seconds)
            .unwrap_or_else(|| FIFTEEN_MINUTES_SEC.into());

        let cook_seconds = cook
            .and_then(span_to_seconds)
            .unwrap_or_else(|| THIRTY_MINUTES_SEC.into());

        let total_seconds = total
            .and_then(span_to_seconds)
            .unwrap_or_else(|| (FIFTEEN_MINUTES_SEC + THIRTY_MINUTES_SEC).into());

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

fn span_to_seconds(span: Span) -> Option<DurationSeconds> {
    let secs = span
        .to_duration(SpanRelativeTo::days_are_24_hours())
        .ok()?
        .as_secs();
    u64::try_from(secs).ok().map(DurationSeconds::from)
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
    pub prep_seconds: DurationSeconds,
    pub cook_seconds: DurationSeconds,
}
