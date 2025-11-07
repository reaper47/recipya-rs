mod aggregate_rating;

pub use aggregate_rating::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::permutations::person::OrganizationOrPerson;
use crate::permutations::text::NumberOrText;

/// A rating is an evaluation on a numeric scale, such as 1 to 5 stars.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Rating {
    /// The author of this content or rating. Please note that author is special in that HTML 5
    /// provides a special mechanism for indicating authorship via the rel tag. That is equivalent
    /// to this and may be used interchangeably.
    pub author: OrganizationOrPerson,
    /// The highest value allowed in this rating system.
    pub best_rating: NumberOrText,
    /// A short explanation (e.g. one to two sentences) providing background context and other
    /// information that led to the conclusion expressed in the rating. This is particularly
    /// applicable to ratings associated with "fact check" markup using ClaimReview.
    pub rating_explanation: String,
    /// The rating for the content.
    ///
    /// Usage guidelines:
    ///
    /// - Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather
    /// than superficially similar Unicode symbols.
    /// - Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid
    /// using these symbols as a readability separator.
    pub rating_value: NumberOrText,
    /// This Review or Rating is relevant to this part or facet of the itemReviewed.
    pub review_aspect: String,
    /// The lowest value allowed in this rating system.
    pub worst_rating: NumberOrText,
    #[serde(flatten)]
    pub thing: Thing,
}
