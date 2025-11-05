use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::medical_entity::{AnatomicalStructure, MedicalCondition};
use crate::thing::medical_entity::therapeutic_procedure::MedicalTherapy;
use crate::thing::MedicalEntity;

#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AnatomicalSystem {
     /// If applicable, a description of the pathophysiology associated with the anatomical system,
     /// including potential abnormal changes in the mechanical, physical, and biochemical functions
     /// of the system.
     pub associated_pathophysiology: 	String,
     /// Specifying something physically contained by something else. Typically used here for the
     /// underlying anatomical structures, such as organs, that comprise the anatomical system.
     pub comprised_of: 	AnatomicalStructureOrAnatomicalSystem,
     /// A medical condition associated with this anatomy.
     pub related_condition: 	MedicalCondition,
     /// Related anatomical structure(s) that are not part of the system but relate or connect to
     /// it, such as vascular bundles associated with an organ system.
     pub related_structure: 	AnatomicalStructure,
     /// A medical therapy related to this anatomy.
     pub related_therapy: 	MedicalTherapy,
     #[serde(flatten)]
     pub medical_entity: MedicalEntity,
}
