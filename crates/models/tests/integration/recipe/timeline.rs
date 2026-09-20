use time::{PrimitiveDateTime, macros::format_description};
use uuid::Uuid;

use models::{
    Recipe,
    recipe::timeline::{RecipeTimeline, RecipeTimelineForCreate},
    settings::UserSettingDetails,
    user::User,
};
use test_db::default_config;
use test_fixtures::insert_other_user;
use test_harness::build_server_anonymous;

use crate::recipe::utils::a_complete_recipe_for_create;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod tests_all {
    use super::*;

    #[tokio::test]
    async fn test_timeline_all_ok() -> Result<()> {
        let (_, state) = build_server_anonymous(default_config()).await?;
        let user1_id = User::all(&state.mm).await?[0].id;
        let user2_id = insert_other_user(&state.mm, "slava@ukraini.ua").await?.id;
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
        let user = insert_other_user(&state.mm, "no_update_test_user@example.com").await?;
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
        let user = insert_other_user(&state.mm, "update_test_user@example.com").await?;
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
