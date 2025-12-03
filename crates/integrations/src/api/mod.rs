mod common;

pub(crate) mod mealie;

pub use common::{Credentials, FailedRecipes};

use std::str::FromStr;

use serde::Deserialize;
use strum::{EnumIter, EnumString, IntoEnumIterator};

use schema_org::Recipe;

use crate::{
    Error, Result,
    api::mealie::{Authenticated, Mealie, MealieRecipeClient, Unauthenticated},
};

/// Represents a collection of recipe management APIs.
#[derive(Debug, Default, strum_macros::Display, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Api {
    Mealie,
    Nextcloud,
    Tandoor,
    #[default]
    Unknown,
}

impl<'de> Deserialize<'de> for Api {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Api::from_str(&s).or(Ok(Api::Unknown))
    }
}

/// Returns a list of all recipe management APIs Recipya can import.
pub fn all_apis() -> Vec<Api> {
    Api::iter().filter(|a| !matches!(a, Api::Unknown)).collect()
}

impl Api {
    /// Fetches recipes from the given input source and returns a vector of `schema_org::Recipe` objects.
    pub async fn fetch_recipes(
        &self,
        host: &str,
        credentials: Credentials,
    ) -> Result<(Vec<Recipe>, FailedRecipes)> {
        match self {
            Api::Mealie => {
                let recipe_client = MealieRecipeClient::new(host);
                let mealie = Mealie::new(recipe_client).login(credentials).await?;
                let recipes = mealie.fetch_recipes().await?;
                mealie.logout().await?;
                Ok(recipes)
            }
            Api::Nextcloud => todo!("Nextcloud API not implemented"),
            Api::Tandoor => todo!("Tandoor API not implemented"),
            Api::Unknown => Err(Error::UnsupportedApi),
        }
    }
}
