use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ParkAdditionalTypeFieldEnum = String;
///<https://schema.org/Park>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Park {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/openingHours>
    #[serde(rename = "openingHours")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub opening_hours: Vec<String>,
    ///<https://schema.org/containedInPlace>
    #[serde(rename = "containedInPlace")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contained_in_place: Vec<Place>,
    ///<https://schema.org/faxNumber>
    #[serde(rename = "faxNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub fax_number: Vec<String>,
    ///<https://schema.org/map>
    #[serde(rename = "map")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub map: Vec<String>,
    ///<https://schema.org/geoEquals>
    #[serde(rename = "geoEquals")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_equals: Vec<ParkGeoEqualsFieldEnum>,
    ///<https://schema.org/review>
    #[serde(rename = "review")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub review: Vec<Review>,
    ///<https://schema.org/latitude>
    #[serde(rename = "latitude")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub latitude: Vec<ParkLatitudeFieldEnum>,
    ///<https://schema.org/geoDisjoint>
    #[serde(rename = "geoDisjoint")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_disjoint: Vec<ParkGeoDisjointFieldEnum>,
    ///<https://schema.org/tourBookingPage>
    #[serde(rename = "tourBookingPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub tour_booking_page: Vec<String>,
    ///<https://schema.org/longitude>
    #[serde(rename = "longitude")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub longitude: Vec<ParkLongitudeFieldEnum>,
    ///<https://schema.org/photo>
    #[serde(rename = "photo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub photo: Vec<ParkPhotoFieldEnum>,
    ///<https://schema.org/hasDriveThroughService>
    #[serde(rename = "hasDriveThroughService")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_drive_through_service: Vec<String>,
    ///<https://schema.org/geo>
    #[serde(rename = "geo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo: Vec<ParkGeoFieldEnum>,
    ///<https://schema.org/publicAccess>
    #[serde(rename = "publicAccess")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub public_access: Vec<String>,
    ///<https://schema.org/geoCovers>
    #[serde(rename = "geoCovers")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_covers: Vec<ParkGeoCoversFieldEnum>,
    ///<https://schema.org/specialOpeningHoursSpecification>
    #[serde(rename = "specialOpeningHoursSpecification")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub special_opening_hours_specification: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/isAccessibleForFree>
    #[serde(rename = "isAccessibleForFree")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_accessible_for_free: Vec<String>,
    ///<https://schema.org/keywords>
    #[serde(rename = "keywords")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub keywords: Vec<ParkKeywordsFieldEnum>,
    ///<https://schema.org/amenityFeature>
    #[serde(rename = "amenityFeature")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub amenity_feature: Vec<LocationFeatureSpecification>,
    ///<https://schema.org/openingHoursSpecification>
    #[serde(rename = "openingHoursSpecification")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub opening_hours_specification: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/globalLocationNumber>
    #[serde(rename = "globalLocationNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub global_location_number: Vec<String>,
    ///<https://schema.org/smokingAllowed>
    #[serde(rename = "smokingAllowed")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub smoking_allowed: Vec<String>,
    ///<https://schema.org/geoTouches>
    #[serde(rename = "geoTouches")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_touches: Vec<ParkGeoTouchesFieldEnum>,
    ///<https://schema.org/events>
    #[serde(rename = "events")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub events: Vec<Event>,
    ///<https://schema.org/telephone>
    #[serde(rename = "telephone")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub telephone: Vec<String>,
    ///<https://schema.org/slogan>
    #[serde(rename = "slogan")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub slogan: Vec<String>,
    ///<https://schema.org/hasMap>
    #[serde(rename = "hasMap")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_map: Vec<ParkHasMapFieldEnum>,
    ///<https://schema.org/geoCrosses>
    #[serde(rename = "geoCrosses")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_crosses: Vec<ParkGeoCrossesFieldEnum>,
    ///<https://schema.org/aggregateRating>
    #[serde(rename = "aggregateRating")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/hasGS1DigitalLink>
    #[serde(rename = "hasGS1DigitalLink")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_gs1_digital_link: Vec<String>,
    ///<https://schema.org/isicV4>
    #[serde(rename = "isicV4")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub isic_v4: Vec<String>,
    ///<https://schema.org/geoContains>
    #[serde(rename = "geoContains")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_contains: Vec<ParkGeoContainsFieldEnum>,
    ///<https://schema.org/photos>
    #[serde(rename = "photos")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub photos: Vec<ParkPhotosFieldEnum>,
    ///<https://schema.org/maps>
    #[serde(rename = "maps")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub maps: Vec<String>,
    ///<https://schema.org/hasCertification>
    #[serde(rename = "hasCertification")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_certification: Vec<Certification>,
    ///<https://schema.org/containsPlace>
    #[serde(rename = "containsPlace")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contains_place: Vec<Place>,
    ///<https://schema.org/branchCode>
    #[serde(rename = "branchCode")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub branch_code: Vec<String>,
    ///<https://schema.org/maximumAttendeeCapacity>
    #[serde(rename = "maximumAttendeeCapacity")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub maximum_attendee_capacity: Vec<i32>,
    ///<https://schema.org/geoOverlaps>
    #[serde(rename = "geoOverlaps")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_overlaps: Vec<ParkGeoOverlapsFieldEnum>,
    ///<https://schema.org/containedIn>
    #[serde(rename = "containedIn")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contained_in: Vec<Place>,
    ///<https://schema.org/geoIntersects>
    #[serde(rename = "geoIntersects")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_intersects: Vec<ParkGeoIntersectsFieldEnum>,
    ///<https://schema.org/geoWithin>
    #[serde(rename = "geoWithin")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_within: Vec<ParkGeoWithinFieldEnum>,
    ///<https://schema.org/event>
    #[serde(rename = "event")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub event: Vec<Event>,
    ///<https://schema.org/address>
    #[serde(rename = "address")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub address: Vec<ParkAddressFieldEnum>,
    ///<https://schema.org/additionalProperty>
    #[serde(rename = "additionalProperty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/geoCoveredBy>
    #[serde(rename = "geoCoveredBy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_covered_by: Vec<ParkGeoCoveredByFieldEnum>,
    ///<https://schema.org/logo>
    #[serde(rename = "logo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub logo: Vec<ParkLogoFieldEnum>,
    ///<https://schema.org/reviews>
    #[serde(rename = "reviews")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub reviews: Vec<Review>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(rename = "potentialAction")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(rename = "additionalType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_type: Vec<ParkAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<ParkIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<ParkImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<ParkDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub subject_of: Vec<ParkSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<ParkMainEntityOfPageFieldEnum>,
}
