use std::fmt::Formatter;

use schemars::JsonSchema;
use serde::de::{Error, MapAccess};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::permutations::helpers::{has_geoshape_properties, has_place_properties};
use crate::thing::intangible::structured_value::GeoShape;
use crate::thing::place::{AdministrativeArea, Country, Place};

#[derive(Debug, PartialEq, JsonSchema)]
pub enum CountryOrText {
    Country(Box<Country>),
    Text(String),
}

impl Default for CountryOrText {
    fn default() -> Self {
        Self::Text("".to_string())
    }
}

impl<'de> Deserialize<'de> for CountryOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = CountryOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("Country or Text")
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
                let v = Country::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Self::Value::Country(Box::new(v)))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum AdministrativeAreaOrGeoShapeOrPlaceOrText {
    AdministrativeArea(AdministrativeArea),
    GeoShape(GeoShape),
    Place(Place),
    Text(String),
}

impl Default for AdministrativeAreaOrGeoShapeOrPlaceOrText {
    fn default() -> Self {
        Self::Text("".to_string())
    }
}

impl<'de> Deserialize<'de> for AdministrativeAreaOrGeoShapeOrPlaceOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = AdministrativeAreaOrGeoShapeOrPlaceOrText;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("AdministrativeArea, GeoShape, Place or Text")
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
                let value = Value::deserialize(de::value::MapAccessDeserializer::new(map))?;

                let type_hint = value.get("@type").and_then(|v| v.as_str());

                match type_hint {
                    Some("AdministrativeArea") => try_admin_area(value),
                    Some("GeoShape") => try_geoshape(value),
                    Some("Place") => try_place(value),
                    _ => {
                        if has_place_properties(&value) {
                            try_place(value)
                        } else if has_geoshape_properties(&value) {
                            try_geoshape(value)
                        } else {
                            try_admin_area(value)
                        }
                    }
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

fn try_admin_area<'de, E>(v: Value) -> Result<AdministrativeAreaOrGeoShapeOrPlaceOrText, E>
where
    E: Error,
{
    Ok(
        AdministrativeAreaOrGeoShapeOrPlaceOrText::AdministrativeArea(
            serde_json::from_value(v).map_err(E::custom)?,
        ),
    )
}

fn try_geoshape<'de, E>(v: Value) -> Result<AdministrativeAreaOrGeoShapeOrPlaceOrText, E>
where
    E: Error,
{
    Ok(AdministrativeAreaOrGeoShapeOrPlaceOrText::GeoShape(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

fn try_place<'de, E>(v: Value) -> Result<AdministrativeAreaOrGeoShapeOrPlaceOrText, E>
where
    E: Error,
{
    Ok(AdministrativeAreaOrGeoShapeOrPlaceOrText::Place(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}
