use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Action;
use crate::helpers::one_or_many;
use crate::field::*;
use crate::software_application::SoftwareApplication;

///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type EntryPointAdditionalTypeFieldEnum = String;
///<https://schema.org/EntryPoint>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EntryPoint {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/contentType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub content_type: Vec<String>,
    ///<https://schema.org/actionPlatform>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub action_platform: Vec<EntryPointActionPlatformFieldEnum>,
    ///<https://schema.org/encodingType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub encoding_type: Vec<String>,
    ///<https://schema.org/urlTemplate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url_template: Vec<String>,
    ///<https://schema.org/actionApplication>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub action_application: Vec<SoftwareApplication>,
    ///<https://schema.org/httpMethod>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub http_method: Vec<String>,
    ///<https://schema.org/application>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub application: Vec<SoftwareApplication>,
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
    pub additional_type: Vec<EntryPointAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<EntryPointIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<EntryPointImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<EntryPointDescriptionFieldEnum>,
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
    pub subject_of: Vec<EntryPointSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<EntryPointMainEntityOfPageFieldEnum>,
}
