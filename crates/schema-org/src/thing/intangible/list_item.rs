use crate::Thing;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::permutations::text::IntegerOrText;

/// An list item, e.g. a step in a checklist or how-to description.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ListItem {
    /// An entity represented by an entry in a list or data feed (e.g. an 'artist'
    /// in a list of 'artists').
    pub item: Thing,
    /// A link to the ListItem that follows the current one.
    pub next_item: Box<ListItem>,
    /// The position of an item in a series or sequence of items.
    pub position: IntegerOrText,
    /// A link to the ListItem that precedes the current one.
    pub previous_item: Box<ListItem>,
    #[serde(flatten)]
    pub thing: Thing,
}
