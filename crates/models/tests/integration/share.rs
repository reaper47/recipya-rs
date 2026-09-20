use std::assert_matches;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use app::state::AppState;
use models::Recipe;
use models::settings::UserSettingDetails;
use models::share::ShareRecipe;
use models::user::{User, UserForCreate};
use repository::ModelManager;
use test_db::default_config;
use test_fixtures::{TEST_PASSWORD_HASH, get_password_salt};
use test_harness::create_app_state;

use crate::recipe::utils::a_complete_recipe_for_create;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod test_new {

    use super::*;

    async fn add_user(mm: &ModelManager) -> Result<User> {
        Ok(User::new_with_hash(
            mm,
            UserForCreate {
                email: "another@gmail.com".into(),
                password_clear: "12345677".into(),
            },
            get_password_salt(),
            TEST_PASSWORD_HASH.into(),
        )
        .await?)
    }

    #[tokio::test]
    async fn test_generate_shared_default_expiration_recipe_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user_id = add_user(&state.mm).await?.id;
        let recipe_id = insert_recipe(&state, user_id).await?;

        let got = ShareRecipe::new(&state.mm, recipe_id, user_id, None).await?;

        assert_share_recipe(
            &got,
            &ShareRecipe {
                id: 1,
                link: got.link,
                user_id,
                recipe_id,
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
        let state = create_app_state(default_config()).await;
        let user_id = add_user(&state.mm).await?.id;
        let recipe_id = insert_recipe(&state, user_id).await?;
        let expires_at = {
            let dt = OffsetDateTime::now_utc() + Duration::days(14);
            PrimitiveDateTime::new(dt.date(), dt.time())
        };

        let got = ShareRecipe::new(&state.mm, recipe_id, user_id, Some(expires_at)).await?;

        assert_share_recipe(
            &got,
            &ShareRecipe {
                id: got.id,
                link: got.link,
                user_id,
                recipe_id,
                created_at: got.created_at,
                expires_at,
                last_accessed: got.last_accessed,
                click_count: 0,
            },
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_shared_recipe_already_generated_err() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user_id = add_user(&state.mm).await?.id;
        let recipe_id = insert_recipe(&state, user_id).await?;
        let share = ShareRecipe::new(&state.mm, recipe_id, user_id, None).await?;

        let res = ShareRecipe::new(&state.mm, recipe_id, user_id, None).await;

        assert_matches!(res, Ok(got) if got.id == share.id);
        Ok(())
    }

    fn assert_share_recipe(got: &ShareRecipe, want: &ShareRecipe) {
        pretty_assertions::assert_eq!(got.id, want.id);
        pretty_assertions::assert_ne!(got.link, Uuid::nil());
        pretty_assertions::assert_eq!(got.user_id, want.user_id);
        pretty_assertions::assert_eq!(got.recipe_id, want.recipe_id);
        pretty_assertions::assert_eq!(got.click_count, want.click_count);

        let diff = (got.created_at - want.created_at).whole_nanoseconds();
        assert!(diff.abs() <= 1000, "Created at");

        let diff = (got.expires_at - want.expires_at).whole_nanoseconds();
        assert!(diff.abs() <= 1000, "Expires at");

        let diff = (got.last_accessed - want.last_accessed).whole_nanoseconds();
        assert!(diff.abs() <= 1000, "Last accessed at");
    }
}

mod tests_fetch_by_link {
    use super::*;

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
    async fn test_share_exists_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = add_user(&state.mm).await?;
        let recipe_id = insert_recipe(&state, user.id).await?;
        let shared = ShareRecipe::new(&state.mm, recipe_id, user.id, None).await?;

        let (got, _) = ShareRecipe::get_by_link(&state.mm, shared.link).await?;

        pretty_assertions::assert_eq!(got.id, shared.id);
        Ok(())
    }

    #[tokio::test]
    async fn test_exists_err() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = add_user(&state.mm).await?;
        insert_recipe(&state, user.id).await?;

        let res = ShareRecipe::get_by_link(&state.mm, user.id).await;

        match res {
            Ok(_) => panic!("Entry should not have been found"),
            Err(_) => Ok(()),
        }
    }
}

async fn insert_recipe(state: &AppState, user_id: Uuid) -> Result<i64> {
    let (recipe, _) = a_complete_recipe_for_create();
    let settings = UserSettingDetails::get(&state.mm, user_id).await?;
    let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
    Ok(recipe_id)
}
