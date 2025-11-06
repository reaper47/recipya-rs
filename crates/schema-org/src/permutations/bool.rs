use schemars::JsonSchema;
use std::fmt::Formatter;

use serde::de::{Error, MapAccess};
use serde::{Deserialize, Deserializer, de};

use crate::thing::intangible::structured_value::StructuredValue;

#[derive(Debug, PartialEq, JsonSchema)]
pub enum BooleanOrNumberOrStructuredValueOrText {
    Boolean(bool),
    Number(i32),
    StructuredValue(StructuredValue),
    Text(String),
}

impl Default for BooleanOrNumberOrStructuredValueOrText {
    fn default() -> Self {
        Self::Boolean(false)
    }
}

impl<'de> Deserialize<'de> for BooleanOrNumberOrStructuredValueOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = BooleanOrNumberOrStructuredValueOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a boolean, number, StructuredValue object, or text")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Boolean(v))
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v as i32))
            }

            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v as i32))
            }

            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v as i32))
            }

            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v as i32))
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v as i32))
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v as i32))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v as i32))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v as i32))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v as i32))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v as i32))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Number(v as i32))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::Text(v.to_string()))
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
                let v = StructuredValue::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Self::Value::StructuredValue(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}
