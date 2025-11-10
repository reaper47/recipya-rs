use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::field::*;
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
    #[serde(rename = "itemDefectReturnLabelSource")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_defect_return_label_source: Vec<ReturnLabelSourceEnumerationEnum>,
    ///<https://schema.org/itemCondition>
    #[serde(rename = "itemCondition")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_condition: Vec<OfferItemConditionEnum>,
    ///<https://schema.org/returnShippingFeesAmount>
    #[serde(rename = "returnShippingFeesAmount")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_shipping_fees_amount: Vec<MonetaryAmount>,
    ///<https://schema.org/returnPolicyCategory>
    #[serde(rename = "returnPolicyCategory")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_policy_category: Vec<MerchantReturnEnumerationEnum>,
    ///<https://schema.org/validForMemberTier>
    #[serde(rename = "validForMemberTier")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_for_member_tier: Vec<MemberProgramTier>,
    ///<https://schema.org/returnFees>
    #[serde(rename = "returnFees")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_fees: Vec<ReturnFeesEnumerationEnum>,
    ///<https://schema.org/merchantReturnDays>
    #[serde(rename = "merchantReturnDays")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub merchant_return_days: Vec<MerchantReturnPolicyMerchantReturnDaysFieldEnum>,
    ///<https://schema.org/returnPolicySeasonalOverride>
    #[serde(rename = "returnPolicySeasonalOverride")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_policy_seasonal_override: Vec<MerchantReturnPolicySeasonalOverride>,
    ///<https://schema.org/returnLabelSource>
    #[serde(rename = "returnLabelSource")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_label_source: Vec<ReturnLabelSourceEnumerationEnum>,
    ///<https://schema.org/returnPolicyCountry>
    #[serde(rename = "returnPolicyCountry")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_policy_country: Vec<MerchantReturnPolicyReturnPolicyCountryFieldEnum>,
    ///<https://schema.org/returnMethod>
    #[serde(rename = "returnMethod")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub return_method: Vec<ReturnMethodEnumerationEnum>,
    ///<https://schema.org/merchantReturnLink>
    #[serde(rename = "merchantReturnLink")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub merchant_return_link: Vec<String>,
    ///<https://schema.org/refundType>
    #[serde(rename = "refundType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub refund_type: Vec<RefundTypeEnumerationEnum>,
    ///<https://schema.org/applicableCountry>
    #[serde(rename = "applicableCountry")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub applicable_country: Vec<MerchantReturnPolicyApplicableCountryFieldEnum>,
    ///<https://schema.org/customerRemorseReturnLabelSource>
    #[serde(rename = "customerRemorseReturnLabelSource")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub customer_remorse_return_label_source: Vec<ReturnLabelSourceEnumerationEnum>,
    ///<https://schema.org/itemDefectReturnFees>
    #[serde(rename = "itemDefectReturnFees")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_defect_return_fees: Vec<ReturnFeesEnumerationEnum>,
    ///<https://schema.org/customerRemorseReturnShippingFeesAmount>
    #[serde(rename = "customerRemorseReturnShippingFeesAmount")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub customer_remorse_return_shipping_fees_amount: Vec<MonetaryAmount>,
    ///<https://schema.org/itemDefectReturnShippingFeesAmount>
    #[serde(rename = "itemDefectReturnShippingFeesAmount")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_defect_return_shipping_fees_amount: Vec<MonetaryAmount>,
    ///<https://schema.org/inStoreReturnsOffered>
    #[serde(rename = "inStoreReturnsOffered")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub in_store_returns_offered: Vec<String>,
    ///<https://schema.org/additionalProperty>
    #[serde(rename = "additionalProperty")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/restockingFee>
    #[serde(rename = "restockingFee")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub restocking_fee: Vec<MerchantReturnPolicyRestockingFeeFieldEnum>,
    ///<https://schema.org/customerRemorseReturnFees>
    #[serde(rename = "customerRemorseReturnFees")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub customer_remorse_return_fees: Vec<ReturnFeesEnumerationEnum>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(rename = "potentialAction")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(rename = "additionalType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<MerchantReturnPolicyAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<MerchantReturnPolicyIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<MerchantReturnPolicyImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<MerchantReturnPolicyDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<MerchantReturnPolicySubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<MerchantReturnPolicyMainEntityOfPageFieldEnum>,
}
