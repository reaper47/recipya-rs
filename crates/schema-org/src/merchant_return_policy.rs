use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::enums::*;
use crate::helpers::one_or_many;
use crate::field::*;
use crate::member_program_tier::MemberProgramTier;
use crate::{Action, PropertyValue};
use crate::merchant_return_policy_seasonal_override::MerchantReturnPolicySeasonalOverride;
use crate::monetary_amount::MonetaryAmount;

///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MerchantReturnPolicyAdditionalTypeFieldEnum = String;

///<https://schema.org/MerchantReturnPolicy>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MerchantReturnPolicy {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/itemDefectReturnLabelSource>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_defect_return_label_source: Vec<ReturnLabelSourceEnumerationEnum>,
    ///<https://schema.org/itemCondition>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_condition: Vec<OfferItemConditionEnum>,
    ///<https://schema.org/returnShippingFeesAmount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_shipping_fees_amount: Vec<MonetaryAmount>,
    ///<https://schema.org/returnPolicyCategory>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_policy_category: Vec<MerchantReturnEnumerationEnum>,
    ///<https://schema.org/validForMemberTier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_for_member_tier: Vec<MemberProgramTier>,
    ///<https://schema.org/returnFees>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_fees: Vec<ReturnFeesEnumerationEnum>,
    ///<https://schema.org/merchantReturnDays>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub merchant_return_days: Vec<MerchantReturnPolicyMerchantReturnDaysFieldEnum>,
    ///<https://schema.org/returnPolicySeasonalOverride>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_policy_seasonal_override: Vec<MerchantReturnPolicySeasonalOverride>,
    ///<https://schema.org/returnLabelSource>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_label_source: Vec<ReturnLabelSourceEnumerationEnum>,
    ///<https://schema.org/returnPolicyCountry>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_policy_country: Vec<MerchantReturnPolicyReturnPolicyCountryFieldEnum>,
    ///<https://schema.org/returnMethod>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_method: Vec<ReturnMethodEnumerationEnum>,
    ///<https://schema.org/merchantReturnLink>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub merchant_return_link: Vec<String>,
    ///<https://schema.org/refundType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub refund_type: Vec<RefundTypeEnumerationEnum>,
    ///<https://schema.org/applicableCountry>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub applicable_country: Vec<MerchantReturnPolicyApplicableCountryFieldEnum>,
    ///<https://schema.org/customerRemorseReturnLabelSource>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub customer_remorse_return_label_source: Vec<ReturnLabelSourceEnumerationEnum>,
    ///<https://schema.org/itemDefectReturnFees>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_defect_return_fees: Vec<ReturnFeesEnumerationEnum>,
    ///<https://schema.org/customerRemorseReturnShippingFeesAmount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub customer_remorse_return_shipping_fees_amount: Vec<MonetaryAmount>,
    ///<https://schema.org/itemDefectReturnShippingFeesAmount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_defect_return_shipping_fees_amount: Vec<MonetaryAmount>,
    ///<https://schema.org/inStoreReturnsOffered>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub in_store_returns_offered: Vec<String>,
    ///<https://schema.org/additionalProperty>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/restockingFee>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub restocking_fee: Vec<MerchantReturnPolicyRestockingFeeFieldEnum>,
    ///<https://schema.org/customerRemorseReturnFees>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub customer_remorse_return_fees: Vec<ReturnFeesEnumerationEnum>,
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
    pub additional_type: Vec<MerchantReturnPolicyAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<MerchantReturnPolicyIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<MerchantReturnPolicyImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<MerchantReturnPolicyDescriptionFieldEnum>,
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
    pub subject_of: Vec<MerchantReturnPolicySubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<MerchantReturnPolicyMainEntityOfPageFieldEnum>,
}
