#[cfg(test)]
mod tests {
    use models::Recipe;
    use models::recipe::test_utils::a_complete_recipe_for_create;
    use reqwest::Method;
    use testing::utils::{
        TestDb, assert_must_be_logged_in, assert_ws_message, build_server_ws, create_app_state,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn base_uri(recipe_id: i32) -> String {
        format!("/recipes/{recipe_id}/timeline")
    }

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::GET, &base_uri(1)).await?;
        assert_must_be_logged_in(Method::POST, &base_uri(1)).await?;
        assert_must_be_logged_in(Method::PUT, &base_uri(1)).await
    }

    #[tokio::test]
    async fn test_get_recipe_not_exist_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = build_server_ws(config).await?;

        let res = server.get(&base_uri(5)).await;

        res.assert_status_not_found();
        assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"Recipe not found.","status":"alert-error","title":"Operation Failed"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_recipe_exists_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = build_server_ws(config.clone()).await?;
        let state = create_app_state(config).await;
        let _ = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;

        let res = server.get(&base_uri(1)).await;

        res.assert_status_ok();
        assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"Recipe not found.","status":"alert-error","title":"Operation Failed"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_post_add_new_timeline_recipe_not_exist_ok() -> Result<()> {
        todo!()
    }

    #[tokio::test]
    async fn test_post_add_new_timeline_ok() -> Result<()> {
        todo!()
    }

    #[tokio::test]
    async fn test_put_edit_timeline_component_recipe_not_exist_ok() -> Result<()> {
        todo!()
    }

    #[tokio::test]
    async fn test_put_edit_timeline_component_ok() -> Result<()> {
        todo!()
    }
}
