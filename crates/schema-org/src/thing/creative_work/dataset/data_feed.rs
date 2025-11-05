use schemars::JsonSchema;
use serde::Deserialize;

use crate::permutations::thing::DataFeedItemOrTextOrThing;
use crate::thing::creative_work::Dataset;

/// A single feed providing structured information about one or more entities or topics.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DataFeed {
    /// An item within a data feed. Data feeds may have many elements.
    pub data_feed_element: DataFeedItemOrTextOrThing,
    #[serde(flatten)]
    pub dataset: Dataset,
}
