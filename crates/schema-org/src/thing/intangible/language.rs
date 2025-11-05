use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;

/// Natural languages such as Spanish, Tamil, Hindi, English, etc. Formal language code tags
/// expressed in BCP 47 can be used via the alternateName property. The Language type previously
/// also covered programming languages such as Scheme and Lisp, which are now best represented using
/// ComputerLanguage.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Language {
    #[serde(flatten)]
    pub thing: Thing,
}
