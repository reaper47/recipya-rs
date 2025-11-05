use schemars::JsonSchema;
use serde::Deserialize;
use crate::permutations::date::DateOrDateTime;
use crate::permutations::place::PlaceOrPostalAddressOrTextOrVirtualLocation;
use crate::Thing;
use crate::thing::Action;

/// A summary of how users have interacted with this CreativeWork. In most cases, authors will use
/// a subtype to specify the specific type of interaction.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct InteractionCounter {
    /// The endTime of something. For a reserved event or service (e.g.
    /// FoodEstablishmentReservation), the time that it is expected to end. For actions that span a
    /// period of time, when the action was performed. E.g. John wrote a book from January to
    /// December. For media, including audio and video, it's the time offset of the end of a clip
    /// within a larger file.
    ///
    /// Note that Event uses startDate/endDate instead of startTime/endTime, even when describing
    /// dates with times. This situation may be clarified in future revisions.
    pub end_time: 	DateOrDateTime,
    /// The WebSite or SoftwareApplication where the interactions took place.
    pub interaction_service: 	SoftwareApplicationOrWebSite,
    /// The Action representing the type of interaction. For up votes, +1s, etc. use LikeAction.
    /// For down votes use DislikeAction. Otherwise, use the most specific Action.
    pub interaction_type: 	Action,
    /// The location of, for example, where an event is happening, where an organization is located,
    /// or where an action takes place.
    pub location: 	PlaceOrPostalAddressOrTextOrVirtualLocation,
    /// The startTime of something. For a reserved event or service (e.g.
    /// FoodEstablishmentReservation), the time that it is expected to start. For actions that span
    /// a period of time, when the action was performed. E.g. John wrote a book from January to
    /// December. For media, including audio and video, it's the time offset of the start of a clip
    /// within a larger file.
    ///
    /// Note that Event uses startDate/endDate instead of startTime/endTime, even when describing dates with times. This situation may be clarified in future revisions.
    pub start_time: 	DateOrDateTime,
    /// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication.
    pub user_interaction_count: 	i32,
    #[serde(flatten)]
    pub thing: Thing,
}
