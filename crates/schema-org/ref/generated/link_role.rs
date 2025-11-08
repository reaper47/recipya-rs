use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/endDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type LinkRoleEndDateFieldEnum = String;
///<https://schema.org/namedPosition>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LinkRoleNamedPositionFieldEnum = String;
///<https://schema.org/startDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type LinkRoleStartDateFieldEnum = String;
///<https://schema.org/roleName>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LinkRoleRoleNameFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LinkRoleAdditionalTypeFieldEnum = String;
///<https://schema.org/LinkRole>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct LinkRole {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/linkRelationship>
    #[serde(rename = "linkRelationship")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub link_relationship: Vec<String>,
    ///<https://schema.org/inLanguage>
    #[serde(rename = "inLanguage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub in_language: Vec<LinkRoleInLanguageFieldEnum>,
    ///<https://schema.org/endDate>
    #[serde(rename = "endDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub end_date: Vec<LinkRoleEndDateFieldEnum>,
    ///<https://schema.org/namedPosition>
    #[serde(rename = "namedPosition")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub named_position: Vec<LinkRoleNamedPositionFieldEnum>,
    ///<https://schema.org/startDate>
    #[serde(rename = "startDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub start_date: Vec<LinkRoleStartDateFieldEnum>,
    ///<https://schema.org/roleName>
    #[serde(rename = "roleName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub role_name: Vec<LinkRoleRoleNameFieldEnum>,
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
    pub additional_type: Vec<LinkRoleAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<LinkRoleIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<LinkRoleImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<LinkRoleDescriptionFieldEnum>,
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
    pub subject_of: Vec<LinkRoleSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<LinkRoleMainEntityOfPageFieldEnum>,
}
