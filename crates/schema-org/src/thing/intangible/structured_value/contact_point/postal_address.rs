use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::permutations::administrative_area::CountryOrText;
use crate::thing::intangible::structured_value::contact_point::ContactPoint;

/// The mailing address.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
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
