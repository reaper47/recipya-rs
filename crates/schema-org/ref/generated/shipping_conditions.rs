use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ShippingConditionsAdditionalTypeFieldEnum = String;
///<https://schema.org/ShippingConditions>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ShippingConditions {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/transitTime>
    #[serde(rename = "transitTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub transit_time: Vec<ShippingConditionsTransitTimeFieldEnum>,
    ///<https://schema.org/depth>
    #[serde(rename = "depth")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub depth: Vec<ShippingConditionsDepthFieldEnum>,
    ///<https://schema.org/orderValue>
    #[serde(rename = "orderValue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub order_value: Vec<MonetaryAmount>,
    ///<https://schema.org/doesNotShip>
    #[serde(rename = "doesNotShip")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub does_not_ship: Vec<String>,
    ///<https://schema.org/width>
    #[serde(rename = "width")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub width: Vec<ShippingConditionsWidthFieldEnum>,
    ///<https://schema.org/shippingRate>
    #[serde(rename = "shippingRate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub shipping_rate: Vec<ShippingConditionsShippingRateFieldEnum>,
    ///<https://schema.org/height>
    #[serde(rename = "height")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub height: Vec<ShippingConditionsHeightFieldEnum>,
    ///<https://schema.org/weight>
    #[serde(rename = "weight")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub weight: Vec<ShippingConditionsWeightFieldEnum>,
    ///<https://schema.org/shippingOrigin>
    #[serde(rename = "shippingOrigin")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub shipping_origin: Vec<DefinedRegion>,
    ///<https://schema.org/numItems>
    #[serde(rename = "numItems")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub num_items: Vec<QuantitativeValue>,
    ///<https://schema.org/seasonalOverride>
    #[serde(rename = "seasonalOverride")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub seasonal_override: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/shippingDestination>
    #[serde(rename = "shippingDestination")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub shipping_destination: Vec<DefinedRegion>,
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
    pub additional_type: Vec<ShippingConditionsAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<ShippingConditionsIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<ShippingConditionsImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<ShippingConditionsDescriptionFieldEnum>,
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
    pub subject_of: Vec<ShippingConditionsSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<ShippingConditionsMainEntityOfPageFieldEnum>,
}
