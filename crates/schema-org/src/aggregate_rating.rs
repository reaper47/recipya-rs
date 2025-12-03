use serde::{Deserialize, Serialize};

use crate::field::{
    AggregateRatingAuthorFieldEnum, AggregateRatingBestRatingFieldEnum,
    AggregateRatingDescriptionFieldEnum, AggregateRatingImageFieldEnum,
    AggregateRatingRatingValueFieldEnum, AggregateRatingSubjectOfFieldEnum,
    AggregateRatingWorstRatingFieldEnum,
};
use crate::helpers::one_or_many;
use crate::{AtType, Thing};

///<https://schema.org/AggregateRating>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AggregateRating {
    #[serde(rename = "@type", default = "set_type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/reviewCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub review_count: Vec<i32>,
    ///<https://schema.org/itemReviewed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_reviewed: Vec<Thing>,
    ///<https://schema.org/ratingCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rating_count: Vec<i32>,
    ///<https://schema.org/worstRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub worst_rating: Vec<AggregateRatingWorstRatingFieldEnum>,
    ///<https://schema.org/reviewAspect>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub review_aspect: Vec<String>,
    ///<https://schema.org/bestRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub best_rating: Vec<AggregateRatingBestRatingFieldEnum>,
    ///<https://schema.org/author>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<AggregateRatingAuthorFieldEnum>,
    ///<https://schema.org/ratingExplanation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rating_explanation: Vec<String>,
    ///<https://schema.org/ratingValue>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rating_value: Vec<AggregateRatingRatingValueFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<AggregateRatingImageFieldEnum>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<AggregateRatingDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<AggregateRatingSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
}

impl AggregateRating {
    /// Create a new `AggregateRating` with the minimum fields populated for a single rating.
    pub fn new(rating_value: f32, rating_count: i32) -> Self {
        Self {
            r#type: set_type(),
            context: Some("https://schema.org".to_string()),
            rating_value: vec![AggregateRatingRatingValueFieldEnum::Number(rating_value)],
            rating_count: vec![rating_count],
            ..Default::default()
        }
    }
}

fn set_type() -> Option<String> {
    Some(AtType::AggregateRating.to_string())
}
