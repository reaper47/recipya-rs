use schemars::JsonSchema;
use serde::de::{Error, MapAccess};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::permutations::helpers::{
    has_bio_chem_entity_properties, has_creative_work_properties, has_event_properties,
    has_medical_entity_properties, has_organization_properties, has_person_properties,
};
use crate::thing::BioChemEntity;
use crate::thing::Product;
use crate::thing::{CreativeWork, MedicalEntity};
use crate::thing::{Event, Organization, Person};

#[derive(Debug, PartialEq, JsonSchema)]
pub enum ProductOrText {
    Product(Product),
    Text(String),
}

impl Default for ProductOrText {
    fn default() -> Self {
        Self::Text(String::default())
    }
}

impl<'de> Deserialize<'de> for ProductOrText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ProductOrText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("ProductOrText")
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
                let v = Product::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(Self::Value::Product(v))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct {
    BioChemEntity(Box<BioChemEntity>),
    CreativeWork(CreativeWork),
    Event(Event),
    MedicalEntity(MedicalEntity),
    Organization(Organization),
    Person(Person),
    Product(Product),
}

impl Default for BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct {
    fn default() -> Self {
        Self::BioChemEntity(Box::new(BioChemEntity::default()))
    }
}

impl<'de> Deserialize<'de>
    for BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let type_hint = value.get("@type").and_then(|v| v.as_str());

        match type_hint {
            Some("BioChemEntity") => try_bio_chem_entity(value),
            Some("CreativeWork") => try_creative_work(value),
            Some("Event") => try_event(value),
            Some("MedicalEntity") => try_medical_event(value),
            Some("Organization") => try_organization(value),
            Some("Person") => try_person(value),
            Some("Product") => try_product(value),
            _ => {
                if has_bio_chem_entity_properties(&value) {
                    try_event(value)
                } else if has_creative_work_properties(&value) {
                    try_creative_work(value)
                } else if has_event_properties(&value) {
                    try_event(value)
                } else if has_medical_entity_properties(&value) {
                    try_medical_event(value)
                } else if has_organization_properties(&value) {
                    try_organization(value)
                } else if has_person_properties(&value) {
                    try_person(value)
                } else {
                    try_product(value)
                }
            }
        }
    }
}

fn try_bio_chem_entity<'de, E, T>(
    v: Value,
) -> Result<BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct, E>
where
    E: Error,
{
    Ok(BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct::BioChemEntity(serde_json::from_value(v).map_err(E::custom)?))
}

fn try_creative_work<'de, E, T>(
    v: Value,
) -> Result<BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct, E>
where
    E: Error,
{
    Ok(BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct::CreativeWork(serde_json::from_value(v).map_err(E::custom)?))
}

fn try_event<'de, E, T>(
    v: Value,
) -> Result<BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct, E>
where
    E: Error,
{
    Ok(
        BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct::Event(
            serde_json::from_value(v).map_err(E::custom)?,
        ),
    )
}

fn try_medical_event<'de, E, T>(
    v: Value,
) -> Result<BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct, E>
where
    E: Error,
{
    Ok(BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct::MedicalEntity(serde_json::from_value(v).map_err(E::custom)?))
}

fn try_organization<'de, E, T>(
    v: Value,
) -> Result<BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct, E>
where
    E: Error,
{
    Ok(BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct::Organization(serde_json::from_value(v).map_err(E::custom)?))
}

fn try_person<'de, E, T>(
    v: Value,
) -> Result<BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct, E>
where
    E: Error,
{
    Ok(
        BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct::Person(
            serde_json::from_value(v).map_err(E::custom)?,
        ),
    )
}

fn try_product<'de, E, T>(
    v: Value,
) -> Result<BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct, E>
where
    E: Error,
{
    Ok(
        BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct::Product(
            serde_json::from_value(v).map_err(E::custom)?,
        ),
    )
}
