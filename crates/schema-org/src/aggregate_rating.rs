use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::field::{
    AggregateRatingAuthorFieldEnum, AggregateRatingBestRatingFieldEnum,
    AggregateRatingDescriptionFieldEnum, AggregateRatingImageFieldEnum,
    AggregateRatingRatingValueFieldEnum, AggregateRatingSubjectOfFieldEnum,
    AggregateRatingWorstRatingFieldEnum,
};
use crate::helpers::{is_smallvec_empty, one_or_many};
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
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub review_count: SmallVec<[i32; 1]>,
    ///<https://schema.org/itemReviewed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub item_reviewed: SmallVec<[Thing; 1]>,
    ///<https://schema.org/ratingCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub rating_count: SmallVec<[i32; 1]>,
    ///<https://schema.org/worstRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub worst_rating: SmallVec<[AggregateRatingWorstRatingFieldEnum; 1]>,
    ///<https://schema.org/reviewAspect>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub review_aspect: SmallVec<[String; 1]>,
    ///<https://schema.org/bestRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub best_rating: SmallVec<[AggregateRatingBestRatingFieldEnum; 1]>,
    ///<https://schema.org/author>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub author: SmallVec<[AggregateRatingAuthorFieldEnum; 1]>,
    ///<https://schema.org/ratingExplanation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub rating_explanation: SmallVec<[String; 1]>,
    ///<https://schema.org/ratingValue>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub rating_value: SmallVec<[AggregateRatingRatingValueFieldEnum; 1]>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub image: SmallVec<[AggregateRatingImageFieldEnum; 2]>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub description: SmallVec<[AggregateRatingDescriptionFieldEnum; 1]>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub alternate_name: SmallVec<[String; 1]>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub url: SmallVec<[String; 1]>,
    ///<https://schema.org/subjectOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub subject_of: SmallVec<[AggregateRatingSubjectOfFieldEnum; 1]>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub name: SmallVec<[String; 1]>,
}

fn set_type() -> Option<String> {
    Some(AtType::AggregateRating.to_string())
}
