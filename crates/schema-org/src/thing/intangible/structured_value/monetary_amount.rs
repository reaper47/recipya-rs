use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::permutations::bool::BooleanOrNumberOrStructuredValueOrText;
use crate::permutations::date::DateOrDateTime;

/// A monetary value or range. This type can be used to describe an amount of money such as $50 USD,
/// or a range as in describing a bank account being suitable for a balance between £1,000 and
/// £1,000,000 GBP, or the value of a salary, etc. It is recommended to use PriceSpecification Types
/// to describe the price of an Offer, Invoice, etc.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MonetaryAmount {
    /// The currency in which the monetary amount is expressed.
    ///
    /// Use standard formats: ISO 4217 currency format, e.g. "USD"; Ticker symbol for
    /// cryptocurrencies, e.g. "BTC"; well known names for Local Exchange Trading Systems (LETS) and
    /// other currency types, e.g. "Ithaca HOUR".
    pub currency: String,
    /// The upper value of some characteristic or property.
    pub max_value: i32,
    /// The lower value of some characteristic or property.
    pub min_value: i32,
    /// The date when the item becomes valid.
    pub valid_from: DateOrDateTime,
    /// The date after when the item is not valid. For example the end of an offer, salary period,
    /// or a period of opening hours.
    pub valid_through: DateOrDateTime,
    /// The value of a QuantitativeValue (including Observation) or property value node.
    /// - For QuantitativeValue and MonetaryAmount, the recommended type for values is 'Number'.
    /// - For PropertyValue, it can be 'Text', 'Number', 'Boolean', or 'StructuredValue'.
    /// - Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.
    /// - Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.
    pub value: BooleanOrNumberOrStructuredValueOrText,
    #[serde(flatten)]
    pub thing: Thing,
}
