use schemars::JsonSchema;
use serde::Deserialize;

use crate::permutations::administrative_area::{AdministrativeAreaOrGeoShapeOrPlaceOrText, CountryOrText};
use crate::permutations::product::ProductOrText;
use crate::permutations::text::LanguageOrText;
use crate::Thing;
use crate::thing::intangible::enumeration::ContactPointOption;
use crate::thing::intangible::structured_value::opening_hours_specification::OpeningHoursSpecification;

/// A contact point—for example, a Customer Complaints department.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContactPoint {
    /// The geographic area where a service or offered item is provided. Supersedes serviceArea.
    pub area_served: Box<AdministrativeAreaOrGeoShapeOrPlaceOrText>,

    ///  A language someone may use with or at the item, service or place. Please use one of the
    /// language codes from the IETF BCP 47 standard. See also inLanguage.
    pub available_language: LanguageOrText,

    /// An option available on this contact point (e.g. a toll-free number or support for
    /// hearing-impaired callers).
    pub contact_option: ContactPointOption,

    /// A person or organization can have different contact points, for different purposes.
    /// For example, a sales contact point, a PR contact point and so on. This property is used to
    /// specify the kind of contact point.
    pub contact_type: String,

    /// Email address.
    pub email: String,

    /// The fax number.
    pub fax_number: String,

    /// The hours during which this service or contact is available.
    pub hours_available: OpeningHoursSpecification,

    /// The product or service this support contact point is related to (such as product support
    /// for a particular product line). This can be a specific product or product line
    /// (e.g. "iPhone") or a general category of products or services (e.g. "smartphones").
    pub product_supported: ProductOrText,

    /// The telephone number.
    pub telephone: String,

    #[serde(flatten)]
    pub thing: Thing,
}

/// The mailing address.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PostalAddress {
    /// The country. Recommended to be in 2-letter ISO 3166-1 alpha-2 format, for example "US".
    /// For backward compatibility, a 3-letter ISO 3166-1 alpha-3 country code such as "SGP" or a
    /// full country name such as "Singapore" can also be used.
    pub address_country: CountryOrText,

    /// The locality in which the street address is, and which is in the region.
    /// For example, Mountain View.
    pub address_locality: String,

    /// The region in which the locality is, and which is in the country. For example, California
    /// or another appropriate first-level Administrative division.
    pub address_region: String,

    /// An address extension such as an apartment number, C/O or alternative name.
    pub extended_address: String,

    /// The post office box number for PO box addresses.
    pub post_office_box_number: String,

    /// The postal code. For example, 94043.
    pub postal_code: String,

    /// The street address. For example, 1600 Amphitheatre Pkwy.
    pub street_address: String,

    #[serde(flatten)]
    pub contact_point: ContactPoint,

    #[serde(flatten)]
    pub thing: Thing,
}
