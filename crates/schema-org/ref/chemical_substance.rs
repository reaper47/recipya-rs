use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::field::*;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ChemicalSubstanceAdditionalTypeFieldEnum = String;
///<https://schema.org/ChemicalSubstance>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ChemicalSubstance {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/potentialUse>
    #[serde(rename = "potentialUse")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_use: Vec<DefinedTerm>,
    ///<https://schema.org/chemicalComposition>
    #[serde(rename = "chemicalComposition")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chemical_composition: Vec<String>,
    ///<https://schema.org/chemicalRole>
    #[serde(rename = "chemicalRole")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chemical_role: Vec<DefinedTerm>,
    ///<https://schema.org/isInvolvedInBiologicalProcess>
    #[serde(rename = "isInvolvedInBiologicalProcess")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_involved_in_biological_process: Vec<
        ChemicalSubstanceIsInvolvedInBiologicalProcessFieldEnum,
    >,
    ///<https://schema.org/taxonomicRange>
    #[serde(rename = "taxonomicRange")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub taxonomic_range: Vec<ChemicalSubstanceTaxonomicRangeFieldEnum>,
    ///<https://schema.org/hasRepresentation>
    #[serde(rename = "hasRepresentation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_representation: Vec<ChemicalSubstanceHasRepresentationFieldEnum>,
    ///<https://schema.org/hasMolecularFunction>
    #[serde(rename = "hasMolecularFunction")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_molecular_function: Vec<ChemicalSubstanceHasMolecularFunctionFieldEnum>,
    ///<https://schema.org/hasBioChemEntityPart>
    #[serde(rename = "hasBioChemEntityPart")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_bio_chem_entity_part: Vec<BioChemEntity>,
    ///<https://schema.org/isPartOfBioChemEntity>
    #[serde(rename = "isPartOfBioChemEntity")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_part_of_bio_chem_entity: Vec<BioChemEntity>,
    ///<https://schema.org/bioChemSimilarity>
    #[serde(rename = "bioChemSimilarity")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bio_chem_similarity: Vec<BioChemEntity>,
    ///<https://schema.org/biologicalRole>
    #[serde(rename = "biologicalRole")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub biological_role: Vec<DefinedTerm>,
    ///<https://schema.org/associatedDisease>
    #[serde(rename = "associatedDisease")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_disease: Vec<ChemicalSubstanceAssociatedDiseaseFieldEnum>,
    ///<https://schema.org/funding>
    #[serde(rename = "funding")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funding: Vec<Grant>,
    ///<https://schema.org/bioChemInteraction>
    #[serde(rename = "bioChemInteraction")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bio_chem_interaction: Vec<BioChemEntity>,
    ///<https://schema.org/isEncodedByBioChemEntity>
    #[serde(rename = "isEncodedByBioChemEntity")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_encoded_by_bio_chem_entity: Vec<Gene>,
    ///<https://schema.org/isLocatedInSubcellularLocation>
    #[serde(rename = "isLocatedInSubcellularLocation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_located_in_subcellular_location: Vec<
        ChemicalSubstanceIsLocatedInSubcellularLocationFieldEnum,
    >,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(rename = "potentialAction")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(rename = "additionalType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<ChemicalSubstanceAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<ChemicalSubstanceIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<ChemicalSubstanceImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<ChemicalSubstanceDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<ChemicalSubstanceSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<ChemicalSubstanceMainEntityOfPageFieldEnum>,
}
