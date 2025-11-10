use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type OwnershipInfoAdditionalTypeFieldEnum = String;
///<https://schema.org/OwnershipInfo>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct OwnershipInfo {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/ownedThrough>
    #[serde(rename = "ownedThrough")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub owned_through: Vec<String>,
    ///<https://schema.org/typeOfGood>
    #[serde(rename = "typeOfGood")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub type_of_good: Vec<OwnershipInfoTypeOfGoodFieldEnum>,
    ///<https://schema.org/ownedFrom>
    #[serde(rename = "ownedFrom")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub owned_from: Vec<String>,
    ///<https://schema.org/acquiredFrom>
    #[serde(rename = "acquiredFrom")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub acquired_from: Vec<OwnershipInfoAcquiredFromFieldEnum>,
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
    pub additional_type: Vec<OwnershipInfoAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<OwnershipInfoIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<OwnershipInfoImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<OwnershipInfoDescriptionFieldEnum>,
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
    pub subject_of: Vec<OwnershipInfoSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<OwnershipInfoMainEntityOfPageFieldEnum>,
}
