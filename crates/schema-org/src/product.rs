use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::aggregate_rating::AggregateRating;
use crate::audience::Audience;
use crate::certification::Certification;
use crate::country::Country;
use crate::enums::{AdultOrientedEnumerationEnum, OfferItemConditionEnum};
use crate::helpers::one_or_many;
use crate::field::*;
use crate::grant::Grant;
use crate::merchant_return_policy::MerchantReturnPolicy;
use crate::organization::Organization;
use crate::quantitative_value::QuantitativeValue;
use crate::{Action, PropertyValue, Review};
use crate::energy_consumption_details::EnergyConsumptionDetails;

///<https://schema.org/gtin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ProductGtinFieldEnum = String;
///<https://schema.org/asin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ProductAsinFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ProductAdditionalTypeFieldEnum = String;

///<https://schema.org/Product>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/productionDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub production_date: Vec<String>,
    ///<https://schema.org/isRelatedTo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_related_to: Vec<ProductIsRelatedToFieldEnum>,
    ///<https://schema.org/depth>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depth: Vec<ProductDepthFieldEnum>,
    ///<https://schema.org/negativeNotes>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub negative_notes: Vec<ProductNegativeNotesFieldEnum>,
    ///<https://schema.org/itemCondition>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item_condition: Vec<OfferItemConditionEnum>,
    ///<https://schema.org/isSimilarTo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_similar_to: Vec<ProductIsSimilarToFieldEnum>,
    ///<https://schema.org/gtin8>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin8: Vec<String>,
    ///<https://schema.org/size>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub size: Vec<ProductSizeFieldEnum>,
    ///<https://schema.org/offers>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offers: Vec<ProductOffersFieldEnum>,
    ///<https://schema.org/audience>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<Audience>,
    ///<https://schema.org/review>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub review: Vec<Review>,
    ///<https://schema.org/hasMeasurement>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_measurement: Vec<QuantitativeValue>,
    ///<https://schema.org/sku>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sku: Vec<String>,
    ///<https://schema.org/isConsumableFor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_consumable_for: Vec<Product>,
    ///<https://schema.org/hasAdultConsideration>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_adult_consideration: Vec<AdultOrientedEnumerationEnum>,
    ///<https://schema.org/positiveNotes>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub positive_notes: Vec<ProductPositiveNotesFieldEnum>,
    ///<https://schema.org/colorSwatch>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub color_swatch: Vec<ProductColorSwatchFieldEnum>,
    ///<https://schema.org/width>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub width: Vec<ProductWidthFieldEnum>,
    ///<https://schema.org/material>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<ProductMaterialFieldEnum>,
    ///<https://schema.org/gtin12>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin12: Vec<String>,
    ///<https://schema.org/keywords>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<ProductKeywordsFieldEnum>,
    ///<https://schema.org/model>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub model: Vec<ProductModelFieldEnum>,
    ///<https://schema.org/height>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub height: Vec<ProductHeightFieldEnum>,
    ///<https://schema.org/countryOfOrigin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub country_of_origin: Vec<Country>,
    ///<https://schema.org/isVariantOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_variant_of: Vec<ProductIsVariantOfFieldEnum>,
    ///<https://schema.org/slogan>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub slogan: Vec<String>,
    ///<https://schema.org/award>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub award: Vec<String>,
    ///<https://schema.org/weight>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub weight: Vec<ProductWeightFieldEnum>,
    ///<https://schema.org/mpn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mpn: Vec<String>,
    ///<https://schema.org/pattern>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pattern: Vec<ProductPatternFieldEnum>,
    ///<https://schema.org/mobileUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mobile_url: Vec<String>,
    ///<https://schema.org/manufacturer>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<Organization>,
    ///<https://schema.org/aggregateRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/purchaseDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub purchase_date: Vec<String>,
    ///<https://schema.org/hasGS1DigitalLink>
    #[serde(rename = "hasGS1DigitalLink")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_gs1_digital_link: Vec<String>,
    ///<https://schema.org/isAccessoryOrSparePartFor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_accessory_or_spare_part_for: Vec<Product>,
    ///<https://schema.org/color>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub color: Vec<String>,
    ///<https://schema.org/isFamilyFriendly>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_family_friendly: Vec<String>,
    ///<https://schema.org/hasCertification>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_certification: Vec<Certification>,
    ///<https://schema.org/funding>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funding: Vec<Grant>,
    ///<https://schema.org/releaseDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub release_date: Vec<String>,
    ///<https://schema.org/gtin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin: Vec<ProductGtinFieldEnum>,
    ///<https://schema.org/gtin13>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin13: Vec<String>,
    ///<https://schema.org/brand>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub brand: Vec<ProductBrandFieldEnum>,
    ///<https://schema.org/hasEnergyConsumptionDetails>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_energy_consumption_details: Vec<EnergyConsumptionDetails>,
    ///<https://schema.org/productID>
    #[serde(rename = "productID")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub product_id: Vec<String>,
    ///<https://schema.org/awards>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub awards: Vec<String>,
    ///<https://schema.org/category>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<ProductCategoryFieldEnum>,
    ///<https://schema.org/countryOfAssembly>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub country_of_assembly: Vec<String>,
    ///<https://schema.org/inProductGroupWithID>
    #[serde(rename = "inProductGroupWithID")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub in_product_group_with_id: Vec<String>,
    ///<https://schema.org/gtin14>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gtin14: Vec<String>,
    ///<https://schema.org/asin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub asin: Vec<ProductAsinFieldEnum>,
    ///<https://schema.org/hasMerchantReturnPolicy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_merchant_return_policy: Vec<MerchantReturnPolicy>,
    ///<https://schema.org/additionalProperty>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/nsn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nsn: Vec<String>,
    ///<https://schema.org/logo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub logo: Vec<ProductLogoFieldEnum>,
    ///<https://schema.org/reviews>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reviews: Vec<Review>,
    ///<https://schema.org/countryOfLastProcessing>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub country_of_last_processing: Vec<String>,
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
    pub additional_type: Vec<ProductAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<ProductIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<ProductImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<ProductDescriptionFieldEnum>,
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
    pub subject_of: Vec<ProductSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<ProductMainEntityOfPageFieldEnum>,
}
