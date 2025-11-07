use std::fmt::Formatter;

use schemars::JsonSchema;
use serde::de::Error;
use serde::{Deserialize, Deserializer, de};

/// EventStatusType is an enumeration type whose instances represent several states that an Event
/// may be in.
#[derive(Debug, Default, PartialEq, JsonSchema)]
pub enum EventStatusType {
    EventCancelled,
    EventMovedOnline,
    EventPostponed,
    EventRescheduled,
    #[default]
    EventScheduled,
}

impl<'de> Deserialize<'de> for EventStatusType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = EventStatusType;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an ActionStatusType url")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                EventStatusType::try_from(v).map_err(E::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error,
            {
                EventStatusType::try_from(v.as_str()).map_err(E::custom)
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl TryFrom<&str> for EventStatusType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split("/").last().unwrap_or_default() {
            "EventCancelled" => Ok(Self::EventCancelled),
            "EventMovedOnline" => Ok(Self::EventMovedOnline),
            "EventPostponed" => Ok(Self::EventPostponed),
            "EventRescheduled" => Ok(Self::EventRescheduled),
            "EventScheduled" => Ok(Self::EventScheduled),
            _ => Err("Invalid EventStatusType"),
        }
    }
}
