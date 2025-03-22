mod error;

pub mod recipe;
pub mod share;
pub mod user;
pub mod website;

pub use error::{Error, Result};
pub use recipe::{Recipe, RecipeDetails};
