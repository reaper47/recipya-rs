use uuid::Uuid;

use models::{Recipe, settings::UserSettingDetails};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::recipe::utils::a_complete_recipe_for_create;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod tests_mark_favourite {
    use super::*;

    #[tokio::test]
    async fn test_user_does_not_exist_err() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

        if Recipe::toggle_favourite(&state.mm, Uuid::new_v4(), id)
            .await
            .is_ok()
        {
            panic!("Expected error");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_valid_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let initial_state = recipe.is_favourite;
        let id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

        let current_state = Recipe::toggle_favourite(&state.mm, user.id, id).await?;

        pretty_assertions::assert_eq!(current_state, !initial_state);
        Ok(())
    }
}
