use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type LocksmithAdditionalTypeFieldEnum = String;
///<https://schema.org/Locksmith>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Locksmith {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/currenciesAccepted>
    #[serde(rename = "currenciesAccepted")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub currencies_accepted: Vec<String>,
    ///<https://schema.org/paymentAccepted>
    #[serde(rename = "paymentAccepted")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub payment_accepted: Vec<String>,
    ///<https://schema.org/openingHours>
    #[serde(rename = "openingHours")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub opening_hours: Vec<String>,
    ///<https://schema.org/branchOf>
    #[serde(rename = "branchOf")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub branch_of: Vec<Organization>,
    ///<https://schema.org/priceRange>
    #[serde(rename = "priceRange")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub price_range: Vec<String>,
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
    pub knows_about: Vec<LocksmithKnowsAboutFieldEnum>,
    ///<https://schema.org/diversityStaffingReport>
    #[serde(rename = "diversityStaffingReport")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub diversity_staffing_report: Vec<LocksmithDiversityStaffingReportFieldEnum>,
    ///<https://schema.org/location>
    #[serde(rename = "location")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub location: Vec<LocksmithLocationFieldEnum>,
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
    pub member_of: Vec<LocksmithMemberOfFieldEnum>,
    ///<https://schema.org/knowsLanguage>
    #[serde(rename = "knowsLanguage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub knows_language: Vec<LocksmithKnowsLanguageFieldEnum>,
    ///<https://schema.org/review>
    #[serde(rename = "review")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub review: Vec<Review>,
    ///<https://schema.org/diversityPolicy>
    #[serde(rename = "diversityPolicy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub diversity_policy: Vec<LocksmithDiversityPolicyFieldEnum>,
    ///<https://schema.org/skills>
    #[serde(rename = "skills")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub skills: Vec<LocksmithSkillsFieldEnum>,
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
    pub area_served: Vec<LocksmithAreaServedFieldEnum>,
    ///<https://schema.org/correctionsPolicy>
    #[serde(rename = "correctionsPolicy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub corrections_policy: Vec<LocksmithCorrectionsPolicyFieldEnum>,
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
    pub keywords: Vec<LocksmithKeywordsFieldEnum>,
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
    pub funder: Vec<LocksmithFunderFieldEnum>,
    ///<https://schema.org/events>
    #[serde(rename = "events")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub events: Vec<Event>,
    ///<https://schema.org/acceptedPaymentMethod>
    #[serde(rename = "acceptedPaymentMethod")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub accepted_payment_method: Vec<LocksmithAcceptedPaymentMethodFieldEnum>,
    ///<https://schema.org/vatID>
    #[serde(rename = "vatID")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub vat_id: Vec<String>,
    ///<https://schema.org/member>
    #[serde(rename = "member")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub member: Vec<LocksmithMemberFieldEnum>,
    ///<https://schema.org/telephone>
    #[serde(rename = "telephone")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub telephone: Vec<String>,
    ///<https://schema.org/unnamedSourcesPolicy>
    #[serde(rename = "unnamedSourcesPolicy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub unnamed_sources_policy: Vec<LocksmithUnnamedSourcesPolicyFieldEnum>,
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
    pub sponsor: Vec<LocksmithSponsorFieldEnum>,
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
    pub actionable_feedback_policy: Vec<LocksmithActionableFeedbackPolicyFieldEnum>,
    ///<https://schema.org/members>
    #[serde(rename = "members")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub members: Vec<LocksmithMembersFieldEnum>,
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
    pub ethics_policy: Vec<LocksmithEthicsPolicyFieldEnum>,
    ///<https://schema.org/hasOfferCatalog>
    #[serde(rename = "hasOfferCatalog")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_offer_catalog: Vec<OfferCatalog>,
    ///<https://schema.org/brand>
    #[serde(rename = "brand")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub brand: Vec<LocksmithBrandFieldEnum>,
    ///<https://schema.org/publishingPrinciples>
    #[serde(rename = "publishingPrinciples")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub publishing_principles: Vec<LocksmithPublishingPrinciplesFieldEnum>,
    ///<https://schema.org/hasShippingService>
    #[serde(rename = "hasShippingService")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_shipping_service: Vec<ShippingService>,
    ///<https://schema.org/owns>
    #[serde(rename = "owns")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub owns: Vec<LocksmithOwnsFieldEnum>,
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
    pub service_area: Vec<LocksmithServiceAreaFieldEnum>,
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
    pub founder: Vec<LocksmithFounderFieldEnum>,
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
    pub address: Vec<LocksmithAddressFieldEnum>,
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
    pub ownership_funding_info: Vec<LocksmithOwnershipFundingInfoFieldEnum>,
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
    pub logo: Vec<LocksmithLogoFieldEnum>,
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
    pub additional_type: Vec<LocksmithAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<LocksmithIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<LocksmithImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<LocksmithDescriptionFieldEnum>,
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
    pub subject_of: Vec<LocksmithSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<LocksmithMainEntityOfPageFieldEnum>,
    ///<https://schema.org/containedInPlace>
    #[serde(rename = "containedInPlace")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contained_in_place: Vec<Place>,
    ///<https://schema.org/map>
    #[serde(rename = "map")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub map: Vec<String>,
    ///<https://schema.org/geoEquals>
    #[serde(rename = "geoEquals")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_equals: Vec<LocksmithGeoEqualsFieldEnum>,
    ///<https://schema.org/latitude>
    #[serde(rename = "latitude")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub latitude: Vec<LocksmithLatitudeFieldEnum>,
    ///<https://schema.org/geoDisjoint>
    #[serde(rename = "geoDisjoint")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_disjoint: Vec<LocksmithGeoDisjointFieldEnum>,
    ///<https://schema.org/tourBookingPage>
    #[serde(rename = "tourBookingPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub tour_booking_page: Vec<String>,
    ///<https://schema.org/longitude>
    #[serde(rename = "longitude")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub longitude: Vec<LocksmithLongitudeFieldEnum>,
    ///<https://schema.org/photo>
    #[serde(rename = "photo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub photo: Vec<LocksmithPhotoFieldEnum>,
    ///<https://schema.org/hasDriveThroughService>
    #[serde(rename = "hasDriveThroughService")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_drive_through_service: Vec<String>,
    ///<https://schema.org/geo>
    #[serde(rename = "geo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo: Vec<LocksmithGeoFieldEnum>,
    ///<https://schema.org/publicAccess>
    #[serde(rename = "publicAccess")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub public_access: Vec<String>,
    ///<https://schema.org/geoCovers>
    #[serde(rename = "geoCovers")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_covers: Vec<LocksmithGeoCoversFieldEnum>,
    ///<https://schema.org/specialOpeningHoursSpecification>
    #[serde(rename = "specialOpeningHoursSpecification")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub special_opening_hours_specification: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/isAccessibleForFree>
    #[serde(rename = "isAccessibleForFree")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub is_accessible_for_free: Vec<String>,
    ///<https://schema.org/amenityFeature>
    #[serde(rename = "amenityFeature")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub amenity_feature: Vec<LocationFeatureSpecification>,
    ///<https://schema.org/openingHoursSpecification>
    #[serde(rename = "openingHoursSpecification")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub opening_hours_specification: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/smokingAllowed>
    #[serde(rename = "smokingAllowed")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub smoking_allowed: Vec<String>,
    ///<https://schema.org/geoTouches>
    #[serde(rename = "geoTouches")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_touches: Vec<LocksmithGeoTouchesFieldEnum>,
    ///<https://schema.org/hasMap>
    #[serde(rename = "hasMap")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_map: Vec<LocksmithHasMapFieldEnum>,
    ///<https://schema.org/geoCrosses>
    #[serde(rename = "geoCrosses")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_crosses: Vec<LocksmithGeoCrossesFieldEnum>,
    ///<https://schema.org/geoContains>
    #[serde(rename = "geoContains")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_contains: Vec<LocksmithGeoContainsFieldEnum>,
    ///<https://schema.org/photos>
    #[serde(rename = "photos")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub photos: Vec<LocksmithPhotosFieldEnum>,
    ///<https://schema.org/maps>
    #[serde(rename = "maps")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub maps: Vec<String>,
    ///<https://schema.org/containsPlace>
    #[serde(rename = "containsPlace")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contains_place: Vec<Place>,
    ///<https://schema.org/branchCode>
    #[serde(rename = "branchCode")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub branch_code: Vec<String>,
    ///<https://schema.org/maximumAttendeeCapacity>
    #[serde(rename = "maximumAttendeeCapacity")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub maximum_attendee_capacity: Vec<i32>,
    ///<https://schema.org/geoOverlaps>
    #[serde(rename = "geoOverlaps")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_overlaps: Vec<LocksmithGeoOverlapsFieldEnum>,
    ///<https://schema.org/containedIn>
    #[serde(rename = "containedIn")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contained_in: Vec<Place>,
    ///<https://schema.org/geoIntersects>
    #[serde(rename = "geoIntersects")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_intersects: Vec<LocksmithGeoIntersectsFieldEnum>,
    ///<https://schema.org/geoWithin>
    #[serde(rename = "geoWithin")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_within: Vec<LocksmithGeoWithinFieldEnum>,
    ///<https://schema.org/additionalProperty>
    #[serde(rename = "additionalProperty")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_property: Vec<PropertyValue>,
    ///<https://schema.org/geoCoveredBy>
    #[serde(rename = "geoCoveredBy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geo_covered_by: Vec<LocksmithGeoCoveredByFieldEnum>,
}
