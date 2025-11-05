use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::data_type::text::{CssSelectorType, XPathType};

/// A SpeakableSpecification indicates (typically via xpath or cssSelector) sections of a document
/// that are highlighted as particularly speakable. Instances of this type are expected to be used
/// primarily as values of the speakable property.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SpeakableSpecification {
    /// A CSS selector, e.g. of a SpeakableSpecification or WebPageElement. In the latter case,
    /// multiple matches within a page can constitute a single conceptual "Web page element".
    pub css_selector: 	CssSelectorType,
    /// An XPath, e.g. of a SpeakableSpecification or WebPageElement. In the latter case, multiple
    /// matches within a page can constitute a single conceptual "Web page element".
    pub xpath: 	XPathType,
    #[serde(flatten)]
    pub thing: Thing,
}
