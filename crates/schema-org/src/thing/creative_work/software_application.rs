use schemars::JsonSchema;
use serde::Deserialize;
use url::Url;

use crate::components::{ImageObjectOrUrl, TextOrUrl};
use crate::data_type::text::URL;
use crate::thing::creative_work::dataset::DataFeed;
use crate::thing::CreativeWork;

/// A software application.
#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SoftwareApplication {
    /// Type of software application, e.g. 'Game, Multimedia'.
    pub application_category: 	TextOrUrl,
    /// Subcategory of the application, e.g. 'Arcade Game'.
    pub application_sub_category: 	TextOrUrl,
    /// The name of the application suite to which the application belongs (e.g. Excel
    /// belongs to Office).
    pub application_suite: 	String,
    /// Device required to run the application. Used in cases where a specific make/model is
    /// required to run the application. Supersedes device.
    pub available_on_device: 	String,
    /// Countries for which the application is not supported. You can also provide the two-letter
    /// ISO 3166-1 alpha-2 country code.
    pub countries_not_supported: 	String,
    /// Countries for which the application is supported. You can also provide the two-letter
    /// ISO 3166-1 alpha-2 country code.
    pub countries_supported: 	String,
    /// If the file can be downloaded, URL to download the binary.
    pub download_url: 	URL,
    /// Features or modules provided by this application (and possibly required by other
    /// applications).
    pub feature_list: 	TextOrUrl,
    /// Size of the application / package (e.g. 18MB). In the absence of a unit (MB, KB etc.),
    /// KB will be assumed.
    pub file_size: 	String,
    /// URL at which the app may be installed, if different from the URL of the item.
    pub install_url: 	URL,
    /// Minimum memory requirements.
    pub memory_requirements: 	TextOrUrl,
    /// Operating systems supported (Windows 7, OS X 10.6, Android 1.6).
    pub operating_system: 	String,
    /// Permission(s) required to run the app (for example, a mobile app may require full internet
    /// access or may run only on wifi).
    pub permissions: 	String,
    /// Processor architecture required to run the application (e.g. IA64).
    pub processor_requirements: 	String,
    /// Description of what changed in this version.
    pub release_notes: 	TextOrUrl,
    /// A link to a screenshot image of the app.
    pub screenshot: 	ImageObjectOrUrl,
    /// Additional content for a software application.
    pub software_add_on: 	Box<SoftwareApplication>,
    /// Software application help.
    pub software_help: 	CreativeWork,
    /// Component dependency requirements for application. This includes runtime environments and
    /// shared libraries that are not included in the application distribution package, but required
    /// to run the application (examples: DirectX, Java or .NET runtime). Supersedes requirements.
    pub software_requirements: 	TextOrUrl,
    /// Version of the software instance.
    pub software_version: 	String,
    /// Storage requirements (free space required).
    pub storage_requirements: 	TextOrUrl,
    /// Supporting data for a SoftwareApplication.
    pub supporting_data: 	DataFeed,
    #[serde(flatten)]
    pub creative_work: CreativeWork,
}
