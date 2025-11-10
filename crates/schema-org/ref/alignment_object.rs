use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type AlignmentObjectAdditionalTypeFieldEnum = String;
///<https://schema.org/AlignmentObject>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct AlignmentObject {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/educationalFramework>
    #[serde(rename = "educationalFramework")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub educational_framework: Vec<String>,
    ///<https://schema.org/targetDescription>
    #[serde(rename = "targetDescription")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub target_description: Vec<String>,
    ///<https://schema.org/targetUrl>
    #[serde(rename = "targetUrl")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub target_url: Vec<String>,
    ///<https://schema.org/targetName>
    #[serde(rename = "targetName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub target_name: Vec<String>,
    ///<https://schema.org/alignmentType>
    #[serde(rename = "alignmentType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub alignment_type: Vec<String>,
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
    pub additional_type: Vec<AlignmentObjectAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<AlignmentObjectIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<AlignmentObjectImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<AlignmentObjectDescriptionFieldEnum>,
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
    pub subject_of: Vec<AlignmentObjectSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<AlignmentObjectMainEntityOfPageFieldEnum>,
}
