use schemars::JsonSchema;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::permutations::helpers::has_person_properties;
use crate::thing::Organization;
use crate::thing::place::EducationalOrganization;

#[derive(Debug, PartialEq, JsonSchema)]
pub enum EducationalOrganizationOrOrganization {
    EducationalOrganization(EducationalOrganization),
    Organization(Organization),
}

impl Default for EducationalOrganizationOrOrganization {
    fn default() -> Self {
        Self::Organization(Organization::default())
    }
}

impl<'de> Deserialize<'de> for EducationalOrganizationOrOrganization {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let type_hint = value.get("@type").and_then(|v| v.as_str());

        match type_hint {
            Some("EducationalOrganization") => try_educational_organization(value),
            Some("Organization") => try_organization(value),
            _ => {
                if value.get("alumni").is_some() {
                    try_educational_organization(value)
                } else {
                    try_organization(value)
                }
            }
        }
    }
}

fn try_educational_organization<'de, E>(
    v: Value,
) -> Result<EducationalOrganizationOrOrganization, E>
where
    E: Error,
{
    Ok(
        EducationalOrganizationOrOrganization::EducationalOrganization(
            serde_json::from_value(v).map_err(E::custom)?,
        ),
    )
}

fn try_organization<'de, E>(v: Value) -> Result<EducationalOrganizationOrOrganization, E>
where
    E: Error,
{
    Ok(EducationalOrganizationOrOrganization::Organization(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}
