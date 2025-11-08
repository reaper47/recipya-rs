use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type HowToToolAdditionalTypeFieldEnum = String;
///<https://schema.org/HowToTool>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct HowToTool {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/requiredQuantity>
    #[serde(rename = "requiredQuantity")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub required_quantity: Vec<HowToToolRequiredQuantityFieldEnum>,
    ///<https://schema.org/item>
    #[serde(rename = "item")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item: Vec<Thing>,
    ///<https://schema.org/nextItem>
    #[serde(rename = "nextItem")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub next_item: Vec<ListItem>,
    ///<https://schema.org/previousItem>
    #[serde(rename = "previousItem")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub previous_item: Vec<ListItem>,
    ///<https://schema.org/position>
    #[serde(rename = "position")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub position: Vec<HowToToolPositionFieldEnum>,
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
    pub additional_type: Vec<HowToToolAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<HowToToolIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<HowToToolImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<HowToToolDescriptionFieldEnum>,
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
    pub subject_of: Vec<HowToToolSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<HowToToolMainEntityOfPageFieldEnum>,
}
