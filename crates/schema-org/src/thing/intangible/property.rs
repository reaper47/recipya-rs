use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::thing::intangible::Class;

/// A property, used to indicate attributes and relationships of some Thing; equivalent to
/// rdf:Property.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Property {
    /// Relates a property to a class that is (one of) the type(s) the property is expected to be
    /// used on.
    pub domain_includes: 	Class,
    /// Relates a property to a property that is its inverse. Inverse properties relate the same
    /// pairs of items to each other, but in reversed direction. For example, the 'alumni' and
    /// 'alumniOf' properties are inverseOf each other. Some properties don't have explicit
    /// inverses; in these situations RDFa and JSON-LD syntax for reverse properties can be used.
    pub inverse_of: 	Box<Property>,
    /// Relates a property to a class that constitutes (one of) the expected type(s) for values of
    /// the property.
    pub range_includes: 	Class,
    /// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
    pub superseded_by: 	ClassOrEnumerationOrProperty,
    #[serde(flatten)]
    pub thing: Thing,
}
