mod performing_group;

pub use performing_group::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::data_type::Date;
use crate::data_type::text::URL;
use crate::permutations::administrative_area::AdministrativeAreaOrGeoShapeOrPlaceOrText;
use crate::permutations::brand::BrandOrOrganization;
use crate::permutations::payment::LoanOrCreditOrPaymentMethodOrText;
use crate::permutations::person::OrganizationOrPerson;
use crate::permutations::place::PlaceOrPostalAddressOrTextOrVirtualLocation;
use crate::permutations::postal_address::PostalAddressOrText;
use crate::permutations::text::LanguageOrText;
use crate::permutations::url::ImageObjectOrURL;
use crate::thing::creative_work::Review;
use crate::thing::intangible::demand::Demand;
use crate::thing::intangible::grant::Grant;
use crate::thing::intangible::rating::AggregateRating;
use crate::thing::intangible::structured_value::contact_point::PostalAddress;
use crate::thing::intangible::structured_value::{
    ContactPoint, InteractionCounter, QuantitativeValue,
};
use crate::thing::{Event, Person, Place};

/// An organization such as a school, NGO, corporation, club, etc.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Organization {
    /// The payment method(s) that are accepted in general by an organization, or for some specific
    /// demand or offer.
    pub accepted_payment_method: LoanOrCreditOrPaymentMethodOrText,
    /// For a NewsMediaOrganization or other news-related Organization, a statement about public
    /// engagement activities (for news media, the newsroom’s), including involving the public -
    /// digitally or otherwise -- in coverage decisions, reporting and activities after publication.
    pub actionable_feedback_policy: CreativeWorkOrURL,
    /// Physical address of the item.
    pub address: PostalAddressOrText,
    /// The number of completed interactions for this entity, in a particular role (the 'agent'), in
    /// a particular action (indicated in the statistic), and in a particular context
    /// (i.e. interactionService).
    pub agent_interaction_statistic: InteractionCounter,
    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating: AggregateRating,
    /// Alumni of an organization.
    ///
    /// Inverse property: alumniOf
    pub alumni: Person,
    /// The geographic area where a service or offered item is provided. Supersedes serviceArea.
    pub area_served: AdministrativeAreaOrGeoShapeOrPlaceOrText,
    /// An award won by or for this item. Supersedes awards.
    pub award: String,
    /// The brand(s) associated with a product or service, or the brand(s) maintained by an
    /// organization or business person.
    pub brand: BrandOrOrganization,
    /// The official registration number of a business including the organization that issued it
    /// such as Company House or Chamber of Commerce.
    pub company_registration: Certification,
    /// A contact point for a person or organization. Supersedes contactPoints.
    pub contact_point: ContactPoint,
    /// For an Organization (e.g. NewsMediaOrganization), a statement describing (in news media,
    /// the newsroom’s) disclosure and correction policy for errors.
    pub corrections_policy: CreativeWorkOrURL,
    /// A relationship between an organization and a department of that organization, also described
    /// as an organization (allowing different urls, logos, opening hours). For example: a store
    /// with a pharmacy, or a bakery with a cafe.
    pub department: Organization,
    /// The date that this organization was dissolved.
    pub dissolution_date: Date,
    /// Statement on diversity policy by an Organization e.g. a NewsMediaOrganization. For a
    /// NewsMediaOrganization, a statement describing the newsroom’s diversity policy on both
    /// staffing and sources, typically providing staffing data.
    pub diversity_policy: CreativeWorkOrURL,
    /// For an Organization (often but not necessarily a NewsMediaOrganization), a report on
    /// staffing diversity issues. In a news context this might be for example ASNE or RTDNA (US)
    /// reports, or self-reported.
    pub diversity_staffing_report: ArticleOrURL,
    /// The Dun & Bradstreet DUNS number for identifying an organization or business person.
    pub duns: String,
    /// Email address.
    pub email: String,
    /// Someone working for this organization. Supersedes employees.
    pub employee: Person,
    /// Statement about ethics policy, e.g. of a NewsMediaOrganization regarding journalistic and
    /// publishing practices, or of a Restaurant, a page describing food source policies. In the
    /// case of a NewsMediaOrganization, an ethicsPolicy is typically a statement describing the
    /// personal, organizational, and corporate standards of behavior expected by the organization.
    pub ethics_policy: CreativeWorkOrURL,
    /// Upcoming or past event associated with this place, organization, or action. Supersedes
    /// events.
    pub event: Event,
    /// The fax number.
    pub fax_number: String,
    /// A person or organization who founded this organization. Supersedes founders.
    pub founder: OrganizationOrPerson,
    /// The date that this organization was founded.
    pub founding_date: Date,
    /// The place where the Organization was founded.
    pub founding_location: Place,
    /// A person or organization that supports (sponsors) something through some kind of financial
    /// contribution.
    pub funder: OrganizationOrPerson,
    /// A Grant that directly or indirectly provide funding or sponsorship for this item.
    /// See also ownershipFundingInfo.
    ///
    /// Inverse property: fundedItem
    pub funding: Grant,
    /// The Global Location Number (GLN, sometimes also referred to as International Location Number
    /// or ILN) of the respective organization, person, or place. The GLN is a 13-digit number used
    /// to identify parties and physical locations.
    pub global_location_number: String,
    /// Certification information about a product, organization, service, place, or person.
    pub has_certification: Certification,
    /// A credential awarded to the Person or Organization.
    pub has_credential: EducationalOccupationalCredential,
    /// The GS1 digital link associated with the object. This URL should conform to the particular
    /// requirements of digital links. The link should only contain the Application Identifiers
    /// (AIs) that are relevant for the entity being annotated, for instance a Product or an
    /// Organization, and for the correct granularity. In particular, for products:
    ///
    /// A Digital Link that contains a serial number (AI 21) should only be present on instances of
    /// IndividualProduct
    /// - A Digital Link that contains a lot number (AI 10) should be annotated as SomeProduct if
    /// only products from that lot are sold, or IndividualProduct if there is only a specific
    /// product.
    /// - A Digital Link that contains a global model number (AI 8013) should be attached to a
    /// Product or a ProductModel.
    /// - Other item types should be adapted similarly.
    #[serde(rename = "hasGS1DigitalLink")]
    pub has_gs1_digital_link: URL,
    /// MemberProgram offered by an Organization, for example an eCommerce merchant or an airline.
    pub has_member_program: MemberProgram,
    /// Specifies a MerchantReturnPolicy that may be applicable. Supersedes hasProductReturnPolicy.
    pub has_merchant_return_policy: MerchantReturnPolicy,
    /// Indicates an OfferCatalog listing for this Organization, Person, or Service.
    pub has_offer_catalog: OfferCatalog,
    /// Points-of-Sales operated by the organization or person.
    #[serde(rename = "hasPOS")]
    pub has_pos: Place,
    /// Specification of a shipping service offered by the organization.
    pub has_shipping_service: ShippingService,
    /// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication.
    /// The most specific child type of InteractionCounter should be used. Supersedes
    /// interactionCount.
    pub interaction_statistic: InteractionCounter,
    /// The International Standard of Industrial Classification of All Economic Activities (ISIC),
    /// Revision 4 code for a particular organization, business person, or place.
    pub isic_v4: String,
    /// An organization identifier as defined in ISO 6523(-1). The identifier should be in the
    /// XXXX:YYYYYY:ZZZ or XXXX:YYYYYYformat. Where XXXX is a 4 digit ICD (International Code
    /// Designator), YYYYYY is an OID (Organization Identifier) with all formatting characters
    /// (dots, dashes, spaces) removed with a maximal length of 35 characters, and ZZZ is an
    /// optional OPI (Organization Part Identifier) with a maximum length of 35 characters. The
    /// various components (ICD, OID, OPI) are joined with a colon character (ASCII 0x3a). Note that
    /// many existing organization identifiers defined as attributes like leiCode (0199), duns
    /// (0060) or GLN (0088) can be expressed using ISO-6523. If possible, ISO-6523 codes should be
    /// preferred to populating vatID or taxID, as ISO identifiers are less ambiguous.
    pub iso6523_code: String,
    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are
    /// typically delimited by commas, or by repeating the property.
    pub keywords: DefinedTermOrTextOrURL,
    /// Of a Person, and less typically of an Organization, to indicate a topic that is known
    /// about - suggesting possible expertise but not implying it. We do not distinguish skill
    /// levels here, or relate this to educational content, events, objectives or JobPosting
    /// descriptions.
    pub knows_about: TextOrThingOrURL,
    /// Of a Person, and less typically of an Organization, to indicate a known language. We do not
    /// distinguish skill levels or reading/writing/speaking/signing here. Use language codes from
    /// the IETF BCP 47 standard.
    pub knows_language: LanguageOrText,
    /// The legal address of an organization which acts as the officially registered address used
    /// for legal and tax purposes. The legal address can be different from the place of operations
    /// of a business and other addresses can be part of an organization.
    pub legal_address: PostalAddress,
    /// The official name of the organization, e.g. the registered company name.
    pub legal_name: String,
    /// One or multiple persons who represent this organization legally such as CEO or sole
    /// administrator.
    pub legal_representative: Person,
    /// An organization identifier that uniquely identifies a legal entity as defined in ISO 17442.
    pub lei_code: String,
    /// The location of, for example, where an event is happening, where an organization is located,
    /// or where an action takes place.
    pub location: PlaceOrPostalAddressOrTextOrVirtualLocation,
    /// An associated logo.
    pub logo: ImageObjectOrURL,
    /// A pointer to products or services offered by the organization or person.
    ///
    /// Inverse property: offeredBy
    pub makes_offer: Offer,
    /// A member of an Organization or a ProgramMembership. Organizations can be members of
    /// organizations; ProgramMembership is typically for individuals. Supersedes musicGroupMember,
    /// members.
    ///
    /// Inverse property: memberOf
    pub member: OrganizationOrPerson,
    /// An Organization (or ProgramMembership) to which this Person or Organization belongs.
    ///
    /// Inverse property: member
    pub member_of: MemberProgramTierOrOrganizationOrProgramMembership,
    /// The North American Industry Classification System (NAICS) code for a particular organization
    /// or business person.
    pub naics: String,
    /// nonprofitStatus indicates the legal status of a non-profit organization in its primary place
    /// of business.
    pub nonprofit_status: NonprofitType,
    /// The number of employees in an organization, e.g. business.
    pub number_of_employees: QuantitativeValue,
    /// For an Organization (often but not necessarily a NewsMediaOrganization), a description of
    /// organizational ownership structure; funding and grants. In a news/media setting, this is
    /// with particular reference to editorial independence. Note that the funder is also available
    /// and can be used to make basic funder information machine-readable.
    pub ownership_funding_info: AboutPageOrCreativeWorkOrTextOrURL,
    /// Products owned by the organization or person.
    pub owns: OwnershipInfoOrProduct,
    /// The larger organization that this organization is a subOrganization of, if any. Supersedes
    /// branchOf.
    ///
    /// Inverse property: subOrganization
    pub parent_organization: Organization,
    /// The publishingPrinciples property indicates (typically via URL) a document describing the
    /// editorial principles of an Organization (or individual, e.g. a Person writing a blog) that
    /// relate to their activities as a publisher, e.g. ethics or diversity policies. When applied
    /// to a CreativeWork (e.g. NewsArticle) the principles are those of the party primarily
    /// responsible for the creation of the CreativeWork.
    ///
    /// While such policies are most typically expressed in natural language, sometimes related
    /// information (e.g. indicating a funder) can be expressed using schema.org terminology.
    pub publishing_principles: CreativeWorkOrURL,
    /// A review of the item. Supersedes reviews.
    pub review: Review,
    /// A pointer to products or services sought by the organization or person (demand).
    pub seeks: Demand,
    /// A statement of knowledge, skill, ability, task or any other assertion expressing a
    /// competency that is either claimed by a person, an organization or desired or required to
    /// fulfill a role or to work in an occupation.
    pub skills: DefinedTermOrText,
    /// A slogan or motto associated with the item.
    pub slogan: String,
    /// A person or organization that supports a thing through a pledge, promise, or financial
    /// contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
    pub sponsor: OrganizationOrPerson,
    /// A relationship between two organizations where the first includes the second, e.g., as a
    /// subsidiary. See also: the more specific 'department' property.
    ///
    /// Inverse property: parentOrganization
    pub sub_organization: Organization,
    /// The Tax / Fiscal ID of the organization or person, e.g. the TIN in the US or the CIF/NIF
    /// in Spain.
    #[serde(rename = "taxID")]
    pub tax_id: String,
    /// The telephone number.
    pub telephone: String,
    /// For an Organization (typically a NewsMediaOrganization), a statement about policy on use of
    /// unnamed sources and the decision process required.
    pub unnamed_sources_policy: CreativeWorkOrURL,
    /// The Value-added Tax ID of the organization or person.
    #[serde(rename = "vatID")]
    pub vat_id: String,
    #[serde(flatten)]
    pub thing: Thing,
}
