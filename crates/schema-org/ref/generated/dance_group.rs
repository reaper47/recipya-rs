use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type DanceGroupAdditionalTypeFieldEnum = String;
///<https://schema.org/DanceGroup>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct DanceGroup {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/hasMemberProgram>
    #[serde(rename = "hasMemberProgram")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_member_program: Vec<MemberProgram>,
    ///<https://schema.org/department>
    #[serde(rename = "department")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub department: Vec<Organization>,
    ///<https://schema.org/employees>
    #[serde(rename = "employees")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub employees: Vec<Person>,
    ///<https://schema.org/numberOfEmployees>
    #[serde(rename = "numberOfEmployees")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_employees: Vec<QuantitativeValue>,
    ///<https://schema.org/knowsAbout>
    #[serde(rename = "knowsAbout")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub knows_about: Vec<DanceGroupKnowsAboutFieldEnum>,
    ///<https://schema.org/diversityStaffingReport>
    #[serde(rename = "diversityStaffingReport")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub diversity_staffing_report: Vec<DanceGroupDiversityStaffingReportFieldEnum>,
    ///<https://schema.org/location>
    #[serde(rename = "location")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub location: Vec<DanceGroupLocationFieldEnum>,
    ///<https://schema.org/contactPoint>
    #[serde(rename = "contactPoint")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contact_point: Vec<ContactPoint>,
    ///<https://schema.org/faxNumber>
    #[serde(rename = "faxNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub fax_number: Vec<String>,
    ///<https://schema.org/duns>
    #[serde(rename = "duns")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub duns: Vec<String>,
    ///<https://schema.org/memberOf>
    #[serde(rename = "memberOf")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub member_of: Vec<DanceGroupMemberOfFieldEnum>,
    ///<https://schema.org/knowsLanguage>
    #[serde(rename = "knowsLanguage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub knows_language: Vec<DanceGroupKnowsLanguageFieldEnum>,
    ///<https://schema.org/review>
    #[serde(rename = "review")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub review: Vec<Review>,
    ///<https://schema.org/diversityPolicy>
    #[serde(rename = "diversityPolicy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub diversity_policy: Vec<DanceGroupDiversityPolicyFieldEnum>,
    ///<https://schema.org/skills>
    #[serde(rename = "skills")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub skills: Vec<DanceGroupSkillsFieldEnum>,
    ///<https://schema.org/interactionStatistic>
    #[serde(rename = "interactionStatistic")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/iso6523Code>
    #[serde(rename = "iso6523Code")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub iso6523_code: Vec<String>,
    ///<https://schema.org/agentInteractionStatistic>
    #[serde(rename = "agentInteractionStatistic")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub agent_interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/areaServed>
    #[serde(rename = "areaServed")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub area_served: Vec<DanceGroupAreaServedFieldEnum>,
    ///<https://schema.org/correctionsPolicy>
    #[serde(rename = "correctionsPolicy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub corrections_policy: Vec<DanceGroupCorrectionsPolicyFieldEnum>,
    ///<https://schema.org/leiCode>
    #[serde(rename = "leiCode")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub lei_code: Vec<String>,
    ///<https://schema.org/foundingLocation>
    #[serde(rename = "foundingLocation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub founding_location: Vec<Place>,
    ///<https://schema.org/legalName>
    #[serde(rename = "legalName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub legal_name: Vec<String>,
    ///<https://schema.org/keywords>
    #[serde(rename = "keywords")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub keywords: Vec<DanceGroupKeywordsFieldEnum>,
    ///<https://schema.org/naics>
    #[serde(rename = "naics")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub naics: Vec<String>,
    ///<https://schema.org/legalAddress>
    #[serde(rename = "legalAddress")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub legal_address: Vec<PostalAddress>,
    ///<https://schema.org/globalLocationNumber>
    #[serde(rename = "globalLocationNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub global_location_number: Vec<String>,
    ///<https://schema.org/contactPoints>
    #[serde(rename = "contactPoints")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contact_points: Vec<ContactPoint>,
    ///<https://schema.org/makesOffer>
    #[serde(rename = "makesOffer")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub makes_offer: Vec<Offer>,
    ///<https://schema.org/funder>
    #[serde(rename = "funder")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub funder: Vec<DanceGroupFunderFieldEnum>,
    ///<https://schema.org/events>
    #[serde(rename = "events")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub events: Vec<Event>,
    ///<https://schema.org/acceptedPaymentMethod>
    #[serde(rename = "acceptedPaymentMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub accepted_payment_method: Vec<DanceGroupAcceptedPaymentMethodFieldEnum>,
    ///<https://schema.org/vatID>
    #[serde(rename = "vatID")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub vat_id: Vec<String>,
    ///<https://schema.org/member>
    #[serde(rename = "member")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub member: Vec<DanceGroupMemberFieldEnum>,
    ///<https://schema.org/telephone>
    #[serde(rename = "telephone")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub telephone: Vec<String>,
    ///<https://schema.org/unnamedSourcesPolicy>
    #[serde(rename = "unnamedSourcesPolicy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub unnamed_sources_policy: Vec<DanceGroupUnnamedSourcesPolicyFieldEnum>,
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
    ///<https://schema.org/alumni>
    #[serde(rename = "alumni")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub alumni: Vec<Person>,
    ///<https://schema.org/sponsor>
    #[serde(rename = "sponsor")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub sponsor: Vec<DanceGroupSponsorFieldEnum>,
    ///<https://schema.org/aggregateRating>
    #[serde(rename = "aggregateRating")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/legalRepresentative>
    #[serde(rename = "legalRepresentative")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub legal_representative: Vec<Person>,
    ///<https://schema.org/hasGS1DigitalLink>
    #[serde(rename = "hasGS1DigitalLink")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_gs1_digital_link: Vec<String>,
    ///<https://schema.org/hasPOS>
    #[serde(rename = "hasPOS")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_pos: Vec<Place>,
    ///<https://schema.org/isicV4>
    #[serde(rename = "isicV4")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub isic_v4: Vec<String>,
    ///<https://schema.org/actionableFeedbackPolicy>
    #[serde(rename = "actionableFeedbackPolicy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub actionable_feedback_policy: Vec<DanceGroupActionableFeedbackPolicyFieldEnum>,
    ///<https://schema.org/members>
    #[serde(rename = "members")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub members: Vec<DanceGroupMembersFieldEnum>,
    ///<https://schema.org/parentOrganization>
    #[serde(rename = "parentOrganization")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub parent_organization: Vec<Organization>,
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
    ///<https://schema.org/seeks>
    #[serde(rename = "seeks")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub seeks: Vec<Demand>,
    ///<https://schema.org/ethicsPolicy>
    #[serde(rename = "ethicsPolicy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub ethics_policy: Vec<DanceGroupEthicsPolicyFieldEnum>,
    ///<https://schema.org/hasOfferCatalog>
    #[serde(rename = "hasOfferCatalog")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_offer_catalog: Vec<OfferCatalog>,
    ///<https://schema.org/brand>
    #[serde(rename = "brand")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub brand: Vec<DanceGroupBrandFieldEnum>,
    ///<https://schema.org/publishingPrinciples>
    #[serde(rename = "publishingPrinciples")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub publishing_principles: Vec<DanceGroupPublishingPrinciplesFieldEnum>,
    ///<https://schema.org/hasShippingService>
    #[serde(rename = "hasShippingService")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_shipping_service: Vec<ShippingService>,
    ///<https://schema.org/owns>
    #[serde(rename = "owns")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub owns: Vec<DanceGroupOwnsFieldEnum>,
    ///<https://schema.org/awards>
    #[serde(rename = "awards")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub awards: Vec<String>,
    ///<https://schema.org/nonprofitStatus>
    #[serde(rename = "nonprofitStatus")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub nonprofit_status: Vec<NonprofitType>,
    ///<https://schema.org/serviceArea>
    #[serde(rename = "serviceArea")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub service_area: Vec<DanceGroupServiceAreaFieldEnum>,
    ///<https://schema.org/foundingDate>
    #[serde(rename = "foundingDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub founding_date: Vec<String>,
    ///<https://schema.org/dissolutionDate>
    #[serde(rename = "dissolutionDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub dissolution_date: Vec<String>,
    ///<https://schema.org/hasCredential>
    #[serde(rename = "hasCredential")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_credential: Vec<EducationalOccupationalCredential>,
    ///<https://schema.org/founder>
    #[serde(rename = "founder")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub founder: Vec<DanceGroupFounderFieldEnum>,
    ///<https://schema.org/taxID>
    #[serde(rename = "taxID")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub tax_id: Vec<String>,
    ///<https://schema.org/event>
    #[serde(rename = "event")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub event: Vec<Event>,
    ///<https://schema.org/address>
    #[serde(rename = "address")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub address: Vec<DanceGroupAddressFieldEnum>,
    ///<https://schema.org/hasMerchantReturnPolicy>
    #[serde(rename = "hasMerchantReturnPolicy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_merchant_return_policy: Vec<MerchantReturnPolicy>,
    ///<https://schema.org/employee>
    #[serde(rename = "employee")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub employee: Vec<Person>,
    ///<https://schema.org/email>
    #[serde(rename = "email")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub email: Vec<String>,
    ///<https://schema.org/ownershipFundingInfo>
    #[serde(rename = "ownershipFundingInfo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub ownership_funding_info: Vec<DanceGroupOwnershipFundingInfoFieldEnum>,
    ///<https://schema.org/subOrganization>
    #[serde(rename = "subOrganization")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub sub_organization: Vec<Organization>,
    ///<https://schema.org/companyRegistration>
    #[serde(rename = "companyRegistration")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub company_registration: Vec<Certification>,
    ///<https://schema.org/logo>
    #[serde(rename = "logo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub logo: Vec<DanceGroupLogoFieldEnum>,
    ///<https://schema.org/reviews>
    #[serde(rename = "reviews")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub reviews: Vec<Review>,
    ///<https://schema.org/founders>
    #[serde(rename = "founders")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub founders: Vec<Person>,
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
    pub additional_type: Vec<DanceGroupAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<DanceGroupIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<DanceGroupImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<DanceGroupDescriptionFieldEnum>,
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
    pub subject_of: Vec<DanceGroupSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<DanceGroupMainEntityOfPageFieldEnum>,
}
