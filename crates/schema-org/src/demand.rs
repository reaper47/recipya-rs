use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::field::*;
use crate::{Action, Place};
use crate::business_entity_type::BusinessEntityType;
use crate::business_function::BusinessFunction;
use crate::enums::{DeliveryMethodEnum, ItemAvailabilityEnum, OfferItemConditionEnum};
use crate::price_specification::PriceSpecification;
use crate::quantitative_value::QuantitativeValue;
use crate::type_and_quantity_node::TypeAndQuantityNode;
use crate::warranty_promise::WarrantyPromise;

///<https://schema.org/availabilityStarts>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type DemandAvailabilityStartsFieldEnum = String;
///<https://schema.org/validFrom>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type DemandValidFromFieldEnum = String;
///<https://schema.org/availabilityEnds>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type DemandAvailabilityEndsFieldEnum = String;
///<https://schema.org/gtin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type DemandGtinFieldEnum = String;
///<https://schema.org/validThrough>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type DemandValidThroughFieldEnum = String;
///<https://schema.org/asin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type DemandAsinFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type DemandAdditionalTypeFieldEnum = String;

///<https://schema.org/Demand>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Demand {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/itemCondition>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_condition: Vec<OfferItemConditionEnum>,
    ///<https://schema.org/gtin8>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin8: Vec<String>,
    ///<https://schema.org/eligibleQuantity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_quantity: Vec<QuantitativeValue>,
    ///<https://schema.org/sku>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sku: Vec<String>,
    ///<https://schema.org/eligibleCustomerType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_customer_type: Vec<BusinessEntityType>,
    ///<https://schema.org/eligibleDuration>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_duration: Vec<QuantitativeValue>,
    ///<https://schema.org/areaServed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub area_served: Vec<DemandAreaServedFieldEnum>,
    ///<https://schema.org/availableAtOrFrom>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub available_at_or_from: Vec<Place>,
    ///<https://schema.org/gtin12>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin12: Vec<String>,
    ///<https://schema.org/availabilityStarts>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub availability_starts: Vec<DemandAvailabilityStartsFieldEnum>,
    ///<https://schema.org/inventoryLevel>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inventory_level: Vec<QuantitativeValue>,
    ///<https://schema.org/serialNumber>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub serial_number: Vec<String>,
    ///<https://schema.org/itemOffered>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_offered: Vec<DemandItemOfferedFieldEnum>,
    ///<https://schema.org/acceptedPaymentMethod>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accepted_payment_method: Vec<DemandAcceptedPaymentMethodFieldEnum>,
    ///<https://schema.org/availability>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub availability: Vec<ItemAvailabilityEnum>,
    ///<https://schema.org/mpn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mpn: Vec<String>,
    ///<https://schema.org/includesObject>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub includes_object: Vec<TypeAndQuantityNode>,
    ///<https://schema.org/eligibleRegion>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_region: Vec<DemandEligibleRegionFieldEnum>,
    ///<https://schema.org/eligibleTransactionVolume>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_transaction_volume: Vec<PriceSpecification>,
    ///<https://schema.org/businessFunction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub business_function: Vec<BusinessFunction>,
    ///<https://schema.org/validFrom>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_from: Vec<DemandValidFromFieldEnum>,
    ///<https://schema.org/seller>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub seller: Vec<DemandSellerFieldEnum>,
    ///<https://schema.org/priceSpecification>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub price_specification: Vec<PriceSpecification>,
    ///<https://schema.org/ineligibleRegion>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ineligible_region: Vec<DemandIneligibleRegionFieldEnum>,
    ///<https://schema.org/availableDeliveryMethod>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub available_delivery_method: Vec<DeliveryMethodEnum>,
    ///<https://schema.org/deliveryLeadTime>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub delivery_lead_time: Vec<QuantitativeValue>,
    ///<https://schema.org/availabilityEnds>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub availability_ends: Vec<DemandAvailabilityEndsFieldEnum>,
    ///<https://schema.org/gtin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin: Vec<DemandGtinFieldEnum>,
    ///<https://schema.org/gtin13>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin13: Vec<String>,
    ///<https://schema.org/validThrough>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_through: Vec<DemandValidThroughFieldEnum>,
    ///<https://schema.org/gtin14>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin14: Vec<String>,
    ///<https://schema.org/asin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub asin: Vec<DemandAsinFieldEnum>,
    ///<https://schema.org/advanceBookingRequirement>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub advance_booking_requirement: Vec<QuantitativeValue>,
    ///<https://schema.org/warranty>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warranty: Vec<WarrantyPromise>,
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
    pub additional_type: Vec<DemandAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<DemandIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<DemandImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<DemandDescriptionFieldEnum>,
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
    pub subject_of: Vec<DemandSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<DemandMainEntityOfPageFieldEnum>,
}
