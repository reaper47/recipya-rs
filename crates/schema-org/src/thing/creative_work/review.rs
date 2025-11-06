use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::thing::CreativeWork;

/// A review of an item - for example, of a restaurant, movie, or store.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Review {
    /// An associated ClaimReview, related by specific common content, topic or claim. The
    /// expectation is that this property would be most typically used in cases where a single
    /// activity is conducting both claim reviews and media reviews, in which case
    /// relatedMediaReview would commonly be used on a ClaimReview, while relatedClaimReview would
    /// be used on MediaReview.
    pub associated_claim_review: Box<Review>,
    /// An associated MediaReview, related by specific common content, topic or claim. The
    /// expectation is that this property would be most typically used in cases where a single
    /// activity is conducting both claim reviews and media reviews, in which case
    /// relatedMediaReview would commonly be used on a ClaimReview, while relatedClaimReview would
    /// be used on MediaReview.
    pub associated_media_review: Box<Review>,
    /// An associated Review.
    pub associated_review: Box<Review>,
    /// The item that is being reviewed/rated.
    pub item_reviewed: Thing,
    /// Provides negative considerations regarding something, most typically in pro/con lists for
    /// reviews (alongside positiveNotes). For symmetry
    ///
    /// In the case of a Review, the property describes the itemReviewed from the perspective of the
    /// review; in the case of a Product, the product itself is being described. Since product
    /// descriptions tend to emphasise positive claims, it may be relatively unusual to find
    /// negativeNotes used in this way. Nevertheless for the sake of symmetry, negativeNotes can be
    /// used on Product.
    ///
    /// The property values can be expressed either as unstructured text (repeated as necessary), or
    /// if ordered, as a list (in which case the most negative is at the beginning of the list).
    pub negative_notes: ItemListOrListItemOrTextOrWebContent,
    /// Provides positive considerations regarding something, for example product highlights or (alongside negativeNotes) pro/con lists for reviews.
    ///
    /// In the case of a Review, the property describes the itemReviewed from the perspective of the review; in the case of a Product, the product itself is being described.
    ///
    /// The property values can be expressed either as unstructured text (repeated as necessary), or if ordered, as a list (in which case the most positive is at the beginning of the list).
    pub positive_notes: ItemListOrListItemOrTextOrWebContent,
    /// This Review or Rating is relevant to this part or facet of the itemReviewed.
    pub review_aspect: String,
    /// The actual body of the review.
    pub review_body: String,
    /// The rating given in this review. Note that reviews can themselves be rated. The reviewRating applies to rating given by the review. The aggregateRating property applies to the review itself, as a creative work.
    pub review_rating: Rating,
    #[serde(flatten)]
    pub creative_work: CreativeWork,
}
