use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contact_point::ContactPoint;
use crate::helpers::one_or_many;
use crate::field::*;
use crate::interaction_counter::InteractionCounter;
use crate::member_program::MemberProgram;
use crate::music_recording::MusicRecording;
use crate::organization::Organization;
use crate::person::Person;
use crate::quantitative_value::QuantitativeValue;
use crate::{Action, Event, Place, Review};
use crate::aggregate_rating::AggregateRating;
use crate::certification::Certification;
use crate::demand::Demand;
use crate::educational_occupational_credential::EducationalOccupationalCredential;
use crate::grant::Grant;
use crate::merchant_return_policy::MerchantReturnPolicy;
use crate::music_album::MusicAlbum;
use crate::nonprofit_type::NonprofitType;
use crate::offer::Offer;
use crate::offer_catalog::OfferCatalog;
use crate::postal_address::PostalAddress;
use crate::shipping_service::ShippingService;

///<https://schema.org/genre>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MusicGroupGenreFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type MusicGroupAdditionalTypeFieldEnum = String;

///<https://schema.org/MusicGroup>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MusicGroup {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/track>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub track: Vec<MusicGroupTrackFieldEnum>,
    ///<https://schema.org/album>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub album: Vec<MusicAlbum>,
    ///<https://schema.org/albums>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub albums: Vec<MusicAlbum>,
    ///<https://schema.org/musicGroupMember>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub music_group_member: Vec<Person>,
    ///<https://schema.org/genre>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub genre: Vec<MusicGroupGenreFieldEnum>,
    ///<https://schema.org/tracks>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tracks: Vec<MusicRecording>,
    ///<https://schema.org/hasMemberProgram>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_member_program: Vec<MemberProgram>,
    ///<https://schema.org/department>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub department: Vec<Organization>,
    ///<https://schema.org/employees>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub employees: Vec<Person>,
    ///<https://schema.org/numberOfEmployees>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub number_of_employees: Vec<QuantitativeValue>,
    ///<https://schema.org/knowsAbout>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub knows_about: Vec<MusicGroupKnowsAboutFieldEnum>,
    ///<https://schema.org/diversityStaffingReport>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub diversity_staffing_report: Vec<MusicGroupDiversityStaffingReportFieldEnum>,
    ///<https://schema.org/location>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<MusicGroupLocationFieldEnum>,
    ///<https://schema.org/contactPoint>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_point: Vec<ContactPoint>,
    ///<https://schema.org/faxNumber>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fax_number: Vec<String>,
    ///<https://schema.org/duns>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub duns: Vec<String>,
    ///<https://schema.org/memberOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub member_of: Vec<MusicGroupMemberOfFieldEnum>,
    ///<https://schema.org/knowsLanguage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub knows_language: Vec<MusicGroupKnowsLanguageFieldEnum>,
    ///<https://schema.org/review>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub review: Vec<Review>,
    ///<https://schema.org/diversityPolicy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub diversity_policy: Vec<MusicGroupDiversityPolicyFieldEnum>,
    ///<https://schema.org/skills>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<MusicGroupSkillsFieldEnum>,
    ///<https://schema.org/interactionStatistic>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/iso6523Code>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub iso6523_code: Vec<String>,
    ///<https://schema.org/agentInteractionStatistic>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub agent_interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/areaServed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub area_served: Vec<MusicGroupAreaServedFieldEnum>,
    ///<https://schema.org/correctionsPolicy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub corrections_policy: Vec<MusicGroupCorrectionsPolicyFieldEnum>,
    ///<https://schema.org/leiCode>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lei_code: Vec<String>,
    ///<https://schema.org/foundingLocation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub founding_location: Vec<Place>,
    ///<https://schema.org/legalName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub legal_name: Vec<String>,
    ///<https://schema.org/keywords>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<MusicGroupKeywordsFieldEnum>,
    ///<https://schema.org/naics>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub naics: Vec<String>,
    ///<https://schema.org/legalAddress>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub legal_address: Vec<PostalAddress>,
    ///<https://schema.org/globalLocationNumber>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub global_location_number: Vec<String>,
    ///<https://schema.org/contactPoints>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_points: Vec<ContactPoint>,
    ///<https://schema.org/makesOffer>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub makes_offer: Vec<Offer>,
    ///<https://schema.org/funder>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funder: Vec<MusicGroupFunderFieldEnum>,
    ///<https://schema.org/events>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<Event>,
    ///<https://schema.org/acceptedPaymentMethod>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accepted_payment_method: Vec<MusicGroupAcceptedPaymentMethodFieldEnum>,
    ///<https://schema.org/vatID>
    #[serde(rename = "vatID")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vat_id: Vec<String>,
    ///<https://schema.org/member>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MusicGroupMemberFieldEnum>,
    ///<https://schema.org/telephone>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub telephone: Vec<String>,
    ///<https://schema.org/unnamedSourcesPolicy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unnamed_sources_policy: Vec<MusicGroupUnnamedSourcesPolicyFieldEnum>,
    ///<https://schema.org/slogan>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub slogan: Vec<String>,
    ///<https://schema.org/award>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub award: Vec<String>,
    ///<https://schema.org/alumni>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alumni: Vec<Person>,
    ///<https://schema.org/sponsor>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sponsor: Vec<MusicGroupSponsorFieldEnum>,
    ///<https://schema.org/aggregateRating>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub aggregate_rating: Vec<AggregateRating>,
    ///<https://schema.org/legalRepresentative>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub legal_representative: Vec<Person>,
    ///<https://schema.org/hasGS1DigitalLink>
    #[serde(rename = "hasGS1DigitalLink")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_gs1_digital_link: Vec<String>,
    ///<https://schema.org/hasPOS>
    #[serde(rename = "hasPOS")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_pos: Vec<Place>,
    ///<https://schema.org/isicV4>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub isic_v4: Vec<String>,
    ///<https://schema.org/actionableFeedbackPolicy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actionable_feedback_policy: Vec<MusicGroupActionableFeedbackPolicyFieldEnum>,
    ///<https://schema.org/members>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<MusicGroupMembersFieldEnum>,
    ///<https://schema.org/parentOrganization>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parent_organization: Vec<Organization>,
    ///<https://schema.org/hasCertification>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_certification: Vec<Certification>,
    ///<https://schema.org/funding>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funding: Vec<Grant>,
    ///<https://schema.org/seeks>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub seeks: Vec<Demand>,
    ///<https://schema.org/ethicsPolicy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ethics_policy: Vec<MusicGroupEthicsPolicyFieldEnum>,
    ///<https://schema.org/hasOfferCatalog>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_offer_catalog: Vec<OfferCatalog>,
    ///<https://schema.org/brand>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub brand: Vec<MusicGroupBrandFieldEnum>,
    ///<https://schema.org/publishingPrinciples>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publishing_principles: Vec<MusicGroupPublishingPrinciplesFieldEnum>,
    ///<https://schema.org/hasShippingService>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_shipping_service: Vec<ShippingService>,
    ///<https://schema.org/owns>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub owns: Vec<MusicGroupOwnsFieldEnum>,
    ///<https://schema.org/awards>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub awards: Vec<String>,
    ///<https://schema.org/nonprofitStatus>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nonprofit_status: Vec<NonprofitType>,
    ///<https://schema.org/serviceArea>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service_area: Vec<MusicGroupServiceAreaFieldEnum>,
    ///<https://schema.org/foundingDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub founding_date: Vec<String>,
    ///<https://schema.org/dissolutionDate>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dissolution_date: Vec<String>,
    ///<https://schema.org/hasCredential>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_credential: Vec<EducationalOccupationalCredential>,
    ///<https://schema.org/founder>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub founder: Vec<MusicGroupFounderFieldEnum>,
    ///<https://schema.org/taxID>
    #[serde(rename = "taxID")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tax_id: Vec<String>,
    ///<https://schema.org/event>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<Event>,
    ///<https://schema.org/address>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub address: Vec<MusicGroupAddressFieldEnum>,
    ///<https://schema.org/hasMerchantReturnPolicy>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_merchant_return_policy: Vec<MerchantReturnPolicy>,
    ///<https://schema.org/employee>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub employee: Vec<Person>,
    ///<https://schema.org/email>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub email: Vec<String>,
    ///<https://schema.org/ownershipFundingInfo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ownership_funding_info: Vec<MusicGroupOwnershipFundingInfoFieldEnum>,
    ///<https://schema.org/subOrganization>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sub_organization: Vec<Organization>,
    ///<https://schema.org/companyRegistration>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub company_registration: Vec<Certification>,
    ///<https://schema.org/logo>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub logo: Vec<MusicGroupLogoFieldEnum>,
    ///<https://schema.org/reviews>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reviews: Vec<Review>,
    ///<https://schema.org/founders>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub founders: Vec<Person>,
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
    pub additional_type: Vec<MusicGroupAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<MusicGroupIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<MusicGroupImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<MusicGroupDescriptionFieldEnum>,
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
    pub subject_of: Vec<MusicGroupSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<MusicGroupMainEntityOfPageFieldEnum>,
}
