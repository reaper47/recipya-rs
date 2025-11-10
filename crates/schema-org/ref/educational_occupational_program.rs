use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::field::*;
///<https://schema.org/endDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type EducationalOccupationalProgramEndDateFieldEnum = String;
///<https://schema.org/applicationDeadline>
///<https://schema.org/Date>
///<https://schema.org/Text>
pub type EducationalOccupationalProgramApplicationDeadlineFieldEnum = String;
///<https://schema.org/startDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type EducationalOccupationalProgramStartDateFieldEnum = String;
///<https://schema.org/educationalProgramMode>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type EducationalOccupationalProgramEducationalProgramModeFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type EducationalOccupationalProgramAdditionalTypeFieldEnum = String;
///<https://schema.org/EducationalOccupationalProgram>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EducationalOccupationalProgram {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/termsPerYear>
    #[serde(rename = "termsPerYear")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub terms_per_year: Vec<f32>,
    ///<https://schema.org/endDate>
    #[serde(rename = "endDate")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub end_date: Vec<EducationalOccupationalProgramEndDateFieldEnum>,
    ///<https://schema.org/offers>
    #[serde(rename = "offers")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offers: Vec<EducationalOccupationalProgramOffersFieldEnum>,
    ///<https://schema.org/timeOfDay>
    #[serde(rename = "timeOfDay")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub time_of_day: Vec<String>,
    ///<https://schema.org/applicationDeadline>
    #[serde(rename = "applicationDeadline")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub application_deadline: Vec<
        EducationalOccupationalProgramApplicationDeadlineFieldEnum,
    >,
    ///<https://schema.org/salaryUponCompletion>
    #[serde(rename = "salaryUponCompletion")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub salary_upon_completion: Vec<MonetaryAmountDistribution>,
    ///<https://schema.org/startDate>
    #[serde(rename = "startDate")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub start_date: Vec<EducationalOccupationalProgramStartDateFieldEnum>,
    ///<https://schema.org/maximumEnrollment>
    #[serde(rename = "maximumEnrollment")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub maximum_enrollment: Vec<i32>,
    ///<https://schema.org/provider>
    #[serde(rename = "provider")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub provider: Vec<EducationalOccupationalProgramProviderFieldEnum>,
    ///<https://schema.org/occupationalCategory>
    #[serde(rename = "occupationalCategory")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub occupational_category: Vec<
        EducationalOccupationalProgramOccupationalCategoryFieldEnum,
    >,
    ///<https://schema.org/programPrerequisites>
    #[serde(rename = "programPrerequisites")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub program_prerequisites: Vec<
        EducationalOccupationalProgramProgramPrerequisitesFieldEnum,
    >,
    ///<https://schema.org/termDuration>
    #[serde(rename = "termDuration")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub term_duration: Vec<Duration>,
    ///<https://schema.org/numberOfCredits>
    #[serde(rename = "numberOfCredits")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub number_of_credits: Vec<EducationalOccupationalProgramNumberOfCreditsFieldEnum>,
    ///<https://schema.org/timeToComplete>
    #[serde(rename = "timeToComplete")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub time_to_complete: Vec<Duration>,
    ///<https://schema.org/dayOfWeek>
    #[serde(rename = "dayOfWeek")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub day_of_week: Vec<DayOfWeekEnum>,
    ///<https://schema.org/financialAidEligible>
    #[serde(rename = "financialAidEligible")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub financial_aid_eligible: Vec<
        EducationalOccupationalProgramFinancialAidEligibleFieldEnum,
    >,
    ///<https://schema.org/programType>
    #[serde(rename = "programType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub program_type: Vec<EducationalOccupationalProgramProgramTypeFieldEnum>,
    ///<https://schema.org/trainingSalary>
    #[serde(rename = "trainingSalary")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub training_salary: Vec<MonetaryAmountDistribution>,
    ///<https://schema.org/occupationalCredentialAwarded>
    #[serde(rename = "occupationalCredentialAwarded")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub occupational_credential_awarded: Vec<
        EducationalOccupationalProgramOccupationalCredentialAwardedFieldEnum,
    >,
    ///<https://schema.org/applicationStartDate>
    #[serde(rename = "applicationStartDate")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub application_start_date: Vec<String>,
    ///<https://schema.org/educationalCredentialAwarded>
    #[serde(rename = "educationalCredentialAwarded")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub educational_credential_awarded: Vec<
        EducationalOccupationalProgramEducationalCredentialAwardedFieldEnum,
    >,
    ///<https://schema.org/typicalCreditsPerTerm>
    #[serde(rename = "typicalCreditsPerTerm")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub typical_credits_per_term: Vec<
        EducationalOccupationalProgramTypicalCreditsPerTermFieldEnum,
    >,
    ///<https://schema.org/hasCourse>
    #[serde(rename = "hasCourse")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub has_course: Vec<Course>,
    ///<https://schema.org/educationalProgramMode>
    #[serde(rename = "educationalProgramMode")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub educational_program_mode: Vec<
        EducationalOccupationalProgramEducationalProgramModeFieldEnum,
    >,
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
    pub additional_type: Vec<EducationalOccupationalProgramAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<EducationalOccupationalProgramIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<EducationalOccupationalProgramImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<EducationalOccupationalProgramDescriptionFieldEnum>,
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
    pub subject_of: Vec<EducationalOccupationalProgramSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<
        EducationalOccupationalProgramMainEntityOfPageFieldEnum,
    >,
}
