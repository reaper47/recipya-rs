use schemars::JsonSchema;
use serde::Deserialize;

use crate::data_type::text::URL;
use crate::permutations::text::TextOrURL;
use crate::thing::Product;
use crate::thing::intangible::enumeration::DrugPregnancyCategory;
use crate::thing::medical_entity::medical_intangible::DrugStrength;

/// A chemical or biologic substance, used as a medical therapy, that has a physiological effect on
/// an organism. Here the term drug is used interchangeably with the term medicine although clinical
/// knowledge makes a clear difference between them.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Drug {
    /// An active ingredient, typically chemical compounds and/or biologic substances.
    pub active_ingredient: String,
    /// A route by which this drug may be administered, e.g. 'oral'.
    pub administration_route: String,
    /// Any precaution, guidance, contraindication, etc. related to consumption of alcohol while
    /// taking this drug.
    pub alcohol_warning: String,
    /// An available dosage strength for the drug.
    pub available_strength: DrugStrength,
    /// Any precaution, guidance, contraindication, etc. related to this drug's use by breastfeeding
    /// mothers.
    pub breastfeeding_warning: String,
    /// Description of the absorption and elimination of drugs, including their concentration
    /// (pharmacokinetics, pK) and biological effects (pharmacodynamics, pD). Supersedes
    /// clincalPharmacology.
    pub clinical_pharmacology: String,
    /// A dosage form in which this drug/supplement is available, e.g. 'tablet', 'suspension',
    /// 'injection'.
    pub dosage_form: String,
    /// A dosing schedule for the drug for a given population, either observed, recommended, or
    /// maximum dose based on the type used.
    pub dose_schedule: DoseSchedule,
    /// The class of drug this belongs to (e.g., statins).
    pub drug_class: DrugClass,
    /// The unit in which the drug is measured, e.g. '5 mg tablet'.
    pub drug_unit: String,
    /// Any precaution, guidance, contraindication, etc. related to consumption of specific foods
    /// while taking this drug.
    pub food_warning: String,
    /// The insurance plans that cover this drug.
    pub included_in_health_insurance_plan: HealthInsurancePlan,
    /// Another drug that is known to interact with this drug in a way that impacts the effect of
    /// this drug or causes a risk to the patient. Note: disease interactions are typically captured
    /// as contraindications.
    pub interacting_drug: Box<Self>,
    /// True if the drug is available in a generic form (regardless of name).
    pub is_available_generically: bool,
    /// True if this item's name is a proprietary/brand name (vs. generic name).
    pub is_proprietary: bool,
    /// Link to the drug's label details.
    pub label_details: URL,
    /// The drug or supplement's legal status, including any controlled substance schedules that
    /// apply.
    pub legal_status: DrugLegalStatusOrMedicalEnumerationOrText,
    /// Recommended intake of this supplement for a given population as defined by a specific
    /// recommending authority.
    pub maximum_intake: MaximumDoseSchedule,
    /// The specific biochemical interaction through which this drug or supplement produces its
    /// pharmacological effect.
    pub mechanism_of_action: String,
    /// The generic name of this drug or supplement.
    pub non_proprietary_name: String,
    /// Any information related to overdose on a drug, including signs or symptoms, treatments,
    /// contact information for emergency response.
    pub overdosage: String,
    /// Pregnancy category of this drug.
    pub pregnancy_category: DrugPregnancyCategory,
    /// Any precaution, guidance, contraindication, etc. related to this drug's use during
    /// pregnancy.
    pub pregnancy_warning: String,
    /// Link to prescribing information for the drug.
    pub prescribing_info: URL,
    /// Indicates the status of drug prescription, e.g. local catalogs classifications or whether
    /// the drug is available by prescription or over-the-counter, etc.
    pub prescription_status: DrugPrescriptionStatusOrText,
    /// Proprietary name given to the diet plan, typically by its originator or creator.
    pub proprietary_name: String,
    /// Any other drug related to this one, for example commonly-prescribed alternatives.
    pub related_drug: Box<Self>,
    /// The RxCUI drug identifier from RXNORM.
    pub rxcui: String,
    /// Any FDA or other warnings about the drug (text or URL).
    pub warning: TextOrURL,
    #[serde(flatten)]
    pub product: Product,
}
