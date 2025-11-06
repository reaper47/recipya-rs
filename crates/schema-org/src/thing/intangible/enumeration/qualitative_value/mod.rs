use schemars::JsonSchema;
use serde::Deserialize;

use crate::permutations::defined_term::DefinedTermOrEnumerationOrMeasurementTypeEnumerationOrPropertyValueOrQualitativeValueOrQuantitativeValueOrStructuredValueOrText;
use crate::thing::intangible::enumeration::Enumeration;
use crate::thing::intangible::structured_value::PropertyValue;

/// A predefined value for a product characteristic, e.g. the power cord plug type 'US' or the
/// garment sizes 'S', 'M', 'L', and 'XL'.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QualitativeValue {
    /// A property-value pair representing an additional characteristic of the entity, e.g. a
    /// product feature or another characteristic for which there is no matching property in
    /// schema.org.
    ///
    /// Note: Publishers should be aware that applications designed to use specific schema.org
    /// properties (e.g. https://schema.org/width, https://schema.org/color,
    /// https://schema.org/gtin13, ...) will typically expect such data to be provided using those
    /// properties, rather than using the generic property/value mechanism.
    pub additional_property: 	PropertyValue,
    /// This ordering relation for qualitative values indicates that the subject is equal to the
    /// object.
    pub equal: 	Box<QualitativeValue>,
    /// This ordering relation for qualitative values indicates that the subject is greater than the
    /// object.
    pub greater: 	Box<QualitativeValue>,
    /// This ordering relation for qualitative values indicates that the subject is greater than or
    /// equal to the object.
    pub greater_or_equal: 	Box<QualitativeValue>,
    /// This ordering relation for qualitative values indicates that the subject is lesser than the
    /// object.
    pub lesser: 	Box<QualitativeValue>,
    /// This ordering relation for qualitative values indicates that the subject is lesser than or
    /// equal to the object.
    pub lesser_or_equal: 	Box<QualitativeValue>,
    /// This ordering relation for qualitative values indicates that the subject is not equal to the
    /// object.
    pub non_equal: 	Box<QualitativeValue>,
    /// A secondary value that provides additional information on the original value, e.g. a
    /// reference temperature or a type of measurement.
    pub value_reference: 	DefinedTermOrEnumerationOrMeasurementTypeEnumerationOrPropertyValueOrQualitativeValueOrQuantitativeValueOrStructuredValueOrText,
    #[serde(flatten)]
    pub enumeration: Enumeration,
}
