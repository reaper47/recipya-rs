use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

use reqwest::{
    Client,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

use schema_org::{AtType, Recipe, field::FieldEnum22};
use support::fs::new_fs_support;

use crate::{
    Error, Result,
    api::common::{Credentials, RecipeClient},
};

#[derive(Clone)]
struct Host(String);

impl Host {
    fn new(host: impl Into<String>) -> Self {
        Self(host.into())
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

    fn recipes_url(&self, page: String) -> String {
        format!("{}/api/recipes?page={page}", self.0)
    }

    fn user_url(&self, user_id: Uuid) -> String {
        format!("{}/api/admin/users/{user_id}", self.0)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(deny_unknown_fields)]
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
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct MealieItem {
    id: Option<String>,
    group_id: Option<String>,
    name: String,
    slug: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct MealieTool {
    id: Option<String>,
    group_id: Option<String>,
    name: String,
    slug: String,
    households_with_tools: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
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
struct UserToken {
    name: String,
    id: u32,
    created_at: Option<String>,
}

struct AuthenticatedState;
struct UnauthenticatedState;

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

impl<C: RecipeClient> Unauthenticated<C> for Mealie<UnauthenticatedState, C> {
    async fn login(&mut self, credentials: Credentials) -> Result<Mealie<AuthenticatedState, C>> {
        self.recipe_client.login(credentials).await?;

        Ok(Mealie {
            recipe_client: self.recipe_client.clone(),
            _state: PhantomData,
        })
    }
}

impl<C: RecipeClient> Authenticated<C> for Mealie<AuthenticatedState, C> {
    async fn fetch_recipes(&self) -> Result<Vec<Recipe>> {
        Ok(self.recipe_client.fetch_recipes().await?)
    }

    async fn logout(&mut self) -> Result<Mealie<UnauthenticatedState, C>> {
        self.recipe_client.logout().await?;

        Ok(Mealie {
            recipe_client: self.recipe_client.clone(),
            _state: PhantomData,
        })
    }
}

trait Unauthenticated<C: RecipeClient>: Send + Sync {
    /// Establishes a connection to the host using the provided credentials. Returns a token upon successful login.
    async fn login(&mut self, credentials: Credentials) -> Result<Mealie<AuthenticatedState, C>>;
}

trait Authenticated<C: RecipeClient>: Send + Sync {
    /// Fetches recipes from connected host.
    async fn fetch_recipes(&self) -> Result<Vec<Recipe>>;
    /// Logs out of the Mealie instance.
    async fn logout(&mut self) -> Result<Mealie<UnauthenticatedState, C>>;
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

impl RecipeClient for MealieRecipeClient {
    async fn login(&mut self, credentials: Credentials) -> Result<()> {
        let token = self.login_helper(credentials).await?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))
                .map_err(|err| Error::ApiError(format!("Invalid token: {err}")))?,
        );

        self.client = Client::builder().default_headers(headers).build()?;
        Ok(())
    }

    async fn fetch_recipes(&self) -> Result<Vec<Recipe>> {
        let mut recipes = Vec::new();
        let mut next_page = Some("1".to_string());

        while let Some(page) = next_page {
            let mealie_recipes = self.fetch_recipes_page(page).await?;

            let mut users = HashMap::new();

            for mealie_recipe in mealie_recipes.items {
                if !users.contains_key(&mealie_recipe.user_id) {
                    let user = self.fetch_user(mealie_recipe.user_id).await?;
                    users.insert(mealie_recipe.user_id.clone(), user);
                }

                let _user = &users[&mealie_recipe.user_id];

                recipes.push(Recipe {
                    r#type: Some(AtType::Recipe.to_string()),
                    context: Some("https://schema.org".into()),
                    // nutrition: (),
                    // cooking_method: (),
                    // recipe_yield: (),
                    // recipe_cuisine: (),
                    // ingredients: (),
                    // recipe_ingredient: (),
                    // suitable_for_diet: (),
                    // cook_time: (),
                    // recipe_instructions: (),
                    // recipe_category: (),
                    // steps: (),
                    // r#yield: (),
                    // tool: (),
                    // step: (),
                    // prep_time: (),
                    // estimated_cost: (),
                    // total_time: (),
                    // perform_time: (),
                    // supply: (),
                    // comment: (),
                    // is_based_on_url: (),
                    // translation_of_work: (),
                    // work_translation: (),
                    // mentions: (),
                    // date_created: (),
                    // word_count: (),
                    // size: (),
                    // maintainer: (),
                    // license: (),
                    // expires: (),
                    // comment_count: (),
                    // time_required: (),
                    // review: (),
                    // contributor: (),
                    // interaction_statistic: (),
                    // publisher: (),
                    // credit_text: (),
                    // headline: (),
                    // editor: (),
                    // date_modified: (),
                    // is_accessible_for_free: (),
                    // keywords: (),
                    // provider: (),
                    // creator: (),
                    // sd_date_published: (),
                    // content_reference_time: (),
                    // archived_at: (),
                    // discussion_url: (),
                    // content_rating: (),
                    // country_of_origin: (),
                    // text: (),
                    // award: (),
                    // is_based_on: (),
                    // aggregate_rating: (),
                    // in_language: (),
                    // date_published: (),
                    // sd_publisher: (),
                    // audio: (),
                    // alternative_headline: (),
                    // about: (),
                    // is_part_of: (),
                    // thumbnail: (),
                    // thumbnail_url: (),
                    // copyright_year: (),
                    // work_example: (),
                    // citation: (),
                    // video: (),
                    // awards: (),
                    // producer: (),
                    // schema_version: (),
                    // author: (),
                    // translator: (),
                    // reviews: (),
                    // disambiguating_description: (),
                    image: match self
                        .fetch_recipe_image(mealie_recipe.id, mealie_recipe.image)
                        .await?
                    {
                        Some(s) => vec![FieldEnum22::URL(
                            s.to_str().map(String::from).unwrap_or_default(),
                        )],
                        None => vec![],
                    },
                    // same_as: (),
                    // description: (),
                    // alternate_name: (),
                    // url: (),
                    // subject_of: (),
                    name: match mealie_recipe.name {
                        Some(s) => vec![s],
                        None => vec![mealie_recipe.slug],
                    },
                    ..Default::default()
                });
            }

            next_page = mealie_recipes.next;
        }

        Ok(recipes)
    }

    async fn logout(&mut self) -> Result<()> {
        let res = self.client.post(&self.host.logout_url()).send().await?;

        if res.status().is_client_error() {
            let base = "Failed to logout from the Mealie API";
            error!("{base}: {}", res.text().await?);
            return Err(Error::ApiError(format!(
                "{base} because of a validation error (422)."
            )));
        }

        self.client = reqwest::Client::new();
        Ok(())
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

        let token: String = res.text().await.map_err(|err| {
            let base = "Failed to parse the token returned from the Mealie API response";
            error!("{base}: {err}");
            Error::ApiError(format!("{base}."))
        })?;

        Ok(token)
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

    async fn fetch_recipes_page(&self, page: String) -> Result<MealieRecipes> {
        let res = self.client.get(&self.host.recipes_url(page)).send().await?;

        if res.status().is_client_error() {
            let base = "Failed to fetch recipes using the Mealie API";
            error!("{base}: {}", res.text().await?);
            return Err(Error::ApiError(format!(
                "{base} because of a validation error (422)."
            )));
        }

        Ok(res.json().await?)
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

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[derive(Clone)]
    struct MockRecipeClient;

    impl MockRecipeClient {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl RecipeClient for MockRecipeClient {
        async fn login(&mut self, _: Credentials) -> crate::Result<()> {
            Ok(())
        }

        async fn fetch_recipes(&self) -> crate::Result<Vec<schema_org::Recipe>> {
            todo!()
        }

        async fn logout(&mut self) -> crate::Result<()> {
            Ok(())
        }
    }

    // #[tokio::test]
    // async fn test_new() -> Result<()> {
    //     let mut mealie = Mealie::new(MockRecipeClient::new());
    //     let mut connected_mealie = mealie
    //         .login(Credentials {
    //             username: "demo-user".into(),
    //             password: "admin".into(),
    //         })
    //         .await?;

    //     let recipes = connected_mealie.fetch_recipes().await?;

    //     connected_mealie.logout().await?;
    //     Ok(())
    // }
}
