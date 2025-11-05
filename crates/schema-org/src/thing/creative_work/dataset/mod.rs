mod data_feed;

pub use data_feed::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::CreativeWork;

/// A body of structured information describing some topic(s) of interest.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct  Dataset  {
    /// A downloadable form of this dataset, at a specific location, in a specific format.
    /// This property can be repeated if different variations are available. There is no expectation
    /// that different downloadable distributions must contain exactly equivalent information
    /// (see also DCAT on this point). Different distributions might include or exclude different
    /// subsets of the entire dataset, for example.
    pub distribution: 	DataDownload,
    /// A data catalog which contains this dataset. Supersedes includedDataCatalog, catalog.
    ///
    /// Inverse property: dataset
    pub included_in_data_catalog: 	DataCatalog,
    /// The International Standard Serial Number (ISSN) that identifies this serial publication.
    /// You can repeat this property to identify different formats of, or the linking ISSN (ISSN-L)
    /// for, this serial publication.
    pub issn: 	String,
    /// A subproperty of measurementTechnique that can be used for specifying specific methods,
    /// in particular via MeasurementMethodEnum.
    pub measurement_method: 	DefinedTermOrMeasurementMethodEnumOrTextOrURL,
    ///  	A technique, method or technology used in an Observation, StatisticalVariable or Dataset
    /// (or DataDownload, DataCatalog), corresponding to the method used for measuring the
    /// corresponding variable(s) (for datasets, described using variableMeasured; for Observation,
    /// a StatisticalVariable). Often but not necessarily each variableMeasured will have an
    /// explicit representation as (or mapping to) an property such as those defined in Schema.org,
    /// or other RDF vocabularies and "knowledge graphs". In that case the subproperty of
    /// variableMeasured called measuredProperty is applicable.
    ///
    /// The measurementTechnique property helps when extra clarification is needed about how a
    /// measuredProperty was measured. This is oriented towards scientific and scholarly dataset
    /// publication but may have broader applicability; it is not intended as a full representation
    /// of measurement, but can often serve as a high level summary for dataset discovery.
    ///
    /// For example, if variableMeasured is: molecule concentration, measurementTechnique could be:
    /// "mass spectrometry" or "nmr spectroscopy" or "colorimetry" or "immunofluorescence". If the
    /// variableMeasured is "depression rating", the measurementTechnique could be "Zung Scale" or
    /// "HAM-D" or "Beck Depression Inventory".
    ///
    /// If there are several variableMeasured properties recorded for some given data object, use
    /// a PropertyValue for each variableMeasured and attach the corresponding measurementTechnique.
    /// The value can also be from an enumeration, organized as a MeasurementMetholdEnumeration.
    pub measurement_technique: 	DefinedTermOrMeasurementMethodEnumOrTextOrURL,
    ///  	The variableMeasured property can indicate (repeated as necessary) the variables that
    /// are measured in some dataset, either described as text or as pairs of identifier and
    /// description using PropertyValue, or more explicitly as a StatisticalVariable.
    pub variable_measured: 	PropertyOrPropertyValueOrStatisticalVariableOrText,
    #[serde(flatten)]
    pub creative_work: CreativeWork,
}
