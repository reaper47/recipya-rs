use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/endDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type ScheduleEndDateFieldEnum = String;
///<https://schema.org/endTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type ScheduleEndTimeFieldEnum = String;
///<https://schema.org/startDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type ScheduleStartDateFieldEnum = String;
///<https://schema.org/startTime>
///<https://schema.org/DateTime>
///<https://schema.org/Time>
pub type ScheduleStartTimeFieldEnum = String;
///<https://schema.org/exceptDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type ScheduleExceptDateFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ScheduleAdditionalTypeFieldEnum = String;
///<https://schema.org/Schedule>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Schedule {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/endDate>
    #[serde(rename = "endDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub end_date: Vec<ScheduleEndDateFieldEnum>,
    ///<https://schema.org/endTime>
    #[serde(rename = "endTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub end_time: Vec<ScheduleEndTimeFieldEnum>,
    ///<https://schema.org/startDate>
    #[serde(rename = "startDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub start_date: Vec<ScheduleStartDateFieldEnum>,
    ///<https://schema.org/byMonthDay>
    #[serde(rename = "byMonthDay")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub by_month_day: Vec<i32>,
    ///<https://schema.org/startTime>
    #[serde(rename = "startTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub start_time: Vec<ScheduleStartTimeFieldEnum>,
    ///<https://schema.org/byMonthWeek>
    #[serde(rename = "byMonthWeek")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub by_month_week: Vec<i32>,
    ///<https://schema.org/duration>
    #[serde(rename = "duration")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub duration: Vec<ScheduleDurationFieldEnum>,
    ///<https://schema.org/byMonth>
    #[serde(rename = "byMonth")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub by_month: Vec<i32>,
    ///<https://schema.org/exceptDate>
    #[serde(rename = "exceptDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub except_date: Vec<ScheduleExceptDateFieldEnum>,
    ///<https://schema.org/repeatCount>
    #[serde(rename = "repeatCount")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub repeat_count: Vec<i32>,
    ///<https://schema.org/byDay>
    #[serde(rename = "byDay")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub by_day: Vec<ScheduleByDayFieldEnum>,
    ///<https://schema.org/scheduleTimezone>
    #[serde(rename = "scheduleTimezone")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub schedule_timezone: Vec<String>,
    ///<https://schema.org/repeatFrequency>
    #[serde(rename = "repeatFrequency")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub repeat_frequency: Vec<ScheduleRepeatFrequencyFieldEnum>,
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
    pub additional_type: Vec<ScheduleAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<ScheduleIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<ScheduleImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<ScheduleDescriptionFieldEnum>,
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
    pub subject_of: Vec<ScheduleSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<ScheduleMainEntityOfPageFieldEnum>,
}
