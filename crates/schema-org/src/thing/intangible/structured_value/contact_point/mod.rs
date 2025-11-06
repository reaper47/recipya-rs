mod postal_address;

pub use postal_address::*;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::permutations::administrative_area::AdministrativeAreaOrGeoShapeOrPlaceOrText;
use crate::permutations::product::ProductOrText;
use crate::permutations::text::LanguageOrText;
use crate::thing::intangible::enumeration::ContactPointOption;
use crate::thing::intangible::structured_value::OpeningHoursSpecification;

/// A contact point—for example, a Customer Complaints department.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContactPoint {
    /// The geographic area where a service or offered item is provided. Supersedes serviceArea.
    pub area_served: Box<AdministrativeAreaOrGeoShapeOrPlaceOrText>,
    ///  A language someone may use with or at the item, service or place. Please use one of the
    /// language codes from the IETF BCP 47 standard. See also inLanguage.
    pub available_language: LanguageOrText,
    /// An option available on this contact point (e.g. a toll-free number or support for
    /// hearing-impaired callers).
    pub contact_option: ContactPointOption,
    /// A person or organization can have different contact points, for different purposes.
    /// For example, a sales contact point, a PR contact point and so on. This property is used to
    /// specify the kind of contact point.
    pub contact_type: String,
    /// Email address.
    pub email: String,
    /// The fax number.
    pub fax_number: String,
    /// The hours during which this service or contact is available.
    pub hours_available: OpeningHoursSpecification,
    /// The product or service this support contact point is related to (such as product support
    /// for a particular product line). This can be a specific product or product line
    /// (e.g. "iPhone") or a general category of products or services (e.g. "smartphones").
    pub product_supported: ProductOrText,
    /// The telephone number.
    pub telephone: String,
    #[serde(flatten)]
    pub thing: Thing,
}
