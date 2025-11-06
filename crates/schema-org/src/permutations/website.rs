use schemars::JsonSchema;

use crate::thing::creative_work::{SoftwareApplication, WebSite};

#[derive(Debug, PartialEq, JsonSchema)]
pub enum SoftwareApplicationOrWebSite {
    SoftwareApplication(SoftwareApplication),
    WebSite(WebSite),
}
