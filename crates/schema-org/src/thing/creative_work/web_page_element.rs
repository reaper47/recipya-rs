use schemars::JsonSchema;
use serde::Deserialize;

use crate::data_type::text::{CssSelectorType, XPathType};
use crate::thing::CreativeWork;

/// A web page element, like a table or an image.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WebPageElement {
    /// A CSS selector, e.g. of a SpeakableSpecification or WebPageElement. In the latter case,
    /// multiple matches within a page can constitute a single conceptual "Web page element".
    pub css_selector: CssSelectorType,
    /// An XPath, e.g. of a SpeakableSpecification or WebPageElement. In the latter case, multiple
    /// matches within a page can constitute a single conceptual "Web page element".
    pub xpath: XPathType,
    #[serde(flatten)]
    pub creative_work: CreativeWork,
}
