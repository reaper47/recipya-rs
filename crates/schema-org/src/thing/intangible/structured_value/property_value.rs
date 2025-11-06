use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::permutations::bool::BooleanOrNumberOrStructuredValueOrText;
use crate::permutations::defined_term::DefinedTermOrMeasurementMethodEnumOrTextOrURL;
use crate::permutations::text::TextOrURL;

/// A property-value pair, e.g. representing a feature of a product or place. Use the 'name'
/// property for the name of the property. If there is an additional human-readable version of the
/// value, put that into the 'description' property.
///
/// Always use specific schema.org properties when a) they exist and b) you can populate them. Using
/// PropertyValue as a substitute will typically not trigger the same effect as using the original,
/// specific property.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PropertyValue {
    /// The upper value of some characteristic or property.
    pub max_value: 	i32,
    /// A subproperty of measurementTechnique that can be used for specifying specific methods, in particular via MeasurementMethodEnum.
    pub measurement_method: 	DefinedTermOrMeasurementMethodEnumOrTextOrURL,
    /// A technique, method or technology used in an Observation, StatisticalVariable or Dataset (or DataDownload, DataCatalog), corresponding to the method used for measuring the corresponding variable(s) (for datasets, described using variableMeasured; for Observation, a StatisticalVariable). Often but not necessarily each variableMeasured will have an explicit representation as (or mapping to) an property such as those defined in Schema.org, or other RDF vocabularies and "knowledge graphs". In that case the subproperty of variableMeasured called measuredProperty is applicable.
    ///
    /// The measurementTechnique property helps when extra clarification is needed about how a measuredProperty was measured. This is oriented towards scientific and scholarly dataset publication but may have broader applicability; it is not intended as a full representation of measurement, but can often serve as a high level summary for dataset discovery.
    ///
    /// For example, if variableMeasured is: molecule concentration, measurementTechnique could be: "mass spectrometry" or "nmr spectroscopy" or "colorimetry" or "immunofluorescence". If the variableMeasured is "depression rating", the measurementTechnique could be "Zung Scale" or "HAM-D" or "Beck Depression Inventory".
    ///
    /// If there are several variableMeasured properties recorded for some given data object, use a PropertyValue for each variableMeasured and attach the corresponding measurementTechnique. The value can also be from an enumeration, organized as a MeasurementMetholdEnumeration.
    pub measurement_technique: 	DefinedTermOrMeasurementMethodEnumOrTextOrURL,
    /// The lower value of some characteristic or property.
    pub min_value: 	i32,
    /// A commonly used identifier for the characteristic represented by the property, e.g. a manufacturer or a standard code for a property. propertyID can be (1) a prefixed string, mainly meant to be used with standards for product properties; (2) a site-specific, non-prefixed string (e.g. the primary key of the property or the vendor-specific ID of the property), or (3) a URL indicating the type of the property, either pointing to an external vocabulary, or a Web resource that describes the property (e.g. a glossary entry). Standards bodies should promote a standard prefix for the identifiers of properties from their standards.
    #[serde(rename = "propertyID")]
    pub property_id: 	TextOrURL,
    /// The unit of measurement given using the UN/CEFACT Common Code (3 characters) or a URL. Other codes than the UN/CEFACT Common Code may be used with a prefix followed by a colon.
    pub unit_code: 	TextOrURL,
    /// A string or text indicating the unit of measurement. Useful if you cannot provide a standard unit code for unitCode.
    pub unit_text: 	String,
    /// The value of a QuantitativeValue (including Observation) or property value node.
    ///
    /// - For QuantitativeValue and MonetaryAmount, the recommended type for values is 'Number'.
    /// - For PropertyValue, it can be 'Text', 'Number', 'Boolean', or 'StructuredValue'.
    /// - Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.
    /// - Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.
    pub value: 	BooleanOrNumberOrStructuredValueOrText,
    /// A secondary value that provides additional information on the original value, e.g. a reference temperature or a type of measurement.
    pub value_reference: 	DefinedTermOrEnumerationOrMeasurementTypeEnumerationOrPropertyValueOrQualitativeValueOrQuantitativeValueOrStructuredValueOrText,
    #[serde(flatten)]
    pub thing: Thing,
}
