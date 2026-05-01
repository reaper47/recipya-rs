#[cfg(test)]
mod tests {
    use axum::http::Method;
    use diesel::internal::derives::multiconnection::chrono;
    use diesel::prelude::*;
    use diesel_async::RunQueryDsl;

    use app::state::AppState;
    use models::Recipe;
    use models::share::ShareRecipe;
    use models::user::User;
    use repository::schema;
    use test_db::TestDb;
    use test_fixtures::assert_html;
    use test_models::a_complete_recipe_for_create;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in, create_app_state};
    use uuid::Uuid;

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
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config.clone()).await?;
        let state = create_app_state(config).await;
        let user_id = User::all(&state.mm).await?[0].id;
        let (recipe, _) = a_complete_recipe_for_create();
        let _ = Recipe::create(&state.mm, user_id, &recipe).await?;

        let res = server
            .post(&base_uri(1))
            .form(&ShareRecipeForm::new(None))
            .await;

        let share = get_first_shared_recipe(state, user_id).await;
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
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config.clone()).await?;
        let state = create_app_state(config).await;
        let user_id = User::all(&state.mm).await?[0].id;
        let (recipe, _) = a_complete_recipe_for_create();
        let _ = Recipe::create(&state.mm, user_id, &recipe).await?;
        let expires_at = (chrono::Utc::now() + chrono::Duration::days(31)).naive_utc();

        let res = server
            .post(&base_uri(1))
            .form(&ShareRecipeForm::new(Some(expires_at.to_string())))
            .await;

        let share = get_first_shared_recipe(state, user_id).await;
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
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config.clone()).await?;
        let state = create_app_state(config).await;
        let user_id = User::all(&state.mm).await?[0].id;
        let (recipe, _) = a_complete_recipe_for_create();
        let _ = Recipe::create(&state.mm, user_id, &recipe).await?;
        let now = chrono::Utc::now().naive_utc();

        let res = server
            .post(&base_uri(1))
            .form(&ShareRecipeForm::new(Some("hello".into())))
            .await;

        let share = get_first_shared_recipe(state, user_id).await;
        res.assert_status_ok();
        pretty_assertions::assert_eq!(share.expires_at.signed_duration_since(now).num_days(), 7);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_share_twice_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config.clone()).await?;
        let state = create_app_state(config).await;
        let user_id = User::all(&state.mm).await?[0].id;
        let (recipe, _) = a_complete_recipe_for_create();
        let _ = Recipe::create(&state.mm, user_id, &recipe).await?;
        let expires_at = (chrono::Utc::now() + chrono::Duration::days(31)).naive_utc();
        let _ = server
            .post(&base_uri(1))
            .form(&ShareRecipeForm::new(Some(expires_at.to_string())))
            .await;

        let res = server
            .post(&base_uri(1))
            .form(&ShareRecipeForm::new(Some(expires_at.to_string())))
            .await;

        res.assert_status_ok();
        let share = get_first_shared_recipe(state, user_id).await;
        assert_eq!(share.recipe_id, 1);
        Ok(())
    }

    async fn get_first_shared_recipe(state: AppState, user_id: Uuid) -> ShareRecipe {
        let mut conn = state
            .mm
            .pool
            .get()
            .await
            .expect("a connection from the pool");

        schema::shares_recipes::table
            .filter(
                schema::shares_recipes::recipe_id
                    .eq(1)
                    .and(schema::shares_recipes::user_id.eq(user_id)),
            )
            .first::<ShareRecipe>(&mut conn)
            .await
            .expect("a share recipe must have been fetched")
    }
}
