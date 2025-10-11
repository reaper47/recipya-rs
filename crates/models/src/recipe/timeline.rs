use diesel::prelude::*;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::Recipe;
use crate::user::User;
use crate::{Error, Result};

/// Represents the timeline entity of a recipe stored in the database.
#[derive(
    Debug, Default, PartialEq, AsChangeset, Associations, Queryable, Identifiable, Selectable,
)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::recipe_timelines)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecipeTimeline {
    pub id: i64,
    pub recipe_id: i64,
    pub user_id: i64,
    pub title: String,
    pub comment: Option<String>,
    pub rating: Option<i16>,
    pub image: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = schema::recipe_timelines)]
struct RecipeTimelinePatch<'a> {
    title: Option<&'a str>,
    comment: Option<&'a str>,
    rating: Option<i16>,
    image: Option<Option<Uuid>>,
    created_at: Option<chrono::NaiveDateTime>,
}

/// The minimal struct for creating a new timeline into the database.
#[derive(Default)]
pub struct RecipeTimelineForCreate {
    pub title: String,
    pub comment: Option<String>,
    pub rating: Option<i16>,
    pub image: Option<Uuid>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::recipe_timelines)]
struct TimelineForInsert {
    recipe_id: i64,
    user_id: i64,
    title: String,
    comment: Option<String>,
    rating: Option<i16>,
    image: Option<Uuid>,
    created_at: Option<chrono::NaiveDateTime>,
}

impl RecipeTimeline {
    /// Retrieves all timelines associated with the user's recipe.
    pub async fn all(
        mm: &ModelManager,
        recipe_id: i64,
        user_id: i64,
    ) -> Result<Vec<RecipeTimeline>> {
        use schema::recipe_timelines;

        let mut conn = mm.pool.get().await?;

        let timelines = recipe_timelines::table
            .filter(recipe_timelines::recipe_id.eq(recipe_id))
            .filter(recipe_timelines::user_id.eq(user_id))
            .order(recipe_timelines::created_at.asc())
            .load::<RecipeTimeline>(&mut conn)
            .await?;

        Ok(timelines)
    }

    /// Creates a new timeline in the database for the user's recipe.
    pub async fn create(
        mm: &ModelManager,
        recipe_id: i64,
        user_id: i64,
        timeline_c: &RecipeTimelineForCreate,
    ) -> Result<i64> {
        let mut conn = mm.pool.get().await?;

        let timeline_id = diesel::insert_into(schema::recipe_timelines::table)
            .values(&TimelineForInsert {
                recipe_id,
                user_id,
                title: timeline_c.title.clone(),
                comment: timeline_c.comment.clone(),
                rating: timeline_c.rating,
                image: timeline_c.image,
                created_at: timeline_c.created_at,
            })
            .execute(&mut conn)
            .await?;

        Ok(timeline_id as i64)
    }

    /// Updates the fields of an existing timeline.
    pub async fn edit(
        mm: &ModelManager,
        user_id: i64,
        new_timeline: &RecipeTimeline,
    ) -> Result<RecipeTimeline> {
        let mut conn = mm.pool.get().await?;

        let patch = RecipeTimelinePatch {
            title: Some(&new_timeline.title),
            comment: new_timeline.comment.as_deref(),
            rating: new_timeline.rating,
            image: Some(new_timeline.image),
            created_at: (new_timeline.created_at != chrono::NaiveDateTime::default())
                .then_some(new_timeline.created_at),
        };

        let result = diesel::update(
            schema::recipe_timelines::table
                .filter(schema::recipe_timelines::id.eq(new_timeline.id))
                .filter(schema::recipe_timelines::user_id.eq(user_id)),
        )
        .set(&patch)
        .get_result::<RecipeTimeline>(&mut conn)
        .await?;

        Ok(result)
    }

    /// Gets a single timeline event.
    pub async fn get(
        mm: &ModelManager,
        timeline_id: i64,
        recipe_id: i64,
        user_id: i64,
    ) -> Result<RecipeTimeline> {
        let mut conn = mm.pool.get().await?;

        let timeline = match schema::recipe_timelines::table
            .filter(schema::recipe_timelines::id.eq(timeline_id))
            .filter(schema::recipe_timelines::recipe_id.eq(recipe_id))
            .filter(schema::recipe_timelines::user_id.eq(user_id))
            .first::<RecipeTimeline>(&mut conn)
            .await
        {
            Ok(v) => v,
            Err(diesel::NotFound) => {
                return Err(Error::EntityNotFound {
                    entity: "RecipeTimeline",
                    id: timeline_id,
                });
            }
            Err(e) => return Err(e.into()),
        };

        Ok(timeline)
    }
}

#[cfg(test)]
mod tests {
    use testing::utils::{TestDb, build_server_anonymous, create_app_state, insert_other_user};

    use super::*;
    use crate::recipe::test_utils::a_complete_recipe_for_create;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_all {
        use super::*;

        #[tokio::test]
        async fn test_all_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let _ = build_server_anonymous(config.clone()).await?;
            let user2 = insert_other_user(config.clone(), "slava@ukraini.ua").await?;
            let recipe_id1 = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
            let recipe_id2 = Recipe::create(&state.mm, 2, &a_complete_recipe_for_create()).await?;
            RecipeTimeline::create(&state.mm, 1, 1, &RecipeTimelineForCreate::default()).await?;
            let image = Uuid::new_v4();
            RecipeTimeline::create(
                &state.mm,
                1,
                1,
                &RecipeTimelineForCreate {
                    title: "A title".into(),
                    comment: Some("A comment".into()),
                    rating: Some(5),
                    image: Some(image),
                    created_at: Some(chrono::NaiveDateTime::parse_from_str(
                        "2025-01-22 02:32:28",
                        "%Y-%m-%d %H:%M:%S",
                    )?),
                },
            )
            .await?;
            RecipeTimeline::create(
                &state.mm,
                recipe_id2,
                user2.id,
                &RecipeTimelineForCreate::default(),
            )
            .await?;

            let got = RecipeTimeline::all(&state.mm, recipe_id1, 1).await?;

            pretty_assertions::assert_eq!(got.len(), 2);
            Ok(())
        }
    }

    mod tests_edit {
        use super::*;

        #[tokio::test]
        async fn test_no_updates_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let _ = build_server_anonymous(config.clone()).await?;
            let recipe_id = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
            let timeline_c = RecipeTimelineForCreate {
                title: "A title".into(),
                comment: Some("hello".into()),
                rating: Some(4),
                image: None,
                created_at: Some(chrono::NaiveDateTime::parse_from_str(
                    "2025-01-22 02:32:28",
                    "%Y-%m-%d %H:%M:%S",
                )?),
            };
            let timeline_id = RecipeTimeline::create(&state.mm, recipe_id, 1, &timeline_c).await?;

            RecipeTimeline::edit(
                &state.mm,
                timeline_id,
                &RecipeTimeline {
                    id: timeline_id,
                    recipe_id,
                    user_id: 1,
                    title: timeline_c.title.clone(),
                    comment: timeline_c.comment.clone(),
                    rating: timeline_c.rating,
                    image: timeline_c.image,
                    created_at: timeline_c.created_at.unwrap(),
                },
            )
            .await?;

            let got = RecipeTimeline::all(&state.mm, recipe_id, 1).await?;
            pretty_assertions::assert_eq!(
                got,
                vec![RecipeTimeline {
                    id: 1,
                    recipe_id,
                    user_id: 1,
                    title: timeline_c.title.clone(),
                    comment: timeline_c.comment.clone(),
                    rating: timeline_c.rating,
                    image: None,
                    created_at: timeline_c.created_at.unwrap(),
                }],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_update_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let _ = build_server_anonymous(config.clone()).await?;
            let recipe_id = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
            let timeline_c = RecipeTimelineForCreate {
                title: "A title".into(),
                comment: Some("hello".into()),
                rating: Some(4),
                image: None,
                created_at: Some(chrono::NaiveDateTime::parse_from_str(
                    "2025-01-22 02:32:28",
                    "%Y-%m-%d %H:%M:%S",
                )?),
            };
            let image = Uuid::new_v4();
            let timeline_id = RecipeTimeline::create(&state.mm, recipe_id, 1, &timeline_c).await?;
            let new_timeline = RecipeTimeline {
                id: timeline_id,
                recipe_id,
                user_id: 1,
                title: "A title 2".into(),
                comment: Some("bye".into()),
                rating: Some(1),
                image: Some(image),
                created_at: chrono::NaiveDateTime::parse_from_str(
                    "2025-01-30 02:32:28",
                    "%Y-%m-%d %H:%M:%S",
                )?,
            };

            RecipeTimeline::edit(&state.mm, timeline_id, &new_timeline).await?;

            let got = RecipeTimeline::all(&state.mm, recipe_id, 1).await?;
            pretty_assertions::assert_eq!(
                got,
                vec![RecipeTimeline {
                    id: 1,
                    recipe_id,
                    user_id: 1,
                    title: new_timeline.title.clone(),
                    comment: new_timeline.comment.clone(),
                    rating: new_timeline.rating,
                    image: new_timeline.image,
                    created_at: new_timeline.created_at,
                }],
            );
            Ok(())
        }
    }
}
