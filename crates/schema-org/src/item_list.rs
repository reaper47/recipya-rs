use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::Thing;
use crate::field::{
    ItemListDescriptionFieldEnum, ItemListImageFieldEnum, ItemListItemListElementFieldEnum,
    ItemListItemListOrderFieldEnum, ItemListSubjectOfFieldEnum,
};
use crate::helpers::{is_smallvec_empty, one_or_many};

///<https://schema.org/ItemList>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ItemList {
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    ///<https://schema.org/aggregateElement>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub aggregate_element: SmallVec<[Thing; 1]>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub alternate_name: SmallVec<[String; 1]>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub description: SmallVec<[ItemListDescriptionFieldEnum; 1]>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub disambiguating_description: SmallVec<[String; 1]>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub image: SmallVec<[ItemListImageFieldEnum; 4]>,
    ///<https://schema.org/itemListElement>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub item_list_element: SmallVec<[ItemListItemListElementFieldEnum; 6]>,
    ///<https://schema.org/itemListOrder>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub item_list_order: SmallVec<[ItemListItemListOrderFieldEnum; 1]>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub name: SmallVec<[String; 1]>,
    ///<https://schema.org/numberOfItems>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub number_of_items: SmallVec<[i32; 1]>,
    ///<https://schema.org/subjectOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub subject_of: SmallVec<[ItemListSubjectOfFieldEnum; 1]>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub url: SmallVec<[String; 1]>,
}
