use schemars::JsonSchema;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::permutations::helpers::has_creative_work_properties;
use crate::thing::creative_work::WebPageElement;
use crate::thing::intangible::SpeakableSpecification;

#[derive(Debug, PartialEq, JsonSchema)]
pub enum SpeakableSpecificationOrWebPageElement {
    SpeakableSpecification(SpeakableSpecification),
    WebPageElement(WebPageElement),
}

impl Default for SpeakableSpecificationOrWebPageElement {
    fn default() -> Self {
        Self::SpeakableSpecification(Default::default())
    }
}

impl<'de> Deserialize<'de> for SpeakableSpecificationOrWebPageElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let type_hint = value.get("@type").and_then(|v| v.as_str());

        match type_hint {
            Some("SpeakableSpecification") => try_speakable(value),
            Some("WebPageElement") => try_webpage(value),
            _ => {
                if has_creative_work_properties(&value) {
                    try_webpage(value)
                } else {
                    try_speakable(value)
                }
            }
        }
    }
}

fn try_speakable<'de, E>(v: Value) -> Result<SpeakableSpecificationOrWebPageElement, E>
where
    E: Error,
{
    Ok(
        SpeakableSpecificationOrWebPageElement::SpeakableSpecification(
            serde_json::from_value(v).map_err(E::custom)?,
        ),
    )
}

fn try_webpage<'de, E>(v: Value) -> Result<SpeakableSpecificationOrWebPageElement, E>
where
    E: Error,
{
    Ok(SpeakableSpecificationOrWebPageElement::WebPageElement(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}
