use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/endDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type EmployeeRoleEndDateFieldEnum = String;
///<https://schema.org/namedPosition>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type EmployeeRoleNamedPositionFieldEnum = String;
///<https://schema.org/startDate>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type EmployeeRoleStartDateFieldEnum = String;
///<https://schema.org/roleName>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type EmployeeRoleRoleNameFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type EmployeeRoleAdditionalTypeFieldEnum = String;
///<https://schema.org/EmployeeRole>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct EmployeeRole {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/salaryCurrency>
    #[serde(rename = "salaryCurrency")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub salary_currency: Vec<String>,
    ///<https://schema.org/baseSalary>
    #[serde(rename = "baseSalary")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub base_salary: Vec<EmployeeRoleBaseSalaryFieldEnum>,
    ///<https://schema.org/numberedPosition>
    #[serde(rename = "numberedPosition")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub numbered_position: Vec<f32>,
    ///<https://schema.org/endDate>
    #[serde(rename = "endDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub end_date: Vec<EmployeeRoleEndDateFieldEnum>,
    ///<https://schema.org/namedPosition>
    #[serde(rename = "namedPosition")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub named_position: Vec<EmployeeRoleNamedPositionFieldEnum>,
    ///<https://schema.org/startDate>
    #[serde(rename = "startDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub start_date: Vec<EmployeeRoleStartDateFieldEnum>,
    ///<https://schema.org/roleName>
    #[serde(rename = "roleName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub role_name: Vec<EmployeeRoleRoleNameFieldEnum>,
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
    pub additional_type: Vec<EmployeeRoleAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<EmployeeRoleIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<EmployeeRoleImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<EmployeeRoleDescriptionFieldEnum>,
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
    pub subject_of: Vec<EmployeeRoleSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<EmployeeRoleMainEntityOfPageFieldEnum>,
}
