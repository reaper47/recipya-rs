#[cfg(test)]
mod tests {
    use axum::http::Method;
    use axum_test::TestServer;
    use axum_test::TestWebSocket;

    use config::Config;
    use models::Recipe;
    use models::recipe::structs::recipe::RecipeForCreate;
    use models::recipe::structs::section::Item;
    use models::recipe::structs::section::SectionComponents;
    use models::recipe::structs::section::SectionItem;
    use models::recipe::structs::test_utils::a_complete_recipe_for_create;
    use testing::utils::{
        TestDb, assert_html, assert_must_be_logged_in, assert_ws_message, build_server_ws,
        create_app_state,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    fn base_uri(recipe_id: i64) -> String {
        format!("/recipes/{recipe_id}/scale")
    }

    async fn setup(config: Config) -> Result<(TestServer, TestWebSocket)> {
        let (server, ws_server) = build_server_ws(config.clone()).await?;
        let state = create_app_state(config).await;
        let _ = Recipe::create(&state.mm, 1, &a_complete_recipe_for_create()).await?;
        Ok((server, ws_server))
    }

    #[tokio::test]
    async fn test_must_be_logged_in() -> Result<()> {
        assert_must_be_logged_in(Method::GET, &base_uri(1)).await
    }

    #[tokio::test]
    async fn test_yield_query_param_must_be_specified() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, _ws_server) = setup(config).await?;

        let res = server.get(&base_uri(1)).await;

        res.assert_status_bad_request();
        res.assert_text("Failed to deserialize query string: missing field `yield`");
        Ok(())
    }

    #[tokio::test]
    async fn test_query_param_must_not_be_negative() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, _ws_server) = setup(config).await?;

        let res = server.get(&format!("{}?yield=-1", base_uri(1))).await;

        res.assert_status_bad_request();
        res.assert_text("Failed to deserialize query string: yield: invalid digit found in string");
        Ok(())
    }

    #[tokio::test]
    async fn test_query_param_must_be_greater_than_0() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = setup(config).await?;

        let res = server.get(&format!("{}?yield=0", base_uri(1))).await;

        res.assert_status_bad_request();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Yield must be greater than zero.","status":"alert-error","title":"Operation Failed"}}"#).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_cannot_find_recipe_in_database() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = setup(config).await?;

        let res = server.get(&format!("{}?yield=8", base_uri(999))).await;

        res.assert_status_not_found();
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Recipe not found.","status":"alert-error","title":"Operation Failed"}}"#).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_valid_double_yield() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, _ws_server) = setup(config.clone()).await?;
        let state = create_app_state(config).await;
        let recipe_id = Recipe::create(
            &state.mm,
            1,
            &RecipeForCreate {
                name: "Best Chinese Kale".into(),
                r#yield: Some(4),
                ingredients: SectionComponents::Grouped(vec![
                    SectionItem::new(
                        "Sauce",
                        vec![
                            Item::new("1 cup blue spinach"),
                            Item::new("1/2 tbsp cinnamon"),
                            Item::new("2lb chicken"),
                            Item::new("1/2 cup bread loaf"),
                            Item::new("½ tbsp beef broth"),
                            Item::new("7 1/2 cups flour"),
                            Item::new("2 big apples"),
                            Item::new("Lots of big apples"),
                            Item::new("2.5 slices of bacon"),
                            Item::new("2 1/3 cans of bamboo sticks"),
                            Item::new("1½can of tomato paste"),
                            Item::new("6 ¾ peanut butter jars"),
                            Item::new("7.5mL of whiskey"),
                            Item::new("2 tsp lemon juice"),
                        ],
                    ),
                    SectionItem::new(
                        "Sauce",
                        vec![
                            Item::new("Ground ginger"),
                            Item::new("3 Large or 4 medium ripe Hass avocados"),
                            Item::new("1/4-1/2 teaspoon salt plus more for seasoning"),
                            Item::new("1/2 fresh pineapple, cored and cut into 1 1/2-inch pieces"),
                            Item::new("Un sac de chips de 1kg"),
                            Item::new("Two 15-ounce can Goya beans"),
                            Item::new("4 pounds top quality chicken filet"),
                            Item::new("1/8 cup lemon juice"),
                        ],
                    ),
                ]),
                instructions: SectionComponents::Grouped(vec![SectionItem::new(
                    "Sauce",
                    vec![Item::new("Mix all these ingredients")],
                )]),
                ..Default::default()
            },
        )
        .await?;

        let res = server
            .get(&format!("{}?yield=8", base_uri(recipe_id)))
            .await;

        res.assert_status_ok();
        assert_html(
            res.clone(),
            vec![
                r#"<span class="pl-2">2 cup blue spinach</span>"#,
                r#"<span class="pl-2">1 tbsp cinnamon</span>"#,
                r#"<span class="pl-2">4 lb chicken</span>"#,
                r#"<span class="pl-2">1 cup bread loaf</span>"#,
                r#"<span class="pl-2">1 tbsp beef broth</span>"#,
                r#"<span class="pl-2">15 cup flour</span>"#,
                r#"<span class="pl-2">4 big apples</span>"#,
                r#"<span class="pl-2">Lots of big apples</span>"#,
                r#"<span class="pl-2">5 slices of bacon</span>"#,
                r#"<span class="pl-2">4 2/3 cans of bamboo sticks</span>"#,
                r#"<span class="pl-2">3can of tomato paste</span>"#,
                r#"<span class="pl-2">13 1/2 peanut butter jars</span>"#,
                r#"<span class="pl-2">1 1/2 cl of whiskey</span>"#,
                r#"<span class="pl-2">1 1/3 tbsp lemon juice</span>"#,
                r#"<span class="pl-2">Ground ginger</span>"#,
                r#"<span class="pl-2">1 fresh pineapple, cored and cut into 1 1/2-inch pieces</span>"#,
                r#"<span class="pl-2">Un sac de chips de 2 kg</span>"#,
                r#"<span class="pl-2">Two 30-ounce can Goya beans</span>"#,
                r#"<span class="pl-2">8 lb top quality chicken filet</span>"#,
                r#"<span class="pl-2">1 1/3 tbsp lemon juice</span>"#,
            ],
        );
        Ok(())
    }
}
