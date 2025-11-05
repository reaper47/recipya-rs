use schemars::JsonSchema;
use serde::Deserialize;

use crate::components::{PropertyValueOrTextOrURL, Thing};
use crate::thing::intangible::defined_term::DefinedTerm;
use crate::thing::intangible::grant::Grant;

/// Any biological, chemical, or biochemical thing. For example: a protein; a gene; a chemical; a
/// synthetic chemical.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BioChemEntity {
    /// Disease associated to this BioChemEntity. Such disease can be a MedicalCondition or a URL.
    /// If you want to add an evidence supporting the association, please use PropertyValue.
    pub associated_disease: 	MedicalConditionOrPropertyValueOrUrl,
    /// A BioChemEntity that is known to interact with this item.
    pub bio_chem_interaction: 	Box<Self>,
    /// A similar BioChemEntity, e.g., obtained by fingerprint similarity algorithms.
    pub bio_chem_similarity: 	Box<Self>,
    /// A role played by the BioChemEntity within a biological context.
    pub biological_role: 	DefinedTerm,
    /// A Grant that directly or indirectly provide funding or sponsorship for this item. See also
    /// ownershipFundingInfo.
    ///
    /// Inverse property: fundedItem
    pub funding: 	Grant,
    /// Indicates a BioChemEntity that (in some sense) has this BioChemEntity as a part.
    ///
    /// Inverse property: isPartOfBioChemEntity,
    pub has_bio_chem_entity_part: 	Box<Self>,
    /// Molecular function performed by this BioChemEntity; please use PropertyValue if you want to
    /// include any evidence.
    pub has_molecular_function: 	DefinedTermOrPropertyValueOrUrl,
    /// A common representation such as a protein sequence or chemical structure for this entity.
    /// For images use schema.org/image.
    pub has_representation: PropertyValueOrTextOrURL,
    /// Another BioChemEntity encoding by this one.
    ///
    /// Inverse property: encodesBioChemEntity
    pub is_encoded_by_bio_chem_entity: 	Box<Gene>,
    /// Biological process this BioChemEntity is involved in; please use PropertyValue if you want
    /// to include any evidence.
    pub is_involved_in_biological_process: 	DefinedTermOrPropertyValueOrUrl,
    /// Subcellular location where this BioChemEntity is located; please use PropertyValue if you want to include any evidence.
    pub is_located_in_subcellular_location: 	DefinedTermOrPropertyValueOrUrl,
    /// Indicates a BioChemEntity that is (in some sense) a part of this BioChemEntity.
    ///
    /// Inverse property: hasBioChemEntityPart
    pub is_part_of_bio_chem_entity: 	Box<Self>,
    /// The taxonomic grouping of the organism that expresses, encodes, or in some way related to
    /// the BioChemEntity.
    pub taxonomic_range: 	DefinedTermOrTaxonOrTextOrUrl,
    #[serde(flatten)]
    pub thing: Thing,
}

/// A discrete unit of inheritance which affects one or more biological traits
/// (Source: https://en.wikipedia.org/wiki/Gene). Examples include FOXP2 (Forkhead box protein P2),
/// SCARNA21 (small Cajal body-specific RNA 21), A- (agouti genotype).
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Gene {
    /// Another gene which is a variation of this one.
    pub alternative_of: Box<Self>,
    /// Another BioChemEntity encoded by this one.
    ///
    /// Inverse property: isEncodedByBioChemEntity
    pub encodes_bio_chem_entity: BioChemEntity,
    /// Tissue, organ, biological sample, etc in which activity of this gene has been observed
    /// experimentally. For example brain, digestive system.
    pub expressed_in: AnatomicalStructureOrAnatomicalSystemOrBioChemEntityOrDefinedTerm,
    /// A symbolic representation of a BioChemEntity. For example, a nucleotide sequence of a Gene
    /// or an amino acid sequence of a Protein.
    pub has_bio_polymer_sequence: String,
    #[serde(flatten)]
    pub bio_chem_entity: BioChemEntity,
}
