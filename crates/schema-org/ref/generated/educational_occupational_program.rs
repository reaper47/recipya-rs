use crate::*;
use serde_with::{serde_as, OneOrMany};
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
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct EducationalOccupationalProgram {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/termsPerYear>
    #[serde(rename = "termsPerYear")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub terms_per_year: Vec<f32>,
    ///<https://schema.org/endDate>
    #[serde(rename = "endDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub end_date: Vec<EducationalOccupationalProgramEndDateFieldEnum>,
    ///<https://schema.org/offers>
    #[serde(rename = "offers")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub offers: Vec<EducationalOccupationalProgramOffersFieldEnum>,
    ///<https://schema.org/timeOfDay>
    #[serde(rename = "timeOfDay")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub time_of_day: Vec<String>,
    ///<https://schema.org/applicationDeadline>
    #[serde(rename = "applicationDeadline")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub application_deadline: Vec<
        EducationalOccupationalProgramApplicationDeadlineFieldEnum,
    >,
    ///<https://schema.org/salaryUponCompletion>
    #[serde(rename = "salaryUponCompletion")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub salary_upon_completion: Vec<MonetaryAmountDistribution>,
    ///<https://schema.org/startDate>
    #[serde(rename = "startDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub start_date: Vec<EducationalOccupationalProgramStartDateFieldEnum>,
    ///<https://schema.org/maximumEnrollment>
    #[serde(rename = "maximumEnrollment")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub maximum_enrollment: Vec<i32>,
    ///<https://schema.org/provider>
    #[serde(rename = "provider")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub provider: Vec<EducationalOccupationalProgramProviderFieldEnum>,
    ///<https://schema.org/occupationalCategory>
    #[serde(rename = "occupationalCategory")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub occupational_category: Vec<
        EducationalOccupationalProgramOccupationalCategoryFieldEnum,
    >,
    ///<https://schema.org/programPrerequisites>
    #[serde(rename = "programPrerequisites")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub program_prerequisites: Vec<
        EducationalOccupationalProgramProgramPrerequisitesFieldEnum,
    >,
    ///<https://schema.org/termDuration>
    #[serde(rename = "termDuration")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub term_duration: Vec<Duration>,
    ///<https://schema.org/numberOfCredits>
    #[serde(rename = "numberOfCredits")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_credits: Vec<EducationalOccupationalProgramNumberOfCreditsFieldEnum>,
    ///<https://schema.org/timeToComplete>
    #[serde(rename = "timeToComplete")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub time_to_complete: Vec<Duration>,
    ///<https://schema.org/dayOfWeek>
    #[serde(rename = "dayOfWeek")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub day_of_week: Vec<DayOfWeekEnum>,
    ///<https://schema.org/financialAidEligible>
    #[serde(rename = "financialAidEligible")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub financial_aid_eligible: Vec<
        EducationalOccupationalProgramFinancialAidEligibleFieldEnum,
    >,
    ///<https://schema.org/programType>
    #[serde(rename = "programType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub program_type: Vec<EducationalOccupationalProgramProgramTypeFieldEnum>,
    ///<https://schema.org/trainingSalary>
    #[serde(rename = "trainingSalary")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub training_salary: Vec<MonetaryAmountDistribution>,
    ///<https://schema.org/occupationalCredentialAwarded>
    #[serde(rename = "occupationalCredentialAwarded")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub occupational_credential_awarded: Vec<
        EducationalOccupationalProgramOccupationalCredentialAwardedFieldEnum,
    >,
    ///<https://schema.org/applicationStartDate>
    #[serde(rename = "applicationStartDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub application_start_date: Vec<String>,
    ///<https://schema.org/educationalCredentialAwarded>
    #[serde(rename = "educationalCredentialAwarded")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub educational_credential_awarded: Vec<
        EducationalOccupationalProgramEducationalCredentialAwardedFieldEnum,
    >,
    ///<https://schema.org/typicalCreditsPerTerm>
    #[serde(rename = "typicalCreditsPerTerm")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub typical_credits_per_term: Vec<
        EducationalOccupationalProgramTypicalCreditsPerTermFieldEnum,
    >,
    ///<https://schema.org/hasCourse>
    #[serde(rename = "hasCourse")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub has_course: Vec<Course>,
    ///<https://schema.org/educationalProgramMode>
    #[serde(rename = "educationalProgramMode")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub educational_program_mode: Vec<
        EducationalOccupationalProgramEducationalProgramModeFieldEnum,
    >,
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
    pub additional_type: Vec<EducationalOccupationalProgramAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<EducationalOccupationalProgramIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<EducationalOccupationalProgramImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<EducationalOccupationalProgramDescriptionFieldEnum>,
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
    pub subject_of: Vec<EducationalOccupationalProgramSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<
        EducationalOccupationalProgramMainEntityOfPageFieldEnum,
    >,
}
