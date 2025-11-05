use schemars::JsonSchema;
use crate::thing::{Organization, Person};

#[derive(Debug, PartialEq, JsonSchema)]
pub enum OrganizationOrPerson {
    Organization(Organization),
    Person(Person),
}
