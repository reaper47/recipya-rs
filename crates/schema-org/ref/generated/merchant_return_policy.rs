use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MerchantReturnPolicyAdditionalTypeFieldEnum = String;
///<https://schema.org/MerchantReturnPolicy>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct MerchantReturnPolicy {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/itemDefectReturnLabelSource>
    #[serde(rename = "itemDefectReturnLabelSource")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item_defect_return_label_source: Vec<ReturnLabelSourceEnumerationEnum>,
    ///<https://schema.org/itemCondition>
    #[serde(rename = "itemCondition")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item_condition: Vec<OfferItemConditionEnum>,
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
    ///<https://schema.org/validForMemberTier>
    #[serde(rename = "validForMemberTier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub valid_for_member_tier: Vec<MemberProgramTier>,
    ///<https://schema.org/returnFees>
    #[serde(rename = "returnFees")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub return_fees: Vec<ReturnFeesEnumerationEnum>,
    ///<https://schema.org/merchantReturnDays>
    #[serde(rename = "merchantReturnDays")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub merchant_return_days: Vec<MerchantReturnPolicyMerchantReturnDaysFieldEnum>,
    ///<https://schema.org/returnPolicySeasonalOverride>
    #[serde(rename = "returnPolicySeasonalOverride")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub return_policy_seasonal_override: Vec<MerchantReturnPolicySeasonalOverride>,
    ///<https://schema.org/returnLabelSource>
    #[serde(rename = "returnLabelSource")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub return_label_source: Vec<ReturnLabelSourceEnumerationEnum>,
    ///<https://schema.org/returnPolicyCountry>
    #[serde(rename = "returnPolicyCountry")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub return_policy_country: Vec<MerchantReturnPolicyReturnPolicyCountryFieldEnum>,
    ///<https://schema.org/returnMethod>
    #[serde(rename = "returnMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub return_method: Vec<ReturnMethodEnumerationEnum>,
    ///<https://schema.org/merchantReturnLink>
    #[serde(rename = "merchantReturnLink")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub merchant_return_link: Vec<String>,
    ///<https://schema.org/refundType>
    #[serde(rename = "refundType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub refund_type: Vec<RefundTypeEnumerationEnum>,
    ///<https://schema.org/applicableCountry>
    #[serde(rename = "applicableCountry")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub applicable_country: Vec<MerchantReturnPolicyApplicableCountryFieldEnum>,
    ///<https://schema.org/customerRemorseReturnLabelSource>
    #[serde(rename = "customerRemorseReturnLabelSource")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub customer_remorse_return_label_source: Vec<ReturnLabelSourceEnumerationEnum>,
    ///<https://schema.org/itemDefectReturnFees>
    #[serde(rename = "itemDefectReturnFees")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item_defect_return_fees: Vec<ReturnFeesEnumerationEnum>,
    ///<https://schema.org/customerRemorseReturnShippingFeesAmount>
    #[serde(rename = "customerRemorseReturnShippingFeesAmount")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub customer_remorse_return_shipping_fees_amount: Vec<MonetaryAmount>,
    ///<https://schema.org/itemDefectReturnShippingFeesAmount>
    #[serde(rename = "itemDefectReturnShippingFeesAmount")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item_defect_return_shipping_fees_amount: Vec<MonetaryAmount>,
    ///<https://schema.org/inStoreReturnsOffered>
    #[serde(rename = "inStoreReturnsOffered")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub in_store_returns_offered: Vec<String>,
    ///<https://schema.org/additionalProperty>
    #[serde(rename = "additionalProperty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/restockingFee>
    #[serde(rename = "restockingFee")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub restocking_fee: Vec<MerchantReturnPolicyRestockingFeeFieldEnum>,
    ///<https://schema.org/customerRemorseReturnFees>
    #[serde(rename = "customerRemorseReturnFees")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub customer_remorse_return_fees: Vec<ReturnFeesEnumerationEnum>,
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
    pub additional_type: Vec<MerchantReturnPolicyAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<MerchantReturnPolicyIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<MerchantReturnPolicyImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<MerchantReturnPolicyDescriptionFieldEnum>,
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
    pub subject_of: Vec<MerchantReturnPolicySubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<MerchantReturnPolicyMainEntityOfPageFieldEnum>,
}
