use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/warning>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type DrugWarningFieldEnum = String;
///<https://schema.org/gtin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type DrugGtinFieldEnum = String;
///<https://schema.org/asin>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type DrugAsinFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type DrugAdditionalTypeFieldEnum = String;
///<https://schema.org/Drug>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Drug {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/relatedDrug>
    #[serde(rename = "relatedDrug")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub related_drug: Vec<Drug>,
    ///<https://schema.org/alcoholWarning>
    #[serde(rename = "alcoholWarning")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub alcohol_warning: Vec<String>,
    ///<https://schema.org/clincalPharmacology>
    #[serde(rename = "clincalPharmacology")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub clincal_pharmacology: Vec<String>,
    ///<https://schema.org/maximumIntake>
    #[serde(rename = "maximumIntake")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub maximum_intake: Vec<MaximumDoseSchedule>,
    ///<https://schema.org/dosageForm>
    #[serde(rename = "dosageForm")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub dosage_form: Vec<String>,
    ///<https://schema.org/pregnancyCategory>
    #[serde(rename = "pregnancyCategory")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub pregnancy_category: Vec<DrugPregnancyCategoryEnum>,
    ///<https://schema.org/mechanismOfAction>
    #[serde(rename = "mechanismOfAction")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub mechanism_of_action: Vec<String>,
    ///<https://schema.org/labelDetails>
    #[serde(rename = "labelDetails")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub label_details: Vec<String>,
    ///<https://schema.org/pregnancyWarning>
    #[serde(rename = "pregnancyWarning")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub pregnancy_warning: Vec<String>,
    ///<https://schema.org/warning>
    #[serde(rename = "warning")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub warning: Vec<DrugWarningFieldEnum>,
    ///<https://schema.org/drugUnit>
    #[serde(rename = "drugUnit")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub drug_unit: Vec<String>,
    ///<https://schema.org/isProprietary>
    #[serde(rename = "isProprietary")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_proprietary: Vec<String>,
    ///<https://schema.org/overdosage>
    #[serde(rename = "overdosage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub overdosage: Vec<String>,
    ///<https://schema.org/breastfeedingWarning>
    #[serde(rename = "breastfeedingWarning")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub breastfeeding_warning: Vec<String>,
    ///<https://schema.org/doseSchedule>
    #[serde(rename = "doseSchedule")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub dose_schedule: Vec<DoseSchedule>,
    ///<https://schema.org/clinicalPharmacology>
    #[serde(rename = "clinicalPharmacology")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub clinical_pharmacology: Vec<String>,
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
    ///<https://schema.org/drugClass>
    #[serde(rename = "drugClass")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub drug_class: Vec<DrugClass>,
    ///<https://schema.org/activeIngredient>
    #[serde(rename = "activeIngredient")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub active_ingredient: Vec<String>,
    ///<https://schema.org/foodWarning>
    #[serde(rename = "foodWarning")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub food_warning: Vec<String>,
    ///<https://schema.org/availableStrength>
    #[serde(rename = "availableStrength")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub available_strength: Vec<DrugStrength>,
    ///<https://schema.org/interactingDrug>
    #[serde(rename = "interactingDrug")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub interacting_drug: Vec<Drug>,
    ///<https://schema.org/isAvailableGenerically>
    #[serde(rename = "isAvailableGenerically")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_available_generically: Vec<String>,
    ///<https://schema.org/administrationRoute>
    #[serde(rename = "administrationRoute")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub administration_route: Vec<String>,
    ///<https://schema.org/legalStatus>
    #[serde(rename = "legalStatus")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub legal_status: Vec<DrugLegalStatusFieldEnum>,
    ///<https://schema.org/rxcui>
    #[serde(rename = "rxcui")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub rxcui: Vec<String>,
    ///<https://schema.org/includedInHealthInsurancePlan>
    #[serde(rename = "includedInHealthInsurancePlan")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub included_in_health_insurance_plan: Vec<HealthInsurancePlan>,
    ///<https://schema.org/prescriptionStatus>
    #[serde(rename = "prescriptionStatus")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub prescription_status: Vec<DrugPrescriptionStatusFieldEnum>,
    ///<https://schema.org/prescribingInfo>
    #[serde(rename = "prescribingInfo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub prescribing_info: Vec<String>,
    ///<https://schema.org/productionDate>
    #[serde(rename = "productionDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub production_date: Vec<String>,
    ///<https://schema.org/isRelatedTo>
    #[serde(rename = "isRelatedTo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_related_to: Vec<DrugIsRelatedToFieldEnum>,
    ///<https://schema.org/depth>
    #[serde(rename = "depth")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub depth: Vec<DrugDepthFieldEnum>,
    ///<https://schema.org/negativeNotes>
    #[serde(rename = "negativeNotes")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub negative_notes: Vec<DrugNegativeNotesFieldEnum>,
    ///<https://schema.org/itemCondition>
    #[serde(rename = "itemCondition")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub item_condition: Vec<OfferItemConditionEnum>,
    ///<https://schema.org/isSimilarTo>
    #[serde(rename = "isSimilarTo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_similar_to: Vec<DrugIsSimilarToFieldEnum>,
    ///<https://schema.org/gtin8>
    #[serde(rename = "gtin8")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin8: Vec<String>,
    ///<https://schema.org/size>
    #[serde(rename = "size")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub size: Vec<DrugSizeFieldEnum>,
    ///<https://schema.org/offers>
    #[serde(rename = "offers")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub offers: Vec<DrugOffersFieldEnum>,
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
    pub positive_notes: Vec<DrugPositiveNotesFieldEnum>,
    ///<https://schema.org/colorSwatch>
    #[serde(rename = "colorSwatch")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub color_swatch: Vec<DrugColorSwatchFieldEnum>,
    ///<https://schema.org/width>
    #[serde(rename = "width")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub width: Vec<DrugWidthFieldEnum>,
    ///<https://schema.org/material>
    #[serde(rename = "material")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub material: Vec<DrugMaterialFieldEnum>,
    ///<https://schema.org/gtin12>
    #[serde(rename = "gtin12")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin12: Vec<String>,
    ///<https://schema.org/keywords>
    #[serde(rename = "keywords")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub keywords: Vec<DrugKeywordsFieldEnum>,
    ///<https://schema.org/model>
    #[serde(rename = "model")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub model: Vec<DrugModelFieldEnum>,
    ///<https://schema.org/height>
    #[serde(rename = "height")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub height: Vec<DrugHeightFieldEnum>,
    ///<https://schema.org/countryOfOrigin>
    #[serde(rename = "countryOfOrigin")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub country_of_origin: Vec<Country>,
    ///<https://schema.org/isVariantOf>
    #[serde(rename = "isVariantOf")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_variant_of: Vec<DrugIsVariantOfFieldEnum>,
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
    pub weight: Vec<DrugWeightFieldEnum>,
    ///<https://schema.org/mpn>
    #[serde(rename = "mpn")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub mpn: Vec<String>,
    ///<https://schema.org/pattern>
    #[serde(rename = "pattern")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub pattern: Vec<DrugPatternFieldEnum>,
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
    pub gtin: Vec<DrugGtinFieldEnum>,
    ///<https://schema.org/gtin13>
    #[serde(rename = "gtin13")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gtin13: Vec<String>,
    ///<https://schema.org/brand>
    #[serde(rename = "brand")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub brand: Vec<DrugBrandFieldEnum>,
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
    pub category: Vec<DrugCategoryFieldEnum>,
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
    pub asin: Vec<DrugAsinFieldEnum>,
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
    pub logo: Vec<DrugLogoFieldEnum>,
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
    pub additional_type: Vec<DrugAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<DrugIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<DrugImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<DrugDescriptionFieldEnum>,
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
    pub subject_of: Vec<DrugSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<DrugMainEntityOfPageFieldEnum>,
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
