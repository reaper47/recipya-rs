use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Action, Duration, Review, Thing};
use crate::aggregate_rating::AggregateRating;
use crate::audience::Audience;
use crate::certification::Certification;
use crate::helpers::one_or_many;
use crate::field::*;
use crate::offer_catalog::OfferCatalog;
use crate::opening_hours_specification::OpeningHoursSpecification;
use crate::quantitative_value::QuantitativeValue;
use crate::repayment_specification::RepaymentSpecification;
use crate::service_channel::ServiceChannel;

///<https://schema.org/loanType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LoanOrCreditLoanTypeFieldEnum = String;
///<https://schema.org/feesAndCommissionsSpecification>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LoanOrCreditFeesAndCommissionsSpecificationFieldEnum = String;
///<https://schema.org/termsOfService>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LoanOrCreditTermsOfServiceFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LoanOrCreditAdditionalTypeFieldEnum = String;

///<https://schema.org/LoanOrCredit>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LoanOrCredit {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/requiredCollateral>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required_collateral: Vec<LoanOrCreditRequiredCollateralFieldEnum>,
    ///<https://schema.org/recourseLoan>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recourse_loan: Vec<String>,
    ///<https://schema.org/loanTerm>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loan_term: Vec<QuantitativeValue>,
    ///<https://schema.org/amount>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub amount: Vec<LoanOrCreditAmountFieldEnum>,
    ///<https://schema.org/gracePeriod>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub grace_period: Vec<Duration>,
    ///<https://schema.org/currency>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub currency: Vec<String>,
    ///<https://schema.org/loanType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loan_type: Vec<LoanOrCreditLoanTypeFieldEnum>,
    ///<https://schema.org/loanRepaymentForm>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loan_repayment_form: Vec<RepaymentSpecification>,
    ///<https://schema.org/renegotiableLoan>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub renegotiable_loan: Vec<String>,
    ///<https://schema.org/interestRate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interest_rate: Vec<LoanOrCreditInterestRateFieldEnum>,
    ///<https://schema.org/feesAndCommissionsSpecification>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fees_and_commissions_specification: Vec<
        LoanOrCreditFeesAndCommissionsSpecificationFieldEnum,
    >,
    ///<https://schema.org/annualPercentageRate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub annual_percentage_rate: Vec<LoanOrCreditAnnualPercentageRateFieldEnum>,
    ///<https://schema.org/broker>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub broker: Vec<LoanOrCreditBrokerFieldEnum>,
    ///<https://schema.org/hoursAvailable>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hours_available: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/isRelatedTo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_related_to: Vec<LoanOrCreditIsRelatedToFieldEnum>,
    ///<https://schema.org/produces>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub produces: Vec<Thing>,
    ///<https://schema.org/isSimilarTo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_similar_to: Vec<LoanOrCreditIsSimilarToFieldEnum>,
    ///<https://schema.org/termsOfService>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub terms_of_service: Vec<LoanOrCreditTermsOfServiceFieldEnum>,
    ///<https://schema.org/offers>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offers: Vec<LoanOrCreditOffersFieldEnum>,
    ///<https://schema.org/audience>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<Audience>,
    ///<https://schema.org/review>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub review: Vec<Review>,
    ///<https://schema.org/serviceOutput>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service_output: Vec<Thing>,
    ///<https://schema.org/areaServed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub area_served: Vec<LoanOrCreditAreaServedFieldEnum>,
    ///<https://schema.org/provider>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provider: Vec<LoanOrCreditProviderFieldEnum>,
    ///<https://schema.org/availableChannel>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub available_channel: Vec<ServiceChannel>,
    ///<https://schema.org/slogan>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub slogan: Vec<String>,
    ///<https://schema.org/award>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub award: Vec<String>,
    ///<https://schema.org/aggregateRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/hasCertification>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_certification: Vec<Certification>,
    ///<https://schema.org/hasOfferCatalog>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_offer_catalog: Vec<OfferCatalog>,
    ///<https://schema.org/brand>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub brand: Vec<LoanOrCreditBrandFieldEnum>,
    ///<https://schema.org/providerMobility>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provider_mobility: Vec<String>,
    ///<https://schema.org/category>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<LoanOrCreditCategoryFieldEnum>,
    ///<https://schema.org/serviceType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<LoanOrCreditServiceTypeFieldEnum>,
    ///<https://schema.org/serviceArea>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service_area: Vec<LoanOrCreditServiceAreaFieldEnum>,
    ///<https://schema.org/serviceAudience>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service_audience: Vec<Audience>,
    ///<https://schema.org/logo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub logo: Vec<LoanOrCreditLogoFieldEnum>,
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
    pub additional_type: Vec<LoanOrCreditAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<LoanOrCreditIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<LoanOrCreditImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<LoanOrCreditDescriptionFieldEnum>,
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
    pub subject_of: Vec<LoanOrCreditSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<LoanOrCreditMainEntityOfPageFieldEnum>,
}
