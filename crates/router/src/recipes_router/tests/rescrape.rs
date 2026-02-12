#[cfg(test)]
mod tests {
    use axum::http::Method;

    use axum_test::{TestServer, TestWebSocket};
    use models::{
        Recipe,
        recipe::structs::{
            recipe::RecipeForCreate, test_utils::a_complete_recipe_for_create, types::Source,
        },
        user::User,
    };
    use recipya_scraper::tests::support::scraper::scrape_test_websites;
    use reqwest::StatusCode;
    use testing::utils::{
        TestDb, assert_html, assert_must_be_logged_in, assert_ws_message, build_server_ws,
        create_app_state,
    };

    use crate::recipes_router::params::RecipeScrapeForm;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn base_uri(recipe_id: i64) -> String {
        format!("/recipes/{recipe_id}/rescrape")
    }

    #[tokio::test]
    async fn test_must_be_logged_in() -> Result<()> {
        assert_must_be_logged_in(Method::GET, &base_uri(1)).await
    }

    #[tokio::test]
    async fn test_recipe_not_found_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = build_server_ws(config).await?;

        let res = server.get(&base_uri(99)).await;

        res.assert_status_not_found();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Recipe not found.","status":"alert-error","title":"Operation Failed"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_recipe_url_not_valid_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = build_server_ws(config.clone()).await?;
        let state = create_app_state(config).await;
        let user_id = User::all(&state.mm).await?[0].id;
        let mut recipe = a_complete_recipe_for_create();
        recipe.name = "Not a valid URL".into();
        recipe.source = Source::new("a magazine");
        let _ = Recipe::create(&state.mm, user_id, &recipe).await?;

        let res = server.get(&base_uri(1)).await;

        res.assert_status_internal_server_error();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Error scraping recipe or recipe source is not a URL.","status":"alert-error","title":"Operation Failed"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_scraped_recipe_same_as_old_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = build_server_ws(config.clone()).await?;
        insert_zweigles_recipe(&server, &mut ws_server).await?;

        let res = server.get(&base_uri(1)).await;

        res.assert_status_ok();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Recipe has not changed.","status":"alert-warning","title":"Attention"}}"# ).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_scraped_recipe_different_than_old_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = build_server_ws(config.clone()).await?;
        insert_zweigles_recipe(&server, &mut ws_server).await?;
        let state = create_app_state(config).await;
        let user_id = User::all(&state.mm).await?[0].id;
        let mut recipe = Recipe::get(&state.mm, user_id, 1).await?;
        recipe.keywords = vec!["potatoes".into(), "fish".into()];
        Recipe::update(&state.mm, user_id, 1, &mut RecipeForCreate::from(recipe)).await?;

        let res = server
            .get(&base_uri(1))
            .add_header(axum_htmx::HX_REQUEST, "true")
            .await;

        res.assert_status_ok();
        res.assert_header(axum_htmx::HX_PUSH_URL, "/recipes/1/rescrape");
        res.assert_header(axum_htmx::HX_RESWAP, "innerHTML transition:true");
        assert_html(&res, vec!["todo!()"]);
        Ok(())
    }

    async fn insert_zweigles_recipe(
        server: &TestServer,
        ws_server: &mut TestWebSocket,
    ) -> Result<()> {
        let form = RecipeScrapeForm {
            urls: "https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes".into(),
        };
        scrape_test_websites(1).await?;

        let res = server.post("/recipes/add/website").form(&form).await;

        res.assert_status(StatusCode::ACCEPTED);
        assert_ws_message(ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Fetching recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">0 of 1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"# ).await;
        assert_ws_message(ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default hidden"><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1"></p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">-1 of -1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"# ).await;
        assert_ws_message(ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /recipes/1","message":"Recipe has been added to your collection.","status":"alert-info","title":"Operation Successful"}}"# ).await;
        Ok(())
    }
}
