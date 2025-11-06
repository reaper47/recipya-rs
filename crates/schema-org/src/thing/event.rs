use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::data_type::Date;
use crate::permutations::date::DateOrDateTime;
use crate::permutations::person::OrganizationOrPerson;
use crate::permutations::place::PlaceOrPostalAddressOrTextOrVirtualLocation;
use crate::thing::intangible::Audience;
use crate::thing::intangible::grant::Grant;
use crate::thing::{CreativeWork, Person};

/// An event happening at a certain time and location, such as a concert, lecture, or festival.
/// Ticketing information may be added via the offers property. Repeated events may be structured as
/// separate Event objects.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Event {
    /// The subject matter of the content.
    ///
    /// Inverse property: subjectOf
    pub about: Thing,
    /// An actor (individual or a group), e.g. in TV, radio, movie, video games etc., or in an
    /// event. Actors can be associated with individual items or with a series, episode, clip.
    /// Supersedes actors.
    pub actor: PerformingGroupOrPerson,
    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating: AggregateRating,
    /// A person or organization attending the event. Supersedes attendees.
    pub attendee: OrganizationOrPerson,
    /// An intended audience, i.e. a group for whom something was created.
    /// Supersedes serviceAudience.
    pub audience: Audience,
    /// The person or organization who wrote a composition, or who is the composer of a work
    /// performed at some event.
    pub composer: OrganizationOrPerson,
    /// A secondary contributor to the CreativeWork or Event.
    pub contributor: OrganizationOrPerson,
    /// A director of e.g. TV, radio, movie, video gaming etc. content, or of an event. Directors
    /// can be associated with individual items or with a series, episode, clip. Supersedes
    /// directors.
    pub director: Person,
    /// The time admission will commence.
    pub door_time: DateOrDateTime,
    /// The duration of the item (movie, audio recording, event, etc.) in ISO 8601 duration format.
    pub duration: DurationOrQuantitativeValue,
    /// The end date and time of the item (in ISO 8601 date format).
    pub end_date: DateOrDateTime,
    /// The eventAttendanceMode of an event indicates whether it occurs online, offline, or a mix.
    pub event_attendance_mode: EventAttendanceModeEnumeration,
    /// Associates an Event with a Schedule. There are circumstances where it is preferable to share
    /// a schedule for a series of repeating events rather than data on the individual events
    /// themselves. For example, a website or application might prefer to publish a schedule for a
    /// weekly gym class rather than provide data on every event. A schedule could be processed by
    /// applications to add forthcoming events to a calendar. An Event that is associated with a
    /// Schedule using this property should not have startDate or endDate properties. These are
    /// instead defined within the associated Schedule, this avoids any ambiguity for clients using
    /// the data. The property might have repeated values to specify different schedules, e.g. for
    /// different months or seasons.
    pub event_schedule: Schedule,
    /// An eventStatus of an event represents its status; particularly useful when an event is
    /// cancelled or rescheduled.
    pub event_status: EventStatusType,
    /// A person or organization that supports (sponsors) something through some kind of financial
    /// contribution.
    pub funder: OrganizationOrPerson,
    /// A Grant that directly or indirectly provide funding or sponsorship for this item. See also
    /// ownershipFundingInfo.
    ///
    /// Inverse property: fundedItem
    pub funding: Grant,
    /// The language of the content or performance or used in an action. Please use one of the
    /// language codes from the IETF BCP 47 standard. See also availableLanguage. Supersedes
    /// language.
    pub in_language: LanguageOrText,
    /// A flag to signal that the item, event, or place is accessible for free. Supersedes free.
    pub is_accessible_for_free: bool,
    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are
    /// typically delimited by commas, or by repeating the property.
    pub keywords: DefinedTermOrTextOrURL,
    /// The location of, for example, where an event is happening, where an organization is located,
    /// or where an action takes place.
    pub location: PlaceOrPostalAddressOrTextOrVirtualLocation,
    /// The total number of individuals that may attend an event or venue.
    pub maximum_attendee_capacity: i32,
    /// The maximum physical attendee capacity of an Event whose eventAttendanceMode is
    /// OfflineEventAttendanceMode (or the offline aspects, in the case of a
    /// MixedEventAttendanceMode).
    pub maximum_physical_attendee_capacity: i32,
    /// The maximum virtual attendee capacity of an Event whose eventAttendanceMode is
    /// OnlineEventAttendanceMode (or the online aspects, in the case of a
    /// MixedEventAttendanceMode).
    pub maximum_virtual_attendee_capacity: i32,
    /// An offer to provide this item—for example, an offer to sell a product, rent the DVD of a
    /// movie, perform a service, or give away tickets to an event. Use businessFunction to indicate
    /// the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to
    /// describe a Demand. While this property is listed as expected on a number of common types, it
    /// can be used in others. In that case, using a second type, such as Product or a subtype of
    /// Product, can clarify the nature of the offer.
    ///
    /// Inverse property: itemOffered
    pub offers: DemandOrOffer,
    /// An organizer of an Event.
    pub organizer: OrganizationOrPerson,
    /// A performer at the event—for example, a presenter, musician, musical group or actor.
    /// Supersedes performers.
    pub performer: OrganizationOrPerson,
    /// Used in conjunction with eventStatus for rescheduled or cancelled events. This property
    /// contains the previously scheduled start date. For rescheduled events, the startDate property
    /// should be used for the newly scheduled start date. In the (rare) case of an event that has
    /// been postponed and rescheduled multiple times, this field may be repeated.
    pub previous_start_date: Date,
    /// The CreativeWork that captured all or part of this Event.
    ///
    /// Inverse property: recordedAt
    pub recorded_in: CreativeWork,
    /// The number of attendee places for an event that remain unallocated.
    pub remaining_attendee_capacity: i32,
    /// A review of the item. Supersedes reviews.
    pub review: Review,
    /// A person or organization that supports a thing through a pledge, promise, or financial
    /// contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
    pub sponsor: OrganizationOrPerson,
    /// The start date and time of the item (in ISO 8601 date format).
    pub start_date: DateOrDateTime,
    /// An Event that is part of this event. For example, a conference event includes many
    /// presentations, each of which is a subEvent of the conference. Supersedes subEvents.
    ///
    /// Inverse property: superEvent
    pub sub_event: Event,
    /// An event that this event is a part of. For example, a collection of individual music
    /// performances might each have a music festival as their superEvent.
    ///
    /// Inverse property: subEvent
    pub super_event: Event,
    /// Organization or person who adapts a creative work to different languages, regional
    /// differences and technical requirements of a target market, or that translates during
    /// some event.
    pub translator: OrganizationOrPerson,
    /// The typical expected age range, e.g. '7-9', '11-'.
    pub typical_age_range: String,
    /// A work featured in some event, e.g. exhibited in an ExhibitionEvent. Specific subproperties
    /// are available for workPerformed (e.g. a play), or a workPresented (a Movie at a
    /// ScreeningEvent).
    pub work_featured: CreativeWork,
    /// A work performed in some event, for example a play performed in a TheaterEvent.
    pub work_performed: CreativeWork,
    #[serde(flatten)]
    pub thing: Thing,
}
