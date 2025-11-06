//! # schemaorg
//!
//! Rust type definitions automatically derived from [Schema.org](https://schema.org/).
//!
//! Source: © Schema.org contributors — licensed under [CC-BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/).
//! Rust translation © 2025 Marc-André Charland — licensed under AGPL-3.0 OR CC-BY-SA 3.0.

extern crate core;

mod article;
mod context;
mod nutrition;
mod recipe;

mod data_type;
pub mod permutations;
pub mod thing;
mod types;

pub use article::*;
pub use context::*;
pub use nutrition::*;
pub use recipe::*;
pub use thing::Thing;
