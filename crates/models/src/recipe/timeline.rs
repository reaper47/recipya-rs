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

#[cfg(test)]
mod tests {
    use time::{PrimitiveDateTime, macros::format_description};

    use test_db::default_config;
    use test_utils::{build_server_anonymous, insert_other_user};

    use super::*;
    use crate::recipe::structs::test_utils::a_complete_recipe_for_create;
    use crate::settings::UserSettingDetails;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_all {
        use super::*;

        #[tokio::test]
        async fn test_timeline_all_ok() -> Result<()> {
            let (_, state) = build_server_anonymous(default_config()).await?;
            let user1_id = User::all(&state.mm).await?[0].id;
            let user2_id = insert_other_user(&state, "slava@ukraini.ua").await?.id;
            let settings1 = UserSettingDetails::get(&state.mm, user1_id).await?;
            let settings2 = UserSettingDetails::get(&state.mm, user2_id).await?;
            let recipe_id1 = Recipe::create(
                &state.mm,
                user1_id,
                &a_complete_recipe_for_create().0,
                &settings1,
            )
            .await?;
            let recipe_id2 = Recipe::create(
                &state.mm,
                user2_id,
                &a_complete_recipe_for_create().0,
                &settings2,
            )
            .await?;
            RecipeTimeline::create(
                &state.mm,
                recipe_id1,
                user1_id,
                &RecipeTimelineForCreate::default(),
            )
            .await?;
            let image = Uuid::new_v4();
            RecipeTimeline::create(
                &state.mm,
                recipe_id1,
                user1_id,
                &RecipeTimelineForCreate {
                    title: "A title".into(),
                    comment: Some("A comment".into()),
                    rating: Some(5),
                    image: Some(image),
                    created_at: Some(PrimitiveDateTime::parse(
                        "2025-01-22 02:32:28",
                        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
                    )?),
                },
            )
            .await?;
            RecipeTimeline::create(
                &state.mm,
                recipe_id2,
                user2_id,
                &RecipeTimelineForCreate::default(),
            )
            .await?;

            let got = RecipeTimeline::all(&state.mm, recipe_id1, user1_id).await?;

            pretty_assertions::assert_eq!(got.len(), 2);
            Ok(())
        }
    }

    mod tests_edit {
        use super::*;

        #[tokio::test]
        async fn test_no_updates_ok() -> Result<()> {
            let (_, state) = build_server_anonymous(default_config()).await?;
            let user = insert_other_user(&state, "no_update_test_user@example.com").await?;
            let settings = UserSettingDetails::get(&state.mm, user.id).await?;
            let (recipe, _) = a_complete_recipe_for_create();
            let recipe_id = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;
            let timeline_c = RecipeTimelineForCreate {
                title: "A title".into(),
                comment: Some("hello".into()),
                rating: Some(4),
                image: None,
                created_at: Some(PrimitiveDateTime::parse(
                    "2025-01-22 02:32:28",
                    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
                )?),
            };
            let timeline_id =
                RecipeTimeline::create(&state.mm, recipe_id, user.id, &timeline_c).await?;

            dbg!("OH");
            RecipeTimeline::edit(
                &state.mm,
                user.id,
                &RecipeTimeline {
                    id: timeline_id,
                    recipe_id,
                    user_id: user.id,
                    title: timeline_c.title.clone(),
                    comment: timeline_c.comment.clone(),
                    rating: timeline_c.rating,
                    image: timeline_c.image,
                    created_at: timeline_c.created_at.unwrap(),
                },
            )
            .await?;

            let got = RecipeTimeline::all(&state.mm, recipe_id, user.id).await?;

            pretty_assertions::assert_eq!(
                got,
                vec![RecipeTimeline {
                    id: timeline_id,
                    recipe_id,
                    user_id: user.id,
                    title: timeline_c.title,
                    comment: timeline_c.comment,
                    rating: timeline_c.rating,
                    image: None,
                    created_at: timeline_c.created_at.unwrap(),
                }],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_update_ok() -> Result<()> {
            let (_, state) = build_server_anonymous(default_config()).await?;
            let user = insert_other_user(&state, "update_test_user@example.com").await?;
            let settings = UserSettingDetails::get(&state.mm, user.id).await?;
            let (recipe, _) = a_complete_recipe_for_create();
            let recipe_id = Recipe::create(&state.mm, user.id, &recipe, &settings).await?;
            let timeline_c = RecipeTimelineForCreate {
                title: "A title".into(),
                comment: Some("hello".into()),
                rating: Some(4),
                image: None,
                created_at: Some(PrimitiveDateTime::parse(
                    "2025-01-22 02:32:28",
                    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
                )?),
            };
            let image = Uuid::new_v4();
            let timeline_id =
                RecipeTimeline::create(&state.mm, recipe_id, user.id, &timeline_c).await?;
            let new_timeline = RecipeTimeline {
                id: timeline_id,
                recipe_id,
                user_id: user.id,
                title: "A title 2".into(),
                comment: Some("bye".into()),
                rating: Some(1),
                image: Some(image),
                created_at: PrimitiveDateTime::parse(
                    "2025-01-30 02:32:28",
                    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
                )?,
            };

            RecipeTimeline::edit(&state.mm, user.id, &new_timeline).await?;
            let got = RecipeTimeline::all(&state.mm, recipe_id, user.id).await?;

            pretty_assertions::assert_eq!(
                got,
                vec![RecipeTimeline {
                    id: timeline_id,
                    recipe_id,
                    user_id: user.id,
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
