use schemars::JsonSchema;
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use serde_json::Value;

use crate::permutations::helpers::has_creative_work_properties;
use crate::thing::{CreativeWork, Event};

#[derive(Debug, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum CreativeWorkOrEvent {
    CreativeWork(CreativeWork),
    Event(Event),
}

impl<'de> Deserialize<'de> for CreativeWorkOrEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let type_hint = value
            .get("@type")
            .and_then(|v| v.as_str());

        match type_hint {
            Some("CreativeWork") => try_creative_work(value),
            Some("Event") => try_event(value),
            _ => {
                if has_creative_work_properties(&value) {
                    try_event(value)
                } else {
                    try_creative_work(value)
                }
            }
        }
    }
}

fn try_creative_work<'de, E>(v: Value) -> Result<CreativeWorkOrEvent, E>
where
    E: Error,
{
    Ok(CreativeWorkOrEvent::CreativeWork(serde_json::from_value(v).map_err(E::custom)?))
}

fn try_event<'de, E>(v: Value) -> Result<CreativeWorkOrEvent, E>
where
    E: Error,
{
    Ok(CreativeWorkOrEvent::Event(serde_json::from_value(v).map_err(E::custom)?))
}
