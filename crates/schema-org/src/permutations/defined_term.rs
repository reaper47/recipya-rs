use std::fmt::Formatter;

use schemars::JsonSchema;
use serde::de::{Error, MapAccess};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;
use url::Url;

use crate::data_type::text::URL;
use crate::permutations::helpers::{has_defined_term_set_properties, parse_url_variant};
use crate::thing::creative_work::DefinedTermSet;
use crate::thing::intangible::enumeration::{
    Enumeration, MeasurementMethodEnum, MeasurementTypeEnumeration,
};
use crate::thing::intangible::structured_value::{
    PropertyValue, QuantitativeValue, StructuredValue,
};

#[derive(Debug, PartialEq, JsonSchema)]
pub enum DefinedTermSetOrURL {
    DefinedTermSet(DefinedTermSet),
    URL(URL),
}

impl<'de> Deserialize<'de> for DefinedTermSetOrURL {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DefinedTermSetOrURL;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a URL as a string or an EntryPoint object")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                parse_url_variant(v, Self::Value::URL)
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
                let e = DefinedTermSet::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(DefinedTermSetOrURL::DefinedTermSet(e))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum DefinedTermOrMeasurementMethodEnumOrTextOrURL {
    DefinedTerm(Box<DefinedTermSet>),
    MeasurementMethodEnum(MeasurementMethodEnum),
    Text(String),
    URL(URL),
}

impl Default for DefinedTermOrMeasurementMethodEnumOrTextOrURL {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl<'de> Deserialize<'de> for DefinedTermOrMeasurementMethodEnumOrTextOrURL {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DefinedTermOrMeasurementMethodEnumOrTextOrURL;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter
                    .write_str("DefinedTerm object, MeasurementMethodEnum object, text or a URL")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match Url::parse(v) {
                    Ok(_) => Ok(Self::Value::URL(
                        URL::new(v).map_err(|_| E::custom("invalid url"))?,
                    )),
                    Err(_) => Ok(Self::Value::Text(v.to_string())),
                }
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
                    Some("DefinedTermSet") => try_defined_term_set(value),
                    Some("MeasurementMethodEnum") => try_measurement_method_enum(value),
                    _ => {
                        if has_defined_term_set_properties(&value) {
                            try_defined_term_set(value)
                        } else {
                            try_measurement_method_enum(value)
                        }
                    }
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

fn try_defined_term_set<'de, E>(
    v: Value,
) -> Result<DefinedTermOrMeasurementMethodEnumOrTextOrURL, E>
where
    E: Error,
{
    Ok(DefinedTermOrMeasurementMethodEnumOrTextOrURL::DefinedTerm(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

fn try_measurement_method_enum<'de, E>(
    v: Value,
) -> Result<DefinedTermOrMeasurementMethodEnumOrTextOrURL, E>
where
    E: Error,
{
    Ok(
        DefinedTermOrMeasurementMethodEnumOrTextOrURL::MeasurementMethodEnum(
            serde_json::from_value(v).map_err(E::custom)?,
        ),
    )
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum DefinedTermOrEnumerationOrMeasurementTypeEnumerationOrPropertyValueOrQualitativeValueOrQuantitativeValueOrStructuredValueOrText
{
    DefinedTerm(DefinedTermSet),
    Enumeration(Enumeration),
    MeasurementTypeEnumeration(MeasurementTypeEnumeration),
    PropertyValue(PropertyValue),
    QualitativeValue(QualitativeValue),
    QuantitativeValue(QuantitativeValue),
    StructuredValue(StructuredValue),
    Text(String),
}

impl Default for DefinedTermOrEnumerationOrMeasurementTypeEnumerationOrPropertyValueOrQualitativeValueOrQuantitativeValueOrStructuredValueOrText {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl<'de> Deserialize<'de> for DefinedTermOrEnumerationOrMeasurementTypeEnumerationOrPropertyValueOrQualitativeValueOrQuantitativeValueOrStructuredValueOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DefinedTermOrEnumerationOrMeasurementTypeEnumerationOrPropertyValueOrQualitativeValueOrQuantitativeValueOrStructuredValueOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a URL as a string or an EntryPoint object")
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
                todo!()
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}
