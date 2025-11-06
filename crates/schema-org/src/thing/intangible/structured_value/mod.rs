pub mod contact_point;
mod geoshape;
mod interaction_counter;
mod monetary_amount;
mod opening_hours_specification;
mod property_value;
mod quantitave_value;

pub use geoshape::*;
pub use interaction_counter::*;
pub use opening_hours_specification::*;
pub use property_value::*;
pub use quantitave_value::*;

pub use contact_point::ContactPoint;

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
