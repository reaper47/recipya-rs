use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::permutations::administrative_area::CountryOrText;
use crate::permutations::postal_address::PostalAddressOrText;
use crate::permutations::text::NumberOrText;

/// The geographic shape of a place. A GeoShape can be described using several properties whose
/// values are based on latitude/longitude pairs. Either whitespace or commas can be used to
/// separate latitude and longitude; whitespace should be used when writing a list of several
/// such points.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GeoShape {
    /// Physical address of the item.
    pub address: PostalAddressOrText,
    /// The country. Recommended to be in 2-letter ISO 3166-1 alpha-2 format, for example "US".
    /// For backward compatibility, a 3-letter ISO 3166-1 alpha-3 country code such as "SGP" or a
    /// full country name such as "Singapore" can also be used.
    pub address_country: CountryOrText,
    /// A box is the area enclosed by the rectangle formed by two points. The first point is the
    /// lower corner, the second point is the upper corner. A box is expressed as two points
    /// separated by a space character.
    pub r#box: String,
    /// A circle is the circular region of a specified radius centered at a specified latitude and
    /// longitude. A circle is expressed as a pair followed by a radius in meters.
    pub circle: String,
    /// The elevation of a location (WGS 84). Values may be of the form 'NUMBER UNIT_OF_MEASUREMENT'
    /// (e.g., '1,000 m', '3,200 ft') while numbers alone should be assumed to be a value in meters.
    pub elevation: NumberOrText,
    /// A line is a point-to-point path consisting of two or more points. A line is expressed as a
    /// series of two or more point objects separated by space.
    pub line: String,
    /// A polygon is the area enclosed by a point-to-point path for which the starting and ending
    /// points are the same. A polygon is expressed as a series of four or more space delimited
    /// points where the first and final points are identical.
    pub polygon: String,
    /// The postal code. For example, 94043.
    pub postal_code: String,
    #[serde(flatten)]
    pub thing: Thing,
}
