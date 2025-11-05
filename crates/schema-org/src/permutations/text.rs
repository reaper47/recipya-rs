use schemars::JsonSchema;
use serde::Deserialize;
use serde::de::{Error, MapAccess};
use serde::{de, Deserializer};
use url::Url;

use crate::data_type::text::URL;
use crate::thing::intangible::Language;

#[derive(Debug, PartialEq, JsonSchema)]
pub enum LanguageOrText {
    Language(Language),
    Text(String),
}

impl<'de> Deserialize<'de> for LanguageOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = LanguageOrText;

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
                let v = Language::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Self::Value::Language(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}


#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum NumberOrText {
    Number(i32),
    Text(String),
}

impl Default for NumberOrText {
    fn default() -> Self {
        Self::Number(0)
    }
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum TextOrURL {
    Text(String),
    URL(URL),
}

impl Default for TextOrURL {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl<'de> Deserialize<'de> for TextOrURL {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match Url::parse(&s) {
            Ok(u) => Ok(Self::URL(URL::new(u).map_err(Error::custom)?)),
            Err(_) => Ok(Self::Text(s)),
        }
    }
}
