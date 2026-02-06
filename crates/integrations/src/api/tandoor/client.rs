use std::{marker::PhantomData, path::PathBuf};

use async_trait::async_trait;

use reqwest::Client;
use schema_org::{
    AggregateRating, AtType, DurationOrText, NutritionInformation, Recipe, at_context,
    field::{
        RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
        RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum, RecipeYieldFieldEnum,
    },
};
use support::fs::new_fs_support;
use tracing::{error, info};

use crate::{
    Error, Result,
    api::{
        AuthType, AuthenticatedState, Credentials, MAX_RETRY_ATTEMPTS, UnauthenticatedState,
        assemble_token_header,
        tandoor::{
            host::Host,
            structs::{AuthPayload, RecipeList, RecipeRetrieve, TokenResponse},
        },
    },
};

#[async_trait]
pub trait RecipeClient: Clone + Send + Sync {
    /// Establishes a connection to the host using the provided credentials.
    /// Replaces the client's state with the new authenticated connection.
    async fn login(self, credentials: Credentials) -> Result<Self>;

    /// Fetches all recipe IDs from the connected host.
    async fn fetch_recipe_ids(&self) -> Result<Vec<u32>>;

    /// Fetches a recipe from the connected host.
    async fn fetch_recipe(&self, id: u32) -> Result<Recipe>;
}

#[async_trait]
pub trait Unauthenticated<C: RecipeClient>: Send + Sync {
    /// Establishes a connection to the host using the provided credentials. Returns a token upon successful login.
    async fn login(self, credentials: Credentials) -> Result<Tandoor<AuthenticatedState, C>>;
}

#[async_trait]
pub trait Authenticated<C: RecipeClient>: Send + Sync {
    /// Fetches all recipe IDs from the connected host.
    async fn fetch_recipe_ids(&self) -> Result<Vec<u32>>;

    /// Fetches a recipe from the connected host.
    async fn fetch_recipe(&self, id: u32) -> std::result::Result<Recipe, (u32, Error)>;
}

#[derive(Clone)]
pub struct Tandoor<State, C: RecipeClient> {
    recipe_client: C,
    _state: PhantomData<State>,
}

impl<C: RecipeClient> Tandoor<UnauthenticatedState, C> {
    /// Creates a new unauthenticated Tandoor client.
    pub const fn new(recipe_client: C) -> Self {
        Self {
            recipe_client,
            _state: PhantomData,
        }
    }
}

#[async_trait]
impl<C: RecipeClient + Send> Unauthenticated<C> for Tandoor<UnauthenticatedState, C> {
    async fn login(self, credentials: Credentials) -> Result<Tandoor<AuthenticatedState, C>> {
        info!("Tandoor API: Authenticating");

        Ok(Tandoor {
            recipe_client: self.recipe_client.login(credentials).await?,
            _state: PhantomData,
        })
    }
}

#[async_trait]
impl<C: RecipeClient + Send> Authenticated<C> for Tandoor<AuthenticatedState, C> {
    async fn fetch_recipe_ids(&self) -> Result<Vec<u32>> {
        info!("Tandoor API: Fetching recipes");

        Ok(self.recipe_client.fetch_recipe_ids().await?)
    }

    async fn fetch_recipe(&self, id: u32) -> std::result::Result<Recipe, (u32, Error)> {
        match self.recipe_client.fetch_recipe(id).await {
            Ok(recipe) => Ok(recipe),
            Err(err) => Err((id, err)),
        }
    }
}

#[derive(Clone)]
pub struct TandoorRecipeClient {
    host: Host,
    client: Client,
}

#[async_trait]
impl RecipeClient for TandoorRecipeClient {
    async fn login(self, credentials: Credentials) -> Result<Self> {
        let token = self.login_helper(credentials).await?;

        Ok(Self {
            host: self.host.clone(),
            client: Client::builder()
                .default_headers(assemble_token_header(&AuthType::Bearer, &token)?)
                .build()?,
        })
    }

    async fn fetch_recipe_ids(&self) -> Result<Vec<u32>> {
        let mut recipe_ids = Vec::new();
        let mut next_page = Some(self.host.recipes_url());

        while let Some(page) = next_page {
            let all_recipes = self.fetch_recipes_page(page).await?;
            recipe_ids.extend(all_recipes.results.into_iter().map(|recipe| recipe.id));
            next_page = all_recipes.next;
        }

        Ok(recipe_ids)
    }

    async fn fetch_recipe(&self, id: u32) -> Result<Recipe> {
        match self.fetch_single_recipe_with_retry(id).await {
            Ok((_, recipe)) => Ok(self.build_recipe_schema(recipe).await?),
            Err(err) => Err(Error::ApiError(format!(
                "Failed to fetch Tandoor recipe with ID '{}': {}",
                err.0, err.1
            ))),
        }
    }
}

impl TandoorRecipeClient {
    /// Creates a new instance of the `TandoorRecipeClient`.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            host: Host::new(base_url),
            client: Client::new(),
        }
    }

    async fn login_helper(&self, credentials: Credentials) -> Result<String> {
        let res = self
            .client
            .post(self.host.login_url())
            .form(&AuthPayload {
                username: credentials.username,
                password: credentials.password,
            })
            .send()
            .await?;

        let status = res.status();
        if status.is_client_error() {
            let base = "Failed to authenticate the Tandoor API";
            error!("{base}: {}", res.text().await?);
            return Err(Error::ApiError(format!(
                "{base} because of a validation error ({status:?})."
            )));
        }

        let token: TokenResponse = res.json().await.map_err(|err| {
            let base = "Failed to parse the token returned from the Tandoor API response";
            error!("{base}: {err}");
            Error::ApiError(format!("{base}."))
        })?;

        Ok(token.token)
    }

    async fn fetch_recipes_page(&self, page_url: String) -> Result<RecipeList> {
        let res = self.client.get(page_url).send().await?;

        let status = res.status();
        if status.is_client_error() {
            let base = "Failed to fetch all recipes using the Tandoor API";
            error!("{base}: {}", res.text().await?);
            return Err(Error::ApiError(format!(
                "{base} because of a validation error ({status:?})."
            )));
        }

        Ok(res.json().await?)
    }

    async fn fetch_single_recipe_with_retry(
        &self,
        id: u32,
    ) -> std::result::Result<(u32, RecipeRetrieve), (u32, Error)> {
        for attempt in 1..=MAX_RETRY_ATTEMPTS {
            let delay = std::time::Duration::from_millis(50 * attempt as u64);

            match self.client.get(self.host.recipe_url(id)).send().await {
                Ok(res) if res.status().is_success() => {
                    let bytes = res
                        .bytes()
                        .await
                        .map_err(|err| (id, Error::ApiError(err.to_string())))?;

                    if bytes.is_empty() {
                        if attempt < MAX_RETRY_ATTEMPTS {
                            tokio::time::sleep(delay).await;
                            continue;
                        }
                        return Err((id, Error::ApiError("Empty response body".into())));
                    }

                    let recipe = serde_json::from_slice::<RecipeRetrieve>(&bytes)
                        .map_err(|err| (id, err.into()))?;

                    return Ok((id, recipe));
                }
                Ok(res) => {
                    if attempt < MAX_RETRY_ATTEMPTS {
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    return Err((id, Error::ApiError(res.text().await.unwrap_or_default())));
                }
                Err(err) => {
                    if attempt < MAX_RETRY_ATTEMPTS {
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    return Err((id, Error::ApiError(err.to_string())));
                }
            }
        }

        Err((id, Error::ApiError("Taboor API - Exhausted retries".into())))
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cast_possible_truncation)]
    async fn build_recipe_schema(&self, recipe: RecipeRetrieve) -> Result<Recipe> {
        let (category, keywords) = match &recipe.keywords {
            Some(vec) if !vec.is_empty() => {
                let (first, rest) = vec.split_first().unwrap();
                (Some(first), rest)
            }
            _ => (None, &[][..]),
        };

        Ok(Recipe {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            nutrition: recipe
                .nutrition
                .map(|n| vec![NutritionInformation::from(n)])
                .unwrap_or_default(),
            recipe_yield: match recipe.servings {
                Some(servings) => vec![RecipeYieldFieldEnum::Text(servings.to_string())],
                None => recipe
                    .servings_text
                    .filter(|text| !text.is_empty())
                    .map(|text| vec![RecipeYieldFieldEnum::Text(text)])
                    .unwrap_or_default(),
            },
            recipe_ingredient: recipe
                .steps
                .iter()
                .flat_map(|step| {
                    step.ingredients
                        .iter()
                        .map(|ing| RecipeRecipeIngredientFieldEnum::Text(ing.to_string()))
                })
                .collect(),
            cook_time: recipe
                .waiting_time
                .map(|num_minutes| {
                    vec![DurationOrText::Text(
                        iso8601::Duration::YMDHMS {
                            hour: (num_minutes / 60).cast_unsigned(),
                            minute: (num_minutes % 60).cast_unsigned(),
                            second: 0,
                            millisecond: 0,
                            day: 0,
                            month: 0,
                            year: 0,
                        }
                        .to_string(),
                    )]
                })
                .unwrap_or_default(),
            recipe_instructions: recipe
                .steps
                .into_iter()
                .map(|step| {
                    RecipeRecipeInstructionsFieldEnum::Text(
                        step.instruction
                            .filter(|s| !s.is_empty())
                            .or(Some(step.instructions_markdown))
                            .unwrap_or_default(),
                    )
                })
                .collect(),
            recipe_category: category.map(|c| vec![c.name.clone()]).unwrap_or_default(),
            prep_time: recipe
                .working_time
                .map(|num_minutes| {
                    vec![DurationOrText::Text(
                        iso8601::Duration::YMDHMS {
                            hour: (num_minutes / 60).cast_unsigned(),
                            minute: (num_minutes % 60).cast_unsigned(),
                            second: 0,
                            millisecond: 0,
                            day: 0,
                            month: 0,
                            year: 0,
                        }
                        .to_string(),
                    )]
                })
                .unwrap_or_default(),
            date_created: vec![recipe.created_at.to_string()],
            keywords: keywords
                .iter()
                .map(|k| RecipeKeywordsFieldEnum::TextOrURL(k.name.clone()))
                .collect(),
            aggregate_rating: recipe
                .rating
                .map(|r| {
                    let rating = r.clamp(f64::from(f32::MIN), f64::from(f32::MAX)) as f32;
                    vec![AggregateRating::new(rating, 1)]
                })
                .unwrap_or_default(),
            author: Some(recipe.created_by.username)
                .filter(|s| !s.is_empty())
                .map(|s| vec![RecipeAuthorFieldEnum::new_person(&s)])
                .unwrap_or_default(),
            image: if let Some(image) = recipe.image {
                (self.fetch_recipe_image(image).await?).map_or_else(Vec::new, |s| {
                    vec![RecipeImageFieldEnum::URL(
                        s.to_str().map(String::from).unwrap_or_default(),
                    )]
                })
            } else {
                vec![]
            },
            description: recipe
                .description
                .map(|d| vec![RecipeDescriptionFieldEnum::Text(d)])
                .unwrap_or_default(),
            url: recipe.source_url.map(|u| vec![u]).unwrap_or_default(),
            name: vec![recipe.name],
            ..Default::default()
        })
    }

    async fn fetch_recipe_image(&self, image_url: String) -> Result<Option<PathBuf>> {
        let fs_support = new_fs_support();
        let res = self
            .client
            .get(image_url)
            .header("Accept", "image/webp,image/*,*/*")
            .send()
            .await?;

        let content = res.bytes().await?;

        let image_path_tmp = fs_support
            .upload_to_temp(content)
            .await
            .map_err(|err| Error::ApiError(err.to_string()))?;

        Ok(Some(image_path_tmp))
    }
}
