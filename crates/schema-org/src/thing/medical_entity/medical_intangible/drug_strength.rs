use schemars::JsonSchema;
use serde::Deserialize;

use crate::data_type::Number;
use crate::thing::MedicalEntity;
use crate::thing::medical_entity::medical_intangible::MaximumDoseSchedule;
use crate::thing::place::AdministrativeArea;

/// A specific strength in which a medical drug is available in a specific country.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DrugStrength {
    /// An active ingredient, typically chemical compounds and/or biologic substances.
    pub active_ingredient: String,
    /// The location in which the strength is available.
    pub available_in: AdministrativeArea,
    /// Recommended intake of this supplement for a given population as defined by a specific
    /// recommending authority.
    pub maximum_intake: MaximumDoseSchedule,
    /// The units of an active ingredient's strength, e.g. mg.
    pub strength_unit: String,
    /// The value of an active ingredient's strength, e.g. 325.
    pub strength_value: Number,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
