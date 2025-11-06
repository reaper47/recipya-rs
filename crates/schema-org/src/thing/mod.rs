mod action;
mod bio_chem;
pub mod creative_work;
mod event;
pub mod intangible;
pub mod medical;
pub mod medical_entity;
pub mod organization;
mod person;
pub mod place;
pub mod product;

pub use action::*;
pub use bio_chem::*;
pub use creative_work::CreativeWork;
pub use event::*;
pub use product::*;

pub use medical_entity::MedicalEntity;
pub use organization::Organization;
pub use person::Person;
pub use place::Place;

use schemars::JsonSchema;
use serde::Deserialize;

/// The most generic type of item.
#[derive(Debug, Default, Deserialize, PartialEq, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Thing {}
