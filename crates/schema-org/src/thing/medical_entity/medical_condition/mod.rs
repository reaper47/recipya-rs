mod medical_sign_or_symptom;

pub use medical_sign_or_symptom::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::permutations::anatomy::AnatomicalStructureOrAnatomicalSystemOrSuperficialAnatomy;
use crate::thing::MedicalEntity;
use crate::thing::medical_entity::MedicalTest;
use crate::thing::medical_entity::medical_intangible::DDxElement;
use crate::thing::medical_entity::therapeutic_procedure::MedicalTherapy;

/// Any condition of the human body that affects the normal functioning of a person, whether
/// physically or mentally. Includes diseases, injuries, disabilities, disorders, syndromes, etc.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalCondition {
    /// The anatomy of the underlying organ system or structures associated with this entity.
    pub associated_anatomy: AnatomicalStructureOrAnatomicalSystemOrSuperficialAnatomy,
    /// One of a set of differential diagnoses for the condition. Specifically, a closely-related or
    /// competing diagnosis typically considered later in the cognitive process whereby this medical
    /// condition is distinguished from others most likely responsible for a similar collection of
    /// signs and symptoms to reach the most parsimonious diagnosis or diagnoses in a patient.
    pub differential_diagnosis: DDxElement,
    /// Specifying a drug or medicine used in a medication procedure.
    pub drug: Drug,
    /// The characteristics of associated patients, such as age, gender, race etc.
    pub epidemiology: String,
    /// The likely outcome in either the short term or long term of the medical condition.
    pub expected_prognosis: String,
    /// The expected progression of the condition if it is not treated and allowed to progress
    /// naturally.
    pub natural_progression: String,
    /// Changes in the normal mechanical, physical, and biochemical functions that are associated
    /// with this activity or condition.
    pub pathophysiology: String,
    /// A possible unexpected and unfavorable evolution of a medical condition. Complications may
    /// include worsening of the signs or symptoms of the disease, extension of the condition to
    /// other organ systems, etc.
    pub possible_complication: String,
    /// A possible treatment to address this condition, sign or symptom.
    pub possible_treatment: MedicalTherapy,
    /// A preventative therapy used to prevent an initial occurrence of the medical condition,
    /// such as vaccination.
    pub primary_prevention: MedicalTherapy,
    /// A modifiable or non-modifiable factor that increases the risk of a patient contracting this
    /// condition, e.g. age, coexisting condition.
    pub risk_factor: MedicalRiskFactor,
    /// A preventative therapy used to prevent reoccurrence of the medical condition after an
    /// initial episode of the condition.
    pub secondary_prevention: MedicalTherapy,
    /// A sign or symptom of this condition. Signs are objective or physically observable
    /// manifestations of the medical condition while symptoms are the subjective experience of the
    /// medical condition.
    pub sign_or_symptom: MedicalSignOrSymptom,
    /// The stage of the condition, if applicable.
    pub stage: MedicalConditionStage,
    /// The status of the study (enumerated).
    pub status: EventStatusTypeOrMedicalStudyStatusOrText,
    /// A medical test typically performed given this condition.
    pub typical_test: MedicalTest,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
