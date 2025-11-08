use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/feesAndCommissionsSpecification>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type FinancialProductFeesAndCommissionsSpecificationFieldEnum = String;
///<https://schema.org/termsOfService>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type FinancialProductTermsOfServiceFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type FinancialProductAdditionalTypeFieldEnum = String;
///<https://schema.org/FinancialProduct>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct FinancialProduct {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/interestRate>
    #[serde(rename = "interestRate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub interest_rate: Vec<FinancialProductInterestRateFieldEnum>,
    ///<https://schema.org/feesAndCommissionsSpecification>
    #[serde(rename = "feesAndCommissionsSpecification")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub fees_and_commissions_specification: Vec<
        FinancialProductFeesAndCommissionsSpecificationFieldEnum,
    >,
    ///<https://schema.org/annualPercentageRate>
    #[serde(rename = "annualPercentageRate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub annual_percentage_rate: Vec<FinancialProductAnnualPercentageRateFieldEnum>,
    ///<https://schema.org/broker>
    #[serde(rename = "broker")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub broker: Vec<FinancialProductBrokerFieldEnum>,
    ///<https://schema.org/hoursAvailable>
    #[serde(rename = "hoursAvailable")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub hours_available: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/isRelatedTo>
    #[serde(rename = "isRelatedTo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_related_to: Vec<FinancialProductIsRelatedToFieldEnum>,
    ///<https://schema.org/produces>
    #[serde(rename = "produces")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub produces: Vec<Thing>,
    ///<https://schema.org/isSimilarTo>
    #[serde(rename = "isSimilarTo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_similar_to: Vec<FinancialProductIsSimilarToFieldEnum>,
    ///<https://schema.org/termsOfService>
    #[serde(rename = "termsOfService")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub terms_of_service: Vec<FinancialProductTermsOfServiceFieldEnum>,
    ///<https://schema.org/offers>
    #[serde(rename = "offers")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub offers: Vec<FinancialProductOffersFieldEnum>,
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
    ///<https://schema.org/serviceOutput>
    #[serde(rename = "serviceOutput")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub service_output: Vec<Thing>,
    ///<https://schema.org/areaServed>
    #[serde(rename = "areaServed")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub area_served: Vec<FinancialProductAreaServedFieldEnum>,
    ///<https://schema.org/provider>
    #[serde(rename = "provider")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub provider: Vec<FinancialProductProviderFieldEnum>,
    ///<https://schema.org/availableChannel>
    #[serde(rename = "availableChannel")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub available_channel: Vec<ServiceChannel>,
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
    ///<https://schema.org/aggregateRating>
    #[serde(rename = "aggregateRating")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/hasCertification>
    #[serde(rename = "hasCertification")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_certification: Vec<Certification>,
    ///<https://schema.org/hasOfferCatalog>
    #[serde(rename = "hasOfferCatalog")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_offer_catalog: Vec<OfferCatalog>,
    ///<https://schema.org/brand>
    #[serde(rename = "brand")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub brand: Vec<FinancialProductBrandFieldEnum>,
    ///<https://schema.org/providerMobility>
    #[serde(rename = "providerMobility")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub provider_mobility: Vec<String>,
    ///<https://schema.org/category>
    #[serde(rename = "category")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub category: Vec<FinancialProductCategoryFieldEnum>,
    ///<https://schema.org/serviceType>
    #[serde(rename = "serviceType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub service_type: Vec<FinancialProductServiceTypeFieldEnum>,
    ///<https://schema.org/serviceArea>
    #[serde(rename = "serviceArea")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub service_area: Vec<FinancialProductServiceAreaFieldEnum>,
    ///<https://schema.org/serviceAudience>
    #[serde(rename = "serviceAudience")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub service_audience: Vec<Audience>,
    ///<https://schema.org/logo>
    #[serde(rename = "logo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub logo: Vec<FinancialProductLogoFieldEnum>,
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
    pub additional_type: Vec<FinancialProductAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<FinancialProductIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<FinancialProductImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<FinancialProductDescriptionFieldEnum>,
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
    pub subject_of: Vec<FinancialProductSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<FinancialProductMainEntityOfPageFieldEnum>,
}
