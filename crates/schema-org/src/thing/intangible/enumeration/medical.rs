use schemars::JsonSchema;
use serde::Deserialize;

/// Enumerated categories of medical drug costs.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum DrugCostCategory {
    ReimbursementCap,
    #[default]
    Retail,
    Wholesale,
}

/// Categories that represent an assessment of the risk of fetal injury due to a drug or
/// pharmaceutical used as directed by the mother during pregnancy.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum DrugPregnancyCategory {
    #[default]
    FDAcategoryA,
    FDAcategoryB,
    FDAcategoryC,
    FDAcategoryD,
    FDAcategoryX,
    FDAnotEvaluated,
}

/// Indicates whether this drug is available by prescription or over-the-counter.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum DrugPrescriptionStatus {
    #[default]
    OTC,
    PrescriptionOnly,
}

/// Classes of agents or pathogens that transmit infectious diseases. Enumerated type.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum InfectiousAgentClass {
    #[default]
    Bacteria,
    Fungus,
    MulticellularParasite,
    Prion,
    Protozoa,
    Virus,
}

/// Target audiences types for medical web pages. Enumerated type.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MedicalAudienceType {
    #[default]
    Clinician,
    MedicalResearcher,
}

/// Categories of medical devices, organized by the purpose or intended use of the device.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MedicalDevicePurpose {
    #[default]
    Diagnostic,
    Therapeutic,
}

/// Level of evidence for a medical guideline. Enumerated type.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MedicalEvidenceLevel {
    #[default]
    EvidenceLevelA,
    EvidenceLevelB,
    EvidenceLevelC,
}

/// Any medical imaging modality typically used for diagnostic purposes. Enumerated type.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MedicalImagingTechnique {
    #[default]
    CT,
    MRI,
    PET,
    Radiography,
    Ultrasound,
    XRay,
}

/// Design models for observational medical studies. Enumerated type.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MedicalObservationalStudyDesign {
    #[default]
    CaseSeries,
    CohortStudy,
    CrossSectional,
    Longitudinal,
    Observational,
    Registry,
}

/// An enumeration that describes different types of medical procedures.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MedicalProcedureType {
    #[default]
    NoninvasiveProcedure,
    PercutaneousProcedure,
}

/// Any specific branch of medical science or practice. Medical specialities include clinical
/// specialties that pertain to particular organ systems and their respective disease states, as
/// well as allied health specialties. Enumerated type.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MedicalSpecialty {
    #[default]
    Anesthesia,
    Cardiovascular,
    CommunityHealth,
    Dentistry,
    Dermatology,
    DietNutrition,
    Emergency,
    Endocrine,
    Gastroenterologic,
    Genetic,
    Geriatric,
    Gynecologic,
    Hematologic,
    Infectious,
    LaboratoryScience,
    Midwifery,
    Musculoskeletal,
    Neurologic,
    Nursing,
    Obstetric,
    Oncologic,
    Optometric,
    Otolaryngologic,
    Pathology,
    Pediatric,
    PharmacySpecialty,
    Physiotherapy,
    PlasticSurgery,
    Podiatric,
    PrimaryCare,
    Psychiatric,
    PublicHealth,
    Pulmonary,
    Radiography,
    Renal,
    RespiratoryTherapy,
    Rheumatologic,
    SpeechPathology,
    Surgical,
    Toxicologic,
    Urologic,
}

/// The status of a medical study. Enumerated type.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MedicalStudyStatus {
    #[default]
    ActiveNotRecruiting,
    Completed,
    EnrollingByInvitation,
    NotYetRecruiting,
    Recruiting,
    ResultsAvailable,
    ResultsNotAvailable,
    Suspended,
    Terminated,
    Withdrawn,
}

/// Design models for medical trials. Enumerated type.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MedicalTrialDesign {
    #[default]
    DoubleBlindedTrial,
    InternationalTrial,
    MultiCenterTrial,
    OpenTrial,
    PlaceboControlledTrial,
    RandomizedTrial,
    SingleBlindedTrial,
    SingleCenterTrial,
    TripleBlindedTrial,
}

/// Systems of medical practice.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum MedicineSystem {
    #[default]
    Ayurvedic,
    Chiropractic,
    Homeopathic,
    Osteopathic,
    TraditionalChinese,
    WesternConventional,
}

/// A type of physical examination of a patient performed by a physician.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub enum PhysicalExam {
    #[default]
    Abdomen,
    Appearance,
    CardiovascularExam,
    Ear,
    Eye,
    Genitourinary,
    Head,
    Lung,
    MusculoskeletalExam,
    Neck,
    Neuro,
    Nose,
    Skin,
    Throat,
}

