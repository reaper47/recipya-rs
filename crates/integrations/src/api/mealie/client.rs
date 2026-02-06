use std::collections::{HashMap, hash_map};
use std::marker::PhantomData;
use std::path::PathBuf;

use async_trait::async_trait;
use reqwest::Client;
use schema_org::field::{
    CommentAuthorFieldEnum, RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum,
    RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
    RecipeToolFieldEnum, RecipeYieldFieldEnum,
};
use schema_org::{
    AggregateRating, AtType, Comment, Duration, DurationOrText, NutritionInformation, Recipe,
    at_context,
};
use support::fs::new_fs_support;
use tracing::{error, info};
use uuid::Uuid;

use super::host::Host;
use crate::api::mealie::structs::{
    AuthPayload, MealieRecipe, MealieRecipes, MealieUser, TokenResponse,
};
use crate::api::{
    AuthType, AuthenticatedState, Credentials, MAX_RETRY_ATTEMPTS, UnauthenticatedState,
    assemble_token_header,
};
use crate::{Error, Result};

#[async_trait]
pub trait RecipeClient: Clone + Send + Sync {
    /// Establishes a connection to the host using the provided credentials.
    /// Replaces the client's state with the new authenticated connection.
    async fn login(self, credentials: Credentials) -> Result<Self>;

    /// Fetches all recipe IDs from the connected host.
    async fn fetch_recipe_ids(&self) -> Result<Vec<Uuid>>;

    /// Fetches a recipe from the connected host.
    async fn fetch_recipe(&self, id: Uuid, users: &mut HashMap<Uuid, MealieUser>)
    -> Result<Recipe>;

    /// Logs out of the connected host.
    async fn logout(self) -> Result<Self>;
}

#[async_trait]
pub trait Unauthenticated<C: RecipeClient>: Send + Sync {
    /// Establishes a connection to the host using the provided credentials. Returns a token upon successful login.
    async fn login(self, credentials: Credentials) -> Result<Mealie<AuthenticatedState, C>>;
}

#[async_trait]
pub trait Authenticated<C: RecipeClient>: Send + Sync {
    /// Logs out of the Mealie instance.
    async fn logout(self) -> Result<Mealie<UnauthenticatedState, C>>;

    /// Fetches all recipe IDs from the connected host.
    async fn fetch_recipe_ids(&self) -> Result<Vec<Uuid>>;

    /// Fetches a recipe from the connected host.
    async fn fetch_recipe(
        &self,
        id: Uuid,
        users: &mut HashMap<Uuid, MealieUser>,
    ) -> std::result::Result<Recipe, (Uuid, Error)>;
}

#[derive(Clone)]
pub struct Mealie<State, C: RecipeClient> {
    recipe_client: C,
    _state: PhantomData<State>,
}

impl<C: RecipeClient> Mealie<UnauthenticatedState, C> {
    /// Creates a new unauthenticated Mealie client.
    pub const fn new(recipe_client: C) -> Self {
        Self {
            recipe_client,
            _state: PhantomData,
        }
    }
}

#[async_trait]
impl<C: RecipeClient + Send> Unauthenticated<C> for Mealie<UnauthenticatedState, C> {
    async fn login(mut self, credentials: Credentials) -> Result<Mealie<AuthenticatedState, C>> {
        Ok(Mealie {
            recipe_client: self.recipe_client.login(credentials).await?,
            _state: PhantomData,
        })
    }
}

#[async_trait]
impl<C: RecipeClient + Send> Authenticated<C> for Mealie<AuthenticatedState, C> {
    async fn logout(self) -> Result<Mealie<UnauthenticatedState, C>> {
        Ok(Mealie {
            recipe_client: self.recipe_client.logout().await?,
            _state: PhantomData,
        })
    }

    async fn fetch_recipe_ids(&self) -> Result<Vec<Uuid>> {
        Ok(self.recipe_client.fetch_recipe_ids().await?)
    }

    async fn fetch_recipe(
        &self,
        id: Uuid,
        users: &mut HashMap<Uuid, MealieUser>,
    ) -> std::result::Result<Recipe, (Uuid, Error)> {
        match self.recipe_client.fetch_recipe(id, users).await {
            Ok(recipe) => Ok(recipe),
            Err(err) => Err((id, err)),
        }
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

        let token = self.login_helper(credentials).await?;

        Ok(Self {
            host: self.host.clone(),
            client: Client::builder()
                .default_headers(assemble_token_header(&AuthType::Bearer, &token)?)
                .build()?,
        })
    }

    async fn fetch_recipe(
        &self,
        id: Uuid,
        users: &mut HashMap<Uuid, MealieUser>,
    ) -> Result<Recipe> {
        match self.fetch_single_recipe_with_retry(id).await {
            Ok((_, recipe)) => {
                if let hash_map::Entry::Vacant(entry) = users.entry(recipe.user_id) {
                    let user = self.fetch_user(recipe.user_id).await?;
                    entry.insert(user);
                }
                let user = &users[&recipe.user_id];

                let schema_recipe = self.build_schema_recipe(recipe, user).await?;
                Ok(schema_recipe)
            }
            Err(err) => Err(Error::ApiError(format!(
                "Failed to fetch Mealie recipe with ID '{}': {}",
                err.0, err.1
            ))),
        }
    }

    async fn fetch_recipe_ids(&self) -> Result<Vec<Uuid>> {
        info!("Mealie API: Fetching recipes");

        let mut recipe_ids = Vec::new();
        let mut next_page = Some("1".to_string());

        while let Some(page) = next_page {
            let all_recipes = self.fetch_recipes_page(page).await?;
            recipe_ids.extend(all_recipes.items.into_iter().filter_map(|recipe| recipe.id));
            next_page = all_recipes.next;
        }

        Ok(recipe_ids)
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
    /// Creates a new instance of the `MealieRecipeClient`.
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

    async fn fetch_recipes_page(&self, page: String) -> Result<MealieRecipes> {
        let res = self.client.get(self.host.recipes_url(&page)).send().await?;

        if res.status().is_client_error() {
            let base = "Failed to fetch all recipes using the Mealie API";
            error!("{base}: {}", res.text().await?);
            return Err(Error::ApiError(format!(
                "{base} because of a validation error (422)."
            )));
        }

        Ok(res.json().await?)
    }

    async fn fetch_single_recipe_with_retry(
        &self,
        id: Uuid,
    ) -> std::result::Result<(Uuid, MealieRecipe), (Uuid, Error)> {
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

                    let recipe = serde_json::from_slice::<MealieRecipe>(&bytes)
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

        Err((id, Error::ApiError("Mealie API - Exhausted retries".into())))
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cast_possible_truncation)]
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
            r#type: AtType::Recipe.to_opt(),
            context: at_context(),
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
                .map(|s| {
                    iso8601::datetime(&s).map_or_else(
                        |_| vec![],
                        |d| {
                            vec![Duration {
                                name: vec![d.to_string()],
                                ..Default::default()
                            }]
                        },
                    )
                })
                .unwrap_or_default(),
            comment: recipe
                .comments
                .map(|comments| {
                    comments
                        .into_iter()
                        .map(|comment| Comment {
                            r#type: AtType::Comment.to_opt(),
                            context: at_context(),
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
                vec![i32::try_from(num_comments).unwrap_or_default()]
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
            image: self
                .fetch_recipe_image(recipe.id, recipe.image)
                .await?
                .map(|s| {
                    vec![RecipeImageFieldEnum::URL(
                        s.to_str().map(String::from).unwrap_or_default(),
                    )]
                })
                .unwrap_or_default(),
            description: recipe
                .description
                .map(|s| vec![RecipeDescriptionFieldEnum::Text(s)])
                .unwrap_or_default(),
            url: recipe.org_url.map(|u| vec![u]).unwrap_or_default(),
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
            let base = format!("Failed to fetch user '{user_id}' using the Mealie API");
            error!("{base}: {}", res.text().await?);
            return Err(Error::ApiError(format!(
                "{base} because of a validation error (422)."
            )));
        }

        Ok(res.json().await?)
    }
}
