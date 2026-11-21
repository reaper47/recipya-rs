use diesel::prelude::*;
use diesel::{Queryable, Selectable};
use diesel_async::RunQueryDsl;
use jiff::Timestamp;
use jiff_diesel::ToDiesel;
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
    #[diesel(serialize_as = jiff_diesel::Timestamp, deserialize_as = jiff_diesel::Timestamp)]
    pub created_at: Timestamp,
    /// The timestamp when the shared recipe link expires.
    #[diesel(serialize_as = jiff_diesel::Timestamp, deserialize_as = jiff_diesel::Timestamp)]
    pub expires_at: Timestamp,
    /// The timestamp when the shared recipe was last accessed.
    #[diesel(serialize_as = jiff_diesel::Timestamp, deserialize_as = jiff_diesel::Timestamp)]
    pub last_accessed: Timestamp,
    /// The number of times the shared recipe was opened.
    pub click_count: i32,
}

/// A struct for inserting a new shared recipe into the database.
#[derive(Insertable)]
#[diesel(table_name = schema::shares_recipes)]
pub(crate) struct SharedRecipeForInsert {
    pub user_id: Uuid,
    pub recipe_id: i64,
    pub expires_at: Option<jiff_diesel::Timestamp>,
}

impl ShareRecipe {
    /// Generates a shared recipe from the given recipe for the given user.
    /// Returns the corresponding shared recipe if it already exists in the database.
    pub async fn new(
        mm: &ModelManager,
        recipe_id: i64,
        user_id: Uuid,
        expires_at: Option<Timestamp>,
    ) -> Result<Self> {
        diesel::insert_into(schema::shares_recipes::table)
            .values(&SharedRecipeForInsert {
                user_id,
                recipe_id,
                expires_at: expires_at.map(ToDiesel::to_diesel),
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
