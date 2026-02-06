use std::{marker::PhantomData, path::PathBuf};

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use reqwest::Client;
use schema_org::{
    AtType, DurationOrText, NutritionInformation, at_context,
    field::{
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeKeywordsFieldEnum,
        RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
        RecipeRecipeYieldFieldEnum, RecipeToolFieldEnum,
    },
};
use support::fs::new_fs_support;
use tracing::info;

use crate::{
    Error, Result,
    api::{
        AuthType, AuthenticatedState, Credentials, MAX_RETRY_ATTEMPTS, UnauthenticatedState,
        assemble_token_header,
        nextcloud::{
            host::Host,
            structs::{Recipe, Recipes},
        },
    },
};

#[async_trait]
pub trait Unauthenticated<C: RecipeClient>: Send + Sync {
    /// Establishes a connection to the host using the provided credentials. Returns a token upon successful login.
    async fn login(self, credentials: Credentials) -> Result<Nextcloud<AuthenticatedState, C>>;
}

#[async_trait]
pub trait Authenticated<C: RecipeClient>: Send + Sync {
    /// Fetches all recipe IDs from the connected host.
    async fn fetch_recipe_ids(&self) -> Result<Vec<String>>;

    /// Fetches a recipe from the connected host.
    async fn fetch_recipe(
        &self,
        id: String,
    ) -> std::result::Result<schema_org::Recipe, (String, Error)>;
}

#[async_trait]
pub trait RecipeClient: Clone + Send + Sync {
    /// Establishes a connection to the host using the provided credentials. Returns a token upon successful login.
    async fn login(self, credentials: Credentials) -> Result<Self>;

    /// Fetches all recipe IDs from the connected host.
    async fn fetch_recipe_ids(&self) -> Result<Vec<String>>;

    /// Fetches a recipe from the connected host.
    async fn fetch_recipe(&self, id: &str) -> Result<schema_org::Recipe>;
}

#[derive(Clone)]
pub struct Nextcloud<State, C: RecipeClient> {
    recipe_client: C,
    _state: PhantomData<State>,
}

impl<C: RecipeClient> Nextcloud<UnauthenticatedState, C> {
    /// Creates a new unauthenticated Nextcloud client.
    pub const fn new(recipe_client: C) -> Self {
        Self {
            recipe_client,
            _state: PhantomData,
        }
    }
}

#[async_trait]
impl<C: RecipeClient> Unauthenticated<C> for Nextcloud<UnauthenticatedState, C> {
    async fn login(self, credentials: Credentials) -> Result<Nextcloud<AuthenticatedState, C>> {
        info!("Nextcloud API: Authenticating");

        Ok(Nextcloud {
            recipe_client: self.recipe_client.login(credentials).await?,
            _state: PhantomData,
        })
    }
}

#[async_trait]
impl<C: RecipeClient> Authenticated<C> for Nextcloud<AuthenticatedState, C> {
    async fn fetch_recipe_ids(&self) -> Result<Vec<String>> {
        info!("Nextcloud API: Fetching recipes");

        Ok(self.recipe_client.fetch_recipe_ids().await?)
    }

    async fn fetch_recipe(
        &self,
        id: String,
    ) -> std::result::Result<schema_org::Recipe, (String, Error)> {
        match self.recipe_client.fetch_recipe(&id).await {
            Ok(recipe) => Ok(recipe),
            Err(err) => Err((id, err)),
        }
    }
}

#[derive(Clone)]
pub struct NextcloudRecipeClient {
    host: Host,
    client: Client,
}

#[async_trait]
impl RecipeClient for NextcloudRecipeClient {
    async fn login(self, credentials: Credentials) -> Result<Self> {
        let credentials = format!("{}:{}", credentials.username, credentials.password);
        let token = general_purpose::STANDARD.encode(credentials.as_bytes());

        Ok(Self {
            host: self.host,
            client: Client::builder()
                .default_headers(assemble_token_header(&AuthType::Basic, &token)?)
                .build()?,
        })
    }

    async fn fetch_recipe_ids(&self) -> Result<Vec<String>> {
        Ok(self
            .client
            .get(self.host.recipes_url())
            .send()
            .await?
            .json::<Vec<Recipes>>()
            .await?
            .into_iter()
            .map(|recipe| recipe.id)
            .collect())
    }

    async fn fetch_recipe(&self, id: &str) -> Result<schema_org::Recipe> {
        match self.fetch_single_recipe_with_retry(id).await {
            Ok((_, recipe)) => Ok(self.build_recipe_schema(recipe).await?),
            Err(err) => Err(Error::ApiError(format!(
                "Failed to fetch Nextcloud Cookbook recipe with ID '{}': {}",
                err.0, err.1
            ))),
        }
    }
}

impl NextcloudRecipeClient {
    /// Creates a new instance of the `NextcloudRecipeClient`.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            host: Host::new(base_url),
            client: Client::new(),
        }
    }

    async fn fetch_single_recipe_with_retry(
        &self,
        id: &str,
    ) -> std::result::Result<(String, Recipe), (String, Error)> {
        for attempt in 1..=MAX_RETRY_ATTEMPTS {
            let delay = std::time::Duration::from_millis(50 * attempt as u64);

            match self.client.get(self.host.recipe_url(id)).send().await {
                Ok(res) if res.status().is_success() => {
                    let bytes = res
                        .bytes()
                        .await
                        .map_err(|err| (id.into(), Error::ApiError(err.to_string())))?;

                    if bytes.is_empty() {
                        if attempt < MAX_RETRY_ATTEMPTS {
                            tokio::time::sleep(delay).await;
                            continue;
                        }
                        return Err((id.into(), Error::ApiError("Empty response body".into())));
                    }

                    let recipe = serde_json::from_slice::<Recipe>(&bytes)
                        .map_err(|err| (id.into(), err.into()))?;

                    return Ok((id.into(), recipe));
                }
                Ok(res) => {
                    if attempt < MAX_RETRY_ATTEMPTS {
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    return Err((
                        id.into(),
                        Error::ApiError(res.text().await.unwrap_or_default()),
                    ));
                }
                Err(err) => {
                    if attempt < MAX_RETRY_ATTEMPTS {
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    return Err((id.into(), Error::ApiError(err.to_string())));
                }
            }
        }

        Err((
            id.into(),
            Error::ApiError("Nextcloud Cookbook API - Exhausted retries".into()),
        ))
    }

    async fn build_recipe_schema(&self, recipe: Recipe) -> Result<schema_org::Recipe> {
        let keywords: Vec<_> = if recipe.keywords.is_empty() {
            recipe.keywords.split(',').collect()
        } else {
            vec![]
        };

        let image = self.get_recipe_image(&recipe).await?;

        Ok(schema_org::Recipe {
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
            nutrition: match recipe.nutrition {
                Some(n) if n.is_empty() => vec![],
                Some(n) => vec![NutritionInformation::from(n)],
                None => vec![],
            },
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text(
                recipe.recipe_yield.to_string(),
            )],
            recipe_ingredient: recipe
                .recipe_ingredient
                .into_iter()
                .map(RecipeRecipeIngredientFieldEnum::Text)
                .collect(),
            cook_time: vec![DurationOrText::Text(recipe.cook_time.to_string())],
            recipe_instructions: recipe
                .recipe_instructions
                .into_iter()
                .map(RecipeRecipeInstructionsFieldEnum::Text)
                .collect(),
            recipe_category: if recipe.recipe_category.is_empty() {
                vec![]
            } else {
                vec![recipe.recipe_category]
            },
            tool: recipe
                .tool
                .into_iter()
                .map(RecipeToolFieldEnum::Text)
                .collect(),
            prep_time: vec![DurationOrText::Text(recipe.prep_time.to_string())],
            total_time: vec![DurationOrText::Text(recipe.total_time.to_string())],
            date_created: vec![recipe.date_created.to_string()],
            date_modified: recipe
                .date_modified
                .map(|date| vec![date.to_string()])
                .unwrap_or_default(),
            keywords: keywords
                .into_iter()
                .map(|kw| RecipeKeywordsFieldEnum::TextOrURL(kw.into()))
                .collect(),
            image,
            description: if recipe.description.is_empty() {
                vec![]
            } else {
                vec![RecipeDescriptionFieldEnum::Text(recipe.description)]
            },
            url: if recipe.url.is_empty() {
                vec![]
            } else {
                vec![recipe.url]
            },
            name: vec![recipe.name],
            ..Default::default()
        })
    }

    async fn get_recipe_image(&self, recipe: &Recipe) -> Result<Vec<RecipeImageFieldEnum>> {
        if recipe.image.is_none() && recipe.image_url.is_empty() {
            return Ok(vec![]);
        }

        let img_path = self
            .fetch_recipe_image(&self.host.recipe_image_url(&recipe.id))
            .await?;

        Ok(img_path
            .and_then(|path| {
                path.to_str()
                    .map(|s| vec![RecipeImageFieldEnum::URL(s.to_string())])
            })
            .unwrap_or_default())
    }

    async fn fetch_recipe_image(&self, url: &str) -> Result<Option<PathBuf>> {
        let content = self
            .client
            .get(url)
            .header("Accept", "image/webp,image/*,*/*")
            .send()
            .await?
            .bytes()
            .await?;

        let image_path_tmp = new_fs_support()
            .upload_to_temp(content)
            .await
            .map_err(|err| Error::ApiError(err.to_string()))?;

        Ok(Some(image_path_tmp))
    }
}
