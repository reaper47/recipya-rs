use std::collections::{HashMap, hash_map};
use std::marker::PhantomData;
use std::path::PathBuf;

use async_trait::async_trait;
use futures::StreamExt;
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use schema_org::field::{
    CommentAuthorFieldEnum, RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum,
    RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
    RecipeToolFieldEnum, RecipeYieldFieldEnum,
};
use schema_org::{
    AggregateRating, AtType, Comment, Duration, DurationOrText, NutritionInformation, Recipe,
};
use support::fs::new_fs_support;
use tracing::{error, info};
use uuid::Uuid;

use super::host::Host;
use crate::api::FailedRecipes;
use crate::api::mealie::structs::{
    AuthPayload, MealieRecipe, MealieRecipes, MealieUser, TokenResponse,
};
use crate::api::{Credentials, common::RecipeClient};
use crate::{Error, Result};

pub struct AuthenticatedState;
pub struct UnauthenticatedState;

#[async_trait]
pub trait Unauthenticated<C: RecipeClient>: Send + Sync {
    /// Establishes a connection to the host using the provided credentials. Returns a token upon successful login.
    async fn login(self, credentials: Credentials) -> Result<Mealie<AuthenticatedState, C>>;
}

#[async_trait]
pub trait Authenticated<C: RecipeClient>: Send + Sync {
    /// Fetches recipes from connected host.
    async fn fetch_recipes(&self) -> Result<(Vec<Recipe>, FailedRecipes)>;
    /// Logs out of the Mealie instance.
    async fn logout(self) -> Result<Mealie<UnauthenticatedState, C>>;
}

#[derive(Clone)]
pub struct Mealie<State, C: RecipeClient> {
    recipe_client: C,
    _state: PhantomData<State>,
}

impl<C: RecipeClient> Mealie<UnauthenticatedState, C> {
    pub fn new(recipe_client: C) -> Self {
        Self {
            recipe_client,
            _state: PhantomData,
        }
    }
}

#[async_trait]
impl<C: RecipeClient + Send> Unauthenticated<C> for Mealie<UnauthenticatedState, C> {
    async fn login(mut self, credentials: Credentials) -> Result<Mealie<AuthenticatedState, C>> {
        let client = self.recipe_client.login(credentials).await?;

        Ok(Mealie {
            recipe_client: client,
            _state: PhantomData,
        })
    }
}

#[async_trait]
impl<C: RecipeClient + Send> Authenticated<C> for Mealie<AuthenticatedState, C> {
    async fn fetch_recipes(&self) -> Result<(Vec<Recipe>, FailedRecipes)> {
        Ok(self.recipe_client.fetch_recipes().await?)
    }

    async fn logout(self) -> Result<Mealie<UnauthenticatedState, C>> {
        Ok(Mealie {
            recipe_client: self.recipe_client.logout().await?,
            _state: PhantomData,
        })
    }
}

#[derive(Clone)]
pub struct MealieRecipeClient {
    host: Host,
    client: Client,
}

#[async_trait]
impl RecipeClient for MealieRecipeClient {
    async fn login(self, credentials: Credentials) -> Result<Self> {
        info!("Mealie API: Authenticating");

        let host = self.host.clone();
        let token = self.login_helper(credentials).await?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|err| Error::ApiError(format!("Invalid token: {err}")))?,
        );

        Ok(Self {
            host,
            client: Client::builder().default_headers(headers).build()?,
        })
    }

    async fn fetch_recipes(&self) -> Result<(Vec<Recipe>, FailedRecipes)> {
        info!("Mealie API: Fetching recipes");

        let recipe_ids = self.fetch_recipe_ids().await?;
        let recipe_details = self.fetch_recipes_helper(recipe_ids).await?;
        let mut recipes = Vec::with_capacity(recipe_details.0.len());
        let mut users = HashMap::new();

        for recipe in recipe_details.0 {
            if let hash_map::Entry::Vacant(entry) = users.entry(recipe.user_id) {
                let user = self.fetch_user(recipe.user_id).await?;
                entry.insert(user);
            }
            let user = &users[&recipe.user_id];

            let schema_recipe = self.build_schema_recipe(recipe, user).await?;
            recipes.push(schema_recipe);
        }

        Ok((recipes, recipe_details.1))
    }

    async fn logout(self) -> Result<Self> {
        info!("Mealie API: Logging out");

        let host = self.host.clone();
        let res = self.client.post(self.host.logout_url()).send().await?;

        if res.status().is_client_error() {
            let base = "Failed to logout from the Mealie API";
            error!("{base}: {}", res.text().await?);
            return Err(Error::ApiError(format!(
                "{base} because of a validation error (422)."
            )));
        }

        Ok(Self {
            host,
            client: Client::new(),
        })
    }
}

impl MealieRecipeClient {
    /// Creates a new instance of the MealieRecipeClient.
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: Host::new(host),
            client: Client::new(),
        }
    }

    async fn login_helper(&self, credentials: Credentials) -> Result<String> {
        let res = self
            .client
            .post(self.host.login_url())
            .form(&AuthPayload::new(
                credentials.username,
                credentials.password,
                true,
            ))
            .send()
            .await?;

        if res.status().is_client_error() {
            let base = "Failed to authenticate the Mealie API";
            error!("{base}: {}", res.text().await?);
            return Err(Error::ApiError(format!(
                "{base} because of a validation error (422).",
            )));
        }

        let token: TokenResponse = res.json().await.map_err(|err| {
            let base = "Failed to parse the token returned from the Mealie API response";
            error!("{base}: {err}");
            Error::ApiError(format!("{base}."))
        })?;

        Ok(token.access_token)
    }

    async fn fetch_recipe_image(
        &self,
        id: Option<Uuid>,
        image: Option<String>,
    ) -> Result<Option<PathBuf>> {
        let fs_support = new_fs_support();
        if let Some(id) = id
            && image.is_some()
        {
            let res = self
                .client
                .get(self.host.recipe_image_url(id))
                .header("Accept", "image/webp,image/*,*/*")
                .send()
                .await?;

            let content = res.bytes().await?;

            let image_path_tmp = fs_support
                .upload_to_temp(content)
                .await
                .map_err(|err| Error::ApiError(err.to_string()))?;

            Ok(Some(image_path_tmp))
        } else {
            Ok(None)
        }
    }

    async fn fetch_recipe_ids(&self) -> Result<Vec<Uuid>> {
        let mut recipe_ids = Vec::new();
        let mut next_page = Some("1".to_string());

        while let Some(page) = next_page {
            let all_recipes = self.fetch_recipes_page(page).await?;
            recipe_ids.extend(all_recipes.items.into_iter().filter_map(|recipe| recipe.id));
            next_page = all_recipes.next;
        }

        Ok(recipe_ids)
    }

    async fn fetch_recipes_page(&self, page: String) -> Result<MealieRecipes> {
        let res = self.client.get(self.host.recipes_url(page)).send().await?;

        if res.status().is_client_error() {
            let base = "Failed to fetch all recipes using the Mealie API";
            error!("{base}: {}", res.text().await?);
            return Err(Error::ApiError(format!(
                "{base} because of a validation error (422)."
            )));
        }

        Ok(res.json().await?)
    }

    async fn fetch_recipes_helper(
        &self,
        ids: Vec<Uuid>,
    ) -> Result<(Vec<MealieRecipe>, Vec<(Uuid, Error)>)> {
        let mut all_recipes = Vec::new();
        let mut all_failures = Vec::new();

        const BATCH_SIZE: usize = 10;

        for (batch_num, batch) in ids.chunks(BATCH_SIZE).enumerate() {
            info!(
                "Mealie API Import - Processing batch {} ({} recipes)",
                batch_num + 1,
                batch.len()
            );

            let results = futures::stream::iter(batch.iter().copied().map(|id| {
                let client = self.client.clone();
                let host = self.host.clone();

                async move {
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

                    for attempt in 0..3 {
                        return match client.get(host.recipe_url(id)).send().await {
                            Ok(res) => {
                                let status = res.status();

                                if !status.is_success() {
                                    if attempt < 2 {
                                        tokio::time::sleep(std::time::Duration::from_millis(
                                            200 * (attempt + 1) as u64,
                                        ))
                                        .await;
                                        continue;
                                    }
                                    return Err((id, Error::ApiError(format!("HTTP {status}"))));
                                }

                                match res.bytes().await {
                                    Ok(bytes) => {
                                        if bytes.is_empty() {
                                            if attempt < 2 {
                                                tokio::time::sleep(
                                                    std::time::Duration::from_millis(
                                                        200 * (attempt + 1) as u64,
                                                    ),
                                                )
                                                .await;
                                                continue;
                                            }
                                            return Err((
                                                id,
                                                Error::ApiError("Empty response body".into()),
                                            ));
                                        }

                                        match serde_json::from_slice::<MealieRecipe>(&bytes) {
                                            Ok(recipe) => Ok((id, recipe)),
                                            Err(err) => {
                                                if attempt < 2 {
                                                    tokio::time::sleep(
                                                        std::time::Duration::from_millis(
                                                            200 * (attempt + 1) as u64,
                                                        ),
                                                    )
                                                    .await;
                                                    continue;
                                                }
                                                Err((id, err.into()))
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        if attempt < 2 {
                                            tokio::time::sleep(std::time::Duration::from_millis(
                                                200 * (attempt + 1) as u64,
                                            ))
                                            .await;
                                            continue;
                                        }
                                        Err((id, Error::ApiError(err.to_string())))
                                    }
                                }
                            }
                            Err(err) => {
                                if attempt < 2 {
                                    tokio::time::sleep(std::time::Duration::from_millis(
                                        200 * (attempt + 1) as u64,
                                    ))
                                    .await;
                                    continue;
                                }
                                Err((id, Error::ApiError(err.to_string())))
                            }
                        };
                    }
                    unreachable!()
                }
            }))
            .buffer_unordered(10)
            .collect::<Vec<_>>()
            .await;

            for result in results {
                match result {
                    Ok((_, recipe)) => all_recipes.push(recipe),
                    Err((id, err)) => all_failures.push((
                        id,
                        Error::ApiError(format!(
                            "Failed to fetch recipe '{id}' using the Mealie API: {err}"
                        )),
                    )),
                }
            }

            if batch_num < (ids.len() / BATCH_SIZE) {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        }

        Ok((all_recipes, all_failures))
    }

    async fn build_schema_recipe(&self, recipe: MealieRecipe, user: &MealieUser) -> Result<Recipe> {
        let (recipe_category, keywords) = recipe.extract_category_and_keywords();

        let recipe_yield = format!(
            "{} {}",
            recipe.recipe_yield_quantity, recipe.recipe_servings
        );

        let num_comments = recipe.comments.iter().len();

        let mut instructions = recipe
            .recipe_instructions
            .map(|instructions| {
                instructions
                    .into_iter()
                    .map(|ins| match ins.summary {
                        Some(summary) => RecipeRecipeInstructionsFieldEnum::new_section(
                            &summary,
                            ins.text.lines().collect(),
                        ),
                        None => RecipeRecipeInstructionsFieldEnum::Text(ins.text),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        instructions.extend(
            recipe
                .notes
                .map(|notes| {
                    notes
                        .into_iter()
                        .map(|note| {
                            if note.title.is_empty() {
                                RecipeRecipeInstructionsFieldEnum::Text(note.text)
                            } else {
                                RecipeRecipeInstructionsFieldEnum::new_section(
                                    &note.title,
                                    note.text.lines().collect(),
                                )
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
        );

        Ok(Recipe {
            r#type: Some(AtType::Recipe.to_string()),
            context: Some("https://schema.org".into()),
            nutrition: recipe
                .nutrition
                .map(|n| vec![NutritionInformation::from(n)])
                .unwrap_or_default(),
            recipe_yield: match recipe_yield.trim() {
                "" => vec![],
                s => vec![RecipeYieldFieldEnum::Text(s.to_string())],
            },
            recipe_ingredient: recipe
                .recipe_ingredient
                .into_iter()
                .map(|ing| RecipeRecipeIngredientFieldEnum::Text(ing.display))
                .collect(),
            cook_time: recipe
                .cook_time
                .map(|s| vec![DurationOrText::Text(s)])
                .unwrap_or_default(),
            recipe_instructions: instructions,
            recipe_category,
            tool: recipe
                .tools
                .into_iter()
                .map(|tool| RecipeToolFieldEnum::Text(tool.name))
                .collect(),
            prep_time: recipe
                .prep_time
                .map(|s| vec![DurationOrText::Text(s)])
                .unwrap_or_default(),
            total_time: recipe
                .total_time
                .map(|s| vec![DurationOrText::Text(s)])
                .unwrap_or_default(),
            perform_time: recipe
                .perform_time
                .map(|s| match iso8601::datetime(&s) {
                    Ok(d) => vec![Duration {
                        name: vec![d.to_string()],
                        ..Default::default()
                    }],
                    Err(_) => vec![],
                })
                .unwrap_or_default(),
            comment: recipe
                .comments
                .map(|comments| {
                    comments
                        .into_iter()
                        .map(|comment| Comment {
                            r#type: Some(AtType::Comment.to_string()),
                            context: Some("https://schema.org".into()),
                            text: vec![comment.text],
                            date_created: vec![comment.created_at],
                            date_modified: vec![comment.updated_at],
                            author: comment
                                .user
                                .author()
                                .map(|s| vec![CommentAuthorFieldEnum::new_person(&s)])
                                .unwrap_or_default(),
                            ..Default::default()
                        })
                        .collect()
                })
                .unwrap_or_default(),
            date_created: recipe.created_at.map(|s| vec![s]).unwrap_or_default(),
            comment_count: if num_comments > 0 {
                vec![num_comments as i32]
            } else {
                vec![]
            },
            date_modified: recipe.updated_at.map(|s| vec![s]).unwrap_or_default(),
            keywords,
            aggregate_rating: recipe
                .rating
                .map(|n| {
                    n.as_f64()
                        .map(|n| vec![AggregateRating::new(n as f32, 1)])
                        .unwrap_or_default()
                })
                .unwrap_or_default(),
            author: user
                .author()
                .map(|s: String| vec![RecipeAuthorFieldEnum::new_person(&s)])
                .unwrap_or_default(),
            image: match self.fetch_recipe_image(recipe.id, recipe.image).await? {
                Some(s) => vec![RecipeImageFieldEnum::URL(
                    s.to_str().map(String::from).unwrap_or_default(),
                )],
                None => vec![],
            },
            description: recipe
                .description
                .map(|s| vec![RecipeDescriptionFieldEnum::Text(s)])
                .unwrap_or_default(),
            url: recipe
                .org_url
                .map(|u| vec![u.to_string()])
                .unwrap_or_default(),
            name: match recipe.name {
                Some(s) => vec![s],
                None => vec![recipe.slug],
            },
            ..Default::default()
        })
    }

    async fn fetch_user(&self, user_id: Uuid) -> Result<MealieUser> {
        let res = self.client.get(self.host.user_url(user_id)).send().await?;

        if res.status().is_client_error() {
            let base = "Failed to fetch user '{user_id}' using the Mealie API";
            error!("{base}: {}", res.text().await?);
            return Err(Error::ApiError(format!(
                "{base} because of a validation error (422)."
            )));
        }

        Ok(res.json().await?)
    }
}
