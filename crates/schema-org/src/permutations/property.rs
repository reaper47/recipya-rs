use schemars::JsonSchema;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::permutations::helpers::has_property_properties;
use crate::thing::intangible::enumeration::Enumeration;
use crate::thing::intangible::{Class, Property};

#[derive(Debug, PartialEq, JsonSchema)]
pub enum ClassOrEnumerationOrProperty {
    Class(Class),
    Enumeration(Enumeration),
    Property(Property),
}

impl Default for ClassOrEnumerationOrProperty {
    fn default() -> Self {
        Self::Class(Class::default())
    }
}

impl<'de> Deserialize<'de> for ClassOrEnumerationOrProperty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let type_hint = value.get("@type").and_then(|v| v.as_str());

        match type_hint {
            Some("Class") => try_class(value),
            Some("Enumeration") => try_enumeration(value),
            Some("Property") => try_property(value),
            _ => {
                if has_property_properties(&value) {
                    try_property(value)
                } else {
                    try_class(value.clone())
                        .or_else(|_| try_enumeration(value.clone()))
                        .or_else(|_| try_property(value))
                }
            }
        }
    }
}

fn try_class<'de, E>(v: Value) -> Result<ClassOrEnumerationOrProperty, E>
where
    E: Error,
{
    Ok(ClassOrEnumerationOrProperty::Class(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

fn try_enumeration<'de, E>(v: Value) -> Result<ClassOrEnumerationOrProperty, E>
where
    E: Error,
{
    Ok(ClassOrEnumerationOrProperty::Enumeration(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

fn try_property<'de, E>(v: Value) -> Result<ClassOrEnumerationOrProperty, E>
where
    E: Error,
{
    Ok(ClassOrEnumerationOrProperty::Property(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}
