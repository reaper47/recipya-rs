use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::data_type::text::URL;
use crate::permutations::text::LanguageOrText;
use crate::thing::Place;
use crate::thing::intangible::quantity::duration::Duration;
use crate::thing::intangible::service::Service;
use crate::thing::intangible::structured_value::ContactPoint;
use crate::thing::intangible::structured_value::contact_point::PostalAddress;

/// A means for accessing a service, e.g. a government office location, web site, or phone number.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ServiceChannel {
    /// A language someone may use with or at the item, service or place. Please use one of the
    /// language codes from the IETF BCP 47 standard. See also inLanguage.
    pub available_language: LanguageOrText,
    /// Estimated processing time for the service using this channel.
    pub processing_time: Duration,
    /// The service provided by this channel.
    pub provides_service: Service,
    /// The location (e.g. civic structure, local business, etc.) where a person can go to access
    /// the service.
    pub service_location: Place,
    /// The phone number to use to access the service.
    pub service_phone: ContactPoint,
    /// The address for accessing the service by mail.
    pub service_postal_address: PostalAddress,
    /// The number to access the service by text message.
    pub service_sms_number: ContactPoint,
    /// The website to access the service.
    pub service_url: URL,
    #[serde(flatten)]
    pub thing: Thing,
}
