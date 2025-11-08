use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/expectedArrivalFrom>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type ParcelDeliveryExpectedArrivalFromFieldEnum = String;
///<https://schema.org/expectedArrivalUntil>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type ParcelDeliveryExpectedArrivalUntilFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ParcelDeliveryAdditionalTypeFieldEnum = String;
///<https://schema.org/ParcelDelivery>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ParcelDelivery {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/partOfOrder>
    #[serde(rename = "partOfOrder")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub part_of_order: Vec<Order>,
    ///<https://schema.org/deliveryAddress>
    #[serde(rename = "deliveryAddress")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub delivery_address: Vec<PostalAddress>,
    ///<https://schema.org/itemShipped>
    #[serde(rename = "itemShipped")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item_shipped: Vec<Product>,
    ///<https://schema.org/expectedArrivalFrom>
    #[serde(rename = "expectedArrivalFrom")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub expected_arrival_from: Vec<ParcelDeliveryExpectedArrivalFromFieldEnum>,
    ///<https://schema.org/expectedArrivalUntil>
    #[serde(rename = "expectedArrivalUntil")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub expected_arrival_until: Vec<ParcelDeliveryExpectedArrivalUntilFieldEnum>,
    ///<https://schema.org/provider>
    #[serde(rename = "provider")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub provider: Vec<ParcelDeliveryProviderFieldEnum>,
    ///<https://schema.org/trackingUrl>
    #[serde(rename = "trackingUrl")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub tracking_url: Vec<String>,
    ///<https://schema.org/carrier>
    #[serde(rename = "carrier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub carrier: Vec<Organization>,
    ///<https://schema.org/hasDeliveryMethod>
    #[serde(rename = "hasDeliveryMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_delivery_method: Vec<DeliveryMethodEnum>,
    ///<https://schema.org/trackingNumber>
    #[serde(rename = "trackingNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub tracking_number: Vec<String>,
    ///<https://schema.org/originAddress>
    #[serde(rename = "originAddress")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub origin_address: Vec<PostalAddress>,
    ///<https://schema.org/deliveryStatus>
    #[serde(rename = "deliveryStatus")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub delivery_status: Vec<DeliveryEvent>,
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
    pub additional_type: Vec<ParcelDeliveryAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<ParcelDeliveryIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<ParcelDeliveryImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<ParcelDeliveryDescriptionFieldEnum>,
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
    pub subject_of: Vec<ParcelDeliverySubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<ParcelDeliveryMainEntityOfPageFieldEnum>,
}
