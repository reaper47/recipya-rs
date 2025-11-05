use schemars::JsonSchema;
use serde::Deserialize;

use crate::medical_entity::MedicalEntity;

/// A condition or factor that serves as a reason to withhold a certain medical therapy.
/// Contraindications can be absolute (there are no reasonable circumstances for undertaking a
/// course of action) or relative (the patient is at higher risk of complications, but these risks
/// may be outweighed by other considerations or mitigated by other measures).
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalContraindication {
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
