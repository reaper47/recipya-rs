use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::creative_work::CreativeWork;
use crate::thing::intangible::defined_term::DefinedTerm;

#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DefinedTermSet {
    /// A Defined Term contained in this term set.
    pub has_defined_term: DefinedTerm,
    #[serde(flatten)]
    pub creative_work: CreativeWork,
}
