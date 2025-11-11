use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Action, DefinedTerm};
use crate::helpers::one_or_many;
use crate::field::*;
use crate::gene::Gene;
use crate::grant::Grant;

///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type BioChemEntityAdditionalTypeFieldEnum = String;

///<https://schema.org/BioChemEntity>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BioChemEntity {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/isInvolvedInBiologicalProcess>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_involved_in_biological_process: Vec<
        BioChemEntityIsInvolvedInBiologicalProcessFieldEnum,
    >,
    ///<https://schema.org/taxonomicRange>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub taxonomic_range: Vec<BioChemEntityTaxonomicRangeFieldEnum>,
    ///<https://schema.org/hasRepresentation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_representation: Vec<BioChemEntityHasRepresentationFieldEnum>,
    ///<https://schema.org/hasMolecularFunction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_molecular_function: Vec<BioChemEntityHasMolecularFunctionFieldEnum>,
    ///<https://schema.org/hasBioChemEntityPart>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_bio_chem_entity_part: Vec<BioChemEntity>,
    ///<https://schema.org/isPartOfBioChemEntity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_part_of_bio_chem_entity: Vec<BioChemEntity>,
    ///<https://schema.org/bioChemSimilarity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bio_chem_similarity: Vec<BioChemEntity>,
    ///<https://schema.org/biologicalRole>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub biological_role: Vec<DefinedTerm>,
    ///<https://schema.org/associatedDisease>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub associated_disease: Vec<BioChemEntityAssociatedDiseaseFieldEnum>,
    ///<https://schema.org/funding>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub funding: Vec<Grant>,
    ///<https://schema.org/bioChemInteraction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bio_chem_interaction: Vec<BioChemEntity>,
    ///<https://schema.org/isEncodedByBioChemEntity>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_encoded_by_bio_chem_entity: Vec<Gene>,
    ///<https://schema.org/isLocatedInSubcellularLocation>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub is_located_in_subcellular_location: Vec<
        BioChemEntityIsLocatedInSubcellularLocationFieldEnum,
    >,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<BioChemEntityAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<BioChemEntityIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<BioChemEntityImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<BioChemEntityDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<BioChemEntitySubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<BioChemEntityMainEntityOfPageFieldEnum>,
}
