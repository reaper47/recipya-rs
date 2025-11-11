use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::business_entity_type::BusinessEntityType;
use crate::enums::{AdultOrientedEnumerationEnum, DeliveryMethodEnum, ItemAvailabilityEnum, OfferItemConditionEnum};
use crate::helpers::one_or_many;
use crate::field::*;
use crate::member_program_tier::MemberProgramTier;
use crate::quantitative_value::QuantitativeValue;
use crate::{Action, Place, PropertyValue, Review};
use crate::aggregate_rating::AggregateRating;
use crate::business_function::BusinessFunction;
use crate::merchant_return_policy::MerchantReturnPolicy;
use crate::offer::Offer;
use crate::offer_shipping_details::OfferShippingDetails;
use crate::price_specification::PriceSpecification;
use crate::type_and_quantity_node::TypeAndQuantityNode;
use crate::warranty_promise::WarrantyPromise;

///<https://schema.org/availabilityStarts>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type AggregateOfferAvailabilityStartsFieldEnum = String;
///<https://schema.org/validFrom>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type AggregateOfferValidFromFieldEnum = String;
///<https://schema.org/availabilityEnds>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type AggregateOfferAvailabilityEndsFieldEnum = String;
///<https://schema.org/gtin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type AggregateOfferGtinFieldEnum = String;
///<https://schema.org/validThrough>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type AggregateOfferValidThroughFieldEnum = String;
///<https://schema.org/asin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type AggregateOfferAsinFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type AggregateOfferAdditionalTypeFieldEnum = String;

///<https://schema.org/AggregateOffer>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AggregateOffer {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/offerCount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offer_count: Vec<i32>,
    ///<https://schema.org/offers>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offers: Vec<AggregateOfferOffersFieldEnum>,
    ///<https://schema.org/lowPrice>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub low_price: Vec<AggregateOfferLowPriceFieldEnum>,
    ///<https://schema.org/highPrice>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub high_price: Vec<AggregateOfferHighPriceFieldEnum>,
    ///<https://schema.org/itemCondition>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_condition: Vec<OfferItemConditionEnum>,
    ///<https://schema.org/gtin8>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin8: Vec<String>,
    ///<https://schema.org/validForMemberTier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_for_member_tier: Vec<MemberProgramTier>,
    ///<https://schema.org/eligibleQuantity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_quantity: Vec<QuantitativeValue>,
    ///<https://schema.org/review>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub review: Vec<Review>,
    ///<https://schema.org/hasMeasurement>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_measurement: Vec<QuantitativeValue>,
    ///<https://schema.org/checkoutPageURLTemplate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub checkout_page_url_template: Vec<String>,
    ///<https://schema.org/sku>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sku: Vec<String>,
    ///<https://schema.org/priceCurrency>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub price_currency: Vec<String>,
    ///<https://schema.org/eligibleCustomerType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_customer_type: Vec<BusinessEntityType>,
    ///<https://schema.org/hasAdultConsideration>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_adult_consideration: Vec<AdultOrientedEnumerationEnum>,
    ///<https://schema.org/eligibleDuration>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_duration: Vec<QuantitativeValue>,
    ///<https://schema.org/areaServed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub area_served: Vec<AggregateOfferAreaServedFieldEnum>,
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
    pub availability_starts: Vec<AggregateOfferAvailabilityStartsFieldEnum>,
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
    pub item_offered: Vec<AggregateOfferItemOfferedFieldEnum>,
    ///<https://schema.org/acceptedPaymentMethod>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accepted_payment_method: Vec<AggregateOfferAcceptedPaymentMethodFieldEnum>,
    ///<https://schema.org/availability>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub availability: Vec<ItemAvailabilityEnum>,
    ///<https://schema.org/leaseLength>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lease_length: Vec<AggregateOfferLeaseLengthFieldEnum>,
    ///<https://schema.org/mpn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mpn: Vec<String>,
    ///<https://schema.org/priceValidUntil>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub price_valid_until: Vec<String>,
    ///<https://schema.org/mobileUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mobile_url: Vec<String>,
    ///<https://schema.org/aggregateRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/hasGS1DigitalLink>
    #[serde(rename = "hasGS1DigitalLink")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_gs1_digital_link: Vec<String>,
    ///<https://schema.org/includesObject>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub includes_object: Vec<TypeAndQuantityNode>,
    ///<https://schema.org/eligibleRegion>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_region: Vec<AggregateOfferEligibleRegionFieldEnum>,
    ///<https://schema.org/eligibleTransactionVolume>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligible_transaction_volume: Vec<PriceSpecification>,
    ///<https://schema.org/isFamilyFriendly>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_family_friendly: Vec<String>,
    ///<https://schema.org/businessFunction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub business_function: Vec<BusinessFunction>,
    ///<https://schema.org/validFrom>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_from: Vec<AggregateOfferValidFromFieldEnum>,
    ///<https://schema.org/seller>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub seller: Vec<AggregateOfferSellerFieldEnum>,
    ///<https://schema.org/priceSpecification>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub price_specification: Vec<PriceSpecification>,
    ///<https://schema.org/shippingDetails>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shipping_details: Vec<OfferShippingDetails>,
    ///<https://schema.org/ineligibleRegion>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ineligible_region: Vec<AggregateOfferIneligibleRegionFieldEnum>,
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
    pub availability_ends: Vec<AggregateOfferAvailabilityEndsFieldEnum>,
    ///<https://schema.org/gtin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin: Vec<AggregateOfferGtinFieldEnum>,
    ///<https://schema.org/price>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub price: Vec<AggregateOfferPriceFieldEnum>,
    ///<https://schema.org/gtin13>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin13: Vec<String>,
    ///<https://schema.org/addOn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub add_on: Vec<Offer>,
    ///<https://schema.org/validThrough>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_through: Vec<AggregateOfferValidThroughFieldEnum>,
    ///<https://schema.org/category>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<AggregateOfferCategoryFieldEnum>,
    ///<https://schema.org/gtin14>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin14: Vec<String>,
    ///<https://schema.org/offeredBy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offered_by: Vec<AggregateOfferOfferedByFieldEnum>,
    ///<https://schema.org/asin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub asin: Vec<AggregateOfferAsinFieldEnum>,
    ///<https://schema.org/hasMerchantReturnPolicy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_merchant_return_policy: Vec<MerchantReturnPolicy>,
    ///<https://schema.org/additionalProperty>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/advanceBookingRequirement>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub advance_booking_requirement: Vec<QuantitativeValue>,
    ///<https://schema.org/reviews>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reviews: Vec<Review>,
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
    pub additional_type: Vec<AggregateOfferAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<AggregateOfferIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<AggregateOfferImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<AggregateOfferDescriptionFieldEnum>,
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
    pub subject_of: Vec<AggregateOfferSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<AggregateOfferMainEntityOfPageFieldEnum>,
}
