use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::Action;
use crate::class::Class;
use crate::helpers::one_or_many;
use crate::field::*;

///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type PropertyAdditionalTypeFieldEnum = String;

///<https://schema.org/Property>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Property {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/rangeIncludes>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub range_includes: Vec<Class>,
    ///<https://schema.org/domainIncludes>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub domain_includes: Vec<Class>,
    ///<https://schema.org/inverseOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inverse_of: Vec<Property>,
    ///<https://schema.org/supersededBy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub superseded_by: Vec<PropertySupersededByFieldEnum>,
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
    pub additional_type: Vec<PropertyAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<PropertyIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<PropertyImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<PropertyDescriptionFieldEnum>,
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
    pub subject_of: Vec<PropertySubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<PropertyMainEntityOfPageFieldEnum>,
}
