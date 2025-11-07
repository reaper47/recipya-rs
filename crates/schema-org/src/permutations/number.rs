use schemars::JsonSchema;
use serde::Deserialize;

use crate::data_type::Number;
use crate::thing::intangible::enumeration::QualitativeValue;

#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum NumberOrQualitativeValue {
    Number(Number),
    QualitativeValue(QualitativeValue),
}

impl Default for NumberOrQualitativeValue {
    fn default() -> Self {
        Self::Number(Number::default())
    }
}
