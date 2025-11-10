use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ShippingDeliveryTimeAdditionalTypeFieldEnum = String;
///<https://schema.org/ShippingDeliveryTime>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ShippingDeliveryTime {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/transitTime>
    #[serde(rename = "transitTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub transit_time: Vec<ShippingDeliveryTimeTransitTimeFieldEnum>,
    ///<https://schema.org/cutoffTime>
    #[serde(rename = "cutoffTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub cutoff_time: Vec<String>,
    ///<https://schema.org/handlingTime>
    #[serde(rename = "handlingTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub handling_time: Vec<ShippingDeliveryTimeHandlingTimeFieldEnum>,
    ///<https://schema.org/businessDays>
    #[serde(rename = "businessDays")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub business_days: Vec<ShippingDeliveryTimeBusinessDaysFieldEnum>,
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
    pub additional_type: Vec<ShippingDeliveryTimeAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<ShippingDeliveryTimeIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<ShippingDeliveryTimeImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<ShippingDeliveryTimeDescriptionFieldEnum>,
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
    pub subject_of: Vec<ShippingDeliveryTimeSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<ShippingDeliveryTimeMainEntityOfPageFieldEnum>,
}
