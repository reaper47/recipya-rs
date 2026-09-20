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
