use schemars::JsonSchema;
use serde::{de, Deserialize, Deserializer};
use serde::de::{Error, MapAccess};

use crate::thing::{Event, Organization, Person};
use crate::thing::BioChemEntity;
use crate::thing::{CreativeWork, MedicalEntity};
use crate::thing::Product;

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
    BioChemEntity(BioChemEntity),
    CreativeWork(CreativeWork),
    Event(Event),
    MedicalEntity(MedicalEntity),
    Organization(Organization),
    Person(Person),
    Product(Product),
}

impl Default for BioChemEntityOrCreativeWorkOrEventOrMedicalEntityOrOrganizationOrPersonOrProduct {
    fn default() -> Self {
        Self::BioChemEntity(BioChemEntity::default())
    }
}
