use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::field::*;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type PatientAdditionalTypeFieldEnum = String;
///<https://schema.org/Patient>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Patient {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/drug>
    #[serde(rename = "drug")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub drug: Vec<Drug>,
    ///<https://schema.org/healthCondition>
    #[serde(rename = "healthCondition")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub health_condition: Vec<MedicalCondition>,
    ///<https://schema.org/diagnosis>
    #[serde(rename = "diagnosis")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<MedicalCondition>,
    ///<https://schema.org/knowsAbout>
    #[serde(rename = "knowsAbout")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub knows_about: Vec<PatientKnowsAboutFieldEnum>,
    ///<https://schema.org/performerIn>
    #[serde(rename = "performerIn")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub performer_in: Vec<Event>,
    ///<https://schema.org/jobTitle>
    #[serde(rename = "jobTitle")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub job_title: Vec<PatientJobTitleFieldEnum>,
    ///<https://schema.org/contactPoint>
    #[serde(rename = "contactPoint")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_point: Vec<ContactPoint>,
    ///<https://schema.org/pronouns>
    #[serde(rename = "pronouns")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pronouns: Vec<PatientPronounsFieldEnum>,
    ///<https://schema.org/faxNumber>
    #[serde(rename = "faxNumber")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fax_number: Vec<String>,
    ///<https://schema.org/duns>
    #[serde(rename = "duns")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub duns: Vec<String>,
    ///<https://schema.org/memberOf>
    #[serde(rename = "memberOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub member_of: Vec<PatientMemberOfFieldEnum>,
    ///<https://schema.org/knowsLanguage>
    #[serde(rename = "knowsLanguage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub knows_language: Vec<PatientKnowsLanguageFieldEnum>,
    ///<https://schema.org/skills>
    #[serde(rename = "skills")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<PatientSkillsFieldEnum>,
    ///<https://schema.org/deathPlace>
    #[serde(rename = "deathPlace")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub death_place: Vec<Place>,
    ///<https://schema.org/deathDate>
    #[serde(rename = "deathDate")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub death_date: Vec<String>,
    ///<https://schema.org/interactionStatistic>
    #[serde(rename = "interactionStatistic")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/agentInteractionStatistic>
    #[serde(rename = "agentInteractionStatistic")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub agent_interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/worksFor>
    #[serde(rename = "worksFor")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub works_for: Vec<Organization>,
    ///<https://schema.org/additionalName>
    #[serde(rename = "additionalName")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_name: Vec<String>,
    ///<https://schema.org/callSign>
    #[serde(rename = "callSign")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub call_sign: Vec<String>,
    ///<https://schema.org/nationality>
    #[serde(rename = "nationality")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nationality: Vec<Country>,
    ///<https://schema.org/affiliation>
    #[serde(rename = "affiliation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub affiliation: Vec<Organization>,
    ///<https://schema.org/height>
    #[serde(rename = "height")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub height: Vec<PatientHeightFieldEnum>,
    ///<https://schema.org/siblings>
    #[serde(rename = "siblings")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub siblings: Vec<Person>,
    ///<https://schema.org/naics>
    #[serde(rename = "naics")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub naics: Vec<String>,
    ///<https://schema.org/workLocation>
    #[serde(rename = "workLocation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub work_location: Vec<PatientWorkLocationFieldEnum>,
    ///<https://schema.org/globalLocationNumber>
    #[serde(rename = "globalLocationNumber")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub global_location_number: Vec<String>,
    ///<https://schema.org/contactPoints>
    #[serde(rename = "contactPoints")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_points: Vec<ContactPoint>,
    ///<https://schema.org/homeLocation>
    #[serde(rename = "homeLocation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub home_location: Vec<PatientHomeLocationFieldEnum>,
    ///<https://schema.org/makesOffer>
    #[serde(rename = "makesOffer")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub makes_offer: Vec<Offer>,
    ///<https://schema.org/funder>
    #[serde(rename = "funder")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funder: Vec<PatientFunderFieldEnum>,
    ///<https://schema.org/givenName>
    #[serde(rename = "givenName")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub given_name: Vec<String>,
    ///<https://schema.org/vatID>
    #[serde(rename = "vatID")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vat_id: Vec<String>,
    ///<https://schema.org/knows>
    #[serde(rename = "knows")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub knows: Vec<Person>,
    ///<https://schema.org/telephone>
    #[serde(rename = "telephone")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub telephone: Vec<String>,
    ///<https://schema.org/award>
    #[serde(rename = "award")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub award: Vec<String>,
    ///<https://schema.org/children>
    #[serde(rename = "children")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Person>,
    ///<https://schema.org/weight>
    #[serde(rename = "weight")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub weight: Vec<PatientWeightFieldEnum>,
    ///<https://schema.org/birthDate>
    #[serde(rename = "birthDate")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub birth_date: Vec<String>,
    ///<https://schema.org/sponsor>
    #[serde(rename = "sponsor")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sponsor: Vec<PatientSponsorFieldEnum>,
    ///<https://schema.org/hasPOS>
    #[serde(rename = "hasPOS")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_pos: Vec<Place>,
    ///<https://schema.org/isicV4>
    #[serde(rename = "isicV4")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub isic_v4: Vec<String>,
    ///<https://schema.org/colleagues>
    #[serde(rename = "colleagues")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub colleagues: Vec<Person>,
    ///<https://schema.org/gender>
    #[serde(rename = "gender")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gender: Vec<PatientGenderFieldEnum>,
    ///<https://schema.org/netWorth>
    #[serde(rename = "netWorth")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub net_worth: Vec<PatientNetWorthFieldEnum>,
    ///<https://schema.org/hasCertification>
    #[serde(rename = "hasCertification")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_certification: Vec<Certification>,
    ///<https://schema.org/funding>
    #[serde(rename = "funding")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funding: Vec<Grant>,
    ///<https://schema.org/birthPlace>
    #[serde(rename = "birthPlace")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub birth_place: Vec<Place>,
    ///<https://schema.org/seeks>
    #[serde(rename = "seeks")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub seeks: Vec<Demand>,
    ///<https://schema.org/familyName>
    #[serde(rename = "familyName")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub family_name: Vec<String>,
    ///<https://schema.org/hasOfferCatalog>
    #[serde(rename = "hasOfferCatalog")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_offer_catalog: Vec<OfferCatalog>,
    ///<https://schema.org/hasOccupation>
    #[serde(rename = "hasOccupation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_occupation: Vec<Occupation>,
    ///<https://schema.org/brand>
    #[serde(rename = "brand")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub brand: Vec<PatientBrandFieldEnum>,
    ///<https://schema.org/publishingPrinciples>
    #[serde(rename = "publishingPrinciples")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub publishing_principles: Vec<PatientPublishingPrinciplesFieldEnum>,
    ///<https://schema.org/parent>
    #[serde(rename = "parent")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parent: Vec<Person>,
    ///<https://schema.org/sibling>
    #[serde(rename = "sibling")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sibling: Vec<Person>,
    ///<https://schema.org/follows>
    #[serde(rename = "follows")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub follows: Vec<Person>,
    ///<https://schema.org/relatedTo>
    #[serde(rename = "relatedTo")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub related_to: Vec<Person>,
    ///<https://schema.org/owns>
    #[serde(rename = "owns")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub owns: Vec<PatientOwnsFieldEnum>,
    ///<https://schema.org/honorificPrefix>
    #[serde(rename = "honorificPrefix")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub honorific_prefix: Vec<String>,
    ///<https://schema.org/awards>
    #[serde(rename = "awards")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub awards: Vec<String>,
    ///<https://schema.org/spouse>
    #[serde(rename = "spouse")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub spouse: Vec<Person>,
    ///<https://schema.org/parents>
    #[serde(rename = "parents")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<Person>,
    ///<https://schema.org/hasCredential>
    #[serde(rename = "hasCredential")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_credential: Vec<EducationalOccupationalCredential>,
    ///<https://schema.org/alumniOf>
    #[serde(rename = "alumniOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alumni_of: Vec<PatientAlumniOfFieldEnum>,
    ///<https://schema.org/taxID>
    #[serde(rename = "taxID")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tax_id: Vec<String>,
    ///<https://schema.org/address>
    #[serde(rename = "address")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub address: Vec<PatientAddressFieldEnum>,
    ///<https://schema.org/colleague>
    #[serde(rename = "colleague")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub colleague: Vec<PatientColleagueFieldEnum>,
    ///<https://schema.org/honorificSuffix>
    #[serde(rename = "honorificSuffix")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub honorific_suffix: Vec<String>,
    ///<https://schema.org/email>
    #[serde(rename = "email")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub email: Vec<String>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(rename = "potentialAction")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(rename = "additionalType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<PatientAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<PatientIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<PatientImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<PatientDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<PatientSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<PatientMainEntityOfPageFieldEnum>,
    ///<https://schema.org/requiredMinAge>
    #[serde(rename = "requiredMinAge")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required_min_age: Vec<i32>,
    ///<https://schema.org/requiredMaxAge>
    #[serde(rename = "requiredMaxAge")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required_max_age: Vec<i32>,
    ///<https://schema.org/suggestedMeasurement>
    #[serde(rename = "suggestedMeasurement")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggested_measurement: Vec<QuantitativeValue>,
    ///<https://schema.org/suggestedAge>
    #[serde(rename = "suggestedAge")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggested_age: Vec<QuantitativeValue>,
    ///<https://schema.org/suggestedMaxAge>
    #[serde(rename = "suggestedMaxAge")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggested_max_age: Vec<f32>,
    ///<https://schema.org/suggestedGender>
    #[serde(rename = "suggestedGender")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggested_gender: Vec<PatientSuggestedGenderFieldEnum>,
    ///<https://schema.org/requiredGender>
    #[serde(rename = "requiredGender")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required_gender: Vec<String>,
    ///<https://schema.org/suggestedMinAge>
    #[serde(rename = "suggestedMinAge")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggested_min_age: Vec<f32>,
    ///<https://schema.org/geographicArea>
    #[serde(rename = "geographicArea")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub geographic_area: Vec<AdministrativeArea>,
    ///<https://schema.org/audienceType>
    #[serde(rename = "audienceType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audience_type: Vec<String>,
}
