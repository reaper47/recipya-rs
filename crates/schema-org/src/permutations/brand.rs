use schemars::JsonSchema;
use serde::Deserialize;
use serde::de::Error;
use serde_json::Value;

use crate::permutations::helpers::has_organization_properties;
use crate::thing::Organization;
use crate::thing::intangible::brand::Brand;

#[derive(Debug, PartialEq, JsonSchema)]
pub enum BrandOrOrganization {
    Brand(Brand),
    Organization(Organization),
}

impl Default for BrandOrOrganization {
    fn default() -> Self {
        Self::Brand(Default::default())
    }
}

impl<'de> Deserialize<'de> for BrandOrOrganization {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let type_hint = value.get("@type").and_then(|v| v.as_str());

        match type_hint {
            Some("Brand") => try_brand(value),
            Some("Organization") => try_organization(value),
            _ => {
                if has_organization_properties(&value) {
                    try_organization(value)
                } else {
                    try_brand(value)
                }
            }
        }
    }
}

fn try_brand<'de, E>(v: Value) -> Result<BrandOrOrganization, E>
where
    E: Error,
{
    Ok(BrandOrOrganization::Brand(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

fn try_organization<'de, E>(v: Value) -> Result<BrandOrOrganization, E>
where
    E: Error,
{
    Ok(BrandOrOrganization::Organization(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}
