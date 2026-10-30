use uuid::Uuid;

use models::{Recipe, settings::UserSettingDetails};
use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use crate::{Result, recipe::utils::a_complete_recipe_for_create};

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

mod tests_update_category {
    use super::*;

    #[tokio::test]
    async fn test_category_empty_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

        Recipe::update_category(&state.mm, "", &Vec::from([id]).try_into().unwrap(), user.id)
            .await?;

        let c = Recipe::get(&state.mm, user.id, id).await?.category;
        assert_eq!(&c, "uncategorized");
        Ok(())
    }

    #[tokio::test]
    async fn test_category_update_new_category_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (mut recipe, _) = a_complete_recipe_for_create();
        let id1 = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;
        recipe.name = "Cheese whiz".into();
        recipe.category = Some("cheese".into());
        let id2 = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

        Recipe::update_category(
            &state.mm,
            "breakfast",
            &Vec::from([id1, id2]).try_into().unwrap(),
            user.id,
        )
        .await?;

        let c1 = Recipe::get(&state.mm, user.id, id1).await?.category;
        let c2 = Recipe::get(&state.mm, user.id, id2).await?.category;
        assert_eq!([&c1, &c2], ["breakfast", "breakfast"]);
        Ok(())
    }
}
