use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/endDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type MerchantReturnPolicySeasonalOverrideEndDateFieldEnum = String;
///<https://schema.org/startDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type MerchantReturnPolicySeasonalOverrideStartDateFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MerchantReturnPolicySeasonalOverrideAdditionalTypeFieldEnum = String;
///<https://schema.org/MerchantReturnPolicySeasonalOverride>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct MerchantReturnPolicySeasonalOverride {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/endDate>
    #[serde(rename = "endDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub end_date: Vec<MerchantReturnPolicySeasonalOverrideEndDateFieldEnum>,
    ///<https://schema.org/returnShippingFeesAmount>
    #[serde(rename = "returnShippingFeesAmount")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub return_shipping_fees_amount: Vec<MonetaryAmount>,
    ///<https://schema.org/returnPolicyCategory>
    #[serde(rename = "returnPolicyCategory")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub return_policy_category: Vec<MerchantReturnEnumerationEnum>,
    ///<https://schema.org/returnFees>
    #[serde(rename = "returnFees")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub return_fees: Vec<ReturnFeesEnumerationEnum>,
    ///<https://schema.org/merchantReturnDays>
    #[serde(rename = "merchantReturnDays")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub merchant_return_days: Vec<
        MerchantReturnPolicySeasonalOverrideMerchantReturnDaysFieldEnum,
    >,
    ///<https://schema.org/startDate>
    #[serde(rename = "startDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub start_date: Vec<MerchantReturnPolicySeasonalOverrideStartDateFieldEnum>,
    ///<https://schema.org/returnMethod>
    #[serde(rename = "returnMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub return_method: Vec<ReturnMethodEnumerationEnum>,
    ///<https://schema.org/refundType>
    #[serde(rename = "refundType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub refund_type: Vec<RefundTypeEnumerationEnum>,
    ///<https://schema.org/restockingFee>
    #[serde(rename = "restockingFee")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub restocking_fee: Vec<MerchantReturnPolicySeasonalOverrideRestockingFeeFieldEnum>,
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
    pub additional_type: Vec<
        MerchantReturnPolicySeasonalOverrideAdditionalTypeFieldEnum,
    >,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<MerchantReturnPolicySeasonalOverrideIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<MerchantReturnPolicySeasonalOverrideImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<MerchantReturnPolicySeasonalOverrideDescriptionFieldEnum>,
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
    pub subject_of: Vec<MerchantReturnPolicySeasonalOverrideSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<
        MerchantReturnPolicySeasonalOverrideMainEntityOfPageFieldEnum,
    >,
}
