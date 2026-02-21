use diesel::dsl::exists;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::{Error, Result, recipe::structs::recipe::Recipe};

impl Recipe {
    /// Deletes a user's recipe from the database.
    pub async fn delete(mm: &ModelManager, recipe_id: i64, user_id: Uuid) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        let num_deleted = diesel::delete(
            schema::recipes::table.filter(exists(
                schema::users_recipes::table
                    .filter(schema::users_recipes::user_id.eq(user_id))
                    .filter(schema::recipes::id.eq(recipe_id))
                    .select(schema::users_recipes::recipe_id),
            )),
        )
        .execute(&mut conn)
        .await?;

        match num_deleted {
            0 => Err(Error::EntityNotFound {
                entity: "recipe",
                id: recipe_id.to_string(),
            }),
            _ => Ok(()),
        }
    }

    pub async fn delete_recipe_category(
        mm: &ModelManager,
        category: &str,
        user_id: Uuid,
    ) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        diesel::delete(
            schema::users_categories::table.filter(
                schema::users_categories::category_id
                    .eq_any(
                        schema::categories::table
                            .filter(schema::categories::name.eq(category))
                            .select(schema::categories::id),
                    )
                    .and(schema::users_categories::user_id.eq(user_id)),
            ),
        )
        .execute(&mut conn)
        .await?;

        if let Ok(category_id) = schema::categories::table
            .filter(schema::categories::name.eq(category))
            .select(schema::categories::id)
            .first::<i64>(&mut conn)
            .await
        {
            let subquery = schema::recipes::table
                .inner_join(
                    schema::categories_recipes::table
                        .on(schema::categories_recipes::recipe_id.eq(schema::recipes::id)),
                )
                .inner_join(
                    schema::users_recipes::table
                        .on(schema::users_recipes::recipe_id.eq(schema::recipes::id)),
                )
                .filter(schema::categories_recipes::category_id.eq(category_id))
                .filter(schema::users_recipes::user_id.eq(user_id))
                .select(schema::recipes::id)
                .into_boxed();

            let _ = diesel::update(
                schema::categories_recipes::table
                    .filter(schema::categories_recipes::recipe_id.eq_any(subquery)),
            )
            .set(
                schema::categories_recipes::category_id.eq(schema::categories::table
                    .filter(schema::categories::name.eq("uncategorized"))
                    .select(schema::categories::id)
                    .first::<i64>(&mut conn)
                    .await?),
            )
            .execute(&mut conn)
            .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recipe::structs::test_utils::a_complete_recipe_for_create;

    use testing::utils::{TestDb, create_app_state, insert_user};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_delete {
        use super::*;

        #[tokio::test]
        #[allow(clippy::too_many_lines)]
        async fn test_delete_recipe_found_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let recipe_id =
                Recipe::create(&state.mm, user.id, &a_complete_recipe_for_create()).await?;

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

            let result: bool = diesel::select(exists(
                schema::cookbooks_recipes::table
                    .filter(schema::cookbooks_recipes::recipe_id.eq(recipe_id)),
            ))
            .get_result(&mut conn)
            .await?;
            assert!(!result, "{}", fail_message("cookbook"));

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
                schema::shares_recipes::table
                    .filter(schema::shares_recipes::recipe_id.eq(recipe_id)),
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
                schema::videos_recipes::table
                    .filter(schema::videos_recipes::recipe_id.eq(recipe_id)),
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
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;

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

    mod tests_delete_recipe_category {
        use super::*;
        use crate::user::UserCategory;

        use diesel_async::RunQueryDsl;

        const A_CATEGORY: &str = "midnight crunchies";

        #[tokio::test]
        async fn test_category_not_found_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let mut conn = state.mm.pool.get().await?;
            let categories_before = schema::users_categories::table
                .load::<UserCategory>(&mut conn)
                .await?;

            Recipe::delete_recipe_category(&state.mm, A_CATEGORY, user.id).await?;

            let categories_after = schema::users_categories::table
                .load::<UserCategory>(&mut conn)
                .await?;
            pretty_assertions::assert_eq!(categories_before.len(), categories_after.len());
            Ok(())
        }

        #[tokio::test]
        async fn test_category_found_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let user = insert_user(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            Recipe::add_category(&state.mm, A_CATEGORY, user.id).await?;
            let mut a_recipe = a_complete_recipe_for_create();
            a_recipe.category = Some(A_CATEGORY.to_string());
            let recipe_id = Recipe::create(&state.mm, user.id, &a_recipe).await?;

            Recipe::delete_recipe_category(&state.mm, A_CATEGORY, user.id).await?;

            let recipe = Recipe::get(&state.mm, user.id, recipe_id).await?;
            pretty_assertions::assert_eq!(recipe.category, "uncategorized");
            Ok(())
        }
    }
}
