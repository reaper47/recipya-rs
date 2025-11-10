use crate::*;
use serde_with::{serde_as, OneOrMany};
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
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct JobPosting {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/hiringOrganization>
    #[serde(rename = "hiringOrganization")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub hiring_organization: Vec<JobPostingHiringOrganizationFieldEnum>,
    ///<https://schema.org/employmentUnit>
    #[serde(rename = "employmentUnit")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub employment_unit: Vec<Organization>,
    ///<https://schema.org/jobImmediateStart>
    #[serde(rename = "jobImmediateStart")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub job_immediate_start: Vec<String>,
    ///<https://schema.org/experienceRequirements>
    #[serde(rename = "experienceRequirements")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub experience_requirements: Vec<JobPostingExperienceRequirementsFieldEnum>,
    ///<https://schema.org/incentives>
    #[serde(rename = "incentives")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub incentives: Vec<String>,
    ///<https://schema.org/responsibilities>
    #[serde(rename = "responsibilities")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub responsibilities: Vec<String>,
    ///<https://schema.org/educationRequirements>
    #[serde(rename = "educationRequirements")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub education_requirements: Vec<JobPostingEducationRequirementsFieldEnum>,
    ///<https://schema.org/skills>
    #[serde(rename = "skills")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub skills: Vec<JobPostingSkillsFieldEnum>,
    ///<https://schema.org/incentiveCompensation>
    #[serde(rename = "incentiveCompensation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub incentive_compensation: Vec<String>,
    ///<https://schema.org/totalJobOpenings>
    #[serde(rename = "totalJobOpenings")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub total_job_openings: Vec<i32>,
    ///<https://schema.org/jobBenefits>
    #[serde(rename = "jobBenefits")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub job_benefits: Vec<String>,
    ///<https://schema.org/securityClearanceRequirement>
    #[serde(rename = "securityClearanceRequirement")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub security_clearance_requirement: Vec<
        JobPostingSecurityClearanceRequirementFieldEnum,
    >,
    ///<https://schema.org/jobStartDate>
    #[serde(rename = "jobStartDate")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub job_start_date: Vec<JobPostingJobStartDateFieldEnum>,
    ///<https://schema.org/datePosted>
    #[serde(rename = "datePosted")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub date_posted: Vec<JobPostingDatePostedFieldEnum>,
    ///<https://schema.org/occupationalCategory>
    #[serde(rename = "occupationalCategory")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub occupational_category: Vec<JobPostingOccupationalCategoryFieldEnum>,
    ///<https://schema.org/physicalRequirement>
    #[serde(rename = "physicalRequirement")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub physical_requirement: Vec<JobPostingPhysicalRequirementFieldEnum>,
    ///<https://schema.org/directApply>
    #[serde(rename = "directApply")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub direct_apply: Vec<String>,
    ///<https://schema.org/industry>
    #[serde(rename = "industry")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub industry: Vec<JobPostingIndustryFieldEnum>,
    ///<https://schema.org/salaryCurrency>
    #[serde(rename = "salaryCurrency")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub salary_currency: Vec<String>,
    ///<https://schema.org/estimatedSalary>
    #[serde(rename = "estimatedSalary")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub estimated_salary: Vec<JobPostingEstimatedSalaryFieldEnum>,
    ///<https://schema.org/qualifications>
    #[serde(rename = "qualifications")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub qualifications: Vec<JobPostingQualificationsFieldEnum>,
    ///<https://schema.org/sensoryRequirement>
    #[serde(rename = "sensoryRequirement")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub sensory_requirement: Vec<JobPostingSensoryRequirementFieldEnum>,
    ///<https://schema.org/workHours>
    #[serde(rename = "workHours")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub work_hours: Vec<String>,
    ///<https://schema.org/baseSalary>
    #[serde(rename = "baseSalary")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub base_salary: Vec<JobPostingBaseSalaryFieldEnum>,
    ///<https://schema.org/experienceInPlaceOfEducation>
    #[serde(rename = "experienceInPlaceOfEducation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub experience_in_place_of_education: Vec<String>,
    ///<https://schema.org/jobLocationType>
    #[serde(rename = "jobLocationType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub job_location_type: Vec<String>,
    ///<https://schema.org/validThrough>
    #[serde(rename = "validThrough")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub valid_through: Vec<JobPostingValidThroughFieldEnum>,
    ///<https://schema.org/specialCommitments>
    #[serde(rename = "specialCommitments")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub special_commitments: Vec<String>,
    ///<https://schema.org/eligibilityToWorkRequirement>
    #[serde(rename = "eligibilityToWorkRequirement")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub eligibility_to_work_requirement: Vec<String>,
    ///<https://schema.org/title>
    #[serde(rename = "title")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub title: Vec<String>,
    ///<https://schema.org/jobLocation>
    #[serde(rename = "jobLocation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub job_location: Vec<Place>,
    ///<https://schema.org/relevantOccupation>
    #[serde(rename = "relevantOccupation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub relevant_occupation: Vec<Occupation>,
    ///<https://schema.org/benefits>
    #[serde(rename = "benefits")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub benefits: Vec<String>,
    ///<https://schema.org/applicationContact>
    #[serde(rename = "applicationContact")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub application_contact: Vec<ContactPoint>,
    ///<https://schema.org/applicantLocationRequirements>
    #[serde(rename = "applicantLocationRequirements")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub applicant_location_requirements: Vec<AdministrativeArea>,
    ///<https://schema.org/employmentType>
    #[serde(rename = "employmentType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub employment_type: Vec<String>,
    ///<https://schema.org/employerOverview>
    #[serde(rename = "employerOverview")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub employer_overview: Vec<String>,
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
    pub additional_type: Vec<JobPostingAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<JobPostingIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<JobPostingImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<JobPostingDescriptionFieldEnum>,
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
    pub subject_of: Vec<JobPostingSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<JobPostingMainEntityOfPageFieldEnum>,
}
