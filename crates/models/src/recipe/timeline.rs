use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use time::PrimitiveDateTime;
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::Recipe;
use crate::user::User;
use crate::{Error, Result};

/// Represents the timeline entity of a recipe stored in the database.
#[derive(Debug, Eq, PartialEq, AsChangeset, Associations, Queryable, Identifiable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::recipe_timelines)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeTimeline {
    pub id: i64,
    pub recipe_id: i64,
    pub user_id: Uuid,
    pub title: String,
    pub comment: Option<String>,
    pub rating: Option<i16>,
    pub image: Option<Uuid>,
    pub created_at: PrimitiveDateTime,
}

impl Default for RecipeTimeline {
    fn default() -> Self {
        Self {
            id: 0,
            recipe_id: 0,
            user_id: Uuid::nil(),
            title: String::new(),
            comment: None,
            rating: None,
            image: None,
            created_at: PrimitiveDateTime::MIN,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = schema::recipe_timelines)]
struct RecipeTimelinePatch<'a> {
    title: Option<&'a str>,
    comment: Option<&'a str>,
    rating: Option<i16>,
    image: Option<Uuid>,
    created_at: Option<PrimitiveDateTime>,
}

/// The minimal struct for creating a new timeline into the database.
#[derive(Default)]
pub struct RecipeTimelineForCreate {
    pub title: String,
    pub comment: Option<String>,
    pub rating: Option<i16>,
    pub image: Option<Uuid>,
    pub created_at: Option<PrimitiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::recipe_timelines)]
struct TimelineForInsert<'a> {
    recipe_id: i64,
    user_id: Uuid,
    title: &'a str,
    comment: Option<&'a str>,
    rating: Option<i16>,
    image: Option<Uuid>,
    created_at: Option<PrimitiveDateTime>,
}

impl RecipeTimeline {
    /// Retrieves all timelines associated with the user's recipe.
    pub async fn all(mm: &ModelManager, recipe_id: i64, user_id: Uuid) -> Result<Vec<Self>> {
        use schema::recipe_timelines;

        let timelines = recipe_timelines::table
            .filter(recipe_timelines::recipe_id.eq(recipe_id))
            .filter(recipe_timelines::user_id.eq(user_id))
            .order(recipe_timelines::created_at.asc())
            .load::<Self>(&mut mm.pool.get().await?)
            .await?;

        Ok(timelines)
    }

    /// Creates a new timeline in the database for the user's recipe.
    pub async fn create(
        mm: &ModelManager,
        recipe_id: i64,
        user_id: Uuid,
        timeline_c: &RecipeTimelineForCreate,
    ) -> Result<i64> {
        let timeline_id: i64 = diesel::insert_into(schema::recipe_timelines::table)
            .values(&TimelineForInsert {
                recipe_id,
                user_id,
                title: timeline_c.title.as_str(),
                comment: timeline_c.comment.as_deref(),
                rating: timeline_c.rating,
                image: timeline_c.image,
                created_at: timeline_c.created_at,
            })
            .returning(schema::recipe_timelines::id)
            .get_result(&mut mm.pool.get().await?)
            .await?;

        Ok(timeline_id)
    }

    /// Updates the fields of an existing timeline.
    pub async fn edit(mm: &ModelManager, user_id: Uuid, new_timeline: &Self) -> Result<Self> {
        let patch = RecipeTimelinePatch {
            title: Some(&new_timeline.title),
            comment: new_timeline.comment.as_deref(),
            rating: new_timeline.rating,
            image: new_timeline.image,
            created_at: (new_timeline.created_at != PrimitiveDateTime::MIN)
                .then_some(new_timeline.created_at),
        };

        let result = diesel::update(
            schema::recipe_timelines::table
                .filter(schema::recipe_timelines::id.eq(new_timeline.id))
                .filter(schema::recipe_timelines::user_id.eq(user_id)),
        )
        .set(&patch)
        .get_result::<Self>(&mut mm.pool.get().await?)
        .await?;

        Ok(result)
    }

    /// Gets a single timeline event.
    pub async fn get(
        mm: &ModelManager,
        timeline_id: i64,
        recipe_id: i64,
        user_id: Uuid,
    ) -> Result<Self> {
        let mut conn = mm.pool.get().await?;

        let timeline = match schema::recipe_timelines::table
            .filter(schema::recipe_timelines::id.eq(timeline_id))
            .filter(schema::recipe_timelines::recipe_id.eq(recipe_id))
            .filter(schema::recipe_timelines::user_id.eq(user_id))
            .first::<Self>(&mut conn)
            .await
        {
            Ok(v) => v,
            Err(diesel::NotFound) => {
                return Err(Error::EntityNotFound {
                    entity: "RecipeTimeline",
                    id: timeline_id.to_string(),
                });
            }
            Err(e) => return Err(e.into()),
        };

        Ok(timeline)
    }
}
