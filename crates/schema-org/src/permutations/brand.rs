use schemars::JsonSchema;

use crate::thing::intangible::brand::Brand;
use crate::thing::Organization;

#[derive(Debug, PartialEq, JsonSchema)]
pub enum BrandOrOrganization {
    Brand(Brand),
    Organization(Organization),
}
