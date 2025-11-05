use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::medical_entity::MedicalContraindication;
use crate::thing::MedicalEntity;

#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum MedicalContraindicationOrText {
    MedicalContraindication(MedicalContraindication),
    Text(String),
}

impl Default for MedicalContraindicationOrText {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum MedicalEntityOrText {
    MedicalEntity(MedicalEntity),
    Text(String),
}

impl Default for MedicalEntityOrText {
    fn default() -> Self {
        Self::Text(String::new())
    }
}
