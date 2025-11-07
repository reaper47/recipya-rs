use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::medical_entity::medical_intangible::DoseSchedule;

/// The maximum dosing schedule considered safe for a drug or supplement as recommended by an
/// authority or by the drug/supplement's manufacturer. Capture the recommending authority in the
/// recognizingAuthority property of MedicalEntity.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MaximumDoseSchedule {
    #[serde(flatten)]
    pub dose_schedule: DoseSchedule,
}
