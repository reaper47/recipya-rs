use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::field::*;
use crate::helpers::one_or_many;
use crate::{Action, CreativeWork, Review, Thing};
use crate::aggregate_rating::AggregateRating;
use crate::audience::Audience;
use crate::enums::{EventAttendanceModeEnumerationEnum, EventStatusTypeEnum};
use crate::grant::Grant;
use crate::person::Person;
use crate::schedule::Schedule;

///<https://schema.org/endDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type EventEndDateFieldEnum = String;
///<https://schema.org/startDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type EventStartDateFieldEnum = String;
///<https://schema.org/doorTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type EventDoorTimeFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type EventAdditionalTypeFieldEnum = String;

///<https://schema.org/Event>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/maximumVirtualAttendeeCapacity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maximum_virtual_attendee_capacity: Vec<i32>,
    ///<https://schema.org/eventSchedule>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event_schedule: Vec<Schedule>,
    ///<https://schema.org/location>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<EventLocationFieldEnum>,
    ///<https://schema.org/endDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub end_date: Vec<EventEndDateFieldEnum>,
    ///<https://schema.org/subEvents>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sub_events: Vec<Event>,
    ///<https://schema.org/offers>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offers: Vec<EventOffersFieldEnum>,
    ///<https://schema.org/actor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<EventActorFieldEnum>,
    ///<https://schema.org/audience>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<Audience>,
    ///<https://schema.org/review>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub review: Vec<Review>,
    ///<https://schema.org/contributor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contributor: Vec<EventContributorFieldEnum>,
    ///<https://schema.org/subEvent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sub_event: Vec<Event>,
    ///<https://schema.org/attendees>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attendees: Vec<EventAttendeesFieldEnum>,
    ///<https://schema.org/recordedIn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recorded_in: Vec<CreativeWork>,
    ///<https://schema.org/startDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub start_date: Vec<EventStartDateFieldEnum>,
    ///<https://schema.org/workFeatured>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub work_featured: Vec<CreativeWork>,
    ///<https://schema.org/isAccessibleForFree>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_accessible_for_free: Vec<String>,
    ///<https://schema.org/keywords>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<EventKeywordsFieldEnum>,
    ///<https://schema.org/maximumPhysicalAttendeeCapacity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maximum_physical_attendee_capacity: Vec<i32>,
    ///<https://schema.org/duration>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub duration: Vec<EventDurationFieldEnum>,
    ///<https://schema.org/typicalAgeRange>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub typical_age_range: Vec<String>,
    ///<https://schema.org/funder>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funder: Vec<EventFunderFieldEnum>,
    ///<https://schema.org/eventStatus>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event_status: Vec<EventStatusTypeEnum>,
    ///<https://schema.org/composer>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub composer: Vec<EventComposerFieldEnum>,
    ///<https://schema.org/workPerformed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub work_performed: Vec<CreativeWork>,
    ///<https://schema.org/organizer>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub organizer: Vec<EventOrganizerFieldEnum>,
    ///<https://schema.org/sponsor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sponsor: Vec<EventSponsorFieldEnum>,
    ///<https://schema.org/aggregateRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/inLanguage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub in_language: Vec<EventInLanguageFieldEnum>,
    ///<https://schema.org/attendee>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attendee: Vec<EventAttendeeFieldEnum>,
    ///<https://schema.org/about>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub about: Vec<Thing>,
    ///<https://schema.org/funding>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funding: Vec<Grant>,
    ///<https://schema.org/maximumAttendeeCapacity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maximum_attendee_capacity: Vec<i32>,
    ///<https://schema.org/previousStartDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub previous_start_date: Vec<String>,
    ///<https://schema.org/doorTime>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub door_time: Vec<EventDoorTimeFieldEnum>,
    ///<https://schema.org/eventAttendanceMode>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event_attendance_mode: Vec<EventAttendanceModeEnumerationEnum>,
    ///<https://schema.org/translator>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub translator: Vec<EventTranslatorFieldEnum>,
    ///<https://schema.org/superEvent>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub super_event: Vec<Event>,
    ///<https://schema.org/performer>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<EventPerformerFieldEnum>,
    ///<https://schema.org/performers>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub performers: Vec<EventPerformersFieldEnum>,
    ///<https://schema.org/remainingAttendeeCapacity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub remaining_attendee_capacity: Vec<i32>,
    ///<https://schema.org/director>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub director: Vec<Person>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<EventAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<EventIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<EventImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<EventDescriptionFieldEnum>,
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
    pub subject_of: Vec<EventSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<EventMainEntityOfPageFieldEnum>,
}
