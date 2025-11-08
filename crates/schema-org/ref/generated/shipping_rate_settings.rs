use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ShippingRateSettingsAdditionalTypeFieldEnum = String;
///<https://schema.org/ShippingRateSettings>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ShippingRateSettings {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/weightPercentage>
    #[serde(rename = "weightPercentage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub weight_percentage: Vec<f32>,
    ///<https://schema.org/doesNotShip>
    #[serde(rename = "doesNotShip")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub does_not_ship: Vec<String>,
    ///<https://schema.org/shippingRate>
    #[serde(rename = "shippingRate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub shipping_rate: Vec<ShippingRateSettingsShippingRateFieldEnum>,
    ///<https://schema.org/isUnlabelledFallback>
    #[serde(rename = "isUnlabelledFallback")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_unlabelled_fallback: Vec<String>,
    ///<https://schema.org/shippingDestination>
    #[serde(rename = "shippingDestination")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub shipping_destination: Vec<DefinedRegion>,
    ///<https://schema.org/orderPercentage>
    #[serde(rename = "orderPercentage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub order_percentage: Vec<f32>,
    ///<https://schema.org/freeShippingThreshold>
    #[serde(rename = "freeShippingThreshold")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub free_shipping_threshold: Vec<ShippingRateSettingsFreeShippingThresholdFieldEnum>,
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
    pub additional_type: Vec<ShippingRateSettingsAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<ShippingRateSettingsIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<ShippingRateSettingsImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<ShippingRateSettingsDescriptionFieldEnum>,
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
    pub subject_of: Vec<ShippingRateSettingsSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<ShippingRateSettingsMainEntityOfPageFieldEnum>,
}
