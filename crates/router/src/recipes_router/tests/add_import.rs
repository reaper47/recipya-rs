#[cfg(test)]
mod tests {
    use axum::http::Method;
    use axum_test::http::StatusCode;
    use axum_test::multipart::{MultipartForm, Part};

    use models::Recipe;
    use serde_json::json;
    use testing::utils::{
        HIDDEN_WS_NOTIFICATION, TestDb, assert_html, assert_must_be_logged_in, assert_ws_message,
        build_server_logged_in, build_server_ws, create_app_state, open_test_file,
    };

    use crate::recipes_router::params::PreviewForm;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    const BASE_URI: &str = "/recipes/add/import";

    fn valid_preview_form() -> PreviewForm {
        PreviewForm {
            json_input: json!({
  "@context": "https://schema.org",
  "@type": "Recipe",
  "cookTime": "PT1H",
  "description": "This classic banana bread recipe comes from my mom -- the walnuts add a nice texture and flavor to the banana bread.",
  "image": "bananabread.jpg",
  "recipeIngredient": [
    "3 or 4 ripe bananas, smashed",
    "1 egg",
    "3/4 cup of sugar"
  ],
  "name": "Mom's World Famous Banana Bread",
  "prepTime": "PT15M",
  "recipeInstructions": "Preheat the oven to 350 degrees. Mix in the ingredients in a bowl. Add the flour last. Pour the mixture into a loaf pan and bake for one hour.",
  "recipeYield": "1 loaf",
}).to_string(),
        }
    }

    #[tokio::test]
    async fn test_must_be_logged_in() -> Result<()> {
        assert_must_be_logged_in(Method::POST, BASE_URI).await
    }

    #[tokio::test]
    async fn test_post_payload_too_large() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let server = build_server_logged_in(config).await?;
        let large_payload = "x".repeat(51 * 1024 * 1024);

        let res = server
            .post(BASE_URI)
            .multipart(
                MultipartForm::new()
                    .add_part("app", Part::text("mealmaster"))
                    .add_part(
                        "file",
                        Part::bytes(large_payload.into_bytes())
                            .file_name("cookmate1.mcb")
                            .mime_type("application/octet-stream"),
                    ),
            )
            .await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_post_error_parsing_files() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = build_server_ws(config.clone()).await?;
        let file = open_test_file("integrations/kalorio1.txt");

        let res = server
            .post(BASE_URI)
            .multipart(
                MultipartForm::new()
                    .add_part("app", Part::text("mealmaster"))
                    .add_part(
                        "file",
                        Part::bytes(file.into_inner())
                            .file_name("kalorio1.txt")
                            .mime_type("application/octet-stream"),
                    ),
            )
            .await;

        res.assert_status(StatusCode::ACCEPTED);
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Parsing recipes...</p><div id="export-progress"><progress max="100" value="1.00"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"An error occurred while parsing the recipes. Please check the logs.","status":"alert-error","title":"Operation Failed"}}"#).await;
        let state = create_app_state(config).await;
        pretty_assertions::assert_eq!(Recipe::count(&state.mm, 1).await?, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_post_valid_request() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let (server, mut ws_server) = build_server_ws(config.clone()).await?;
        let file = open_test_file("integrations/kalorio1.txt");

        let res = server
            .post(BASE_URI)
            .multipart(
                MultipartForm::new()
                    .add_part("app", Part::text("kalorio"))
                    .add_part(
                        "file",
                        Part::bytes(file.into_inner())
                            .file_name("kalorio1.txt")
                            .mime_type("application/octet-stream"),
                    ),
            )
            .await;

        res.assert_status(StatusCode::ACCEPTED);
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Parsing recipes...</p><div id="export-progress"><progress max="100" value="1.00"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Saving recipes</p><div id="export-progress"><progress max="100" value="33.33"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Saving recipes</p><div id="export-progress"><progress max="100" value="66.67"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Saving recipes</p><div id="export-progress"><progress max="100" value="100.00"></progress></div></div></div>"#).await;
        assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
        assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Imported 3 recipes. Skipped 0.","status":"alert-info","title":"Operation Successful"}}"#).await;
        let state = create_app_state(config).await;
        pretty_assertions::assert_eq!(Recipe::count(&state.mm, 1).await?, 3);
        Ok(())
    }

    mod tests_recipe_add_import_preview {
        use super::*;

        const BASE_URI: &str = "/recipes/add/import/preview";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, BASE_URI).await
        }

        #[tokio::test]
        async fn test_post_payload_invalid_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&PreviewForm {
                    json_input: "{1234567}".to_string(),
                })
                .await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r#"<div class="text-error">Invalid JSON: key must be a string at line 1 column 2</div>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_post_payload_valid_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.post(BASE_URI).form(&valid_preview_form()).await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r#"<h2 class="card-title bg-base-200 px-2 pt-2 place-content-center rounded-t-2xl print:border-b print:border-black" style="justify-content: space-between"><span class="text-center pb-2 print:w-full w-full" itemprop="name">Mom's World Famous Banana Bread</span></h2>"#,
                ],
            );
            Ok(())
        }
    }

    mod tests_recipe_add_import_raw_json {
        use super::*;

        const BASE_URI: &str = "/recipes/add/import/raw-json";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, BASE_URI).await
        }

        #[tokio::test]
        async fn test_post_payload_invalid_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&PreviewForm {
                    json_input: "{1234567}".to_string(),
                })
                .await;

            res.assert_status_bad_request();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Error parsing recipe schema JSON.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_post_payload_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.post(BASE_URI).form(&valid_preview_form()).await;

            res.assert_status_ok();
            res.assert_header(axum_htmx::headers::HX_REDIRECT, "/recipes/1");
            Ok(())
        }

        #[tokio::test]
        async fn test_post_payload_double_insert_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;
            let _ = server.post(BASE_URI).form(&valid_preview_form()).await;

            let res = server.post(BASE_URI).form(&valid_preview_form()).await;

            res.assert_status_conflict();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Recipe exists.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }
    }
}
