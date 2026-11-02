use diesel::{ExpressionMethods, QueryDsl, dsl::exists};
use diesel_async::RunQueryDsl;

use models::settings::UserSettingDetails;
use models::user::UserCategory;
use models::{Error, Recipe};
use repository::schema;
use test_db::default_config;
use test_fixtures::{insert_other_user, insert_user};
use test_harness::create_app_state;

use crate::{Result, recipe::utils::a_complete_recipe_for_create};

mod tests_delete {
    use super::*;

    #[tokio::test]
    #[allow(clippy::too_many_lines)]
    async fn test_delete_recipe_found_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;

        Recipe::delete(&state.mm, recipe_id, user.id).await?;

        let res = Recipe::get(&state.mm, user.id, recipe_id).await;
        assert!(res.is_err());
        let mut conn = state.mm.pool.get().await?;

        // categories_recipes
        let result: bool = diesel::select(exists(
            schema::categories_recipes::table
                .filter(schema::categories_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result, "{}", fail_message("category"));

        // let result: bool = diesel::select(exists(
        //     schema::cookbooks_recipes::table
        //         .filter(schema::cookbooks_recipes::recipe_id.eq(recipe_id)),
        // ))
        // .get_result(&mut conn)
        // .await?;
        // assert!(!result, "{}", fail_message("cookbook"));

        // cuisines_recipes
        let result: bool = diesel::select(exists(
            schema::cuisines_recipes::table
                .filter(schema::cuisines_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result, "{}", fail_message("cuisine"));

        // ingredients_recipes
        let result: bool = diesel::select(exists(
            schema::ingredients_recipes::table
                .filter(schema::ingredients_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result, "{}", fail_message("ingredient"));

        // instructions_recipes
        let result: bool = diesel::select(exists(
            schema::instructions_recipes::table
                .filter(schema::instructions_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result, "{}", fail_message("instruction"));

        // keywords_recipes
        let result: bool = diesel::select(exists(
            schema::keywords_recipes::table
                .filter(schema::keywords_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result, "{}", fail_message("keyword"));

        // nutrition
        let result_per_100g: bool = diesel::select(exists(
            schema::nutrition_per_100g::table
                .filter(schema::nutrition_per_100g::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result, "{}", fail_message("nutrition_per_100g"));
        let result_per_serving: bool = diesel::select(exists(
            schema::nutrition_per_serving::table
                .filter(schema::nutrition_per_serving::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result_per_100g, "{}", fail_message("nutrition_per_100g"));
        assert!(
            !result_per_serving,
            "{}",
            fail_message("nutrition_per_serving")
        );

        // shares_recipes
        let result: bool = diesel::select(exists(
            schema::shares_recipes::table.filter(schema::shares_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result, "{}", fail_message("shared recipes"));

        // times
        let result: bool = diesel::select(exists(
            schema::times::table.filter(schema::times::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result, "{}", fail_message("times"));

        // tools_recipes
        let result: bool = diesel::select(exists(
            schema::tools_recipes::table.filter(schema::tools_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result, "{}", fail_message("tool"));

        // videos_recipes
        let result: bool = diesel::select(exists(
            schema::videos_recipes::table.filter(schema::videos_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result, "{}", fail_message("video"));

        // additional_images_recipes
        let result: bool = diesel::select(exists(
            schema::additional_images_recipe::table
                .filter(schema::additional_images_recipe::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        assert!(!result, "{}", fail_message("additional image"));
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_recipe_not_found_err() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;

        match Recipe::delete(&state.mm, 1, user.id).await {
            Ok(()) => panic!("Should not return error"),
            Err(Error::EntityNotFound { .. }) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    fn fail_message(component: &str) -> String {
        format!("should not be any {component} associated with recipe")
    }
}

mod tests_delete_bulk {
    use super::*;

    #[tokio::test]
    async fn test_delete_recipes_found_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "Brazilian Chicken".into();
        let id1 = Recipe::create(&state.mm, user.id, &recipe1, &settings).await?;
        let id2 = Recipe::create(&state.mm, user.id, &recipe2, &settings).await?;

        Recipe::delete_bulk(&state.mm, &[id1, id2], user.id).await?;

        let got = Recipe::all(&state.mm, user.id).await?;
        assert_eq!(got.len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_recipes_some_found_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user1 = insert_user(&state.mm).await?;
        let user2 = insert_other_user(&state.mm, "slava@ukraini.ua").await?;
        let settings1 = UserSettingDetails::get(&state.mm, user1.id).await?;
        let settings2 = UserSettingDetails::get(&state.mm, user2.id).await?;
        let (recipe1, _) = a_complete_recipe_for_create();
        let (mut recipe2, _) = a_complete_recipe_for_create();
        recipe2.name = "Brazilian Chicken".into();
        let id1 = Recipe::create(&state.mm, user1.id, &recipe1, &settings1).await?;
        let id2 = Recipe::create(&state.mm, user2.id, &recipe2, &settings2).await?;

        Recipe::delete_bulk(&state.mm, &[id1, id2], user1.id).await?;

        let got1 = Recipe::all(&state.mm, user1.id).await?;
        assert_eq!(got1.len(), 0);
        let got2 = Recipe::all(&state.mm, user2.id).await?;
        assert_eq!(got2.len(), 1);
        Ok(())
    }
}

mod tests_delete_recipe_category {
    use super::*;

    const A_CATEGORY: &str = "midnight crunchies";

    #[tokio::test]
    async fn test_category_not_found_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let categories_before = schema::users_categories::table
            .load::<UserCategory>(&mut state.mm.pool.get().await?)
            .await?;

        Recipe::delete_recipe_category(&state.mm, A_CATEGORY, user.id).await?;

        let categories_after = schema::users_categories::table
            .load::<UserCategory>(&mut state.mm.pool.get().await?)
            .await?;
        pretty_assertions::assert_eq!(categories_before.len(), categories_after.len());
        Ok(())
    }

    #[tokio::test]
    async fn test_category_found_ok() -> Result<()> {
        let state = create_app_state(default_config()).await;
        let user = insert_user(&state.mm).await?;
        let settings = UserSettingDetails::get(&state.mm, user.id).await?;
        Recipe::add_category(&state.mm, A_CATEGORY, user.id).await?;
        let (mut a_recipe, _) = a_complete_recipe_for_create();
        a_recipe.category = Some(A_CATEGORY.to_string());
        let recipe_id = Recipe::create(&state.mm, user.id, &a_recipe, &settings).await?;

        Recipe::delete_recipe_category(&state.mm, A_CATEGORY, user.id).await?;

        let recipe = Recipe::get(&state.mm, user.id, recipe_id).await?;
        pretty_assertions::assert_eq!(recipe.category, "uncategorized");
        Ok(())
    }
}
