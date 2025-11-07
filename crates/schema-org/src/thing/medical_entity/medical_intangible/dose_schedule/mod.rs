mod maximum_dose_schedule;

pub use maximum_dose_schedule::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::permutations::number::NumberOrQualitativeValue;
use crate::thing::MedicalEntity;

/// A specific dosing schedule for a drug or supplement.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DoseSchedule {
    /// The unit of the dose, e.g. 'mg'.
    pub dose_unit: String,
    /// The value of the dose, e.g. 500.
    pub dose_value: NumberOrQualitativeValue,
    /// How often the dose is taken, e.g. 'daily'.
    pub frequency: String,
    /// Characteristics of the population for which this is intended, or which typically uses it,
    /// e.g. 'adults'.
    pub target_population: String,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
