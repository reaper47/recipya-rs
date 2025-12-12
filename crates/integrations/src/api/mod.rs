mod common;

pub mod mealie;
pub mod nextcloud;
pub mod tandoor;

pub use common::*;

use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_stream::stream;
use futures::{StreamExt, pin_mut};
use futures_core::Stream;
use serde::Deserialize;
use strum::{EnumIter, EnumString, IntoEnumIterator};
use tokio::sync::Mutex;
use uuid::Uuid;

use schema_org::Recipe;

use crate::api::mealie::{
    Authenticated as AuthenticatedMealie, Unauthenticated as UnauthenticatedMealie,
};
use crate::api::nextcloud::{
    Authenticated as AuthenticatedNextcloud, Unauthenticated as UnauthenticatedNextcloud,
};
use crate::api::tandoor::{
    Authenticated as AuthenticatedTandoor, Unauthenticated as UnauthenticatedTandoor,
};
use crate::api::tandoor::{Tandoor, TandoorRecipeClient};
use crate::{
    Error,
    api::{
        mealie::{Mealie, MealieRecipeClient, structs::MealieUser},
        nextcloud::{Nextcloud, NextcloudRecipeClient},
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
        base_url: &str,
        credentials: Credentials,
    ) -> impl Stream<Item = std::result::Result<(String, Recipe, NumRecipes), (String, NumRecipes, Error)>>
    {
        stream! {
            match self {
                Api::Mealie => {
                        let mealie = match Mealie::new(MealieRecipeClient::new(base_url)).login(credentials).await {
                            Ok(m) => Arc::new(m),
                            Err(err) => {
                                yield Err((String::new(), 0,  err));
                                return;
                            }
                        };

                        let recipe_ids = match mealie.fetch_recipe_ids().await {
                            Ok(ids) => ids,
                            Err(err) => {
                                yield Err((String::new(), 0, err));
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
                                    Ok(recipe) => yield Ok((id.to_string(), recipe, num_recipes)),
                                    Err((id, err)) => yield Err((id.to_string(), num_recipes, err)),
                                }
                            }
                        }

                        match Arc::try_unwrap(mealie) {
                            Ok(mealie) => {
                                if let Err(err) = mealie.logout().await {
                                    yield Err((String::new(), num_recipes, err));
                                }
                            },
                            Err(_) => yield Err((String::new(), num_recipes, Error::ApiError("Failed to logout: Arc still shared".into()))),
                        }
                }
                Api::Nextcloud => {
                    let nextcloud = match Nextcloud::new(NextcloudRecipeClient::new(base_url)).login(credentials).await {
                        Ok(n) => Arc::new(n),
                        Err(err) => {
                            yield Err((String::new(), 0,  err));
                            return;
                        }
                    };

                    let recipe_ids = match nextcloud.fetch_recipe_ids().await {
                        Ok(ids) => ids,
                        Err(err) => {
                            yield Err((String::new(), 0, err));
                            return;
                        }
                    };

                    let num_recipes = recipe_ids.len() as i64;

                    let fetches = futures::stream::iter(recipe_ids).map(|id| {
                        let nc = Arc::clone(&nextcloud);

                        async move {
                            match nc.fetch_recipe(id.clone()).await {
                                Ok(recipe) => Ok((id, recipe)),
                                Err(err) => Err(err),
                            }
                        }
                    }).buffer_unordered(16);

                    pin_mut!(fetches);
                    while let Some(result) = fetches.next().await {
                        match result {
                            Ok((id, recipe)) => yield Ok((id, recipe, num_recipes)),
                            Err((id, err)) => yield Err((id, num_recipes, err)),
                        }
                    }
                },
                Api::Tandoor => {
                    let tandoor = match Tandoor::new(TandoorRecipeClient::new(base_url)).login(credentials).await {
                        Ok(t) => Arc::new(t),
                        Err(err) => {
                            yield Err((String::new(), 0,  err));
                            return;
                        }
                    };

                    let recipe_ids = match tandoor.fetch_recipe_ids().await {
                        Ok(ids) => ids,
                        Err(err) => {
                            yield Err((String::new(), 0, err));
                            return;
                        }
                    };

                    let num_recipes = recipe_ids.len() as i64;

                    let fetches = futures::stream::iter(recipe_ids).map(|id| {
                        let tan = Arc::clone(&tandoor);

                        async move {
                            match tan.fetch_recipe(id).await {
                                Ok(recipe) => Ok((id, recipe)),
                                Err(err) => Err(err),
                            }
                        }
                    }).buffer_unordered(16);

                    pin_mut!(fetches);
                    while let Some(result) = fetches.next().await {
                        match result {
                            Ok((id, recipe)) => yield Ok((id.to_string(), recipe, num_recipes)),
                            Err((id, err)) => yield Err((id.to_string(), num_recipes, err)),
                        }
                    }
                },
                Api::Unknown => {
                    yield Err((String::new(), 0, Error::UnsupportedApi));
                    return;
                }
            }
        }
    }
}
