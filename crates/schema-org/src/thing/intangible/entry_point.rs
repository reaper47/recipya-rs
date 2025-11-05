use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::thing::creative_work::SoftwareApplication;

/// An entry point, within some Web-based protocol.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EntryPoint {
    /// An application that can complete the request. Supersedes application.
    pub action_application: 	SoftwareApplication,
    /// The high level platform(s) where the Action can be performed for the given URL. To specify
    /// a specific application or operating system instance, use actionApplication.
    pub action_platform: 	DigitalPlatformEnumerationOrTextOrURL,
    /// The supported content type(s) for an EntryPoint response.
    pub content_type: 	String,
    /// The supported encoding type(s) for an EntryPoint request.
    pub encoding_type: 	String,
    /// An HTTP method that specifies the appropriate HTTP method for a request to an HTTP
    /// EntryPoint. Values are capitalized strings as used in HTTP.
    pub http_method: 	String,
    /// An url template (RFC6570) that will be used to construct the target of the execution of
    /// the action.
    pub url_template: 	String,
    #[serde(flatten)]
    pub thing: Thing,
}
