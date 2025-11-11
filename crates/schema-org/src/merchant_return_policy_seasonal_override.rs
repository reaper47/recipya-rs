use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Action;
use crate::enums::*;
use crate::helpers::one_or_many;
use crate::field::*;
use crate::monetary_amount::MonetaryAmount;

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
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MerchantReturnPolicySeasonalOverride {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/endDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub end_date: Vec<MerchantReturnPolicySeasonalOverrideEndDateFieldEnum>,
    ///<https://schema.org/returnShippingFeesAmount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_shipping_fees_amount: Vec<MonetaryAmount>,
    ///<https://schema.org/returnPolicyCategory>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_policy_category: Vec<MerchantReturnEnumerationEnum>,
    ///<https://schema.org/returnFees>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_fees: Vec<ReturnFeesEnumerationEnum>,
    ///<https://schema.org/merchantReturnDays>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub merchant_return_days: Vec<
        MerchantReturnPolicySeasonalOverrideMerchantReturnDaysFieldEnum,
    >,
    ///<https://schema.org/startDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub start_date: Vec<MerchantReturnPolicySeasonalOverrideStartDateFieldEnum>,
    ///<https://schema.org/returnMethod>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_method: Vec<ReturnMethodEnumerationEnum>,
    ///<https://schema.org/refundType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub refund_type: Vec<RefundTypeEnumerationEnum>,
    ///<https://schema.org/restockingFee>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub restocking_fee: Vec<MerchantReturnPolicySeasonalOverrideRestockingFeeFieldEnum>,
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
    pub additional_type: Vec<
        MerchantReturnPolicySeasonalOverrideAdditionalTypeFieldEnum,
    >,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<MerchantReturnPolicySeasonalOverrideIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<MerchantReturnPolicySeasonalOverrideImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<MerchantReturnPolicySeasonalOverrideDescriptionFieldEnum>,
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
    pub subject_of: Vec<MerchantReturnPolicySeasonalOverrideSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<
        MerchantReturnPolicySeasonalOverrideMainEntityOfPageFieldEnum,
    >,
}
