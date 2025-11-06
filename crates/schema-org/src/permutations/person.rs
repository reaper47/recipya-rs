use schemars::JsonSchema;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::permutations::helpers::has_person_properties;
use crate::thing::{Organization, Person};

#[derive(Debug, PartialEq, JsonSchema)]
pub enum OrganizationOrPerson {
    Organization(Organization),
    Person(Person),
}

impl Default for OrganizationOrPerson {
    fn default() -> Self {
        Self::Person(Person::default())
    }
}

impl<'de> Deserialize<'de> for OrganizationOrPerson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let type_hint = value.get("@type").and_then(|v| v.as_str());

        match type_hint {
            Some("Organization") => try_organization(value),
            Some("Person") => try_person(value),
            _ => {
                if has_person_properties(&value) {
                    try_person(value)
                } else {
                    try_organization(value)
                }
            }
        }
    }
}

pub(crate) fn try_organization<'de, E>(v: Value) -> Result<OrganizationOrPerson, E>
where
    E: Error,
{
    Ok(OrganizationOrPerson::Organization(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

fn try_person<'de, E>(v: Value) -> Result<OrganizationOrPerson, E>
where
    E: Error,
{
    Ok(OrganizationOrPerson::Person(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum PerformingGroupOrPerson {
    PerformingGroup(PerformingGroup),
    Person(Person),
}

impl Default for PerformingGroupOrPerson {
    fn default() -> Self {
        Self::Person(Person::default())
    }
}
