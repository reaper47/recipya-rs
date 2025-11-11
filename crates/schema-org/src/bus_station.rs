use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::field::*;
use crate::{Action, Event, Place, PropertyValue, Review};
use crate::aggregate_rating::AggregateRating;
use crate::certification::Certification;
use crate::location_feature_specification::LocationFeatureSpecification;
use crate::opening_hours_specification::OpeningHoursSpecification;

///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type BusStationAdditionalTypeFieldEnum = String;

///<https://schema.org/BusStation>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BusStation {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/openingHours>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub opening_hours: Vec<String>,
    ///<https://schema.org/containedInPlace>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contained_in_place: Vec<Place>,
    ///<https://schema.org/faxNumber>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fax_number: Vec<String>,
    ///<https://schema.org/map>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub map: Vec<String>,
    ///<https://schema.org/geoEquals>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geo_equals: Vec<BusStationGeoEqualsFieldEnum>,
    ///<https://schema.org/review>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub review: Vec<Review>,
    ///<https://schema.org/latitude>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub latitude: Vec<BusStationLatitudeFieldEnum>,
    ///<https://schema.org/geoDisjoint>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geo_disjoint: Vec<BusStationGeoDisjointFieldEnum>,
    ///<https://schema.org/tourBookingPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tour_booking_page: Vec<String>,
    ///<https://schema.org/longitude>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub longitude: Vec<BusStationLongitudeFieldEnum>,
    ///<https://schema.org/photo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub photo: Vec<BusStationPhotoFieldEnum>,
    ///<https://schema.org/hasDriveThroughService>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_drive_through_service: Vec<String>,
    ///<https://schema.org/geo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geo: Vec<BusStationGeoFieldEnum>,
    ///<https://schema.org/publicAccess>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub public_access: Vec<String>,
    ///<https://schema.org/geoCovers>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geo_covers: Vec<BusStationGeoCoversFieldEnum>,
    ///<https://schema.org/specialOpeningHoursSpecification>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub special_opening_hours_specification: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/isAccessibleForFree>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_accessible_for_free: Vec<String>,
    ///<https://schema.org/keywords>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<BusStationKeywordsFieldEnum>,
    ///<https://schema.org/amenityFeature>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub amenity_feature: Vec<LocationFeatureSpecification>,
    ///<https://schema.org/openingHoursSpecification>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub opening_hours_specification: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/globalLocationNumber>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub global_location_number: Vec<String>,
    ///<https://schema.org/smokingAllowed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub smoking_allowed: Vec<String>,
    ///<https://schema.org/geoTouches>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geo_touches: Vec<BusStationGeoTouchesFieldEnum>,
    ///<https://schema.org/events>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<Event>,
    ///<https://schema.org/telephone>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub telephone: Vec<String>,
    ///<https://schema.org/slogan>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub slogan: Vec<String>,
    ///<https://schema.org/hasMap>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_map: Vec<BusStationHasMapFieldEnum>,
    ///<https://schema.org/geoCrosses>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geo_crosses: Vec<BusStationGeoCrossesFieldEnum>,
    ///<https://schema.org/aggregateRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/hasGS1DigitalLink>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_gs1_digital_link: Vec<String>,
    ///<https://schema.org/isicV4>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub isic_v4: Vec<String>,
    ///<https://schema.org/geoContains>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geo_contains: Vec<BusStationGeoContainsFieldEnum>,
    ///<https://schema.org/photos>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub photos: Vec<BusStationPhotosFieldEnum>,
    ///<https://schema.org/maps>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maps: Vec<String>,
    ///<https://schema.org/hasCertification>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_certification: Vec<Certification>,
    ///<https://schema.org/containsPlace>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contains_place: Vec<Place>,
    ///<https://schema.org/branchCode>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub branch_code: Vec<String>,
    ///<https://schema.org/maximumAttendeeCapacity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maximum_attendee_capacity: Vec<i32>,
    ///<https://schema.org/geoOverlaps>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geo_overlaps: Vec<BusStationGeoOverlapsFieldEnum>,
    ///<https://schema.org/containedIn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contained_in: Vec<Place>,
    ///<https://schema.org/geoIntersects>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geo_intersects: Vec<BusStationGeoIntersectsFieldEnum>,
    ///<https://schema.org/geoWithin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geo_within: Vec<BusStationGeoWithinFieldEnum>,
    ///<https://schema.org/event>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<Event>,
    ///<https://schema.org/address>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub address: Vec<BusStationAddressFieldEnum>,
    ///<https://schema.org/additionalProperty>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/geoCoveredBy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geo_covered_by: Vec<BusStationGeoCoveredByFieldEnum>,
    ///<https://schema.org/logo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub logo: Vec<BusStationLogoFieldEnum>,
    ///<https://schema.org/reviews>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reviews: Vec<Review>,
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
    pub additional_type: Vec<BusStationAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<BusStationIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<BusStationImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<BusStationDescriptionFieldEnum>,
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
    pub subject_of: Vec<BusStationSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<BusStationMainEntityOfPageFieldEnum>,
}
