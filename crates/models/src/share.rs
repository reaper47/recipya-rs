use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::error::Result;
use crate::{Error, Recipe, RecipeDetails};

/// Represents a shared recipe
#[derive(Debug, Eq, PartialEq, Queryable, Identifiable, Selectable)]
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
    pub user_id: Uuid,
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
pub(crate) struct SharedRecipeForInsert {
    pub user_id: Uuid,
    pub recipe_id: i64,
    pub expires_at: Option<NaiveDateTime>,
}

impl ShareRecipe {
    /// Generates a shared recipe from the given recipe for the given user.
    /// Returns the corresponding shared recipe if it already exists in the database.
    pub async fn new(
        mm: &ModelManager,
        recipe_id: i64,
        user_id: Uuid,
        expires_at: Option<NaiveDateTime>,
    ) -> Result<Self> {
        diesel::insert_into(schema::shares_recipes::table)
            .values(&SharedRecipeForInsert {
                user_id,
                recipe_id,
                expires_at,
            })
            .on_conflict((
                schema::shares_recipes::user_id,
                schema::shares_recipes::recipe_id,
            ))
            .do_update()
            .set(schema::shares_recipes::last_accessed.eq(diesel::dsl::now))
            .returning(Self::as_returning())
            .get_result(&mut mm.pool.get().await?)
            .await
            .map_err(Error::from)
    }

    /// Retrieves a shared recipe by its link UUID.
    pub async fn get_by_link(mm: &ModelManager, link: Uuid) -> Result<(Self, RecipeDetails)> {
        let share = schema::shares_recipes::table
            .filter(schema::shares_recipes::link.eq(link))
            .first::<Self>(&mut mm.pool.get().await?)
            .await
            .map_err(Error::from)?;

        let recipe = Recipe::get(mm, share.user_id, share.recipe_id).await?;

        Ok((share, recipe))
    }
}

#[cfg(test)]
mod tests {
    use app::state::AppState;
    use config::Config;
    use test_db::TestDb;
    use test_utils::build_server_logged_in;

    use super::*;
    use crate::{
        recipe::structs::test_utils::a_complete_recipe_for_create, settings::UserSettingDetails,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_new {
        use diesel::internal::derives::multiconnection::chrono;

        use test_utils::create_app_state;

        use super::*;
        use crate::user::{User, UserForCreate};

        async fn add_user(mm: &ModelManager) -> Result<User> {
            Ok(User::new(
                mm,
                UserForCreate {
                    email: "another@gmail.com".into(),
                    password_clear: "12345677".into(),
                },
            )
            .await?)
        }

        #[tokio::test]
        async fn test_generate_shared_default_expiration_recipe_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let user_id = add_user(&state.mm).await?.id;
            insert_recipe(&config, &state, user_id).await?;

            let got = ShareRecipe::new(&state.mm, 1, user_id, None).await?;

            assert_share_recipe(
                &got,
                &ShareRecipe {
                    id: 1,
                    link: got.link,
                    user_id,
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
            let user_id = add_user(&state.mm).await?.id;
            insert_recipe(&config, &state, user_id).await?;
            let expires_at = chrono::Utc::now() + chrono::Duration::days(14);

            let got =
                ShareRecipe::new(&state.mm, 1, user_id, Some(expires_at.naive_local())).await?;

            assert_share_recipe(
                &got,
                &ShareRecipe {
                    id: 1,
                    link: got.link,
                    user_id,
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
            let user_id = add_user(&state.mm).await?.id;
            insert_recipe(&config, &state, user_id).await?;
            let share = ShareRecipe::new(&state.mm, 1, user_id, None).await?;

            let res = ShareRecipe::new(&state.mm, 1, user_id, None).await;

            assert!(matches!(res, Ok(got) if got.id == share.id));
            Ok(())
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
        use test_utils::create_app_state;

        use super::*;
        use crate::user::{User, UserForCreate};

        async fn add_user(mm: &ModelManager) -> Result<User> {
            Ok(User::new(
                mm,
                UserForCreate {
                    email: "thisisit@gmail.com".into(),
                    password_clear: "12345677".into(),
                },
            )
            .await?)
        }

        #[tokio::test]
        async fn test_exists_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let user = add_user(&state.mm).await?;
            insert_recipe(&config, &state, user.id).await?;
            let shared = ShareRecipe::new(&state.mm, 1, user.id, None).await?;

            let (got, _) = ShareRecipe::get_by_link(&state.mm, shared.link).await?;

            pretty_assertions::assert_eq!(got.id, shared.id);
            Ok(())
        }

        #[tokio::test]
        async fn test_exists_err() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let user = add_user(&state.mm).await?;
            insert_recipe(&config, &state, user.id).await?;

            let res = ShareRecipe::get_by_link(&state.mm, user.id).await;

            match res {
                Ok(_) => panic!("Entry should not have been found"),
                Err(_) => Ok(()),
            }
        }
    }

    async fn insert_recipe(config: &Config, state: &AppState, user_id: Uuid) -> Result<()> {
        let _ = build_server_logged_in(config.clone()).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let _ = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        Ok(())
    }
}
