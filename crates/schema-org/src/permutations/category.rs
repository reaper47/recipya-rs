use schemars::JsonSchema;
use serde::Deserialize;

use crate::Thing;
use crate::data_type::text::URL;
use crate::thing::intangible::defined_term::CategoryCode;
use crate::thing::intangible::enumeration::PhysicalActivityCategory;

#[derive(Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum CategoryCodeOrPhysicalActivityCategoryOrTextOrThingOrURL {
    CategoryCode(CategoryCode),
    PhysicalActivityCategory(PhysicalActivityCategory),
    Text(String),
    Thing(Thing),
    URL(URL),
}
