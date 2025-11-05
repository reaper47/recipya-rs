use schemars::JsonSchema;
use serde::{de, Deserialize, Deserializer};
use serde::de::{Error, MapAccess};

use crate::intangible::DataFeedItem;
use crate::Thing;
use crate::permutations::helpers::has_datafeed_properties;

#[derive(Debug, PartialEq, JsonSchema)]
pub enum DataFeedItemOrTextOrThing {
    DataFeedItem(DataFeedItem),
    Text(String),
    Thing(Thing),
}

impl Default for DataFeedItemOrTextOrThing {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl<'de> Deserialize<'de> for DataFeedItemOrTextOrThing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DataFeedItemOrTextOrThing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("DataFeedItem, Text, or Thing")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(DataFeedItemOrTextOrThing::Text(v.to_string()))
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
                let value = serde_json::Value::deserialize(de::value::MapAccessDeserializer::new(map))?;

                let type_hint = value
                    .get("@type")
                    .and_then(|v| v.as_str());

                match type_hint {
                    Some("DataFeedItem") => try_datafeed_item(value),
                    Some("Thing") => try_thing(value),
                    _ => {
                        if has_datafeed_properties(&value) {
                            try_datafeed_item(value)
                        } else {
                            try_thing(value)
                        }
                    }
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

fn try_datafeed_item<'de, E>(v: serde_json::Value) -> Result<DataFeedItemOrTextOrThing, E>
where
    E: Error
{
    Ok(DataFeedItemOrTextOrThing::DataFeedItem(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

fn try_thing<'de, E>(v: serde_json::Value) -> Result<DataFeedItemOrTextOrThing, E>
where
    E: Error,
{
    Ok(DataFeedItemOrTextOrThing::Thing(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}
