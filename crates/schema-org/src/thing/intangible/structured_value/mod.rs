mod contact_point;
mod geoshape;
mod opening_hours_specification;
mod quantitave_value;
mod property_value;
mod monetary_amount;
mod interaction_counter;

pub use contact_point::*;
pub use geoshape::*;
pub use opening_hours_specification::*;
pub use quantitave_value::*;
pub use property_value::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;

/// Structured values are used when the value of a property has a more complex structure than simply
/// being a textual value or a reference to another thing.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StructuredValue {
    #[serde(flatten)]
    pub thing: Thing,
}
