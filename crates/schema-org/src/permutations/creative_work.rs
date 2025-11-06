use schemars::JsonSchema;
use serde::de::{Error, MapAccess};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::permutations::helpers::has_creative_work_properties;
use crate::thing::{CreativeWork, Event};

#[derive(Debug, PartialEq, JsonSchema)]
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

        let type_hint = value.get("@type").and_then(|v| v.as_str());

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
    Ok(CreativeWorkOrEvent::CreativeWork(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

fn try_event<'de, E>(v: Value) -> Result<CreativeWorkOrEvent, E>
where
    E: Error,
{
    Ok(CreativeWorkOrEvent::Event(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum CreativeWorkOrText {
    CreativeWork(CreativeWork),
    Text(String),
}

impl Default for CreativeWorkOrText {
    fn default() -> Self {
        CreativeWorkOrText::Text(String::new())
    }
}

impl<'de> Deserialize<'de> for CreativeWorkOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = CreativeWorkOrText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a string or a Language object")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Text(value.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.visit_str(&v)
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let v = CreativeWork::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Self::Value::CreativeWork(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}
