use std::fmt::Formatter;

use schemars::JsonSchema;
use serde::de::Error;
use serde::{Deserialize, Deserializer, de};

/// The status of a medical study. Enumerated type.
#[derive(Debug, Default, PartialEq, JsonSchema)]
pub enum MedicalStudyStatus {
    ActiveNotRecruiting,
    Completed,
    EnrollingByInvitation,
    NotYetRecruiting,
    Recruiting,
    ResultsAvailable,
    #[default]
    ResultsNotAvailable,
    Suspended,
    Terminated,
    Withdrawn,
}

impl<'de> Deserialize<'de> for MedicalStudyStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = MedicalStudyStatus;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a MedicalStudyStatus url")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                MedicalStudyStatus::try_from(v).map_err(E::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                MedicalStudyStatus::try_from(v.as_str()).map_err(E::custom)
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl TryFrom<&str> for MedicalStudyStatus {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split("/").last().unwrap_or_default() {
            "ActiveNotRecruiting" => Ok(Self::ActiveNotRecruiting),
            "Completed" => Ok(Self::Completed),
            "EnrollingByInvitation" => Ok(Self::EnrollingByInvitation),
            "NotYetRecruiting" => Ok(Self::NotYetRecruiting),
            "Recruiting" => Ok(Self::Recruiting),
            "ResultsAvailable" => Ok(Self::ResultsAvailable),
            "ResultsNotAvailable" => Ok(Self::ResultsNotAvailable),
            "Suspended" => Ok(Self::Suspended),
            "Terminated" => Ok(Self::Terminated),
            "Withdrawn" => Ok(Self::Withdrawn),
            _ => Err("Invalid MedicalStudyStatus"),
        }
    }
}
