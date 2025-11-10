use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::helpers::one_or_many;
use crate::field::*;
///<https://schema.org/securityClearanceRequirement>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type JobPostingSecurityClearanceRequirementFieldEnum = String;
///<https://schema.org/jobStartDate>
///<https://schema.org/Date>
///<https://schema.org/Text>
pub type JobPostingJobStartDateFieldEnum = String;
///<https://schema.org/datePosted>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type JobPostingDatePostedFieldEnum = String;
///<https://schema.org/validThrough>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type JobPostingValidThroughFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type JobPostingAdditionalTypeFieldEnum = String;
///<https://schema.org/JobPosting>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JobPosting {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/hiringOrganization>
    #[serde(rename = "hiringOrganization")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hiring_organization: Vec<JobPostingHiringOrganizationFieldEnum>,
    ///<https://schema.org/employmentUnit>
    #[serde(rename = "employmentUnit")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub employment_unit: Vec<Organization>,
    ///<https://schema.org/jobImmediateStart>
    #[serde(rename = "jobImmediateStart")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub job_immediate_start: Vec<String>,
    ///<https://schema.org/experienceRequirements>
    #[serde(rename = "experienceRequirements")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub experience_requirements: Vec<JobPostingExperienceRequirementsFieldEnum>,
    ///<https://schema.org/incentives>
    #[serde(rename = "incentives")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub incentives: Vec<String>,
    ///<https://schema.org/responsibilities>
    #[serde(rename = "responsibilities")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub responsibilities: Vec<String>,
    ///<https://schema.org/educationRequirements>
    #[serde(rename = "educationRequirements")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub education_requirements: Vec<JobPostingEducationRequirementsFieldEnum>,
    ///<https://schema.org/skills>
    #[serde(rename = "skills")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<JobPostingSkillsFieldEnum>,
    ///<https://schema.org/incentiveCompensation>
    #[serde(rename = "incentiveCompensation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub incentive_compensation: Vec<String>,
    ///<https://schema.org/totalJobOpenings>
    #[serde(rename = "totalJobOpenings")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub total_job_openings: Vec<i32>,
    ///<https://schema.org/jobBenefits>
    #[serde(rename = "jobBenefits")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub job_benefits: Vec<String>,
    ///<https://schema.org/securityClearanceRequirement>
    #[serde(rename = "securityClearanceRequirement")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub security_clearance_requirement: Vec<
        JobPostingSecurityClearanceRequirementFieldEnum,
    >,
    ///<https://schema.org/jobStartDate>
    #[serde(rename = "jobStartDate")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub job_start_date: Vec<JobPostingJobStartDateFieldEnum>,
    ///<https://schema.org/datePosted>
    #[serde(rename = "datePosted")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub date_posted: Vec<JobPostingDatePostedFieldEnum>,
    ///<https://schema.org/occupationalCategory>
    #[serde(rename = "occupationalCategory")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub occupational_category: Vec<JobPostingOccupationalCategoryFieldEnum>,
    ///<https://schema.org/physicalRequirement>
    #[serde(rename = "physicalRequirement")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub physical_requirement: Vec<JobPostingPhysicalRequirementFieldEnum>,
    ///<https://schema.org/directApply>
    #[serde(rename = "directApply")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub direct_apply: Vec<String>,
    ///<https://schema.org/industry>
    #[serde(rename = "industry")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub industry: Vec<JobPostingIndustryFieldEnum>,
    ///<https://schema.org/salaryCurrency>
    #[serde(rename = "salaryCurrency")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub salary_currency: Vec<String>,
    ///<https://schema.org/estimatedSalary>
    #[serde(rename = "estimatedSalary")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub estimated_salary: Vec<JobPostingEstimatedSalaryFieldEnum>,
    ///<https://schema.org/qualifications>
    #[serde(rename = "qualifications")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub qualifications: Vec<JobPostingQualificationsFieldEnum>,
    ///<https://schema.org/sensoryRequirement>
    #[serde(rename = "sensoryRequirement")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sensory_requirement: Vec<JobPostingSensoryRequirementFieldEnum>,
    ///<https://schema.org/workHours>
    #[serde(rename = "workHours")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub work_hours: Vec<String>,
    ///<https://schema.org/baseSalary>
    #[serde(rename = "baseSalary")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub base_salary: Vec<JobPostingBaseSalaryFieldEnum>,
    ///<https://schema.org/experienceInPlaceOfEducation>
    #[serde(rename = "experienceInPlaceOfEducation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub experience_in_place_of_education: Vec<String>,
    ///<https://schema.org/jobLocationType>
    #[serde(rename = "jobLocationType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub job_location_type: Vec<String>,
    ///<https://schema.org/validThrough>
    #[serde(rename = "validThrough")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub valid_through: Vec<JobPostingValidThroughFieldEnum>,
    ///<https://schema.org/specialCommitments>
    #[serde(rename = "specialCommitments")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub special_commitments: Vec<String>,
    ///<https://schema.org/eligibilityToWorkRequirement>
    #[serde(rename = "eligibilityToWorkRequirement")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub eligibility_to_work_requirement: Vec<String>,
    ///<https://schema.org/title>
    #[serde(rename = "title")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub title: Vec<String>,
    ///<https://schema.org/jobLocation>
    #[serde(rename = "jobLocation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub job_location: Vec<Place>,
    ///<https://schema.org/relevantOccupation>
    #[serde(rename = "relevantOccupation")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub relevant_occupation: Vec<Occupation>,
    ///<https://schema.org/benefits>
    #[serde(rename = "benefits")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub benefits: Vec<String>,
    ///<https://schema.org/applicationContact>
    #[serde(rename = "applicationContact")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub application_contact: Vec<ContactPoint>,
    ///<https://schema.org/applicantLocationRequirements>
    #[serde(rename = "applicantLocationRequirements")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub applicant_location_requirements: Vec<AdministrativeArea>,
    ///<https://schema.org/employmentType>
    #[serde(rename = "employmentType")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub employment_type: Vec<String>,
    ///<https://schema.org/employerOverview>
    #[serde(rename = "employerOverview")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub employer_overview: Vec<String>,
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
    pub additional_type: Vec<JobPostingAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<JobPostingIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<JobPostingImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<JobPostingDescriptionFieldEnum>,
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
    pub subject_of: Vec<JobPostingSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<JobPostingMainEntityOfPageFieldEnum>,
}
