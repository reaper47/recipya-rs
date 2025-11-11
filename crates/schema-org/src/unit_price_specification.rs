use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Action;
use crate::enums::PriceComponentTypeEnumerationEnum;
use crate::helpers::one_or_many;
use crate::field::*;
use crate::member_program_tier::MemberProgramTier;
use crate::price_specification::PriceSpecification;
use crate::quantitative_value::QuantitativeValue;

///<https://schema.org/unitCode>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type UnitPriceSpecificationUnitCodeFieldEnum = String;
///<https://schema.org/validFrom>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type UnitPriceSpecificationValidFromFieldEnum = String;
///<https://schema.org/validThrough>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type UnitPriceSpecificationValidThroughFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type UnitPriceSpecificationAdditionalTypeFieldEnum = String;

///<https://schema.org/UnitPriceSpecification>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnitPriceSpecification {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/referenceQuantity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reference_quantity: Vec<QuantitativeValue>,
    ///<https://schema.org/billingDuration>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub billing_duration: Vec<UnitPriceSpecificationBillingDurationFieldEnum>,
    ///<https://schema.org/billingStart>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub billing_start: Vec<f32>,
    ///<https://schema.org/billingIncrement>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub billing_increment: Vec<f32>,
    ///<https://schema.org/priceComponentType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub price_component_type: Vec<PriceComponentTypeEnumerationEnum>,
    ///<https://schema.org/priceType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub price_type: Vec<UnitPriceSpecificationPriceTypeFieldEnum>,
    ///<https://schema.org/unitText>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unit_text: Vec<String>,
    ///<https://schema.org/unitCode>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unit_code: Vec<UnitPriceSpecificationUnitCodeFieldEnum>,
    ///<https://schema.org/validForMemberTier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_for_member_tier: Vec<MemberProgramTier>,
    ///<https://schema.org/eligibleQuantity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_quantity: Vec<QuantitativeValue>,
    ///<https://schema.org/priceCurrency>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub price_currency: Vec<String>,
    ///<https://schema.org/eligibleTransactionVolume>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_transaction_volume: Vec<PriceSpecification>,
    ///<https://schema.org/validFrom>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_from: Vec<UnitPriceSpecificationValidFromFieldEnum>,
    ///<https://schema.org/price>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub price: Vec<UnitPriceSpecificationPriceFieldEnum>,
    ///<https://schema.org/maxPrice>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub max_price: Vec<f32>,
    ///<https://schema.org/validThrough>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_through: Vec<UnitPriceSpecificationValidThroughFieldEnum>,
    ///<https://schema.org/membershipPointsEarned>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub membership_points_earned: Vec<
        UnitPriceSpecificationMembershipPointsEarnedFieldEnum,
    >,
    ///<https://schema.org/valueAddedTaxIncluded>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value_added_tax_included: Vec<String>,
    ///<https://schema.org/minPrice>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub min_price: Vec<f32>,
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
    pub additional_type: Vec<UnitPriceSpecificationAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<UnitPriceSpecificationIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<UnitPriceSpecificationImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<UnitPriceSpecificationDescriptionFieldEnum>,
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
    pub subject_of: Vec<UnitPriceSpecificationSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<UnitPriceSpecificationMainEntityOfPageFieldEnum>,
}
