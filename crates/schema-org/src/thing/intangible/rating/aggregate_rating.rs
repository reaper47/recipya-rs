use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::thing::intangible::rating::Rating;

/// The average rating based on multiple ratings or reviews.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AggregateRating {
    /// The item that is being reviewed/rated.
    pub item_reviewed: Thing,
    /// The count of total number of ratings.
    pub rating_count: i32,
    /// The count of total number of reviews.
    pub review_count: i32,
    #[serde(flatten)]
    pub rating: Rating,
}
