use schemars::JsonSchema;
use serde::Deserialize;

use crate::data_type::Date;
use crate::permutations::brand::BrandOrOrganization;
use crate::permutations::postal_address::PostalAddressOrText;
use crate::permutations::text::LanguageOrText;
use crate::Thing;
use crate::thing::{Event, Organization, Place};
use crate::thing::intangible::structured_value::ContactPoint;
use crate::thing::intangible::grant::Grant;
use crate::thing::place::Country;

/// A person (alive, dead, undead, or fictional).
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Person {
    /// An additional name for a Person, can be used for a middle name.
    pub additional_name: 	String,
    /// Physical address of the item.
    pub address: 	PostalAddressOrText,
    /// An organization that this person is affiliated with. For example, a school/university, a
    /// club, or a team.
    pub affiliation: 	Organization,
    /// The number of completed interactions for this entity, in a particular role (the 'agent'),
    /// in a particular action (indicated in the statistic), and in a particular context (i.e.
    /// interactionService).
    pub agent_interaction_statistic: 	InteractionCounter,
    /// An organization that the person is an alumni of.
    ///
    /// Inverse property: alumni
    pub alumni_of: 	EducationalOrganizationOrOrganization,
    /// An award won by or for this item. Supersedes awards.
    pub award: 	String,
    /// Date of birth.
    pub birth_date: 	Date,
    /// The place where the person was born.
    pub birth_place: 	Place,
    /// The brand(s) associated with a product or service, or the brand(s) maintained by an
    /// organization or business person.
    pub brand: 	BrandOrOrganization,
    /// A callsign, as used in broadcasting and radio communications to identify people, radio and
    /// TV stations, or vehicles.
    pub call_sign: 	String,
    /// A child of the person.
    pub children: 	Person,
    /// A colleague of the person. Supersedes colleagues.
    pub colleague: 	PersonOrURL,
    /// A contact point for a person or organization. Supersedes contactPoints.
    pub contact_point: 	ContactPoint,
    /// Date of death.
    pub death_date: 	Date,
    /// The place where the person died.
    pub death_place: 	Place,
    /// The Dun & Bradstreet DUNS number for identifying an organization or business person.
    pub duns: 	String,
    /// Email address.
    pub email: 	String,
    /// Family name. In the U.S., the last name of a Person.
    pub family_name: 	String,
    /// The fax number.
    pub fax_number: 	String,
    /// The most generic uni-directional social relation.
    pub follows: 	Person,
    /// A person or organization that supports (sponsors) something through some kind of financial
    /// contribution.
    pub funder: 	OrganizationOrPerson,
    /// A Grant that directly or indirectly provide funding or sponsorship for this item. See also
    /// ownershipFundingInfo.
    ///
    /// Inverse property: fundedItem
    pub funding: 	Grant,
    /// Gender of something, typically a Person, but possibly also fictional characters, animals,
    /// etc. While https://schema.org/Male and https://schema.org/Female may be used, text strings
    /// are also acceptable for people who are not a binary gender. The gender property can also be
    /// used in an extended sense to cover e.g. the gender of sports teams. As with the gender of
    /// individuals, we do not try to enumerate all possibilities. A mixed-gender SportsTeam can be
    /// indicated with a text value of "Mixed".
    pub gender: 	GenderTypeOrText,
    /// Given name. In the U.S., the first name of a Person.
    pub given_name: 	String,
    /// The Global Location Number (GLN, sometimes also referred to as International Location Number
    /// or ILN) of the respective organization, person, or place. The GLN is a 13-digit number used
    /// to identify parties and physical locations.
    pub global_location_number: 	String,
    /// Certification information about a product, organization, service, place, or person.
    pub has_certification: 	Certification,
    /// A credential awarded to the Person or Organization.
    pub has_credential: 	EducationalOccupationalCredential,
    /// The Person's occupation. For past professions, use Role for expressing dates.
    pub has_occupation: 	Occupation,
    /// Indicates an OfferCatalog listing for this Organization, Person, or Service.
    pub has_offer_catalog: 	OfferCatalog,
    /// Points-of-Sales operated by the organization or person.
    #[serde(rename = "hasPOS")]
    pub has_pos: 	Place,
    /// The height of the item.
    pub height: 	DistanceOrQuantitativeValue,
    /// A contact location for a person's residence.
    pub home_location: 	ContactPointOrPlace,
    /// An honorific prefix preceding a Person's name such as Dr/Mrs/Mr.
    pub honorific_prefix: 	String,
    /// An honorific suffix following a Person's name such as M.D./PhD/MSCSW.
    pub honorific_suffix: 	String,
    /// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication.
    /// The most specific child type of InteractionCounter should be used. Supersedes
    /// interactionCount.
    pub interaction_statistic: 	InteractionCounter,
    /// The International Standard of Industrial Classification of All Economic Activities (ISIC),
    /// Revision 4 code for a particular organization, business person, or place.
    pub isic_v4: 	String,
    /// The job title of the person (for example, Financial Manager).
    pub job_title: 	DefinedTermOrText,
    /// The most generic bi-directional social/work relation.
    pub knows: 	Person,
    /// Of a Person, and less typically of an Organization, to indicate a topic that is known about
    /// - suggesting possible expertise but not implying it. We do not distinguish skill levels
    /// here, or relate this to educational content, events, objectives or JobPosting descriptions.
    pub knows_about: 	TextOrThingOrURL,
    /// Of a Person, and less typically of an Organization, to indicate a known language. We do not
    /// distinguish skill levels or reading/writing/speaking/signing here. Use language codes from
    /// the IETF BCP 47 standard.
    pub knows_language: 	LanguageOrText,
    /// A pointer to products or services offered by the organization or person.
    ///
    /// Inverse property: offeredBy
    pub makes_offer: 	Offer,
    /// An Organization (or ProgramMembership) to which this Person or Organization belongs.
    ///
    /// Inverse property: member
    pub member_of: 	MemberProgramTierOrOrganizationOrProgramMembership,
    /// The North American Industry Classification System (NAICS) code for a particular organization
    /// or business person.
    pub naics: 	String,
    /// Nationality of the person.
    pub nationality: 	Country,
    /// The total financial value of the person as calculated by subtracting the total value of
    /// liabilities from the total value of assets.
    pub net_worth: 	MonetaryAmountOrPriceSpecification,
    /// Products owned by the organization or person.
    pub owns: 	OwnershipInfoOrProduct,
    /// A parent of this person. Supersedes parents.
    pub parent: 	Person,
    /// Event that this person is a performer or participant in.
    pub performer_in: 	Event,
    /// A short string listing or describing pronouns for a person. Typically the person concerned
    /// is the best authority as pronouns are a critical part of personal identity and expression.
    /// Publishers and consumers of this information are reminded to treat this data responsibly,
    /// take country-specific laws related to gender expression into account, and be wary of
    /// out-of-date data and drawing unwarranted inferences about the person being described.
    ///
    /// In English, formulations such as "they/them", "she/her", and "he/him" are commonly used
    /// online and can also be used here. We do not intend to enumerate all possible micro-syntaxes
    /// in all languages. More structured and well-defined external values for pronouns can be
    /// referenced using the StructuredValue or DefinedTerm values.
    pub pronouns: 	DefinedTermOrStructuredValueOrText,
    /// The publishingPrinciples property indicates (typically via URL) a document describing the
    /// editorial principles of an Organization (or individual, e.g. a Person writing a blog) that
    /// relate to their activities as a publisher, e.g. ethics or diversity policies. When applied
    /// to a CreativeWork (e.g. NewsArticle) the principles are those of the party primarily
    /// responsible for the creation of the CreativeWork.
    ///
    /// While such policies are most typically expressed in natural language, sometimes related
    /// information (e.g. indicating a funder) can be expressed using schema.org terminology.
    pub publishing_principles: 	CreativeWorkOrURL,
    /// The most generic familial relation.
    pub related_to: 	Person,
    /// A pointer to products or services sought by the organization or person (demand).
    pub seeks: 	Demand,
    /// A sibling of the person. Supersedes siblings.
    pub sibling: 	Person,
    /// A statement of knowledge, skill, ability, task or any other assertion expressing a
    /// competency that is either claimed by a person, an organization or desired or required to
    /// fulfill a role or to work in an occupation.
    pub skills: 	DefinedTermOrText,
    /// A person or organization that supports a thing through a pledge, promise, or financial
    /// contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
    pub sponsor: 	OrganizationOrPerson,
    /// The person's spouse.
    pub spouse: 	Person,
    /// The Tax / Fiscal ID of the organization or person, e.g. the TIN in the US or the CIF/NIF
    /// in Spain.
    #[serde(rename = "taxID")]
    pub tax_id: 	String,
    /// The telephone number.
    pub telephone: 	String,
    /// The Value-added Tax ID of the organization or person.
    #[serde(rename = "vatID")]
    pub vat_id: 	String,
    /// The weight of the product or person.
    pub weight: 	MassOrQuantitativeValue,
    /// A contact location for a person's place of work.
    pub work_location: 	ContactPointOrPlace,
    /// Organizations that the person works for.
    pub works_for: 	Organization,
    #[serde(flatten)]
    pub thing: Thing,
}
