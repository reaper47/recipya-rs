use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;

/// A Category Code.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CategoryCode {
    /// A short textual code that uniquely identifies the value.
    pub code_value: String,
    /// A CategoryCodeSet that contains this category code.
    pub in_code_set: CategoryCodeSetOrUrl,
    #[serde(flatten)]
    pub defined_term: DefinedTerm,
}

/// A word, name, acronym, phrase, etc. with a formal definition. Often used in the context of
/// category or subject classification, glossaries or dictionaries, product or creative work types,
/// etc. Use the name property for the term being defined, use termCode if the term has an
/// alpha-numeric code allocated, use description to provide the definition of the term.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DefinedTerm {
    /// A DefinedTermSet that contains this term.
    pub in_defined_term_set: DefinedTermSetOrUrl,
    /// A code that identifies this DefinedTerm within a DefinedTermSet.
    pub term_code: String,
    #[serde(flatten)]
    pub thing: Thing,
}
