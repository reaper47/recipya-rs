use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Action;
use crate::opening_hours_specification::OpeningHoursSpecification;
use crate::field::*;
use crate::helpers::one_or_many;

///<https://schema.org/validFrom>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type LocationFeatureSpecificationValidFromFieldEnum = String;
///<https://schema.org/validThrough>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type LocationFeatureSpecificationValidThroughFieldEnum = String;
///<https://schema.org/propertyID>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LocationFeatureSpecificationPropertyIDFieldEnum = String;
///<https://schema.org/unitCode>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LocationFeatureSpecificationUnitCodeFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LocationFeatureSpecificationAdditionalTypeFieldEnum = String;

///<https://schema.org/LocationFeatureSpecification>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LocationFeatureSpecification {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/hoursAvailable>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hours_available: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/validFrom>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_from: Vec<LocationFeatureSpecificationValidFromFieldEnum>,
    ///<https://schema.org/validThrough>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_through: Vec<LocationFeatureSpecificationValidThroughFieldEnum>,
    ///<https://schema.org/measurementMethod>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub measurement_method: Vec<LocationFeatureSpecificationMeasurementMethodFieldEnum>,
    ///<https://schema.org/minValue>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub min_value: Vec<f32>,
    ///<https://schema.org/propertyID>
    #[serde(rename = "propertyID")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub property_id: Vec<LocationFeatureSpecificationPropertyIDFieldEnum>,
    ///<https://schema.org/maxValue>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub max_value: Vec<f32>,
    ///<https://schema.org/valueReference>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value_reference: Vec<LocationFeatureSpecificationValueReferenceFieldEnum>,
    ///<https://schema.org/measurementTechnique>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub measurement_technique: Vec<
        LocationFeatureSpecificationMeasurementTechniqueFieldEnum,
    >,
    ///<https://schema.org/unitText>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unit_text: Vec<String>,
    ///<https://schema.org/value>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LocationFeatureSpecificationValueFieldEnum>,
    ///<https://schema.org/unitCode>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unit_code: Vec<LocationFeatureSpecificationUnitCodeFieldEnum>,
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
    pub additional_type: Vec<LocationFeatureSpecificationAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<LocationFeatureSpecificationIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<LocationFeatureSpecificationImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<LocationFeatureSpecificationDescriptionFieldEnum>,
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
    pub subject_of: Vec<LocationFeatureSpecificationSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<LocationFeatureSpecificationMainEntityOfPageFieldEnum>,
}
