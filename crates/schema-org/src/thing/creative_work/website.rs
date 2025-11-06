use schemars::JsonSchema;
use serde::Deserialize;

use crate::thing::CreativeWork;

/// A WebSite is a set of related web pages and other items typically served from a single web
/// domain and accessible via URLs.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WebSite {
    /// The International Standard Serial Number (ISSN) that identifies this serial publication.
    /// You can repeat this property to identify different formats of, or the linking ISSN (ISSN-L)
    /// for, this serial publication.
    pub issn: String,
    #[serde(flatten)]
    pub creative_work: CreativeWork,
}
