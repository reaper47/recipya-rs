use diesel::prelude::*;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::Recipe;
use crate::Result;
use crate::user::User;

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
    pub comment: Option<String>,
    pub rating: Option<i16>,
    pub image: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
}

/// The minimal struct for creating a new timeline into the database.
pub struct RecipeTimelineForCreate {
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
        timeline_id: i64,
        user_id: i64,
        timeline_c: &RecipeTimelineForCreate,
    ) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        let timeline = schema::recipe_timelines::table
            .filter(schema::recipe_timelines::id.eq(timeline_id))
            .filter(schema::recipe_timelines::user_id.eq(user_id))
            .first::<RecipeTimeline>(&mut conn)
            .await?;

        diesel::update(schema::recipe_timelines::table)
            .set(&RecipeTimeline {
                id: timeline.id,
                recipe_id: timeline.recipe_id,
                user_id,
                comment: timeline_c.comment.clone(),
                rating: timeline_c.rating,
                image: timeline_c.image,
                created_at: timeline_c.created_at.unwrap_or(timeline.created_at),
            })
            .execute(&mut conn)
            .await?;

        Ok(())
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
            RecipeTimeline::create(
                &state.mm,
                1,
                1,
                &RecipeTimelineForCreate {
                    comment: None,
                    rating: None,
                    image: None,
                    created_at: None,
                },
            )
            .await?;
            let image = Uuid::new_v4();
            RecipeTimeline::create(
                &state.mm,
                1,
                1,
                &RecipeTimelineForCreate {
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
                &RecipeTimelineForCreate {
                    comment: None,
                    rating: None,
                    image: None,
                    created_at: None,
                },
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
                comment: Some("hello".into()),
                rating: Some(4),
                image: None,
                created_at: Some(chrono::NaiveDateTime::parse_from_str(
                    "2025-01-22 02:32:28",
                    "%Y-%m-%d %H:%M:%S",
                )?),
            };
            let timeline_id = RecipeTimeline::create(&state.mm, recipe_id, 1, &timeline_c).await?;

            RecipeTimeline::edit(&state.mm, timeline_id, 1, &timeline_c).await?;

            let got = RecipeTimeline::all(&state.mm, recipe_id, 1).await?;
            pretty_assertions::assert_eq!(
                got,
                vec![RecipeTimeline {
                    id: 1,
                    recipe_id,
                    user_id: 1,
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
            let original_timeline_c = RecipeTimelineForCreate {
                comment: Some("hello".into()),
                rating: Some(4),
                image: None,
                created_at: Some(chrono::NaiveDateTime::parse_from_str(
                    "2025-01-22 02:32:28",
                    "%Y-%m-%d %H:%M:%S",
                )?),
            };
            let image = Uuid::new_v4();
            let new_timeline_c = RecipeTimelineForCreate {
                comment: Some("bye".into()),
                rating: Some(1),
                image: Some(image),
                created_at: Some(chrono::NaiveDateTime::parse_from_str(
                    "2025-01-30 02:32:28",
                    "%Y-%m-%d %H:%M:%S",
                )?),
            };
            let timeline_id =
                RecipeTimeline::create(&state.mm, recipe_id, 1, &original_timeline_c).await?;

            RecipeTimeline::edit(&state.mm, timeline_id, 1, &new_timeline_c).await?;

            let got = RecipeTimeline::all(&state.mm, recipe_id, 1).await?;
            pretty_assertions::assert_eq!(
                got,
                vec![RecipeTimeline {
                    id: 1,
                    recipe_id,
                    user_id: 1,
                    comment: new_timeline_c.comment.clone(),
                    rating: new_timeline_c.rating,
                    image: new_timeline_c.image,
                    created_at: new_timeline_c.created_at.unwrap(),
                }],
            );
            Ok(())
        }
    }
}
