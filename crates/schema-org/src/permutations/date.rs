use schemars::JsonSchema;
use serde::{Deserialize, Deserializer};

use crate::data_type::{Date, DateTime};

#[derive(Debug, PartialEq, JsonSchema)]
pub enum DateOrDateTime {
    Date(Date),
    DateTime(DateTime),
}

impl Default for DateOrDateTime {
    fn default() -> Self {
        Self::Date(Date::default())
    }
}

impl<'de> Deserialize<'de> for DateOrDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.to_lowercase().contains('t') {
            let dt = DateTime::new(s).map_err(serde::de::Error::custom)?;
            return Ok(Self::DateTime(dt));
        }

        match iso8601::datetime(&s) {
            Ok(parsed) => {
                let has_time = parsed.time.hour != 0
                    || parsed.time.minute != 0
                    || parsed.time.second != 0
                    || parsed.time.millisecond != 0;

                if has_time {
                    DateTime::new(s)
                        .map(Self::DateTime)
                        .map_err(serde::de::Error::custom)
                } else {
                    Date::new(s)
                        .map(Self::Date)
                        .map_err(serde::de::Error::custom)
                }
            }
            Err(_) => Date::new(s).map(Self::Date).map_err(|err| {
                serde::de::Error::custom(format!("Invalid date or datetime format: {err}"))
            }),
        }
    }
}
