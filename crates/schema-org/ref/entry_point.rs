use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type EntryPointAdditionalTypeFieldEnum = String;
///<https://schema.org/EntryPoint>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct EntryPoint {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/contentType>
    #[serde(rename = "contentType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub content_type: Vec<String>,
    ///<https://schema.org/actionPlatform>
    #[serde(rename = "actionPlatform")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub action_platform: Vec<EntryPointActionPlatformFieldEnum>,
    ///<https://schema.org/encodingType>
    #[serde(rename = "encodingType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub encoding_type: Vec<String>,
    ///<https://schema.org/urlTemplate>
    #[serde(rename = "urlTemplate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub url_template: Vec<String>,
    ///<https://schema.org/actionApplication>
    #[serde(rename = "actionApplication")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub action_application: Vec<SoftwareApplication>,
    ///<https://schema.org/httpMethod>
    #[serde(rename = "httpMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub http_method: Vec<String>,
    ///<https://schema.org/application>
    #[serde(rename = "application")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub application: Vec<SoftwareApplication>,
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
    pub additional_type: Vec<EntryPointAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<EntryPointIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<EntryPointImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<EntryPointDescriptionFieldEnum>,
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
    pub subject_of: Vec<EntryPointSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<EntryPointMainEntityOfPageFieldEnum>,
}
