use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/validFrom>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type OpeningHoursSpecificationValidFromFieldEnum = String;
///<https://schema.org/validThrough>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type OpeningHoursSpecificationValidThroughFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type OpeningHoursSpecificationAdditionalTypeFieldEnum = String;
///<https://schema.org/OpeningHoursSpecification>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct OpeningHoursSpecification {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/closes>
    #[serde(rename = "closes")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub closes: Vec<String>,
    ///<https://schema.org/dayOfWeek>
    #[serde(rename = "dayOfWeek")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub day_of_week: Vec<DayOfWeekEnum>,
    ///<https://schema.org/opens>
    #[serde(rename = "opens")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub opens: Vec<String>,
    ///<https://schema.org/validFrom>
    #[serde(rename = "validFrom")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub valid_from: Vec<OpeningHoursSpecificationValidFromFieldEnum>,
    ///<https://schema.org/validThrough>
    #[serde(rename = "validThrough")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub valid_through: Vec<OpeningHoursSpecificationValidThroughFieldEnum>,
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
    pub additional_type: Vec<OpeningHoursSpecificationAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<OpeningHoursSpecificationIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<OpeningHoursSpecificationImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<OpeningHoursSpecificationDescriptionFieldEnum>,
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
    pub subject_of: Vec<OpeningHoursSpecificationSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<OpeningHoursSpecificationMainEntityOfPageFieldEnum>,
}
