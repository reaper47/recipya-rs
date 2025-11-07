use schemars::JsonSchema;
use serde::Deserialize;

use crate::permutations::person::OrganizationOrPerson;
use crate::thing::MedicalEntity;
use crate::thing::medical_entity::MedicalCondition;
use crate::thing::place::AdministrativeArea;

/// A medical study is an umbrella type covering all kinds of research studies relating to human
/// medicine or health, including observational studies and interventional trials and registries,
/// randomized, controlled or not. When the specific type of study is known, use one of the
/// extensions of this type, such as MedicalTrial or MedicalObservationalStudy. Also, note that this
/// type should be used to mark up data that describes the study itself; to tag an article that
/// publishes the results of a study, use MedicalScholarlyArticle. Note: use the code property of
/// MedicalEntity to store study IDs, e.g. clinicaltrials.gov ID.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MedicalStudy {
    /// Specifying the health condition(s) of a patient, medical study, or other target audience.
    pub health_condition: MedicalCondition,
    /// A person or organization that supports a thing through a pledge, promise, or financial
    /// contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
    pub sponsor: OrganizationOrPerson,
    /// The status of the study (enumerated).
    pub status: EventStatusTypeOrMedicalStudyStatusOrText,
    /// The location in which the study is taking/took place.
    pub study_location: AdministrativeArea,
    /// A subject of the study, i.e. one of the medical conditions, therapies, devices, drugs, etc.
    /// investigated by the study.
    pub study_subject: MedicalEntity,
    #[serde(flatten)]
    pub medical_entity: MedicalEntity,
}
