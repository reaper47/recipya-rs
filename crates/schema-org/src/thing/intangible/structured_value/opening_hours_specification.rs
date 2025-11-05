use schemars::JsonSchema;
use serde::Deserialize;

use crate::data_type::time::Time;
use crate::permutations::date::DateOrDateTime;
use crate::thing::intangible::enumeration::DayOfWeek;
use crate::Thing;

/// A structured value providing information about the opening hours of a place or a certain service
/// inside a place.
///
/// The place is open if the opens property is specified, and closed otherwise.
///
/// If the value for the closes property is less than the value for the opens property then the hour
/// range is assumed to span over the next day.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OpeningHoursSpecification {
    /// The closing hour of the place or service on the given day(s) of the week.
    pub closes: Box<Time>,
    /// The day of the week for which these opening hours are valid.
    pub day_of_week: DayOfWeek,
    /// The opening hour of the place or service on the given day(s) of the week.
    pub opens: Box<Time>,
    /// The date when the item becomes valid.
    pub valid_from: DateOrDateTime,
    /// The date after when the item is not valid. For example the end of an offer, salary period,
    /// or a period of opening hours.
    pub valid_through: DateOrDateTime,
    #[serde(flatten)]
    pub thing: Thing,
}

