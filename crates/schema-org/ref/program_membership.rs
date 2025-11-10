use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ProgramMembershipAdditionalTypeFieldEnum = String;
///<https://schema.org/ProgramMembership>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ProgramMembership {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/programName>
    #[serde(rename = "programName")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub program_name: Vec<String>,
    ///<https://schema.org/hostingOrganization>
    #[serde(rename = "hostingOrganization")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub hosting_organization: Vec<Organization>,
    ///<https://schema.org/member>
    #[serde(rename = "member")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub member: Vec<ProgramMembershipMemberFieldEnum>,
    ///<https://schema.org/membershipNumber>
    #[serde(rename = "membershipNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub membership_number: Vec<String>,
    ///<https://schema.org/members>
    #[serde(rename = "members")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub members: Vec<ProgramMembershipMembersFieldEnum>,
    ///<https://schema.org/membershipPointsEarned>
    #[serde(rename = "membershipPointsEarned")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub membership_points_earned: Vec<ProgramMembershipMembershipPointsEarnedFieldEnum>,
    ///<https://schema.org/program>
    #[serde(rename = "program")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub program: Vec<MemberProgram>,
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
    pub additional_type: Vec<ProgramMembershipAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<ProgramMembershipIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<ProgramMembershipImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<ProgramMembershipDescriptionFieldEnum>,
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
    pub subject_of: Vec<ProgramMembershipSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<ProgramMembershipMainEntityOfPageFieldEnum>,
}
