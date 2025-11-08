use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/petsAllowed>
///<https://schema.org/Boolean>
///<https://schema.org/Text>
pub type FloorPlanPetsAllowedFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type FloorPlanAdditionalTypeFieldEnum = String;
///<https://schema.org/FloorPlan>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct FloorPlan {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/layoutImage>
    #[serde(rename = "layoutImage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub layout_image: Vec<FloorPlanLayoutImageFieldEnum>,
    ///<https://schema.org/amenityFeature>
    #[serde(rename = "amenityFeature")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub amenity_feature: Vec<LocationFeatureSpecification>,
    ///<https://schema.org/numberOfBedrooms>
    #[serde(rename = "numberOfBedrooms")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_bedrooms: Vec<FloorPlanNumberOfBedroomsFieldEnum>,
    ///<https://schema.org/numberOfRooms>
    #[serde(rename = "numberOfRooms")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_rooms: Vec<FloorPlanNumberOfRoomsFieldEnum>,
    ///<https://schema.org/numberOfBathroomsTotal>
    #[serde(rename = "numberOfBathroomsTotal")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_bathrooms_total: Vec<i32>,
    ///<https://schema.org/numberOfPartialBathrooms>
    #[serde(rename = "numberOfPartialBathrooms")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_partial_bathrooms: Vec<f32>,
    ///<https://schema.org/petsAllowed>
    #[serde(rename = "petsAllowed")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub pets_allowed: Vec<FloorPlanPetsAllowedFieldEnum>,
    ///<https://schema.org/isPlanForApartment>
    #[serde(rename = "isPlanForApartment")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_plan_for_apartment: Vec<Accommodation>,
    ///<https://schema.org/numberOfFullBathrooms>
    #[serde(rename = "numberOfFullBathrooms")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_full_bathrooms: Vec<f32>,
    ///<https://schema.org/numberOfAccommodationUnits>
    #[serde(rename = "numberOfAccommodationUnits")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_accommodation_units: Vec<QuantitativeValue>,
    ///<https://schema.org/floorSize>
    #[serde(rename = "floorSize")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub floor_size: Vec<QuantitativeValue>,
    ///<https://schema.org/numberOfAvailableAccommodationUnits>
    #[serde(rename = "numberOfAvailableAccommodationUnits")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_available_accommodation_units: Vec<QuantitativeValue>,
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
    pub additional_type: Vec<FloorPlanAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<FloorPlanIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<FloorPlanImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<FloorPlanDescriptionFieldEnum>,
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
    pub subject_of: Vec<FloorPlanSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<FloorPlanMainEntityOfPageFieldEnum>,
}
