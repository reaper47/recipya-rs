use schemars::JsonSchema;

use crate::thing::medical::AnatomicalSystem;
use crate::thing::medical_entity::AnatomicalStructure;

#[derive(Debug, PartialEq, JsonSchema)]
pub enum AnatomicalStructureOrAnatomicalSystem {
    AnatomicalStructure(AnatomicalStructure),
    AnatomicalSystem(AnatomicalSystem),
}

impl Default for AnatomicalStructureOrAnatomicalSystem {
    fn default() -> Self {
        Self::AnatomicalStructure(AnatomicalStructure::default())
    }
}
