use std::borrow::Cow;

use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Date(pub iso8601::Date);

impl JsonSchema for Date {
    fn schema_name() -> Cow<'static, str> {
        "Iso8601Date".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        String::json_schema(generator)
    }
}
