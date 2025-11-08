use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type BusinessAudienceAdditionalTypeFieldEnum = String;
///<https://schema.org/BusinessAudience>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct BusinessAudience {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/numberOfEmployees>
    #[serde(rename = "numberOfEmployees")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub number_of_employees: Vec<QuantitativeValue>,
    ///<https://schema.org/yearlyRevenue>
    #[serde(rename = "yearlyRevenue")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub yearly_revenue: Vec<QuantitativeValue>,
    ///<https://schema.org/yearsInOperation>
    #[serde(rename = "yearsInOperation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub years_in_operation: Vec<QuantitativeValue>,
    ///<https://schema.org/geographicArea>
    #[serde(rename = "geographicArea")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub geographic_area: Vec<AdministrativeArea>,
    ///<https://schema.org/audienceType>
    #[serde(rename = "audienceType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub audience_type: Vec<String>,
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
    pub additional_type: Vec<BusinessAudienceAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<BusinessAudienceIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<BusinessAudienceImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<BusinessAudienceDescriptionFieldEnum>,
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
    pub subject_of: Vec<BusinessAudienceSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<BusinessAudienceMainEntityOfPageFieldEnum>,
}
