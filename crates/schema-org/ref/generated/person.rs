use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type PersonAdditionalTypeFieldEnum = String;
///<https://schema.org/Person>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Person {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/knowsAbout>
    #[serde(rename = "knowsAbout")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub knows_about: Vec<PersonKnowsAboutFieldEnum>,
    ///<https://schema.org/performerIn>
    #[serde(rename = "performerIn")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub performer_in: Vec<Event>,
    ///<https://schema.org/jobTitle>
    #[serde(rename = "jobTitle")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub job_title: Vec<PersonJobTitleFieldEnum>,
    ///<https://schema.org/contactPoint>
    #[serde(rename = "contactPoint")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contact_point: Vec<ContactPoint>,
    ///<https://schema.org/pronouns>
    #[serde(rename = "pronouns")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub pronouns: Vec<PersonPronounsFieldEnum>,
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
    pub member_of: Vec<PersonMemberOfFieldEnum>,
    ///<https://schema.org/knowsLanguage>
    #[serde(rename = "knowsLanguage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub knows_language: Vec<PersonKnowsLanguageFieldEnum>,
    ///<https://schema.org/skills>
    #[serde(rename = "skills")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub skills: Vec<PersonSkillsFieldEnum>,
    ///<https://schema.org/deathPlace>
    #[serde(rename = "deathPlace")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub death_place: Vec<Place>,
    ///<https://schema.org/deathDate>
    #[serde(rename = "deathDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub death_date: Vec<String>,
    ///<https://schema.org/interactionStatistic>
    #[serde(rename = "interactionStatistic")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/agentInteractionStatistic>
    #[serde(rename = "agentInteractionStatistic")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub agent_interaction_statistic: Vec<InteractionCounter>,
    ///<https://schema.org/worksFor>
    #[serde(rename = "worksFor")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub works_for: Vec<Organization>,
    ///<https://schema.org/additionalName>
    #[serde(rename = "additionalName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_name: Vec<String>,
    ///<https://schema.org/callSign>
    #[serde(rename = "callSign")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub call_sign: Vec<String>,
    ///<https://schema.org/nationality>
    #[serde(rename = "nationality")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub nationality: Vec<Country>,
    ///<https://schema.org/affiliation>
    #[serde(rename = "affiliation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub affiliation: Vec<Organization>,
    ///<https://schema.org/height>
    #[serde(rename = "height")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub height: Vec<PersonHeightFieldEnum>,
    ///<https://schema.org/siblings>
    #[serde(rename = "siblings")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub siblings: Vec<Person>,
    ///<https://schema.org/naics>
    #[serde(rename = "naics")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub naics: Vec<String>,
    ///<https://schema.org/workLocation>
    #[serde(rename = "workLocation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub work_location: Vec<PersonWorkLocationFieldEnum>,
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
    ///<https://schema.org/homeLocation>
    #[serde(rename = "homeLocation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub home_location: Vec<PersonHomeLocationFieldEnum>,
    ///<https://schema.org/makesOffer>
    #[serde(rename = "makesOffer")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub makes_offer: Vec<Offer>,
    ///<https://schema.org/funder>
    #[serde(rename = "funder")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub funder: Vec<PersonFunderFieldEnum>,
    ///<https://schema.org/givenName>
    #[serde(rename = "givenName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub given_name: Vec<String>,
    ///<https://schema.org/vatID>
    #[serde(rename = "vatID")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub vat_id: Vec<String>,
    ///<https://schema.org/knows>
    #[serde(rename = "knows")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub knows: Vec<Person>,
    ///<https://schema.org/telephone>
    #[serde(rename = "telephone")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub telephone: Vec<String>,
    ///<https://schema.org/award>
    #[serde(rename = "award")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub award: Vec<String>,
    ///<https://schema.org/children>
    #[serde(rename = "children")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub children: Vec<Person>,
    ///<https://schema.org/weight>
    #[serde(rename = "weight")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub weight: Vec<PersonWeightFieldEnum>,
    ///<https://schema.org/birthDate>
    #[serde(rename = "birthDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub birth_date: Vec<String>,
    ///<https://schema.org/sponsor>
    #[serde(rename = "sponsor")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub sponsor: Vec<PersonSponsorFieldEnum>,
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
    ///<https://schema.org/colleagues>
    #[serde(rename = "colleagues")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub colleagues: Vec<Person>,
    ///<https://schema.org/gender>
    #[serde(rename = "gender")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub gender: Vec<PersonGenderFieldEnum>,
    ///<https://schema.org/netWorth>
    #[serde(rename = "netWorth")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub net_worth: Vec<PersonNetWorthFieldEnum>,
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
    ///<https://schema.org/birthPlace>
    #[serde(rename = "birthPlace")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub birth_place: Vec<Place>,
    ///<https://schema.org/seeks>
    #[serde(rename = "seeks")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub seeks: Vec<Demand>,
    ///<https://schema.org/familyName>
    #[serde(rename = "familyName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub family_name: Vec<String>,
    ///<https://schema.org/hasOfferCatalog>
    #[serde(rename = "hasOfferCatalog")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_offer_catalog: Vec<OfferCatalog>,
    ///<https://schema.org/hasOccupation>
    #[serde(rename = "hasOccupation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_occupation: Vec<Occupation>,
    ///<https://schema.org/brand>
    #[serde(rename = "brand")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub brand: Vec<PersonBrandFieldEnum>,
    ///<https://schema.org/publishingPrinciples>
    #[serde(rename = "publishingPrinciples")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub publishing_principles: Vec<PersonPublishingPrinciplesFieldEnum>,
    ///<https://schema.org/parent>
    #[serde(rename = "parent")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub parent: Vec<Person>,
    ///<https://schema.org/sibling>
    #[serde(rename = "sibling")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub sibling: Vec<Person>,
    ///<https://schema.org/follows>
    #[serde(rename = "follows")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub follows: Vec<Person>,
    ///<https://schema.org/relatedTo>
    #[serde(rename = "relatedTo")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub related_to: Vec<Person>,
    ///<https://schema.org/owns>
    #[serde(rename = "owns")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub owns: Vec<PersonOwnsFieldEnum>,
    ///<https://schema.org/honorificPrefix>
    #[serde(rename = "honorificPrefix")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub honorific_prefix: Vec<String>,
    ///<https://schema.org/awards>
    #[serde(rename = "awards")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub awards: Vec<String>,
    ///<https://schema.org/spouse>
    #[serde(rename = "spouse")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub spouse: Vec<Person>,
    ///<https://schema.org/parents>
    #[serde(rename = "parents")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub parents: Vec<Person>,
    ///<https://schema.org/hasCredential>
    #[serde(rename = "hasCredential")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_credential: Vec<EducationalOccupationalCredential>,
    ///<https://schema.org/alumniOf>
    #[serde(rename = "alumniOf")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub alumni_of: Vec<PersonAlumniOfFieldEnum>,
    ///<https://schema.org/taxID>
    #[serde(rename = "taxID")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub tax_id: Vec<String>,
    ///<https://schema.org/address>
    #[serde(rename = "address")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub address: Vec<PersonAddressFieldEnum>,
    ///<https://schema.org/colleague>
    #[serde(rename = "colleague")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub colleague: Vec<PersonColleagueFieldEnum>,
    ///<https://schema.org/honorificSuffix>
    #[serde(rename = "honorificSuffix")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub honorific_suffix: Vec<String>,
    ///<https://schema.org/email>
    #[serde(rename = "email")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub email: Vec<String>,
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
    pub additional_type: Vec<PersonAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<PersonIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<PersonImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<PersonDescriptionFieldEnum>,
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
    pub subject_of: Vec<PersonSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<PersonMainEntityOfPageFieldEnum>,
}
