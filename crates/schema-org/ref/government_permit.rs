use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/validFrom>
///<https://schema.org/Date>
///<https://schema.org/DateTime>
pub type GovernmentPermitValidFromFieldEnum = String;
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type GovernmentPermitAdditionalTypeFieldEnum = String;
///<https://schema.org/GovernmentPermit>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct GovernmentPermit {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/issuedBy>
    #[serde(rename = "issuedBy")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub issued_by: Vec<Organization>,
    ///<https://schema.org/validUntil>
    #[serde(rename = "validUntil")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub valid_until: Vec<String>,
    ///<https://schema.org/validFrom>
    #[serde(rename = "validFrom")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub valid_from: Vec<GovernmentPermitValidFromFieldEnum>,
    ///<https://schema.org/validIn>
    #[serde(rename = "validIn")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub valid_in: Vec<AdministrativeArea>,
    ///<https://schema.org/validFor>
    #[serde(rename = "validFor")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub valid_for: Vec<Duration>,
    ///<https://schema.org/issuedThrough>
    #[serde(rename = "issuedThrough")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub issued_through: Vec<Service>,
    ///<https://schema.org/permitAudience>
    #[serde(rename = "permitAudience")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub permit_audience: Vec<Audience>,
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
    pub additional_type: Vec<GovernmentPermitAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<GovernmentPermitIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<GovernmentPermitImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<GovernmentPermitDescriptionFieldEnum>,
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
    pub subject_of: Vec<GovernmentPermitSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<GovernmentPermitMainEntityOfPageFieldEnum>,
}
