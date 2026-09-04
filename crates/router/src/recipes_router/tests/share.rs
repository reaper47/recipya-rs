#[cfg(test)]
mod tests {
    use axum::http::Method;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;
    use time::{Duration, OffsetDateTime, PrimitiveDateTime};
    use uuid::Uuid;

    use app::state::AppState;
    use models::share::ShareRecipe;
    use models::user::User;
    use models::{Recipe, settings::UserSettingDetails};
    use repository::schema;
    use test_db::default_config;
    use test_fixtures::assert_html;
    use test_models::a_complete_recipe_for_create;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in};

    use crate::recipes_router::params::ShareRecipeForm;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn base_uri(recipe_id: i64) -> String {
        format!("/recipes/{recipe_id}/share")
    }

    impl ShareRecipeForm {
        pub fn new(datetime: Option<String>) -> Self {
            Self { datetime }
        }
    }

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::POST, &base_uri(1)).await
    }

    #[tokio::test]
    async fn test_default_expires_at_time_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;

        let res = server
            .post(&base_uri(recipe_id))
            .form(&ShareRecipeForm::new(None))
            .await;

        let share = get_first_shared_recipe(state, user_id, recipe_id).await;
        res.assert_status_ok();
        assert_html(
            &res,
            &[
                &format!(
                    r#"<label><input class="input" type="url" value="http://localhost:8078/shared/r/{}" readonly="readonly"></label>"#,
                    share.link
                ),
                &format!(
                    r#"<button class="btn btn-neutral" id="copy-button" title="Copy to clipboard" onClick="copyToClipboard('http://localhost:8078/shared/r/{}')">Copy</button>"#,
                    share.link
                ),
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_custom_expires_at_time_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        let expires_at = {
            let dt = OffsetDateTime::now_utc() + Duration::days(31);
            PrimitiveDateTime::new(dt.date(), dt.time())
        };

        let res = server
            .post(&base_uri(recipe_id))
            .form(&ShareRecipeForm::new(Some(expires_at.to_string())))
            .await;

        let share = get_first_shared_recipe(state, user_id, recipe_id).await;
        res.assert_status_ok();
        assert_html(
            &res,
            &[
                &format!(
                    r#"<label><input class="input" type="url" value="http://localhost:8078/shared/r/{}" readonly="readonly"></label>"#,
                    share.link
                ),
                &format!(
                    r#"<button class="btn btn-neutral" id="copy-button" title="Copy to clipboard" onClick="copyToClipboard('http://localhost:8078/shared/r/{}')">Copy</button>"#,
                    share.link
                ),
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_expires_at_time_defaults_to_7_days_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        let now = OffsetDateTime::now_utc();

        let res = server
            .post(&base_uri(recipe_id))
            .form(&ShareRecipeForm::new(Some("hello".into())))
            .await;

        let share = get_first_shared_recipe(state, user_id, recipe_id).await;
        res.assert_status_ok();
        pretty_assertions::assert_eq!(
            ((share.expires_at.assume_utc() - now).whole_hours() + 1) / 24,
            7
        );
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_share_twice_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;
        let user_id = User::all(&state.mm).await?[0].id;
        let settings = UserSettingDetails::get(&state.mm, user_id).await?;
        let (recipe, _) = a_complete_recipe_for_create();
        let recipe_id = Recipe::create(&state.mm, user_id, &recipe, &settings).await?;
        let expires_at = {
            let dt = OffsetDateTime::now_utc() + Duration::days(31);
            PrimitiveDateTime::new(dt.date(), dt.time())
        };
        let _ = server
            .post(&base_uri(recipe_id))
            .form(&ShareRecipeForm::new(Some(expires_at.to_string())))
            .await;

        let res = server
            .post(&base_uri(recipe_id))
            .form(&ShareRecipeForm::new(Some(expires_at.to_string())))
            .await;

        res.assert_status_ok();
        let share = get_first_shared_recipe(state, user_id, recipe_id).await;
        assert_eq!(share.recipe_id, recipe_id);
        Ok(())
    }

    async fn get_first_shared_recipe(
        state: AppState,
        user_id: Uuid,
        recipe_id: i64,
    ) -> ShareRecipe {
        let mut conn = state
            .mm
            .pool
            .get()
            .await
            .expect("a connection from the pool");

        schema::shares_recipes::table
            .filter(
                schema::shares_recipes::recipe_id
                    .eq(recipe_id)
                    .and(schema::shares_recipes::user_id.eq(user_id)),
            )
            .first::<ShareRecipe>(&mut conn)
            .await
            .expect("a share recipe must have been fetched")
    }
}
