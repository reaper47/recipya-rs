use schemars::JsonSchema;
use serde::Deserialize;

use crate::components::{AggregateRating, ImageObjectOrUrl, Review, Thing};

/// A brand is a name used by an organization or business person for labeling a product, product
/// group, or similar.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Brand {
    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating: AggregateRating,
    /// An associated logo.
    pub logo: ImageObjectOrUrl,
    /// A review of the item. Supersedes reviews.
    pub review: Review,
    /// A slogan or motto associated with the item. 
    pub slogan: String,
    #[serde(flatten)]
    pub thing: Thing,
}
