use schemars::JsonSchema;
use serde::Deserialize;

use crate::medical_entity::therapeutic_procedure::MedicalTherapy;
use crate::medical_entity::{MedicalCondition, MedicalEntity};
use crate::thing::medical::AnatomicalSystem;

/// Any part of the human body, typically a component of an anatomical system. Organs, tissues, and
/// cells are all anatomical structures.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AnatomicalStructure {
    /// If applicable, a description of the pathophysiology associated with the anatomical system,
    /// including potential abnormal changes in the mechanical, physical, and biochemical functions of the system.
    pub associated_pathophysiology: String,
    /// Location in the body of the anatomical structure.
    pub body_location: String,
    /// Other anatomical structures to which this structure is connected.
    pub connected_to: Box<Self>,
    /// An image containing a diagram that illustrates the structure and/or its component
    /// substructures and/or connections with other structures.
    pub diagram: ImageObject,
    /// The anatomical or organ system that this structure is part of.
    pub part_of_system: Box<AnatomicalSystem>,
    /// A medical condition associated with this anatomy.
    pub related_condition: MedicalCondition,
    /// A medical therapy related to this anatomy.
    pub related_therapy: MedicalTherapy,
    /// Component (sub-)structure(s) that comprise this anatomical structure.
    pub sub_structure: Box<Self>,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
