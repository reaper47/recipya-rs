mod action;
pub mod intangible;
pub mod place;
pub mod product;
pub mod creative_work;
mod bio_chem;
pub mod medical;
pub mod medical_entity;
mod event;
pub mod organization;
mod person;

pub use action::*;
pub use creative_work::CreativeWork;
pub use event::*;
pub use product::*;
pub use bio_chem::*;

pub use medical_entity::MedicalEntity;
pub use organization::Organization;
pub use place::Place;
pub use person::Person;

use schemars::JsonSchema;
use serde::Deserialize;

/// The most generic type of item.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Thing {

}
