use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/endTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type RegisterActionEndTimeFieldEnum = String;
///<https://schema.org/startTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type RegisterActionStartTimeFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type RegisterActionAdditionalTypeFieldEnum = String;
///<https://schema.org/RegisterAction>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct RegisterAction {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/location>
    #[serde(rename = "location")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub location: Vec<RegisterActionLocationFieldEnum>,
    ///<https://schema.org/agent>
    #[serde(rename = "agent")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub agent: Vec<RegisterActionAgentFieldEnum>,
    ///<https://schema.org/error>
    #[serde(rename = "error")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub error: Vec<Thing>,
    ///<https://schema.org/target>
    #[serde(rename = "target")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub target: Vec<RegisterActionTargetFieldEnum>,
    ///<https://schema.org/endTime>
    #[serde(rename = "endTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub end_time: Vec<RegisterActionEndTimeFieldEnum>,
    ///<https://schema.org/startTime>
    #[serde(rename = "startTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub start_time: Vec<RegisterActionStartTimeFieldEnum>,
    ///<https://schema.org/result>
    #[serde(rename = "result")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub result: Vec<Thing>,
    ///<https://schema.org/object>
    #[serde(rename = "object")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub object: Vec<Thing>,
    ///<https://schema.org/provider>
    #[serde(rename = "provider")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub provider: Vec<RegisterActionProviderFieldEnum>,
    ///<https://schema.org/participant>
    #[serde(rename = "participant")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub participant: Vec<RegisterActionParticipantFieldEnum>,
    ///<https://schema.org/instrument>
    #[serde(rename = "instrument")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub instrument: Vec<Thing>,
    ///<https://schema.org/actionProcess>
    #[serde(rename = "actionProcess")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub action_process: Vec<HowTo>,
    ///<https://schema.org/actionStatus>
    #[serde(rename = "actionStatus")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub action_status: Vec<ActionStatusTypeEnum>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(rename = "disambiguatingDescription")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(rename = "potentialAction")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(rename = "additionalType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub additional_type: Vec<RegisterActionAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<RegisterActionIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<RegisterActionImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<RegisterActionDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(rename = "alternateName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(rename = "url")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(rename = "subjectOf")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub subject_of: Vec<RegisterActionSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<RegisterActionMainEntityOfPageFieldEnum>,
}
