use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type OfferCatalogAdditionalTypeFieldEnum = String;
///<https://schema.org/OfferCatalog>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct OfferCatalog {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/numberOfItems>
    #[serde(rename = "numberOfItems")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_items: Vec<i32>,
    ///<https://schema.org/itemListElement>
    #[serde(rename = "itemListElement")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item_list_element: Vec<OfferCatalogItemListElementFieldEnum>,
    ///<https://schema.org/aggregateElement>
    #[serde(rename = "aggregateElement")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub aggregate_element: Vec<Thing>,
    ///<https://schema.org/itemListOrder>
    #[serde(rename = "itemListOrder")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item_list_order: Vec<OfferCatalogItemListOrderFieldEnum>,
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
    pub additional_type: Vec<OfferCatalogAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<OfferCatalogIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<OfferCatalogImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<OfferCatalogDescriptionFieldEnum>,
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
    pub subject_of: Vec<OfferCatalogSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<OfferCatalogMainEntityOfPageFieldEnum>,
}
