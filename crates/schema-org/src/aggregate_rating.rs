use serde::{Deserialize, Serialize};

use crate::field::{
    AggregateRatingAuthorFieldEnum, AggregateRatingBestRatingFieldEnum,
    AggregateRatingDescriptionFieldEnum, AggregateRatingImageFieldEnum,
    AggregateRatingRatingValueFieldEnum, AggregateRatingSubjectOfFieldEnum,
    AggregateRatingWorstRatingFieldEnum, FieldEnum8,
};
use crate::helpers::{one_or_many, one_or_many_string_or_num};
use crate::{AtType, Thing, at_context};

const COMMON_SCALES: [f32; 3] = [5.0, 10.0, 100.0];

///<https://schema.org/AggregateRating>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct AggregateRating {
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/reviewCount>
    #[serde(default, deserialize_with = "one_or_many_string_or_num")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub review_count: Vec<i32>,
    ///<https://schema.org/itemReviewed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_reviewed: Vec<Thing>,
    ///<https://schema.org/ratingCount>
    #[serde(default, deserialize_with = "one_or_many_string_or_num")]
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
            r#type: AtType::AggregateRating.to_opt(),
            context: at_context(),
            rating_value: vec![AggregateRatingRatingValueFieldEnum::Number(rating_value)],
            rating_count: vec![rating_count],
            ..Default::default()
        }
    }

    /// Normalizes the rating between 0 and 10.
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn normalize_over_ten(&self) -> u8 {
        let to_f32 = |v: &[FieldEnum8]| {
            v.first().map_or(0.0, |field| match field {
                AggregateRatingRatingValueFieldEnum::Number(n) => *n,
                AggregateRatingRatingValueFieldEnum::Text(s) => {
                    s.parse::<f32>().unwrap_or_default()
                }
            })
        };

        let rating = to_f32(&self.rating_value);
        let best_rating = Some(to_f32(&self.best_rating))
            .filter(|&b| b > 0.0)
            .unwrap_or_else(|| {
                COMMON_SCALES
                    .into_iter()
                    .find(|&scale| rating <= scale)
                    .unwrap_or(rating)
            });

        let normalized = if best_rating <= 5.0 {
            rating * 2.0
        } else if best_rating <= 10.0 {
            (rating * 2.0).round() / 2.0
        } else {
            ((rating / best_rating * 10.) * 10.0).round() / 10.0
        };

        normalized as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rating = AggregateRating::new(4.5, 10);

        assert_eq!(rating.r#type, AtType::AggregateRating.to_opt());
        assert_eq!(rating.context, at_context());
        assert_eq!(
            rating.rating_value,
            vec![AggregateRatingRatingValueFieldEnum::Number(4.5)]
        );
        assert_eq!(rating.rating_count, vec![10]);
    }

    mod tests_normalize_over_10 {
        use super::*;

        #[test]
        fn test_rating_less_than_5() {
            let rating = AggregateRating::new(4.5, 10);

            let got = rating.normalize_over_ten();

            assert_eq!(got, 9)
        }

        #[test]
        fn test_rating_less_than_10() {
            let rating = AggregateRating::new(7.74, 201);

            let got = rating.normalize_over_ten();

            assert_eq!(got, 7)
        }

        #[test]
        fn test_rating_less_than_100() {
            let rating = AggregateRating::new(21.0, 10);

            let got = rating.normalize_over_ten();

            assert_eq!(got, 2)
        }
    }
}
