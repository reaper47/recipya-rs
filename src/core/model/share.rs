use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::core::model::error::Result;
use crate::core::model::{Error, Recipe, RecipeDetails};
use crate::core::repository::{ModelManager, schema};

/// Represents a shared recipe
#[derive(Debug, PartialEq, Queryable, Identifiable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::shares_recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShareRecipe {
    /// The unique identifier of the shared recipe.
    pub id: i64,
    /// The URI of shared recipe to be appended to the server's base URL.
    pub link: Uuid,
    /// The foreign key linking the shared recipe to its creator.
    pub user_id: i64,
    /// The foreign key linking the shared recipe to its content.
    pub recipe_id: i64,
    /// The timestamp when the shared recipe link was generated.
    pub created_at: NaiveDateTime,
    /// The timestamp when the shared recipe link expires.
    pub expires_at: NaiveDateTime,
    /// The timestamp when the shared recipe was last accessed.
    pub last_accessed: NaiveDateTime,
    /// The number of times the shared recipe was opened.
    pub click_count: i32,
}

/// A struct for inserting a new shared recipe into the database.
#[derive(Insertable)]
#[diesel(table_name = schema::shares_recipes)]
pub(super) struct SharedRecipeForInsert {
    pub user_id: i64,
    pub recipe_id: i64,
    pub expires_at: Option<NaiveDateTime>,
}

impl ShareRecipe {
    /// Generates a shared recipe from the given recipe for the given user.
    /// Returns the corresponding shared recipe if it already exists in the database.
    pub async fn new(
        mm: &ModelManager,
        recipe_id: i64,
        user_id: i64,
        expires_at: Option<NaiveDateTime>,
    ) -> Result<ShareRecipe> {
        let mut conn = mm.pool.get().await?;

        diesel::insert_into(schema::shares_recipes::table)
            .values(&SharedRecipeForInsert {
                recipe_id,
                user_id,
                expires_at,
            })
            .on_conflict_do_nothing()
            .returning(ShareRecipe::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(Error::from)
    }

    /// Retrieves a shared recipe by its link UUID.
    pub async fn get_by_link(
        mm: &ModelManager,
        link: Uuid,
    ) -> Result<(ShareRecipe, RecipeDetails)> {
        let mut conn = mm.pool.get().await?;

        let share = schema::shares_recipes::table
            .filter(schema::shares_recipes::link.eq(link))
            .first::<ShareRecipe>(&mut conn)
            .await
            .map_err(Error::from)?;

        let recipe = Recipe::get(mm, share.user_id, share.recipe_id).await?;

        Ok((share, recipe))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::config::Config;
    use crate::core::model::Recipe;
    use crate::server::AppState;
    use crate::server::test_utils::{a_complete_recipe_for_create, build_server_logged_in};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_new {
        use super::*;
        use diesel::internal::derives::multiconnection::chrono;

        use crate::server::AppState;
        use crate::server::test_utils::{TestDb, create_app_state};

        #[tokio::test]
        async fn test_generate_shared_default_expiration_recipe_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            insert_recipe(&config, &state).await?;

            let got = ShareRecipe::new(&state.mm, 1, 1, None).await?;

            assert_share_recipe(
                &got,
                &ShareRecipe {
                    id: 1,
                    link: got.link,
                    user_id: 1,
                    recipe_id: 1,
                    created_at: got.created_at,
                    expires_at: got.expires_at,
                    last_accessed: got.last_accessed,
                    click_count: 0,
                },
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_generate_shared_recipe_custom_expiration_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            insert_recipe(&config, &state).await?;
            let expires_at = chrono::Utc::now() + chrono::Duration::days(14);

            let got = ShareRecipe::new(&state.mm, 1, 1, Some(expires_at.naive_local())).await?;

            assert_share_recipe(
                &got,
                &ShareRecipe {
                    id: 1,
                    link: got.link,
                    user_id: 1,
                    recipe_id: 1,
                    created_at: got.created_at,
                    expires_at: expires_at.naive_local(),
                    last_accessed: got.last_accessed,
                    click_count: 0,
                },
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_shared_recipe_already_generated_err() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            insert_recipe(&config, &state).await?;
            let _ = ShareRecipe::new(&state.mm, 1, 1, None).await?;

            let res = ShareRecipe::new(&state.mm, 1, 1, None).await;

            match res {
                Ok(_) => panic!("Should not have inserted an entry in the database"),
                Err(_) => Ok(()),
            }
        }

        fn assert_share_recipe(got: &ShareRecipe, want: &ShareRecipe) {
            pretty_assertions::assert_eq!(got.id, want.id);
            pretty_assertions::assert_ne!(got.link, Uuid::nil());
            pretty_assertions::assert_eq!(got.user_id, want.user_id);
            pretty_assertions::assert_eq!(got.recipe_id, want.recipe_id);
            pretty_assertions::assert_eq!(got.click_count, want.click_count);

            let diff = (got.created_at - want.created_at)
                .num_nanoseconds()
                .unwrap_or(i64::MAX);
            assert!(diff.abs() <= 1000, "Created at");

            let diff = (got.expires_at - want.expires_at)
                .num_nanoseconds()
                .unwrap_or(i64::MAX);
            assert!(diff.abs() <= 1000, "Expires at");

            let diff = (got.last_accessed - want.last_accessed)
                .num_nanoseconds()
                .unwrap_or(i64::MAX);
            assert!(diff.abs() <= 1000, "Last accessed at");
        }
    }

    mod tests_fetch_by_link {
        use super::*;
        use crate::server::AppState;
        use crate::server::test_utils::{TestDb, create_app_state};

        #[tokio::test]
        async fn test_exists_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            insert_recipe(&config, &state).await?;
            let shared = ShareRecipe::new(&state.mm, 1, 1, None).await?;

            let (got, _) = ShareRecipe::get_by_link(&state.mm, shared.link).await?;

            pretty_assertions::assert_eq!(got.id, shared.id);
            Ok(())
        }

        #[tokio::test]
        async fn test_exists_err() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            insert_recipe(&config, &state).await?;

            let res = ShareRecipe::get_by_link(&state.mm, Uuid::new_v4()).await;

            match res {
                Ok(_) => panic!("Entry should not have been found"),
                Err(_) => Ok(()),
            }
        }
    }

    async fn insert_recipe(config: &Config, state: &AppState) -> Result<()> {
        let _ = build_server_logged_in(config.clone()).await?;
        let _ = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
        Ok(())
    }
}
