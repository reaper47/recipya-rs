use crate::*;
use serde_with::{serde_as, OneOrMany};
///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type PostalAddressAdditionalTypeFieldEnum = String;
///<https://schema.org/PostalAddress>
#[serde_as]
#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct PostalAddress {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/addressLocality>
    #[serde(rename = "addressLocality")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub address_locality: Vec<String>,
    ///<https://schema.org/postalCode>
    #[serde(rename = "postalCode")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub postal_code: Vec<String>,
    ///<https://schema.org/postOfficeBoxNumber>
    #[serde(rename = "postOfficeBoxNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub post_office_box_number: Vec<String>,
    ///<https://schema.org/addressCountry>
    #[serde(rename = "addressCountry")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub address_country: Vec<PostalAddressAddressCountryFieldEnum>,
    ///<https://schema.org/extendedAddress>
    #[serde(rename = "extendedAddress")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub extended_address: Vec<String>,
    ///<https://schema.org/addressRegion>
    #[serde(rename = "addressRegion")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub address_region: Vec<String>,
    ///<https://schema.org/streetAddress>
    #[serde(rename = "streetAddress")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub street_address: Vec<String>,
    ///<https://schema.org/hoursAvailable>
    #[serde(rename = "hoursAvailable")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub hours_available: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/faxNumber>
    #[serde(rename = "faxNumber")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub fax_number: Vec<String>,
    ///<https://schema.org/contactOption>
    #[serde(rename = "contactOption")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contact_option: Vec<ContactPointOptionEnum>,
    ///<https://schema.org/availableLanguage>
    #[serde(rename = "availableLanguage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub available_language: Vec<PostalAddressAvailableLanguageFieldEnum>,
    ///<https://schema.org/areaServed>
    #[serde(rename = "areaServed")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub area_served: Vec<PostalAddressAreaServedFieldEnum>,
    ///<https://schema.org/contactType>
    #[serde(rename = "contactType")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub contact_type: Vec<String>,
    ///<https://schema.org/productSupported>
    #[serde(rename = "productSupported")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub product_supported: Vec<PostalAddressProductSupportedFieldEnum>,
    ///<https://schema.org/telephone>
    #[serde(rename = "telephone")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub telephone: Vec<String>,
    ///<https://schema.org/serviceArea>
    #[serde(rename = "serviceArea")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub service_area: Vec<PostalAddressServiceAreaFieldEnum>,
    ///<https://schema.org/email>
    #[serde(rename = "email")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub email: Vec<String>,
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
    pub additional_type: Vec<PostalAddressAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(rename = "identifier")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub identifier: Vec<PostalAddressIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(rename = "image")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub image: Vec<PostalAddressImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(rename = "sameAs")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(rename = "description")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub description: Vec<PostalAddressDescriptionFieldEnum>,
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
    pub subject_of: Vec<PostalAddressSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(rename = "name")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(rename = "mainEntityOfPage")]
    #[serde_as(as = "OneOrMany<_>")]
    #[serde(default)]
    pub main_entity_of_page: Vec<PostalAddressMainEntityOfPageFieldEnum>,
}
