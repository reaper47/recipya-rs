use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::enums::{AdultOrientedEnumerationEnum, OfferItemConditionEnum};
use crate::field::{
    ProductColorSwatchFieldEnum, ProductDescriptionFieldEnum, ProductImageFieldEnum,
    ProductKeywordsFieldEnum, ProductLogoFieldEnum, ProductMaterialFieldEnum,
    ProductNegativeNotesFieldEnum, ProductPositiveNotesFieldEnum, ProductSizeFieldEnum,
    ProductSubjectOfFieldEnum, ProductWeightFieldEnum,
};
use crate::helpers::{is_smallvec_empty, one_or_many};
use crate::{AggregateRating, Country, Organization, PropertyValue, QuantitativeValue, Review};

///<https://schema.org/gtin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ProductGtinFieldEnum = String;
///<https://schema.org/asin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ProductAsinFieldEnum = String;

///<https://schema.org/Product>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    #[serde(rename = "@context")]
    pub context: Option<String>,
    ///<https://schema.org/productionDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub production_date: SmallVec<[String; 1]>,
    ///<https://schema.org/negativeNotes>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub negative_notes: SmallVec<[ProductNegativeNotesFieldEnum; 1]>,
    ///<https://schema.org/itemCondition>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub item_condition: SmallVec<[OfferItemConditionEnum; 1]>,
    ///<https://schema.org/gtin8>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub gtin8: SmallVec<[String; 1]>,
    ///<https://schema.org/size>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub size: SmallVec<[ProductSizeFieldEnum; 1]>,
    ///<https://schema.org/review>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub review: SmallVec<[Review; 4]>,
    ///<https://schema.org/hasMeasurement>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub has_measurement: SmallVec<[QuantitativeValue; 1]>,
    ///<https://schema.org/sku>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub sku: SmallVec<[String; 1]>,
    ///<https://schema.org/isConsumableFor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub is_consumable_for: SmallVec<[Box<Product>; 1]>,
    ///<https://schema.org/hasAdultConsideration>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub has_adult_consideration: SmallVec<[AdultOrientedEnumerationEnum; 1]>,
    ///<https://schema.org/positiveNotes>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub positive_notes: SmallVec<[ProductPositiveNotesFieldEnum; 1]>,
    ///<https://schema.org/colorSwatch>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub color_swatch: SmallVec<[ProductColorSwatchFieldEnum; 1]>,
    ///<https://schema.org/material>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub material: SmallVec<[ProductMaterialFieldEnum; 1]>,
    ///<https://schema.org/gtin12>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub gtin12: SmallVec<[String; 1]>,
    ///<https://schema.org/keywords>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub keywords: SmallVec<[ProductKeywordsFieldEnum; 6]>,
    ///<https://schema.org/countryOfOrigin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub country_of_origin: SmallVec<[Country; 1]>,
    ///<https://schema.org/slogan>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub slogan: SmallVec<[String; 1]>,
    ///<https://schema.org/award>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub award: SmallVec<[String; 1]>,
    ///<https://schema.org/weight>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub weight: SmallVec<[ProductWeightFieldEnum; 1]>,
    ///<https://schema.org/mpn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub mpn: SmallVec<[String; 1]>,
    ///<https://schema.org/mobileUrl>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub mobile_url: SmallVec<[String; 1]>,
    ///<https://schema.org/manufacturer>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub manufacturer: SmallVec<[Organization; 1]>,
    ///<https://schema.org/aggregateRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub aggregate_rating: SmallVec<[AggregateRating; 1]>,
    ///<https://schema.org/purchaseDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub purchase_date: SmallVec<[String; 1]>,
    ///<https://schema.org/hasGS1DigitalLink>
    #[serde(rename = "hasGS1DigitalLink")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub has_gs1_digital_link: SmallVec<[String; 1]>,
    ///<https://schema.org/isAccessoryOrSparePartFor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub is_accessory_or_spare_part_for: SmallVec<[Box<Product>; 1]>,
    ///<https://schema.org/color>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub color: SmallVec<[String; 1]>,
    ///<https://schema.org/releaseDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub release_date: SmallVec<[String; 1]>,
    ///<https://schema.org/gtin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub gtin: SmallVec<[ProductGtinFieldEnum; 1]>,
    ///<https://schema.org/gtin13>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub gtin13: SmallVec<[String; 1]>,
    ///<https://schema.org/productID>
    #[serde(rename = "productID")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub product_id: SmallVec<[String; 1]>,
    ///<https://schema.org/awards>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub awards: SmallVec<[String; 3]>,
    ///<https://schema.org/countryOfAssembly>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub country_of_assembly: SmallVec<[String; 1]>,
    ///<https://schema.org/inProductGroupWithID>
    #[serde(rename = "inProductGroupWithID")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub in_product_group_with_id: SmallVec<[String; 1]>,
    ///<https://schema.org/gtin14>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub gtin14: SmallVec<[String; 1]>,
    ///<https://schema.org/asin>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub asin: SmallVec<[ProductAsinFieldEnum; 1]>,
    ///<https://schema.org/additionalProperty>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub additional_property: SmallVec<[PropertyValue; 1]>,
    ///<https://schema.org/nsn>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub nsn: SmallVec<[String; 1]>,
    ///<https://schema.org/logo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub logo: SmallVec<[ProductLogoFieldEnum; 1]>,
    ///<https://schema.org/reviews>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub reviews: SmallVec<[Review; 6]>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub disambiguating_description: SmallVec<[String; 1]>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub image: SmallVec<[ProductImageFieldEnum; 3]>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub same_as: SmallVec<[String; 1]>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub description: SmallVec<[ProductDescriptionFieldEnum; 1]>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub alternate_name: SmallVec<[String; 1]>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub url: SmallVec<[String; 1]>,
    ///<https://schema.org/subjectOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub subject_of: SmallVec<[ProductSubjectOfFieldEnum; 1]>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "is_smallvec_empty")]
    pub name: SmallVec<[String; 1]>,
}
