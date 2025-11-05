use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::permutations::date::DateOrDateTime;

/// A single item within a larger data feed.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DataFeedItem {
    /// The date on which the CreativeWork was created or the item was added to a DataFeed.
    pub date_created: 	DateOrDateTime,
    /// The datetime the item was removed from the DataFeed.
    pub date_deleted: 	DateOrDateTime,
    /// The date on which the CreativeWork was most recently modified or when the item's entry was
    /// modified within a DataFeed.
    pub date_modified: 	DateOrDateTime,
    /// An entity represented by an entry in a list or data feed (e.g. an 'artist' in a list of
    /// 'artists').
    pub item: 	Thing, 	
    #[serde(flatten)]
    pub thing: Thing,
}
