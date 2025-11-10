use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::field::*;
///<https://schema.org/endTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type ExerciseActionEndTimeFieldEnum = String;
///<https://schema.org/startTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type ExerciseActionStartTimeFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ExerciseActionAdditionalTypeFieldEnum = String;
///<https://schema.org/ExerciseAction>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ExerciseAction {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/distance>
    #[serde(rename = "distance")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub distance: Vec<Distance>,
    ///<https://schema.org/diet>
    #[serde(rename = "diet")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub diet: Vec<Diet>,
    ///<https://schema.org/sportsActivityLocation>
    #[serde(rename = "sportsActivityLocation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sports_activity_location: Vec<SportsActivityLocation>,
    ///<https://schema.org/exerciseRelatedDiet>
    #[serde(rename = "exerciseRelatedDiet")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exercise_related_diet: Vec<Diet>,
    ///<https://schema.org/toLocation>
    #[serde(rename = "toLocation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub to_location: Vec<Place>,
    ///<https://schema.org/sportsEvent>
    #[serde(rename = "sportsEvent")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sports_event: Vec<SportsEvent>,
    ///<https://schema.org/exerciseType>
    #[serde(rename = "exerciseType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exercise_type: Vec<String>,
    ///<https://schema.org/course>
    #[serde(rename = "course")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub course: Vec<Place>,
    ///<https://schema.org/exerciseCourse>
    #[serde(rename = "exerciseCourse")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exercise_course: Vec<Place>,
    ///<https://schema.org/exercisePlan>
    #[serde(rename = "exercisePlan")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exercise_plan: Vec<ExercisePlan>,
    ///<https://schema.org/sportsTeam>
    #[serde(rename = "sportsTeam")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sports_team: Vec<SportsTeam>,
    ///<https://schema.org/fromLocation>
    #[serde(rename = "fromLocation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub from_location: Vec<Place>,
    ///<https://schema.org/opponent>
    #[serde(rename = "opponent")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub opponent: Vec<Person>,
    ///<https://schema.org/audience>
    #[serde(rename = "audience")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub audience: Vec<Audience>,
    ///<https://schema.org/event>
    #[serde(rename = "event")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<Event>,
    ///<https://schema.org/location>
    #[serde(rename = "location")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<ExerciseActionLocationFieldEnum>,
    ///<https://schema.org/agent>
    #[serde(rename = "agent")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub agent: Vec<ExerciseActionAgentFieldEnum>,
    ///<https://schema.org/error>
    #[serde(rename = "error")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub error: Vec<Thing>,
    ///<https://schema.org/target>
    #[serde(rename = "target")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<ExerciseActionTargetFieldEnum>,
    ///<https://schema.org/endTime>
    #[serde(rename = "endTime")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub end_time: Vec<ExerciseActionEndTimeFieldEnum>,
    ///<https://schema.org/startTime>
    #[serde(rename = "startTime")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub start_time: Vec<ExerciseActionStartTimeFieldEnum>,
    ///<https://schema.org/result>
    #[serde(rename = "result")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<Thing>,
    ///<https://schema.org/object>
    #[serde(rename = "object")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub object: Vec<Thing>,
    ///<https://schema.org/provider>
    #[serde(rename = "provider")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provider: Vec<ExerciseActionProviderFieldEnum>,
    ///<https://schema.org/participant>
    #[serde(rename = "participant")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<ExerciseActionParticipantFieldEnum>,
    ///<https://schema.org/instrument>
    #[serde(rename = "instrument")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instrument: Vec<Thing>,
    ///<https://schema.org/actionProcess>
    #[serde(rename = "actionProcess")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub action_process: Vec<HowTo>,
    ///<https://schema.org/actionStatus>
    #[serde(rename = "actionStatus")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub action_status: Vec<ActionStatusTypeEnum>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(rename = "potentialAction")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(rename = "additionalType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<ExerciseActionAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<ExerciseActionIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<ExerciseActionImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<ExerciseActionDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<ExerciseActionSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<ExerciseActionMainEntityOfPageFieldEnum>,
}
