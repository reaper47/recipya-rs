use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/unitCode>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ObservationUnitCodeFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ObservationAdditionalTypeFieldEnum = String;
///<https://schema.org/Observation>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Observation {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/measurementMethod>
    #[serde(rename = "measurementMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub measurement_method: Vec<ObservationMeasurementMethodFieldEnum>,
    ///<https://schema.org/observationPeriod>
    #[serde(rename = "observationPeriod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub observation_period: Vec<String>,
    ///<https://schema.org/measurementQualifier>
    #[serde(rename = "measurementQualifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub measurement_qualifier: Vec<Enumeration>,
    ///<https://schema.org/variableMeasured>
    #[serde(rename = "variableMeasured")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub variable_measured: Vec<ObservationVariableMeasuredFieldEnum>,
    ///<https://schema.org/marginOfError>
    #[serde(rename = "marginOfError")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub margin_of_error: Vec<QuantitativeValue>,
    ///<https://schema.org/measurementTechnique>
    #[serde(rename = "measurementTechnique")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub measurement_technique: Vec<ObservationMeasurementTechniqueFieldEnum>,
    ///<https://schema.org/observationDate>
    #[serde(rename = "observationDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub observation_date: Vec<String>,
    ///<https://schema.org/observationAbout>
    #[serde(rename = "observationAbout")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub observation_about: Vec<ObservationObservationAboutFieldEnum>,
    ///<https://schema.org/measuredProperty>
    #[serde(rename = "measuredProperty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub measured_property: Vec<Property>,
    ///<https://schema.org/measurementDenominator>
    #[serde(rename = "measurementDenominator")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub measurement_denominator: Vec<StatisticalVariable>,
    ///<https://schema.org/minValue>
    #[serde(rename = "minValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub min_value: Vec<f32>,
    ///<https://schema.org/maxValue>
    #[serde(rename = "maxValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub max_value: Vec<f32>,
    ///<https://schema.org/valueReference>
    #[serde(rename = "valueReference")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value_reference: Vec<ObservationValueReferenceFieldEnum>,
    ///<https://schema.org/unitText>
    #[serde(rename = "unitText")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub unit_text: Vec<String>,
    ///<https://schema.org/value>
    #[serde(rename = "value")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub value: Vec<ObservationValueFieldEnum>,
    ///<https://schema.org/additionalProperty>
    #[serde(rename = "additionalProperty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/unitCode>
    #[serde(rename = "unitCode")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub unit_code: Vec<ObservationUnitCodeFieldEnum>,
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
    pub additional_type: Vec<ObservationAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<ObservationIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<ObservationImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<ObservationDescriptionFieldEnum>,
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
    pub subject_of: Vec<ObservationSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<ObservationMainEntityOfPageFieldEnum>,
}
