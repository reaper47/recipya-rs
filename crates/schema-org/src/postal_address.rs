use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Action;
use crate::enums::ContactPointOptionEnum;
use crate::helpers::one_or_many;
use crate::field::*;
use crate::opening_hours_specification::OpeningHoursSpecification;

///<https://schema.org/additionalType>
///<https://schema.org/Text>
///<https://schema.org/URL>
pub type PostalAddressAdditionalTypeFieldEnum = String;

///<https://schema.org/PostalAddress>
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PostalAddress {
    #[serde(rename = "@context")]
    pub context: String,
    ///<https://schema.org/addressLocality>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub address_locality: Vec<String>,
    ///<https://schema.org/postalCode>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub postal_code: Vec<String>,
    ///<https://schema.org/postOfficeBoxNumber>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub post_office_box_number: Vec<String>,
    ///<https://schema.org/addressCountry>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub address_country: Vec<PostalAddressAddressCountryFieldEnum>,
    ///<https://schema.org/extendedAddress>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extended_address: Vec<String>,
    ///<https://schema.org/addressRegion>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub address_region: Vec<String>,
    ///<https://schema.org/streetAddress>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub street_address: Vec<String>,
    ///<https://schema.org/hoursAvailable>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hours_available: Vec<OpeningHoursSpecification>,
    ///<https://schema.org/faxNumber>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fax_number: Vec<String>,
    ///<https://schema.org/contactOption>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_option: Vec<ContactPointOptionEnum>,
    ///<https://schema.org/availableLanguage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub available_language: Vec<PostalAddressAvailableLanguageFieldEnum>,
    ///<https://schema.org/areaServed>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub area_served: Vec<PostalAddressAreaServedFieldEnum>,
    ///<https://schema.org/contactType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_type: Vec<String>,
    ///<https://schema.org/productSupported>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub product_supported: Vec<PostalAddressProductSupportedFieldEnum>,
    ///<https://schema.org/telephone>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub telephone: Vec<String>,
    ///<https://schema.org/serviceArea>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service_area: Vec<PostalAddressServiceAreaFieldEnum>,
    ///<https://schema.org/email>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub email: Vec<String>,
    ///<https://schema.org/disambiguatingDescription>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disambiguating_description: Vec<String>,
    ///<https://schema.org/potentialAction>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub potential_action: Vec<Action>,
    ///<https://schema.org/additionalType>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_type: Vec<PostalAddressAdditionalTypeFieldEnum>,
    ///<https://schema.org/identifier>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<PostalAddressIdentifierFieldEnum>,
    ///<https://schema.org/image>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<PostalAddressImageFieldEnum>,
    ///<https://schema.org/sameAs>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub same_as: Vec<String>,
    ///<https://schema.org/description>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<PostalAddressDescriptionFieldEnum>,
    ///<https://schema.org/alternateName>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_name: Vec<String>,
    ///<https://schema.org/url>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub url: Vec<String>,
    ///<https://schema.org/subjectOf>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject_of: Vec<PostalAddressSubjectOfFieldEnum>,
    ///<https://schema.org/name>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    ///<https://schema.org/mainEntityOfPage>
    #[serde(default, deserialize_with = "one_or_many")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub main_entity_of_page: Vec<PostalAddressMainEntityOfPageFieldEnum>,
}
