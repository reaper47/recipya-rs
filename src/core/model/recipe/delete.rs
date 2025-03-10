use super::structs::*;

use diesel::dsl::exists;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::core::model::{Error, Result};
use crate::core::repository::ModelManager;

impl Recipe {
    /// Deletes a user's recipe from the database.
    pub async fn delete(mm: &ModelManager, recipe_id: i64, user_id: i64) -> Result<()> {
        use crate::core::repository::schema;

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
                id: recipe_id,
            }),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::repository::schema;
    use crate::server::AppState;
    use crate::server::test_utils::{TestDb, a_complete_recipe_for_create, insert_user};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_delete_recipe_found_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let user = insert_user(config.clone()).await?;
        let state = AppState::new(config.clone()).await?;
        let recipe_id = Recipe::create(&state.mm, user.id, &a_complete_recipe_for_create()).await?;

        Recipe::delete(&state.mm, recipe_id, user.id).await?;

        let res = Recipe::get(&state.mm, recipe_id, user.id).await;
        pretty_assertions::assert_eq!(res.is_err(), true);
        let mut conn = state.mm.pool.get().await?;
        // categories_recipes
        let result: bool = diesel::select(exists(
            schema::categories_recipes::table
                .filter(schema::categories_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("category"));
        let result: bool = diesel::select(exists(
            schema::cookbooks_recipes::table
                .filter(schema::cookbooks_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("cookbook"));
        // cuisines_recipes
        let result: bool = diesel::select(exists(
            schema::cuisines_recipes::table
                .filter(schema::cuisines_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("cuisine"));
        // ingredients_recipes
        let result: bool = diesel::select(exists(
            schema::ingredients_recipes::table
                .filter(schema::ingredients_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("ingredient"));
        // instructions_recipes
        let result: bool = diesel::select(exists(
            schema::instructions_recipes::table
                .filter(schema::instructions_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("instruction"));
        // keywords_recipes
        let result: bool = diesel::select(exists(
            schema::keywords_recipes::table
                .filter(schema::keywords_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("keyword"));
        // nutrition
        let result: bool = diesel::select(exists(
            schema::nutrition::table.filter(schema::nutrition::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("nutrition"));
        // shares_recipes
        let result: bool = diesel::select(exists(
            schema::shares_recipes::table.filter(schema::shares_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("shared recipes"));
        // times
        let result: bool = diesel::select(exists(
            schema::times::table.filter(schema::times::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("times"));
        // tools_recipes
        let result: bool = diesel::select(exists(
            schema::tools_recipes::table.filter(schema::tools_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("tool"));
        // videos_recipes
        let result: bool = diesel::select(exists(
            schema::videos_recipes::table.filter(schema::videos_recipes::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("video"));
        // additional_images_recipes
        let result: bool = diesel::select(exists(
            schema::additional_images_recipe::table
                .filter(schema::additional_images_recipe::recipe_id.eq(recipe_id)),
        ))
        .get_result(&mut conn)
        .await?;
        pretty_assertions::assert_eq!(result, false, "{}", fail_message("additional image"));
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_recipe_not_found_err() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let user = insert_user(config.clone()).await?;
        let state = AppState::new(config.clone()).await?;

        match Recipe::delete(&state.mm, 1, user.id).await {
            Ok(_) => panic!("Should not return error"),
            Err(Error::EntityNotFound { .. }) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    fn fail_message(component: &str) -> String {
        format!("should not be any {component} associated with recipe")
    }
}
