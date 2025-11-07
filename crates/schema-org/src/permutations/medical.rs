use std::fmt::Formatter;

use schemars::JsonSchema;
use serde::de::{Error, MapAccess};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::permutations::helpers::has_medical_condition_properties;
use crate::thing::MedicalEntity;
use crate::thing::intangible::enumeration::medical_enumeration::MedicalStudyStatus;
use crate::thing::intangible::enumeration::status_enumeration::EventStatusType;
use crate::thing::intangible::structured_value::PropertyValue;
use crate::thing::medical_entity::{MedicalCondition, MedicalContraindication};

#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum EventStatusTypeOrMedicalStudyStatusOrText {
    EventStatusType(EventStatusType),
    MedicalStudyStatus(MedicalStudyStatus),
    Text(String),
}

impl Default for EventStatusTypeOrMedicalStudyStatusOrText {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum MedicalContraindicationOrText {
    MedicalContraindication(MedicalContraindication),
    Text(String),
}

impl Default for MedicalContraindicationOrText {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl<'de> Deserialize<'de> for MedicalContraindicationOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = MedicalContraindicationOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("MedicalContraindication or Text")
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
                let v = MedicalContraindication::deserialize(
                    de::value::MapAccessDeserializer::new(map),
                )?;
                Ok(Self::Value::MedicalContraindication(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum MedicalEntityOrText {
    MedicalEntity(MedicalEntity),
    Text(String),
}

impl Default for MedicalEntityOrText {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl<'de> Deserialize<'de> for MedicalEntityOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = MedicalEntityOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("MedicalContraindication or Text")
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
                let v = MedicalEntity::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Self::Value::MedicalEntity(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum MedicalConditionOrPropertyValueOrURL {
    MedicalCondition(MedicalCondition),
    PropertyValue(PropertyValue),
    URL(String),
}

impl Default for MedicalConditionOrPropertyValueOrURL {
    fn default() -> Self {
        Self::URL(String::new())
    }
}

impl<'de> Deserialize<'de> for MedicalConditionOrPropertyValueOrURL {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = MedicalConditionOrPropertyValueOrURL;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("AdministrativeArea, GeoShape, Place or Text")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Self::Value::URL(v.to_string()))
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
                let value = Value::deserialize(de::value::MapAccessDeserializer::new(map))?;

                let type_hint = value.get("@type").and_then(|v| v.as_str());

                match type_hint {
                    Some("MedicalCondition") => try_medical_condition(value),
                    Some("PropertyValue") => try_property_medical_condition(value),
                    _ => {
                        if has_medical_condition_properties(&value) {
                            try_medical_condition(value)
                        } else {
                            try_property_medical_condition(value)
                        }
                    }
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

fn try_medical_condition<'de, E>(v: Value) -> Result<MedicalConditionOrPropertyValueOrURL, E>
where
    E: Error,
{
    Ok(MedicalConditionOrPropertyValueOrURL::MedicalCondition(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

fn try_property_medical_condition<'de, E>(
    v: Value,
) -> Result<MedicalConditionOrPropertyValueOrURL, E>
where
    E: Error,
{
    Ok(MedicalConditionOrPropertyValueOrURL::PropertyValue(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}
