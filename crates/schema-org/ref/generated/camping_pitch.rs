use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/petsAllowed>
///<https://schema.org/Boolean>
///<https://schema.org/Text>
pub type CampingPitchPetsAllowedFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type CampingPitchAdditionalTypeFieldEnum = String;
///<https://schema.org/CampingPitch>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CampingPitch {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/permittedUsage>
    #[serde(rename = "permittedUsage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub permitted_usage: Vec<String>,
    ///<https://schema.org/occupancy>
    #[serde(rename = "occupancy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub occupancy: Vec<QuantitativeValue>,
    ///<https://schema.org/tourBookingPage>
    #[serde(rename = "tourBookingPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub tour_booking_page: Vec<String>,
    ///<https://schema.org/bed>
    #[serde(rename = "bed")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub bed: Vec<CampingPitchBedFieldEnum>,
    ///<https://schema.org/amenityFeature>
    #[serde(rename = "amenityFeature")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub amenity_feature: Vec<LocationFeatureSpecification>,
    ///<https://schema.org/numberOfBedrooms>
    #[serde(rename = "numberOfBedrooms")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_bedrooms: Vec<CampingPitchNumberOfBedroomsFieldEnum>,
    ///<https://schema.org/accommodationFloorPlan>
    #[serde(rename = "accommodationFloorPlan")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub accommodation_floor_plan: Vec<FloorPlan>,
    ///<https://schema.org/numberOfRooms>
    #[serde(rename = "numberOfRooms")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_rooms: Vec<CampingPitchNumberOfRoomsFieldEnum>,
    ///<https://schema.org/yearBuilt>
    #[serde(rename = "yearBuilt")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub year_built: Vec<f32>,
    ///<https://schema.org/leaseLength>
    #[serde(rename = "leaseLength")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub lease_length: Vec<CampingPitchLeaseLengthFieldEnum>,
    ///<https://schema.org/numberOfBathroomsTotal>
    #[serde(rename = "numberOfBathroomsTotal")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_bathrooms_total: Vec<i32>,
    ///<https://schema.org/floorLevel>
    #[serde(rename = "floorLevel")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub floor_level: Vec<String>,
    ///<https://schema.org/numberOfPartialBathrooms>
    #[serde(rename = "numberOfPartialBathrooms")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_partial_bathrooms: Vec<f32>,
    ///<https://schema.org/petsAllowed>
    #[serde(rename = "petsAllowed")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub pets_allowed: Vec<CampingPitchPetsAllowedFieldEnum>,
    ///<https://schema.org/numberOfFullBathrooms>
    #[serde(rename = "numberOfFullBathrooms")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_full_bathrooms: Vec<f32>,
    ///<https://schema.org/floorSize>
    #[serde(rename = "floorSize")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub floor_size: Vec<QuantitativeValue>,
    ///<https://schema.org/accommodationCategory>
    #[serde(rename = "accommodationCategory")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub accommodation_category: Vec<String>,
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
    pub geo_equals: Vec<CampingPitchGeoEqualsFieldEnum>,
    ///<https://schema.org/review>
    #[serde(rename = "review")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub review: Vec<Review>,
    ///<https://schema.org/latitude>
    #[serde(rename = "latitude")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub latitude: Vec<CampingPitchLatitudeFieldEnum>,
    ///<https://schema.org/geoDisjoint>
    #[serde(rename = "geoDisjoint")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_disjoint: Vec<CampingPitchGeoDisjointFieldEnum>,
    ///<https://schema.org/longitude>
    #[serde(rename = "longitude")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub longitude: Vec<CampingPitchLongitudeFieldEnum>,
    ///<https://schema.org/photo>
    #[serde(rename = "photo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub photo: Vec<CampingPitchPhotoFieldEnum>,
    ///<https://schema.org/hasDriveThroughService>
    #[serde(rename = "hasDriveThroughService")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_drive_through_service: Vec<String>,
    ///<https://schema.org/geo>
    #[serde(rename = "geo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo: Vec<CampingPitchGeoFieldEnum>,
    ///<https://schema.org/publicAccess>
    #[serde(rename = "publicAccess")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub public_access: Vec<String>,
    ///<https://schema.org/geoCovers>
    #[serde(rename = "geoCovers")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_covers: Vec<CampingPitchGeoCoversFieldEnum>,
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
    pub keywords: Vec<CampingPitchKeywordsFieldEnum>,
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
    pub geo_touches: Vec<CampingPitchGeoTouchesFieldEnum>,
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
    pub has_map: Vec<CampingPitchHasMapFieldEnum>,
    ///<https://schema.org/geoCrosses>
    #[serde(rename = "geoCrosses")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_crosses: Vec<CampingPitchGeoCrossesFieldEnum>,
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
    pub geo_contains: Vec<CampingPitchGeoContainsFieldEnum>,
    ///<https://schema.org/photos>
    #[serde(rename = "photos")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub photos: Vec<CampingPitchPhotosFieldEnum>,
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
    pub geo_overlaps: Vec<CampingPitchGeoOverlapsFieldEnum>,
    ///<https://schema.org/containedIn>
    #[serde(rename = "containedIn")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contained_in: Vec<Place>,
    ///<https://schema.org/geoIntersects>
    #[serde(rename = "geoIntersects")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_intersects: Vec<CampingPitchGeoIntersectsFieldEnum>,
    ///<https://schema.org/geoWithin>
    #[serde(rename = "geoWithin")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_within: Vec<CampingPitchGeoWithinFieldEnum>,
    ///<https://schema.org/event>
    #[serde(rename = "event")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub event: Vec<Event>,
    ///<https://schema.org/address>
    #[serde(rename = "address")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub address: Vec<CampingPitchAddressFieldEnum>,
    ///<https://schema.org/additionalProperty>
    #[serde(rename = "additionalProperty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/geoCoveredBy>
    #[serde(rename = "geoCoveredBy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_covered_by: Vec<CampingPitchGeoCoveredByFieldEnum>,
    ///<https://schema.org/logo>
    #[serde(rename = "logo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub logo: Vec<CampingPitchLogoFieldEnum>,
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
    pub additional_type: Vec<CampingPitchAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<CampingPitchIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<CampingPitchImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<CampingPitchDescriptionFieldEnum>,
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
    pub subject_of: Vec<CampingPitchSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<CampingPitchMainEntityOfPageFieldEnum>,
}
