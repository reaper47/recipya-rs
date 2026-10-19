use axum::http::Method;
use axum_test::TestResponse;
use axum_test::http::StatusCode;
use axum_test::multipart::{MultipartForm, Part};
use serde_json::json;

use models::Recipe;
use router::params::PreviewForm;
use test_db::default_config;
use test_fixtures::open_test_file;
use test_harness::{
    assert_html, assert_must_be_logged_in, assert_sse_message, build_server_logged_in,
    build_server_sse,
};

use crate::{FSI, PDI};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

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

fn other_valid_preview_form1() -> PreviewForm {
    PreviewForm {
        json_input: json!({
          "@context": "https://schema.org",
          "@type": "Recipe",
          "name": "Dry Toast",
          "description": "When you can't keep anything down",
          "recipeYield": "1 servings",
          "prepTime": "PT3M",
          "totalTime": "PT3M",
          "recipeCategory": "sandwich",
          "recipeIngredient": [
            {
              "@type": "PropertyValue",
              "value": "1",
              "name": "whole wheat bread",
              "unitCode": "slice"
            }
          ],
          "recipeInstructions": [
            {
              "@type": "HowToStep",
              "text": "Toast for 2 minutes."
            }
          ]
        })
        .to_string(),
    }
}

fn other_valid_preview_form2() -> PreviewForm {
    PreviewForm {
        json_input: json!({
          "@context": "https://schema.org",
          "@type": "Recipe",
          "name": "Dry Toast",
          "description": "When you can't keep anything down",
          "recipeYield": "1 servings",
          "prepTime": "PT3M",
          "totalTime": "PT3M",
          "recipeCategory": "sandwich",
          "recipeIngredient": [
            {
              "@type": "PropertyValue",
              "name": "whole wheat bread",
              "value": {
                "@type": "QuantitativeValue",
                "value": "1",
                "unitText": "slice"
              }
            }
          ],
          "recipeInstructions": [
            {
              "@type": "HowToStep",
              "text": "Toast for 2 minutes."
            }
          ]
        })
        .to_string(),
    }
}

mod tests_import_app {
    use models::{reports::ViewReport, user::User};
    use test_db::default_config;

    use super::*;

    const BASE_URI: &str = "/recipes/add/import/app";

    #[tokio::test]
    async fn test_must_be_logged_in() -> Result<()> {
        assert_must_be_logged_in(Method::POST, BASE_URI).await
    }

    #[tokio::test]
    async fn test_post_error_parsing_files() -> Result<()> {
        let (server, mut ws_server, state) = build_server_sse(default_config()).await?;
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
        assert_sse_message(&mut ws_server, r#"{"notification":{"type":"snack","message":"<div id=\"sse-notification-container\" class=\"z-20 fixed bottom-0 right-0 p-6 cursor-default \"><div class=\"bg-blue-500 text-white px-4 py-2 rounded shadow-md\"><p class=\"font-medium text-center pb-1\">Parsing recipes…</p><div class=\"flex justify-between items-center text-sm mb-2\"><span class=\"font-semibold\">1 of 100</span><span class=\"font-semibold\">1.0%</span></div><div id=\"export-progress\"><progress max=\"100\" value=\"1.00\"></progress></div></div></div>","status":"alert-info","title":"Success"}}"#).await;
        assert_sse_message(&mut ws_server, r#"{"notification":{"type":"snack","message":"<div id=\"sse-notification-container\" class=\"z-20 fixed bottom-0 right-0 p-6 cursor-default hidden\"><div class=\"bg-blue-500 text-white px-4 py-2 rounded shadow-md\"><p class=\"font-medium text-center pb-1\"></p><div class=\"flex justify-between items-center text-sm mb-2\"><span class=\"font-semibold\">-1 of -1</span><span class=\"font-semibold\">0.0%</span></div><div id=\"export-progress\"><progress max=\"100\" value=\"0.00\"></progress></div></div></div>","status":"alert-info","title":"Success"}}"#).await;
        assert_sse_message(&mut ws_server, r#"{"notification":{"type":"toast","message":"An error occurred while parsing the recipes. Please check the logs.","status":"alert-error","title":"Operation failed"}}"#).await;
        let user_id = User::all(&state.mm).await?[0].id;
        pretty_assertions::assert_eq!(Recipe::count(&state.mm, user_id).await?, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_post_valid_request() -> Result<()> {
        let (server, mut ws_server, state) = build_server_sse(default_config()).await?;
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
        assert_sse_message(&mut ws_server, r#"{"notification":{"type":"snack","message":"<div id=\"sse-notification-container\" class=\"z-20 fixed bottom-0 right-0 p-6 cursor-default \"><div class=\"bg-blue-500 text-white px-4 py-2 rounded shadow-md\"><p class=\"font-medium text-center pb-1\">Parsing recipes…</p><div class=\"flex justify-between items-center text-sm mb-2\"><span class=\"font-semibold\">1 of 100</span><span class=\"font-semibold\">1.0%</span></div><div id=\"export-progress\"><progress max=\"100\" value=\"1.00\"></progress></div></div></div>","status":"alert-info","title":"Success"}}"#).await;
        assert_sse_message(&mut ws_server, r#"{"notification":{"type":"snack","message":"<div id=\"sse-notification-container\" class=\"z-20 fixed bottom-0 right-0 p-6 cursor-default \"><div class=\"bg-blue-500 text-white px-4 py-2 rounded shadow-md\"><p class=\"font-medium text-center pb-1\">Saving media</p><div class=\"flex justify-between items-center text-sm mb-2\"><span class=\"font-semibold\">1 of 3</span><span class=\"font-semibold\">33.3%</span></div><div id=\"export-progress\"><progress max=\"100\" value=\"33.33\"></progress></div></div></div>","status":"alert-info","title":"Success"}}"#).await;
        assert_sse_message(&mut ws_server, r#"{"notification":{"type":"snack","message":"<div id=\"sse-notification-container\" class=\"z-20 fixed bottom-0 right-0 p-6 cursor-default \"><div class=\"bg-blue-500 text-white px-4 py-2 rounded shadow-md\"><p class=\"font-medium text-center pb-1\">Saving media</p><div class=\"flex justify-between items-center text-sm mb-2\"><span class=\"font-semibold\">3 of 3</span><span class=\"font-semibold\">100.0%</span></div><div id=\"export-progress\"><progress max=\"100\" value=\"100.00\"></progress></div></div></div>","status":"alert-info","title":"Success"}}"#).await;
        assert_sse_message(&mut ws_server, r#"{"notification":{"type":"snack","message":"<div id=\"sse-notification-container\" class=\"z-20 fixed bottom-0 right-0 p-6 cursor-default \"><div class=\"bg-blue-500 text-white px-4 py-2 rounded shadow-md\"><p class=\"font-medium text-center pb-1\">Saving recipes</p><div class=\"flex justify-between items-center text-sm mb-2\"><span class=\"font-semibold\">3 of 3</span><span class=\"font-semibold\">100.0%</span></div><div id=\"export-progress\"><progress max=\"100\" value=\"100.00\"></progress></div></div></div>","status":"alert-info","title":"Success"}}"#).await;
        assert_sse_message(&mut ws_server, r#"{"notification":{"type":"snack","message":"<div id=\"sse-notification-container\" class=\"z-20 fixed bottom-0 right-0 p-6 cursor-default hidden\"><div class=\"bg-blue-500 text-white px-4 py-2 rounded shadow-md\"><p class=\"font-medium text-center pb-1\"></p><div class=\"flex justify-between items-center text-sm mb-2\"><span class=\"font-semibold\">-1 of -1</span><span class=\"font-semibold\">0.0%</span></div><div id=\"export-progress\"><progress max=\"100\" value=\"0.00\"></progress></div></div></div>","status":"alert-info","title":"Success"}}"#).await;
        assert_sse_message(&mut ws_server, "{\"notification\":{\"type\":\"toast\",\"action\":\"View /reports?view=latest\",\"message\":\"Imported \u{2068}3\u{2069} recipes. Skipped \u{2068}0\u{2069}.\",\"status\":\"alert-info\",\"title\":\"Success\"}}",).await;
        let user_id = User::all(&state.mm).await?[0].id;
        pretty_assertions::assert_eq!(Recipe::count(&state.mm, user_id).await?, 3);
        let reports = ViewReport::fetch_all(&state.mm, 1, user_id).await?;
        assert_eq!(reports.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_post_payload_too_large() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;
        let large_payload = "x".repeat(101 * 1024 * 1024);

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
}

mod tests_recipe_add_import_preview {
    use super::*;

    const BASE_URI: &str = "/recipes/add/import/preview";

    fn assert_other_json_html(res: &TestResponse) {
        assert_html(
            res,
            &[
                r#"<span class="text-center pb-2 print:w-full w-full" itemprop="name">Dry Toast</span>"#,
                &format!(r#"<p class="text-sm text-center">{FSI}4{PDI} servings</p>"#),
                r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time">"#,
                r#"<time datetime="PT0S">0s</time></div>"#,
                r#"<ul class="col-span-6 w-full print:mb-2" style="column-count: 1"><li class="text-sm"><label><input type="checkbox"></label><span class="pl-2">1 slice whole wheat bread</span></li></ul>"#,
            ],
        );
    }

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::POST, BASE_URI).await
    }

    #[tokio::test]
    async fn test_post_payload_invalid_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server
            .post(BASE_URI)
            .form(&PreviewForm {
                json_input: "{1234567}".to_string(),
            })
            .await;

        res.assert_status_ok();
        assert_html(
            &res,
            &[
                r#"<div class="text-error">Invalid JSON: key must be a string at line 1 column 2</div>"#,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_payload_with_properties1_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server
            .post(BASE_URI)
            .form(&other_valid_preview_form1())
            .await;

        res.assert_status_ok();
        assert_other_json_html(&res);
        Ok(())
    }

    #[tokio::test]
    async fn test_post_payload_with_properties2_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server
            .post(BASE_URI)
            .form(&other_valid_preview_form2())
            .await;

        res.assert_status_ok();
        assert_other_json_html(&res);
        Ok(())
    }

    #[tokio::test]
    async fn test_post_payload_valid_ok() -> Result<()> {
        let (server, _) = build_server_logged_in(default_config()).await?;

        let res = server.post(BASE_URI).form(&valid_preview_form()).await;

        res.assert_status_ok();
        assert_html(
            &res,
            &[
                r#"<h2 class="card-title bg-base-200 px-2 pt-2 place-content-center rounded-t-2xl print:border-b print:border-black" style="justify-content: space-between"><span class="text-center pb-2 print:w-full w-full" itemprop="name">Mom's World Famous Banana Bread</span></h2>"#,
            ],
        );
        Ok(())
    }
}

mod tests_recipe_add_import_raw_json {
    use models::{Recipe, reports::ViewReport, user::User};

    use super::*;

    const BASE_URI: &str = "/recipes/add/import/raw-json";

    #[tokio::test]
    async fn test_must_be_logged_in_ok() -> Result<()> {
        assert_must_be_logged_in(Method::POST, BASE_URI).await
    }

    #[tokio::test]
    async fn test_post_payload_invalid_ok() -> Result<()> {
        let (server, mut ws_server, _) = build_server_sse(default_config()).await?;

        let res = server
            .post(BASE_URI)
            .form(&PreviewForm {
                json_input: "{1234567}".to_string(),
            })
            .await;

        res.assert_status_bad_request();
        assert_sse_message(&mut ws_server, r#"{"notification":{"type":"toast","message":"Error parsing recipe schema JSON.","status":"alert-error","title":"Operation failed"}}"#).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_post_payload_ok() -> Result<()> {
        let (server, state) = build_server_logged_in(default_config()).await?;

        let res = server.post(BASE_URI).form(&valid_preview_form()).await;

        res.assert_status_ok();
        let user_id = User::all(&state.mm).await?[0].id;
        let recipe_id = Recipe::all(&state.mm, user_id)
            .await?
            .last()
            .map_or(1, |r| r.id);
        let reports = ViewReport::fetch_all(&state.mm, 1, user_id).await?;
        res.assert_header(axum_htmx::HX_REDIRECT, format!("/recipes/{recipe_id}"));
        assert_eq!(reports.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_post_payload_double_insert_ok() -> Result<()> {
        let (server, mut ws_server, _) = build_server_sse(default_config()).await?;
        let _ = server.post(BASE_URI).form(&valid_preview_form()).await;

        let res = server.post(BASE_URI).form(&valid_preview_form()).await;

        res.assert_status_conflict();
        assert_sse_message(
            &mut ws_server,
            r#"{"headers": {"HX-Trigger": "refreshReports"}}"#,
        )
        .await;
        assert_sse_message(&mut ws_server, r#"{"notification":{"type":"toast","message":"Recipe exists.","status":"alert-error","title":"Operation failed"}}"#).await;
        Ok(())
    }
}
