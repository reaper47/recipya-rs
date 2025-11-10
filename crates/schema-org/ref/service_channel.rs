use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type ServiceChannelAdditionalTypeFieldEnum = String;
///<https://schema.org/ServiceChannel>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ServiceChannel {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/serviceSmsNumber>
    #[serde(rename = "serviceSmsNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub service_sms_number: Vec<ContactPoint>,
    ///<https://schema.org/availableLanguage>
    #[serde(rename = "availableLanguage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub available_language: Vec<ServiceChannelAvailableLanguageFieldEnum>,
    ///<https://schema.org/serviceUrl>
    #[serde(rename = "serviceUrl")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub service_url: Vec<String>,
    ///<https://schema.org/servicePhone>
    #[serde(rename = "servicePhone")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub service_phone: Vec<ContactPoint>,
    ///<https://schema.org/providesService>
    #[serde(rename = "providesService")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub provides_service: Vec<Service>,
    ///<https://schema.org/processingTime>
    #[serde(rename = "processingTime")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub processing_time: Vec<Duration>,
    ///<https://schema.org/serviceLocation>
    #[serde(rename = "serviceLocation")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub service_location: Vec<Place>,
    ///<https://schema.org/servicePostalAddress>
    #[serde(rename = "servicePostalAddress")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub service_postal_address: Vec<PostalAddress>,
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
    pub additional_type: Vec<ServiceChannelAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<ServiceChannelIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<ServiceChannelImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<ServiceChannelDescriptionFieldEnum>,
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
    pub subject_of: Vec<ServiceChannelSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<ServiceChannelMainEntityOfPageFieldEnum>,
}
