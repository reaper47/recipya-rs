use schemars::JsonSchema;
use serde::Deserialize;
use serde::de::Error;
use serde_json::Value;

use crate::permutations::helpers::{
    has_anatomical_structure_properties, has_anatomical_system_properties,
};
use crate::thing::medical::AnatomicalSystem;
use crate::thing::medical_entity::{AnatomicalStructure, SuperficialAnatomy};

#[derive(Debug, PartialEq, JsonSchema)]
pub enum AnatomicalStructureOrAnatomicalSystem {
    AnatomicalStructure(AnatomicalStructure),
    AnatomicalSystem(Box<AnatomicalSystem>),
}

impl Default for AnatomicalStructureOrAnatomicalSystem {
    fn default() -> Self {
        Self::AnatomicalStructure(AnatomicalStructure::default())
    }
}

impl<'de> Deserialize<'de> for AnatomicalStructureOrAnatomicalSystem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let type_hint = value.get("@type").and_then(|v| v.as_str());

        match type_hint {
            Some("AnatomicalStructure") => try_anatomical_structure1(value),
            Some("AnatomicalSystem") => try_anatomical_system1(value),
            _ => {
                if has_anatomical_structure_properties(&value) {
                    try_anatomical_structure1(value)
                } else {
                    try_anatomical_system1(value)
                }
            }
        }
    }
}

fn try_anatomical_structure1<'de, E>(v: Value) -> Result<AnatomicalStructureOrAnatomicalSystem, E>
where
    E: Error,
{
    Ok(AnatomicalStructureOrAnatomicalSystem::AnatomicalStructure(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

fn try_anatomical_system1<'de, E>(v: Value) -> Result<AnatomicalStructureOrAnatomicalSystem, E>
where
    E: Error,
{
    Ok(AnatomicalStructureOrAnatomicalSystem::AnatomicalSystem(
        serde_json::from_value(v).map_err(E::custom)?,
    ))
}

#[derive(Debug, PartialEq, JsonSchema)]
pub enum AnatomicalStructureOrAnatomicalSystemOrSuperficialAnatomy {
    AnatomicalStructure(AnatomicalStructure),
    AnatomicalSystem(AnatomicalSystem),
    SuperficialAnatomy(SuperficialAnatomy),
}

impl Default for AnatomicalStructureOrAnatomicalSystemOrSuperficialAnatomy {
    fn default() -> Self {
        Self::AnatomicalStructure(AnatomicalStructure::default())
    }
}

impl<'de> Deserialize<'de> for AnatomicalStructureOrAnatomicalSystemOrSuperficialAnatomy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let type_hint = value.get("@type").and_then(|v| v.as_str());

        match type_hint {
            Some("AnatomicalStructure") => try_anatomical_structure2(value),
            Some("AnatomicalSystem") => try_anatomical_system2(value),
            Some("SuperficialAnatomy") => try_superficial_anatomy(value),
            _ => {
                if has_anatomical_structure_properties(&value) {
                    try_anatomical_structure2(value)
                } else if has_anatomical_system_properties(&value) {
                    try_anatomical_system2(value)
                } else {
                    try_superficial_anatomy(value)
                }
            }
        }
    }
}

fn try_anatomical_structure2<'de, E>(
    v: Value,
) -> Result<AnatomicalStructureOrAnatomicalSystemOrSuperficialAnatomy, E>
where
    E: Error,
{
    Ok(
        AnatomicalStructureOrAnatomicalSystemOrSuperficialAnatomy::AnatomicalStructure(
            serde_json::from_value(v).map_err(E::custom)?,
        ),
    )
}

fn try_anatomical_system2<'de, E>(
    v: Value,
) -> Result<AnatomicalStructureOrAnatomicalSystemOrSuperficialAnatomy, E>
where
    E: Error,
{
    Ok(
        AnatomicalStructureOrAnatomicalSystemOrSuperficialAnatomy::AnatomicalSystem(
            serde_json::from_value(v).map_err(E::custom)?,
        ),
    )
}

fn try_superficial_anatomy<'de, E>(
    v: Value,
) -> Result<AnatomicalStructureOrAnatomicalSystemOrSuperficialAnatomy, E>
where
    E: Error,
{
    Ok(
        AnatomicalStructureOrAnatomicalSystemOrSuperficialAnatomy::SuperficialAnatomy(
            serde_json::from_value(v).map_err(E::custom)?,
        ),
    )
}
