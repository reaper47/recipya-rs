use std::fmt::Formatter;

use schemars::JsonSchema;
use serde::de::{Error, MapAccess, SeqAccess};
use serde::{Deserialize, Deserializer, de};

use crate::permutations::date::DateOrDateTime;
use crate::permutations::place::PlaceOrPostalAddressOrTextOrVirtualLocation;
use crate::permutations::url::EntryPointOrURL;
use crate::Thing;
use crate::thing::creative_work::HowTo;

/// Enumeration of all possible action values.
#[derive(Debug, PartialEq, JsonSchema)]
pub enum ActionItemOrItems {
    Item(Action),
    Items(Vec<Action>),
}

/// An action performed by a direct agent and indirect participants upon a direct object. Optionally
/// happens at a location with the help of an inanimate instrument. The execution of the action
/// may produce a result. Specific action sub-type documentation specifies the exact expectation
/// of each argument/role.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Action {
    /// The type of schema object.
    #[serde(rename = "@type")]
    pub r#type: Option<String>,

    /// Description of the process by which the action was performed.
    pub action_process: Option<Box<HowTo>>,

    /// Indicates the current disposition of the Action.
    #[serde(default, deserialize_with = "deserialize_option")]
    pub action_status: Option<ActionStatusType>,

    /// The direct performer or driver of the action (animate or inanimate). E.g. John wrote a book.
    pub agent: Option<OrganizationOrPerson>,

    /// The endTime of something. For a reserved event or service (e.g.
    /// FoodEstablishmentReservation), the time that it is expected to end. For actions that span a
    /// period of time, when the action was performed. E.g. John wrote a book from January to
    /// December. For media, including audio and video, it's the time offset of the end of a clip
    /// within a larger file.
    ///
    /// Note that Event uses startDate/endDate instead of startTime/endTime, even when describing
    /// dates with times. This situation may be clarified in future revisions.
    pub end_time: Option<DateOrDateTime>,

    /// For failed actions, more information on the cause of the failure.
    pub error: Option<Box<Thing>>,

    /// The object that helped the agent perform the action. E.g. John wrote a book with a pen.
    pub instrument: Option<Box<Thing>>,

    /// The location of, for example, where an event is happening, where an organization is
    /// located, or where an action takes place.
    pub location: Option<PlaceOrPostalAddressOrTextOrVirtualLocation>,

    /// The object upon which the action is carried out, whose state is kept intact or changed.
    /// Also known as the semantic roles patient, affected or undergoer (which change their state)
    /// or theme (which doesn't). E.g. John read a book.
    pub object: Option<OrganizationOrPerson>,

    /// Other co-agents that participated in the action indirectly. E.g. John wrote a book with
    /// Steve.
    pub participant: Option<OrganizationOrPerson>,

    /// The service provider, service operator, or service performer; the goods producer. Another
    /// party (a seller) may offer those services or goods on behalf of the provider. A provider
    /// may also serve as the seller. Supersedes carrier.
    pub provider: Option<OrganizationOrPerson>,

    /// The result produced in the action. E.g. John wrote a book.
    pub result: Option<Box<Thing>>,

    /// The startTime of something. For a reserved event or service
    /// (e.g. FoodEstablishmentReservation), the time that it is expected to start. For actions that
    /// span a period of time, when the action was performed. E.g. John wrote a book from January to
    /// December. For media, including audio and video, it's the time offset of the start of a clip
    /// within a larger file.
    ///
    /// Note that Event uses startDate/endDate instead of startTime/endTime, even when describing
    /// dates with times. This situation may be clarified in future revisions.
    pub start_time: Option<DateOrDateTime>,

    /// Indicates a target EntryPoint, or url, for an Action.
    pub target: Option<EntryPointOrURL>,

    #[serde(flatten)]
    pub thing: Option<Box<Thing>>,
}

impl<'de> Deserialize<'de> for ActionItemOrItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ActionItemOrItems;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an Action or an array of Actions")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec: Vec<Action> = Vec::new();
                while let Some(action) = seq.next_element::<Action>()? {
                    vec.push(action);
                }
                Ok(ActionItemOrItems::Items(vec))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let action = Action::deserialize(de::value::MapAccessDeserializer::new(map))?;
                Ok(ActionItemOrItems::Item(action))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

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
