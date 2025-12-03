use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

use async_trait::async_trait;
use futures::StreamExt;
use reqwest::{
    Client,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;

use schema_org::{
    AggregateRating, AtType, Comment, Duration, DurationOrText, Energy, Mass, NutritionInformation,
    Recipe,
    field::{
        CommentAuthorFieldEnum, FieldEnum22, RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum,
        RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum, RecipeToolFieldEnum, RecipeYieldFieldEnum,
    },
};
use support::fs::new_fs_support;

use crate::{
    Error, Result,
    api::common::{Credentials, FailedRecipes, RecipeClient},
};

#[derive(Clone)]
struct Host(String);

impl Host {
    fn new(host: impl Into<String>) -> Self {
        let mut host = host.into();
        if host.ends_with('/') {
            host.pop();
        }
        Self(host)
    }

    fn login_url(&self) -> String {
        format!("{}/api/auth/token", self.0)
    }

    fn logout_url(&self) -> String {
        format!("{}/api/auth/logout", self.0)
    }

    fn recipe_image_url(&self, recipe_id: Uuid) -> String {
        format!(
            "{}/api/media/recipes/{recipe_id}/images/original.webp",
            self.0
        )
    }

    fn recipe_url(&self, recipe_id: Uuid) -> String {
        format!("{}/api/recipes/{recipe_id}", self.0)
    }

    fn recipes_url(&self, page: String) -> String {
        format!("{}/api/recipes?page={page}", self.0)
    }

    fn user_url(&self, user_id: Uuid) -> String {
        format!("{}/api/admin/users/{user_id}", self.0)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct MealieRecipes {
    page: u32,
    per_page: u32,
    total: u32,
    total_pages: u32,
    items: Vec<MealieRecipe>,
    next: Option<String>,
    previous: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct MealieRecipe {
    id: Option<Uuid>,
    user_id: Uuid,
    household_id: Uuid,
    group_id: Uuid,
    name: Option<String>,
    slug: String,
    image: Option<String>,
    recipe_servings: u32,
    recipe_yield_quantity: u32,
    recipe_yield: Option<String>,
    total_time: Option<String>,
    prep_time: Option<String>,
    cook_time: Option<String>,
    perform_time: Option<String>,
    description: Option<String>,
    recipe_category: Option<Vec<MealieItem>>,
    tags: Option<Vec<MealieItem>>,
    tools: Vec<MealieTool>,
    rating: Option<u8>,
    #[serde(rename = "orgURL")]
    org_url: Option<String>,
    date_added: Option<String>,
    date_updated: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    last_made: Option<String>,
    recipe_ingredient: Vec<MealieIngredient>,
    recipe_instructions: Option<Vec<MealieInstruction>>,
    nutrition: Option<MealieNutrition>,
    settings: Option<MealieSettings>,
    assets: Option<Vec<MealieAsset>>,
    notes: Option<Vec<MealieNote>>,
    comments: Option<Vec<MealieComment>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct MealieTool {
    id: Option<String>,
    group_id: Option<String>,
    name: String,
    slug: String,
    households_with_tools: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct MealieUser {
    id: Uuid,
    username: Option<String>,
    full_name: Option<String>,
    email: Option<String>,
    auth_method: String,
    admin: bool,
    group: String,
    household: String,
    advanced: bool,
    can_invite: bool,
    can_manage: bool,
    can_manage_household: bool,
    can_organize: bool,
    group_id: Uuid,
    group_slug: String,
    household_id: Uuid,
    household_slug: String,
    tokens: Option<Vec<UserToken>>,
    cache_key: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct UserToken {
    name: String,
    id: u32,
    created_at: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct MealieIngredient {
    quantity: Option<u32>,
    unit: Option<MealieIngredientUnit>,
    food: Option<Vec<MealieIngredientFood>>,
    note: Option<String>,
    display: String,
    title: Option<String>,
    original_text: Option<String>,
    reference_id: Uuid,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct MealieIngredientUnit {
    id: Uuid,
    name: String,
    plural_name: Option<String>,
    description: String,
    fraction: bool,
    abbreviation: String,
    plural_abbreviation: Option<String>,
    use_abbreviation: bool,
    aliases: Vec<MealieItem>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct MealieIngredientFood {
    id: Uuid,
    name: String,
    plural_name: Option<String>,
    description: String,
    label_id: Option<String>,
    aliases: Vec<MealieItem>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct MealieItem {
    id: Option<String>,
    group_id: Option<String>,
    name: String,
    slug: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
struct MealieInstruction {
    id: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct MealieNutrition {
    calories: Option<String>,
    carbohydrate_content: Option<String>,
    cholesterol_content: Option<String>,
    fat_content: Option<String>,
    fiber_content: Option<String>,
    protein_content: Option<String>,
    saturated_fat_content: Option<String>,
    sodium_content: Option<String>,
    sugar_content: Option<String>,
    trans_fat_content: Option<String>,
    unsaturated_fat_content: Option<String>,
}

impl From<MealieNutrition> for NutritionInformation {
    fn from(n: MealieNutrition) -> Self {
        Self {
            calories: n.calories.map(|s| vec![Energy::new(s)]).unwrap_or_default(),
            carbohydrate_content: n
                .carbohydrate_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            cholesterol_content: n
                .cholesterol_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            context: Some("https://schema.org".into()),
            fat_content: n
                .fat_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            fiber_content: n
                .fiber_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            protein_content: n
                .protein_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            saturated_fat_content: n
                .saturated_fat_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            serving_size: vec![],
            sodium_content: n
                .sodium_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            sugar_content: n
                .sugar_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            r#type: Some(AtType::NutritionInformation.to_string()),
            trans_fat_content: n
                .trans_fat_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
            unsaturated_fat_content: n
                .unsaturated_fat_content
                .map(|s| vec![Mass::new(s)])
                .unwrap_or_default(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct MealieSettings {
    public: bool,
    show_nutrition: bool,
    show_assets: bool,
    landscape_view: bool,
    disable_comments: bool,
    locked: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct MealieAsset {
    name: String,
    icon: String,
    file_name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct MealieNote {
    title: String,
    text: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct MealieComment {
    recipe_id: Uuid,
    text: String,
    id: Uuid,
    created_at: String,
    updated_at: String,
    user_id: Uuid,
    user: MealieUser,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

pub struct AuthenticatedState;
pub struct UnauthenticatedState;

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

#[derive(Serialize)]
struct AuthPayload {
    username: String,
    password: String,
    remember_me: bool,
}

#[derive(Clone)]
pub struct MealieRecipeClient {
    host: Host,
    client: Client,
}

impl MealieRecipe {
    fn extract_category_and_keywords(&self) -> (Vec<String>, Vec<RecipeKeywordsFieldEnum>) {
        let mut categories = self
            .recipe_category
            .iter()
            .flatten()
            .map(|c| c.name.clone());

        let category = categories.next().map(|c| vec![c]).unwrap_or_default();

        let mut keywords = categories.collect::<Vec<_>>();
        keywords.extend(self.tags.iter().flatten().map(|tag| tag.name.clone()));

        (
            category,
            keywords
                .into_iter()
                .map(|keyword| RecipeKeywordsFieldEnum::TextOrURL(keyword))
                .collect(),
        )
    }
}

impl MealieUser {
    fn author(&self) -> Option<String> {
        if let Some(u) = self.full_name.clone() {
            Some(u)
        } else if let Some(u) = self.username.clone() {
            Some(u)
        } else if let Some(u) = self.email.clone() {
            Some(u)
        } else {
            None
        }
    }
}

#[async_trait]
impl RecipeClient for MealieRecipeClient {
    async fn login(self, credentials: Credentials) -> Result<Self> {
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
        let recipe_ids = self.fetch_recipe_ids().await?;
        let recipe_details = self.fetch_recipes_helper(recipe_ids).await?;
        let mut recipes = Vec::with_capacity(recipe_details.0.len());
        let mut users = HashMap::new();

        for recipe in recipe_details.0 {
            if !users.contains_key(&recipe.user_id) {
                let user = self.fetch_user(recipe.user_id).await?;
                users.insert(recipe.user_id.clone(), user);
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
        let res = self.client.post(&self.host.logout_url()).send().await?;

        if res.status().is_client_error() {
            let base = "Failed to logout from the Mealie API";
            error!("{base}: {}", res.text().await?);
            return Err(Error::ApiError(format!(
                "{base} because of a validation error (422)."
            )));
        }

        Ok(Self {
            host,
            client: reqwest::Client::new(),
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
            .post(&self.host.login_url())
            .form(&AuthPayload {
                username: credentials.username.to_string(),
                password: credentials.password.to_string(),
                remember_me: true,
            })
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
                .get(&self.host.recipe_image_url(id))
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
        let res = self.client.get(&self.host.recipes_url(page)).send().await?;

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
        let results = futures::stream::iter(ids.into_iter().map(|id| async move {
            match self.client.get(&self.host.recipe_url(id)).send().await {
                Ok(res) => match res.json::<MealieRecipe>().await {
                    Ok(recipe) => Ok((id, recipe)),
                    Err(err) => Err((id, err)),
                },
                Err(err) => Err((id, err)),
            }
        }))
        .buffer_unordered(100)
        .collect::<Vec<_>>()
        .await;

        let mut recipes = Vec::new();
        let mut failures = Vec::new();

        for result in results {
            match result {
                Ok((_, recipe)) => recipes.push(recipe),
                Err((id, err)) => failures.push((
                    id,
                    Error::ApiError(format!(
                        "Failed to fetch recipe '{id}' using the Mealie API: {err}"
                    )),
                )),
            }
        }

        Ok((recipes, failures))
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
                .map(|i| vec![AggregateRating::new(i as f32, 1)])
                .unwrap_or_default(),
            author: user
                .author()
                .map(|s| vec![RecipeAuthorFieldEnum::new_person(&s)])
                .unwrap_or_default(),
            image: match self.fetch_recipe_image(recipe.id, recipe.image).await? {
                Some(s) => vec![FieldEnum22::URL(
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
        let res = self.client.get(&self.host.user_url(user_id)).send().await?;

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
