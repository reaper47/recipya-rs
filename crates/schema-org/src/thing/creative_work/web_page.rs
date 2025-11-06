use schemars::JsonSchema;
use serde::Deserialize;

use crate::data_type::text::URL;
use crate::permutations::person::OrganizationOrPerson;
use crate::permutations::url::SpeakableSpecificationOrURL;
use crate::thing::CreativeWork;
use crate::thing::creative_work::WebPageElement;
use crate::thing::intangible::enumeration::Specialty;
use crate::thing::medical_entity::ImageObject;
use crate::types::date::Date;

/// A web page. Every web page is implicitly assumed to be declared to be of type WebPage, so the
/// various properties about that webpage, such as breadcrumb may be used. We recommend explicit
/// declaration if these properties are specified, but if they are found outside of an itemscope,
/// they will be assumed to be about the page.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WebPage {
    /// A set of links that can help a user understand and navigate a website hierarchy.
    pub breadcrumb: BreadcrumbListOrText,
    /// Date on which the content on this web page was last reviewed for accuracy and/or
    /// completeness.
    pub last_reviewed: Date,
    /// Indicates if this web page element is the main subject of the page. Supersedes aspect.
    pub main_content_of_page: Box<WebPageElement>,
    /// Indicates the main image on the page.
    pub primary_image_of_page: ImageObject,
    /// A link related to this web page, for example to other related web pages.
    pub related_link: URL,
    /// People or organizations that have reviewed the content on this web page for accuracy and/or
    /// completeness.
    pub reviewed_by: OrganizationOrPerson,
    /// One of the more significant URLs on the page. Typically, these are the non-navigation links
    /// that are clicked on the most. Supersedes significantLinks.
    pub significant_link: URL,
    /// Indicates sections of a Web page that are particularly 'speakable' in the sense of being
    /// highlighted as being especially appropriate for text-to-speech conversion. Other sections
    /// of a page may also be usefully spoken in particular circumstances; the 'speakable' property
    /// serves to indicate the parts most likely to be generally useful for speech.
    ///
    /// The speakable property can be repeated an arbitrary number of times, with three kinds of
    /// possible 'content-locator' values:
    ///
    /// 1.) id-value URL references - uses id-value of an element in the page being annotated. The
    /// simplest use of speakable has (potentially relative) URL values, referencing identified
    /// sections of the document concerned.
    ///
    /// 2.) CSS Selectors - addresses content in the annotated page, e.g. via class attribute. Use
    /// the cssSelector property.
    ///
    /// 3.) XPaths - addresses content via XPaths (assuming an XML view of the content). Use the
    /// xpath property.
    ///
    /// For more sophisticated markup of speakable sections beyond simple ID references, either CSS
    /// selectors or XPath expressions to pick out document section(s) as speakable. For this we
    /// define a supporting type, SpeakableSpecification which is defined to be a possible value of
    /// the speakable property.
    pub speakable: SpeakableSpecificationOrURL,
    /// One of the domain specialities to which this web page's content applies.
    pub specialty: Specialty,
    #[serde(flatten)]
    pub creative_work: CreativeWork,
}
