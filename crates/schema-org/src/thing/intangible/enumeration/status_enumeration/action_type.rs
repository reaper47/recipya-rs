use schemars::JsonSchema;
use serde::de::Error;
use serde::{Deserialize, Deserializer, de};
use std::fmt::Formatter;

/// The status of an Action.
#[derive(Debug, PartialEq, JsonSchema)]
pub enum ActionStatusType {
    ActiveActionStatus,
    CompletedActionStatus,
    FailedActionStatus,
    PotentialActionStatus,
}

impl<'de> Deserialize<'de> for ActionStatusType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ActionStatusType;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an ActionStatusType url")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                ActionStatusType::try_from(v).map_err(E::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                ActionStatusType::try_from(v.as_str()).map_err(E::custom)
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl TryFrom<&str> for ActionStatusType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split("/").last().unwrap_or_default() {
            "ActiveActionStatus" => Ok(Self::ActiveActionStatus),
            "CompletedActionStatus" => Ok(Self::CompletedActionStatus),
            "FailedActionStatus" => Ok(Self::FailedActionStatus),
            _ => Err("Invalid ActionStatusType"),
        }
    }
}
