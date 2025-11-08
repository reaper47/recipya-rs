use crate::*;
use serde_with::{serde_as, OneOrMany};
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
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Demand {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/itemCondition>
    #[serde(rename = "itemCondition")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item_condition: Vec<OfferItemConditionEnum>,
    ///<https://schema.org/gtin8>
    #[serde(rename = "gtin8")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin8: Vec<String>,
    ///<https://schema.org/eligibleQuantity>
    #[serde(rename = "eligibleQuantity")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub eligible_quantity: Vec<QuantitativeValue>,
    ///<https://schema.org/sku>
    #[serde(rename = "sku")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub sku: Vec<String>,
    ///<https://schema.org/eligibleCustomerType>
    #[serde(rename = "eligibleCustomerType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub eligible_customer_type: Vec<BusinessEntityType>,
    ///<https://schema.org/eligibleDuration>
    #[serde(rename = "eligibleDuration")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub eligible_duration: Vec<QuantitativeValue>,
    ///<https://schema.org/areaServed>
    #[serde(rename = "areaServed")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub area_served: Vec<DemandAreaServedFieldEnum>,
    ///<https://schema.org/availableAtOrFrom>
    #[serde(rename = "availableAtOrFrom")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub available_at_or_from: Vec<Place>,
    ///<https://schema.org/gtin12>
    #[serde(rename = "gtin12")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin12: Vec<String>,
    ///<https://schema.org/availabilityStarts>
    #[serde(rename = "availabilityStarts")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub availability_starts: Vec<DemandAvailabilityStartsFieldEnum>,
    ///<https://schema.org/inventoryLevel>
    #[serde(rename = "inventoryLevel")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub inventory_level: Vec<QuantitativeValue>,
    ///<https://schema.org/serialNumber>
    #[serde(rename = "serialNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub serial_number: Vec<String>,
    ///<https://schema.org/itemOffered>
    #[serde(rename = "itemOffered")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item_offered: Vec<DemandItemOfferedFieldEnum>,
    ///<https://schema.org/acceptedPaymentMethod>
    #[serde(rename = "acceptedPaymentMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub accepted_payment_method: Vec<DemandAcceptedPaymentMethodFieldEnum>,
    ///<https://schema.org/availability>
    #[serde(rename = "availability")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub availability: Vec<ItemAvailabilityEnum>,
    ///<https://schema.org/mpn>
    #[serde(rename = "mpn")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub mpn: Vec<String>,
    ///<https://schema.org/includesObject>
    #[serde(rename = "includesObject")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub includes_object: Vec<TypeAndQuantityNode>,
    ///<https://schema.org/eligibleRegion>
    #[serde(rename = "eligibleRegion")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub eligible_region: Vec<DemandEligibleRegionFieldEnum>,
    ///<https://schema.org/eligibleTransactionVolume>
    #[serde(rename = "eligibleTransactionVolume")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub eligible_transaction_volume: Vec<PriceSpecification>,
    ///<https://schema.org/businessFunction>
    #[serde(rename = "businessFunction")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub business_function: Vec<BusinessFunction>,
    ///<https://schema.org/validFrom>
    #[serde(rename = "validFrom")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub valid_from: Vec<DemandValidFromFieldEnum>,
    ///<https://schema.org/seller>
    #[serde(rename = "seller")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub seller: Vec<DemandSellerFieldEnum>,
    ///<https://schema.org/priceSpecification>
    #[serde(rename = "priceSpecification")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub price_specification: Vec<PriceSpecification>,
    ///<https://schema.org/ineligibleRegion>
    #[serde(rename = "ineligibleRegion")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub ineligible_region: Vec<DemandIneligibleRegionFieldEnum>,
    ///<https://schema.org/availableDeliveryMethod>
    #[serde(rename = "availableDeliveryMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub available_delivery_method: Vec<DeliveryMethodEnum>,
    ///<https://schema.org/deliveryLeadTime>
    #[serde(rename = "deliveryLeadTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub delivery_lead_time: Vec<QuantitativeValue>,
    ///<https://schema.org/availabilityEnds>
    #[serde(rename = "availabilityEnds")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub availability_ends: Vec<DemandAvailabilityEndsFieldEnum>,
    ///<https://schema.org/gtin>
    #[serde(rename = "gtin")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin: Vec<DemandGtinFieldEnum>,
    ///<https://schema.org/gtin13>
    #[serde(rename = "gtin13")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin13: Vec<String>,
    ///<https://schema.org/validThrough>
    #[serde(rename = "validThrough")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub valid_through: Vec<DemandValidThroughFieldEnum>,
    ///<https://schema.org/gtin14>
    #[serde(rename = "gtin14")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin14: Vec<String>,
    ///<https://schema.org/asin>
    #[serde(rename = "asin")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub asin: Vec<DemandAsinFieldEnum>,
    ///<https://schema.org/advanceBookingRequirement>
    #[serde(rename = "advanceBookingRequirement")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub advance_booking_requirement: Vec<QuantitativeValue>,
    ///<https://schema.org/warranty>
    #[serde(rename = "warranty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub warranty: Vec<WarrantyPromise>,
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
    pub additional_type: Vec<DemandAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<DemandIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<DemandImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<DemandDescriptionFieldEnum>,
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
    pub subject_of: Vec<DemandSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<DemandMainEntityOfPageFieldEnum>,
}
