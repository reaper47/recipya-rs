use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type QuantitativeValueDistributionAdditionalTypeFieldEnum = String;
///<https://schema.org/QuantitativeValueDistribution>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct QuantitativeValueDistribution {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/percentile90>
    #[serde(rename = "percentile90")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub percentile90: Vec<f32>,
    ///<https://schema.org/percentile75>
    #[serde(rename = "percentile75")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub percentile75: Vec<f32>,
    ///<https://schema.org/median>
    #[serde(rename = "median")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub median: Vec<f32>,
    ///<https://schema.org/percentile25>
    #[serde(rename = "percentile25")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub percentile25: Vec<f32>,
    ///<https://schema.org/duration>
    #[serde(rename = "duration")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub duration: Vec<QuantitativeValueDistributionDurationFieldEnum>,
    ///<https://schema.org/percentile10>
    #[serde(rename = "percentile10")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub percentile10: Vec<f32>,
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
    pub additional_type: Vec<QuantitativeValueDistributionAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<QuantitativeValueDistributionIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<QuantitativeValueDistributionImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<QuantitativeValueDistributionDescriptionFieldEnum>,
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
    pub subject_of: Vec<QuantitativeValueDistributionSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<QuantitativeValueDistributionMainEntityOfPageFieldEnum>,
}
