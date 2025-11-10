use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type OrderItemAdditionalTypeFieldEnum = String;
///<https://schema.org/OrderItem>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct OrderItem {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/orderedItem>
    #[serde(rename = "orderedItem")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub ordered_item: Vec<OrderItemOrderedItemFieldEnum>,
    ///<https://schema.org/orderDelivery>
    #[serde(rename = "orderDelivery")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub order_delivery: Vec<ParcelDelivery>,
    ///<https://schema.org/orderItemNumber>
    #[serde(rename = "orderItemNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub order_item_number: Vec<String>,
    ///<https://schema.org/orderItemStatus>
    #[serde(rename = "orderItemStatus")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub order_item_status: Vec<OrderStatusEnum>,
    ///<https://schema.org/orderQuantity>
    #[serde(rename = "orderQuantity")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub order_quantity: Vec<OrderItemOrderQuantityFieldEnum>,
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
    pub additional_type: Vec<OrderItemAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<OrderItemIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<OrderItemImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<OrderItemDescriptionFieldEnum>,
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
    pub subject_of: Vec<OrderItemSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<OrderItemMainEntityOfPageFieldEnum>,
}
