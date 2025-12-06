mod common;

pub mod mealie;

pub use common::{Credentials, FailedRecipes};
use futures::{StreamExt, pin_mut};
use tokio::sync::Mutex;
use uuid::Uuid;

use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_stream::stream;
use futures_core::Stream;
use serde::Deserialize;
use strum::{EnumIter, EnumString, IntoEnumIterator};

use schema_org::Recipe;

use crate::{
    Error,
    api::mealie::{
        Authenticated, Mealie, MealieRecipeClient, Unauthenticated, structs::MealieUser,
    },
};

type NumRecipes = i64;

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
    pub fn fetch_recipes_stream(
        &self,
        host: &str,
        credentials: Credentials,
    ) -> impl Stream<Item = std::result::Result<(Uuid, Recipe, NumRecipes), (Uuid, NumRecipes, Error)>>
    {
        stream! {
            match self {
                Api::Mealie => {
                        let mealie = match Mealie::new(MealieRecipeClient::new(host)).login(credentials).await {
                            Ok(m) => Arc::new(m),
                            Err(err) => {
                                yield Err((Uuid::nil(), 0,  err));
                                return;
                            }
                        };

                        let recipe_ids = match mealie.fetch_recipe_ids().await {
                            Ok(ids) => ids,
                            Err(err) => {
                                yield Err((Uuid::nil(), 0, err));
                                return;
                            }
                        };

                        let num_recipes = recipe_ids.len() as i64;

                        {
                            let users = Arc::new(Mutex::new(HashMap::<Uuid, MealieUser>::new()));

                            let fetches = futures::stream::iter(recipe_ids).map(|id| {
                                let users = Arc::clone(&users);
                                let mealie = Arc::clone(&mealie);

                                async move {
                                    let mut users_guard = users.lock().await;
                                    let recipe = mealie.fetch_recipe(id, &mut users_guard).await;
                                    (id, recipe)
                                }
                            }).buffer_unordered(10);

                            pin_mut!(fetches);
                            while let Some((id, recipe)) = fetches.next().await {
                                match recipe {
                                    Ok(recipe) => yield Ok((id, recipe, num_recipes)),
                                    Err(err) => yield Err((err.0, num_recipes, err.1)),
                                }
                            }
                        }

                        match Arc::try_unwrap(mealie) {
                            Ok(mealie) => {
                                if let Err(err) = mealie.logout().await {
                                    yield Err((Uuid::nil(), num_recipes, err));
                                }
                            },
                            Err(_) => yield Err((Uuid::nil(), num_recipes, Error::ApiError("Failed to logout: Arc still shared".into()))),
                        }
                }
                Api::Nextcloud => todo!("Nextcloud API not implemented"),
                Api::Tandoor => todo!("Tandoor API not implemented"),
                Api::Unknown => {
                    yield Err((Uuid::nil(), 0, Error::UnsupportedApi));
                    return;
                }
            }
        }
    }
}
