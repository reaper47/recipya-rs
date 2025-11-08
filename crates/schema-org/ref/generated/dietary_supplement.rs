use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/gtin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type DietarySupplementGtinFieldEnum = String;
///<https://schema.org/asin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type DietarySupplementAsinFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type DietarySupplementAdditionalTypeFieldEnum = String;
///<https://schema.org/DietarySupplement>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct DietarySupplement {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/maximumIntake>
    #[serde(rename = "maximumIntake")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub maximum_intake: Vec<MaximumDoseSchedule>,
    ///<https://schema.org/mechanismOfAction>
    #[serde(rename = "mechanismOfAction")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub mechanism_of_action: Vec<String>,
    ///<https://schema.org/isProprietary>
    #[serde(rename = "isProprietary")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_proprietary: Vec<String>,
    ///<https://schema.org/proprietaryName>
    #[serde(rename = "proprietaryName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub proprietary_name: Vec<String>,
    ///<https://schema.org/nonProprietaryName>
    #[serde(rename = "nonProprietaryName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub non_proprietary_name: Vec<String>,
    ///<https://schema.org/activeIngredient>
    #[serde(rename = "activeIngredient")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub active_ingredient: Vec<String>,
    ///<https://schema.org/targetPopulation>
    #[serde(rename = "targetPopulation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub target_population: Vec<String>,
    ///<https://schema.org/recommendedIntake>
    #[serde(rename = "recommendedIntake")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub recommended_intake: Vec<RecommendedDoseSchedule>,
    ///<https://schema.org/safetyConsideration>
    #[serde(rename = "safetyConsideration")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub safety_consideration: Vec<String>,
    ///<https://schema.org/legalStatus>
    #[serde(rename = "legalStatus")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub legal_status: Vec<DietarySupplementLegalStatusFieldEnum>,
    ///<https://schema.org/productionDate>
    #[serde(rename = "productionDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub production_date: Vec<String>,
    ///<https://schema.org/isRelatedTo>
    #[serde(rename = "isRelatedTo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_related_to: Vec<DietarySupplementIsRelatedToFieldEnum>,
    ///<https://schema.org/depth>
    #[serde(rename = "depth")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub depth: Vec<DietarySupplementDepthFieldEnum>,
    ///<https://schema.org/negativeNotes>
    #[serde(rename = "negativeNotes")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub negative_notes: Vec<DietarySupplementNegativeNotesFieldEnum>,
    ///<https://schema.org/itemCondition>
    #[serde(rename = "itemCondition")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item_condition: Vec<OfferItemConditionEnum>,
    ///<https://schema.org/isSimilarTo>
    #[serde(rename = "isSimilarTo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_similar_to: Vec<DietarySupplementIsSimilarToFieldEnum>,
    ///<https://schema.org/gtin8>
    #[serde(rename = "gtin8")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin8: Vec<String>,
    ///<https://schema.org/size>
    #[serde(rename = "size")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub size: Vec<DietarySupplementSizeFieldEnum>,
    ///<https://schema.org/offers>
    #[serde(rename = "offers")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub offers: Vec<DietarySupplementOffersFieldEnum>,
    ///<https://schema.org/audience>
    #[serde(rename = "audience")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub audience: Vec<Audience>,
    ///<https://schema.org/review>
    #[serde(rename = "review")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub review: Vec<Review>,
    ///<https://schema.org/hasMeasurement>
    #[serde(rename = "hasMeasurement")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_measurement: Vec<QuantitativeValue>,
    ///<https://schema.org/sku>
    #[serde(rename = "sku")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub sku: Vec<String>,
    ///<https://schema.org/isConsumableFor>
    #[serde(rename = "isConsumableFor")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_consumable_for: Vec<Product>,
    ///<https://schema.org/hasAdultConsideration>
    #[serde(rename = "hasAdultConsideration")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_adult_consideration: Vec<AdultOrientedEnumerationEnum>,
    ///<https://schema.org/positiveNotes>
    #[serde(rename = "positiveNotes")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub positive_notes: Vec<DietarySupplementPositiveNotesFieldEnum>,
    ///<https://schema.org/colorSwatch>
    #[serde(rename = "colorSwatch")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub color_swatch: Vec<DietarySupplementColorSwatchFieldEnum>,
    ///<https://schema.org/width>
    #[serde(rename = "width")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub width: Vec<DietarySupplementWidthFieldEnum>,
    ///<https://schema.org/material>
    #[serde(rename = "material")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub material: Vec<DietarySupplementMaterialFieldEnum>,
    ///<https://schema.org/gtin12>
    #[serde(rename = "gtin12")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin12: Vec<String>,
    ///<https://schema.org/keywords>
    #[serde(rename = "keywords")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub keywords: Vec<DietarySupplementKeywordsFieldEnum>,
    ///<https://schema.org/model>
    #[serde(rename = "model")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub model: Vec<DietarySupplementModelFieldEnum>,
    ///<https://schema.org/height>
    #[serde(rename = "height")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub height: Vec<DietarySupplementHeightFieldEnum>,
    ///<https://schema.org/countryOfOrigin>
    #[serde(rename = "countryOfOrigin")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub country_of_origin: Vec<Country>,
    ///<https://schema.org/isVariantOf>
    #[serde(rename = "isVariantOf")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_variant_of: Vec<DietarySupplementIsVariantOfFieldEnum>,
    ///<https://schema.org/slogan>
    #[serde(rename = "slogan")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub slogan: Vec<String>,
    ///<https://schema.org/award>
    #[serde(rename = "award")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub award: Vec<String>,
    ///<https://schema.org/weight>
    #[serde(rename = "weight")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub weight: Vec<DietarySupplementWeightFieldEnum>,
    ///<https://schema.org/mpn>
    #[serde(rename = "mpn")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub mpn: Vec<String>,
    ///<https://schema.org/pattern>
    #[serde(rename = "pattern")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub pattern: Vec<DietarySupplementPatternFieldEnum>,
    ///<https://schema.org/mobileUrl>
    #[serde(rename = "mobileUrl")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub mobile_url: Vec<String>,
    ///<https://schema.org/manufacturer>
    #[serde(rename = "manufacturer")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub manufacturer: Vec<Organization>,
    ///<https://schema.org/aggregateRating>
    #[serde(rename = "aggregateRating")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/purchaseDate>
    #[serde(rename = "purchaseDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub purchase_date: Vec<String>,
    ///<https://schema.org/hasGS1DigitalLink>
    #[serde(rename = "hasGS1DigitalLink")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_gs1_digital_link: Vec<String>,
    ///<https://schema.org/isAccessoryOrSparePartFor>
    #[serde(rename = "isAccessoryOrSparePartFor")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_accessory_or_spare_part_for: Vec<Product>,
    ///<https://schema.org/color>
    #[serde(rename = "color")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub color: Vec<String>,
    ///<https://schema.org/isFamilyFriendly>
    #[serde(rename = "isFamilyFriendly")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_family_friendly: Vec<String>,
    ///<https://schema.org/hasCertification>
    #[serde(rename = "hasCertification")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_certification: Vec<Certification>,
    ///<https://schema.org/funding>
    #[serde(rename = "funding")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub funding: Vec<Grant>,
    ///<https://schema.org/releaseDate>
    #[serde(rename = "releaseDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub release_date: Vec<String>,
    ///<https://schema.org/gtin>
    #[serde(rename = "gtin")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin: Vec<DietarySupplementGtinFieldEnum>,
    ///<https://schema.org/gtin13>
    #[serde(rename = "gtin13")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin13: Vec<String>,
    ///<https://schema.org/brand>
    #[serde(rename = "brand")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub brand: Vec<DietarySupplementBrandFieldEnum>,
    ///<https://schema.org/hasEnergyConsumptionDetails>
    #[serde(rename = "hasEnergyConsumptionDetails")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_energy_consumption_details: Vec<EnergyConsumptionDetails>,
    ///<https://schema.org/productID>
    #[serde(rename = "productID")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub product_id: Vec<String>,
    ///<https://schema.org/awards>
    #[serde(rename = "awards")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub awards: Vec<String>,
    ///<https://schema.org/category>
    #[serde(rename = "category")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub category: Vec<DietarySupplementCategoryFieldEnum>,
    ///<https://schema.org/countryOfAssembly>
    #[serde(rename = "countryOfAssembly")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub country_of_assembly: Vec<String>,
    ///<https://schema.org/inProductGroupWithID>
    #[serde(rename = "inProductGroupWithID")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub in_product_group_with_id: Vec<String>,
    ///<https://schema.org/gtin14>
    #[serde(rename = "gtin14")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin14: Vec<String>,
    ///<https://schema.org/asin>
    #[serde(rename = "asin")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub asin: Vec<DietarySupplementAsinFieldEnum>,
    ///<https://schema.org/hasMerchantReturnPolicy>
    #[serde(rename = "hasMerchantReturnPolicy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_merchant_return_policy: Vec<MerchantReturnPolicy>,
    ///<https://schema.org/additionalProperty>
    #[serde(rename = "additionalProperty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/nsn>
    #[serde(rename = "nsn")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub nsn: Vec<String>,
    ///<https://schema.org/logo>
    #[serde(rename = "logo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub logo: Vec<DietarySupplementLogoFieldEnum>,
    ///<https://schema.org/reviews>
    #[serde(rename = "reviews")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub reviews: Vec<Review>,
    ///<https://schema.org/countryOfLastProcessing>
    #[serde(rename = "countryOfLastProcessing")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub country_of_last_processing: Vec<String>,
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
    pub additional_type: Vec<DietarySupplementAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<DietarySupplementIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<DietarySupplementImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<DietarySupplementDescriptionFieldEnum>,
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
    pub subject_of: Vec<DietarySupplementSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<DietarySupplementMainEntityOfPageFieldEnum>,
    ///<https://schema.org/code>
    #[serde(rename = "code")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub code: Vec<MedicalCode>,
    ///<https://schema.org/study>
    #[serde(rename = "study")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub study: Vec<MedicalStudy>,
    ///<https://schema.org/recognizingAuthority>
    #[serde(rename = "recognizingAuthority")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub recognizing_authority: Vec<Organization>,
    ///<https://schema.org/guideline>
    #[serde(rename = "guideline")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub guideline: Vec<MedicalGuideline>,
    ///<https://schema.org/medicineSystem>
    #[serde(rename = "medicineSystem")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub medicine_system: Vec<MedicineSystemEnum>,
    ///<https://schema.org/relevantSpecialty>
    #[serde(rename = "relevantSpecialty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub relevant_specialty: Vec<MedicalSpecialtyEnum>,
}
