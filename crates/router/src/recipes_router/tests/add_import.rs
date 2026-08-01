#[cfg(test)]
mod tests {
    use axum::http::Method;
    use axum_test::http::StatusCode;
    use axum_test::multipart::{MultipartForm, Part};

    use models::Recipe;
    use serde_json::json;
    use test_db::TestDb;
    use test_fixtures::{HIDDEN_WS_NOTIFICATION, assert_html, assert_ws_message, open_test_file};
    use test_utils::{
        assert_must_be_logged_in, build_server_logged_in, build_server_ws, create_app_state,
    };

    use crate::recipes_router::params::PreviewForm;

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

        use super::*;

        const BASE_URI: &str = "/recipes/add/import/app";

        #[tokio::test]
        async fn test_must_be_logged_in() -> Result<()> {
            assert_must_be_logged_in(Method::POST, BASE_URI).await
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
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Parsing recipes...</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">1 of 100</span><span class="font-semibold">1.0%</span></div><div id="export-progress"><progress max="100" value="1.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"An error occurred while parsing the recipes. Please check the logs.","status":"alert-error","title":"Operation Failed"}}"#).await;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            pretty_assertions::assert_eq!(Recipe::count(&state.mm, user_id).await?, 0);
            Ok(())
        }

        #[tracing_test::traced_test]
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
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Parsing recipes...</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">1 of 100</span><span class="font-semibold">1.0%</span></div><div id="export-progress"><progress max="100" value="1.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Saving media</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">1 of 3</span><span class="font-semibold">33.3%</span></div><div id="export-progress"><progress max="100" value="33.33"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Saving media</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">3 of 3</span><span class="font-semibold">100.0%</span></div><div id="export-progress"><progress max="100" value="100.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default "><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1">Saving recipes</p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">3 of 3</span><span class="font-semibold">100.0%</span></div><div id="export-progress"><progress max="100" value="100.00"></progress></div></div></div>"#).await;
            assert_ws_message(&mut ws_server, HIDDEN_WS_NOTIFICATION).await;
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","action":"View /reports?view=latest","message":"Imported 3 recipes. Skipped 0.","status":"alert-info","title":"Success"}}"#).await;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            pretty_assertions::assert_eq!(Recipe::count(&state.mm, user_id).await?, 3);
            let reports = ViewReport::fetch_all(&state.mm, 1, user_id).await?;
            assert_eq!(reports.len(), 1);
            Ok(())
        }

        #[tokio::test]
        async fn test_post_payload_too_large() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;
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
        use axum_test::TestResponse;

        use super::*;

        const BASE_URI: &str = "/recipes/add/import/preview";

        fn assert_other_json_html(res: &TestResponse) {
            assert_html(
                res,
                &[
                    r#"<span class="text-center pb-2 print:w-full w-full" itemprop="name">Dry Toast</span>"#,
                    r#"<p class="text-sm text-center">1 servings</p>"#,
                    r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="14px" viewBox="0 0 23 14" version="1.1"><defs><linearGradient id="linear0" gradientUnits="userSpaceOnUse" x1="-125.300003" y1="85.900002" x2="-64.599998" y2="85.900002" gradientTransform="matrix(0.000000000000000013,-0.225806,0.219048,0.000000000000000014,-7.447619,-14.451613)"><stop offset="0.1" style="stop-color:rgb(67.058824%,23.921569%,8.235294%);stop-opacity:1;"></stop><stop offset="0.5" style="stop-color:rgb(78.431373%,51.372549%,30.588235%);stop-opacity:1;"></stop><stop offset="0.8" style="stop-color:rgb(90.196078%,77.254902%,53.333333%);stop-opacity:1;"></stop><stop offset="1" style="stop-color:rgb(94.509804%,87.45098%,62.352941%);stop-opacity:1;"></stop></linearGradient></defs><g id="surface1"><path style=" stroke:none;fill-rule:evenodd;fill:rgb(95.294118%,82.745099%,64.705884%);fill-opacity:1;" d="M 0 8.128906 L 0.21875 6.324219 C 0.4375 5.644531 0.65625 5.195312 1.3125 4.96875 L 8.542969 2.484375 L 8.542969 1.804688 L 8.980469 0.675781 L 9.855469 0.453125 L 10.734375 0.453125 L 10.953125 0.675781 L 12.265625 1.128906 C 13.003906 1 13.761719 1.078125 14.457031 1.355469 C 15.125 1.453125 15.785156 1.601562 16.429688 1.804688 L 17.523438 1.804688 L 18.617188 0.902344 L 20.589844 0.453125 C 21.246094 0.675781 21.6875 0.902344 21.90625 1.355469 C 22.34375 1.804688 22.5625 2.710938 22.34375 4.066406 L 22.125 4.515625 C 22.5625 4.742188 22.78125 5.195312 22.78125 5.644531 L 22.78125 7.675781 L 22.125 8.804688 L 21.027344 9.484375 L 17.304688 11.289062 L 16.210938 11.742188 L 12.921875 13.324219 L 12.046875 13.773438 L 11.390625 14 L 10.078125 14 L 8.542969 13.546875 C 6.269531 12.441406 4.007812 11.308594 1.753906 10.160156 L 0.65625 9.257812 C 0.21875 9.03125 0 8.582031 0 8.128906 Z M 0 8.128906 "></path><path style=" stroke:none;fill-rule:evenodd;fill:url(#linear0);" d="M 1.3125 4.742188 L 8.542969 2.03125 L 8.542969 1.582031 L 8.980469 0.453125 C 9.199219 0.226562 9.636719 0 9.855469 0.226562 L 10.953125 0.226562 L 12.265625 0.902344 C 13.003906 0.773438 13.761719 0.851562 14.457031 1.128906 L 15.769531 1.355469 C 16.308594 1.65625 16.933594 1.738281 17.523438 1.582031 L 17.523438 1.355469 C 17.742188 1.128906 18.179688 0.675781 18.617188 0.675781 C 19.277344 0.226562 19.933594 0.226562 20.589844 0.226562 C 21.027344 0.453125 21.6875 0.675781 21.90625 1.128906 C 22.34375 1.582031 22.5625 2.484375 22.34375 3.839844 L 22.125 4.289062 C 22.5625 4.515625 22.78125 4.96875 22.78125 5.417969 L 22.78125 6.546875 L 22.5625 7.453125 L 22.125 8.582031 L 21.027344 9.257812 L 17.085938 11.289062 L 16.210938 11.515625 L 12.921875 13.097656 L 12.046875 13.546875 L 11.390625 13.773438 L 9.855469 13.773438 L 8.324219 13.324219 C 6.121094 12.21875 3.929688 11.089844 1.753906 9.933594 L 1.535156 9.933594 L 0.4375 9.03125 L 0 7.902344 C 0 7.292969 0.0742188 6.6875 0.21875 6.097656 C 0.21875 5.417969 0.65625 4.96875 1.3125 4.742188 Z M 1.3125 4.742188 "></path><path style="fill:none;stroke-width:0.3;stroke-linecap:butt;stroke-linejoin:miter;stroke:rgb(95.294118%,82.745099%,64.705884%);stroke-opacity:1;stroke-miterlimit:4;" d="M 5.991848 21.001116 L 39.999151 8.995536 L 39.00051 6.00279 L 40.997792 2.006696 L 46.008832 0 L 47.007473 0 L 49.004755 1.003348 L 50.003397 1.003348 L 55.995245 3.996094 C 59.579654 2.923549 63.413723 2.923549 66.998132 3.996094 L 73.007812 6.00279 C 75.272588 6.763951 77.733526 6.763951 79.998302 6.00279 L 84.991508 2.006696 C 88.005265 1.003348 91.001189 0 93.997113 1.003348 C 96.993037 1.003348 99.008152 2.006696 101.005435 3.996094 C 103.002717 6.00279 103.002717 9.998884 102.004076 16.001674 L 102.004076 18.008371 L 104.001359 23.007812 L 105 28.007254 L 104.001359 33.006696 L 101.005435 37.00279 L 95.994395 40.998884 L 78.99966 49.008371 L 75.005095 50.997768 L 74.006454 50.997768 L 58.991168 58.003906 L 54.996603 59.993304 L 52.000679 59.993304 L 51.002038 60.996652 L 46.008832 60.996652 L 39.00051 59.007254 C 28.60394 54.128906 18.278702 49.129464 8.006963 43.991629 L 8.006963 43.00558 L 2.995924 39.995536 L 0 33.992746 C 0 31.294085 0.338825 28.612723 0.998641 26.000558 C 1.997283 23.993862 2.995924 22.004464 5.991848 21.001116 Z M 5.991848 21.001116 " transform="matrix(0.219048,0,0,0.225806,0,0)"></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(25.882354%,9.411765%,1.568628%);fill-opacity:1;" d="M 1.3125 8.128906 L 1.09375 7.675781 L 1.3125 6.097656 L 1.753906 5.644531 L 9.855469 2.933594 L 9.636719 2.257812 C 9.710938 1.882812 9.785156 1.503906 9.855469 1.128906 L 10.078125 1.128906 L 10.515625 1.355469 L 10.734375 1.355469 L 12.265625 2.03125 C 12.914062 1.859375 13.589844 1.859375 14.238281 2.03125 C 14.910156 2.207031 15.570312 2.433594 16.210938 2.710938 L 16.648438 2.710938 L 17.960938 2.484375 L 18.398438 2.03125 L 19.058594 1.582031 L 20.371094 1.355469 L 21.246094 1.804688 L 21.246094 3.839844 L 21.027344 4.066406 L 20.371094 4.515625 L 20.589844 4.515625 L 21.464844 4.96875 L 21.90625 5.417969 L 21.90625 6.324219 L 21.6875 7 L 21.464844 7.675781 L 20.589844 8.128906 C 19.933594 8.582031 18.839844 9.257812 16.867188 9.933594 L 12.484375 11.96875 L 11.828125 12.417969 C 11.617188 12.515625 11.394531 12.589844 11.171875 12.644531 L 10.296875 12.644531 L 8.980469 12.195312 C 6.703125 11.09375 4.441406 9.964844 2.191406 8.804688 Z M 1.3125 8.128906 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(65.490198%,51.372552%,26.274511%);fill-opacity:1;" d="M 6.351562 5.195312 C 8.921875 4.820312 11.476562 4.371094 14.019531 3.839844 L 15.332031 4.289062 L 16.429688 4.515625 C 17.304688 4.515625 17.742188 4.289062 17.960938 4.066406 L 18.179688 3.613281 L 18.617188 3.160156 L 19.933594 2.484375 L 21.027344 2.03125 L 21.027344 3.839844 L 20.808594 4.066406 C 20.371094 4.289062 19.933594 4.515625 19.277344 4.289062 L 17.960938 4.742188 L 19.496094 4.96875 L 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.171875 12.417969 C 10.515625 12.417969 9.636719 12.417969 8.980469 11.96875 C 6.324219 10.59375 3.695312 9.164062 1.09375 7.675781 L 1.3125 7 L 1.535156 6.324219 Z M 6.351562 5.195312 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(56.078434%,44.313726%,22.352941%);fill-opacity:1;" d="M 4.382812 7.902344 L 3.503906 7.902344 L 3.285156 9.257812 L 3.066406 9.03125 L 3.285156 7.675781 L 2.847656 7.226562 L 2.628906 7.453125 L 2.410156 8.804688 L 2.191406 8.582031 L 1.972656 8.582031 L 2.410156 7.226562 L 1.972656 7 L 1.753906 8.355469 L 1.3125 8.128906 L 1.535156 6.546875 L 6.351562 6.324219 L 10.078125 8.128906 L 10.953125 10.839844 L 11.171875 12.417969 L 10.296875 12.417969 L 10.515625 10.839844 L 9.417969 10.613281 L 9.199219 12.195312 L 8.542969 11.96875 L 8.761719 10.386719 L 8.105469 10.160156 L 7.886719 11.515625 L 7.449219 11.289062 L 7.449219 9.710938 L 7.230469 9.03125 L 6.570312 9.257812 L 6.132812 10.613281 L 5.476562 10.386719 L 5.914062 9.03125 L 4.820312 8.128906 L 4.601562 8.355469 L 4.382812 9.710938 L 4.160156 9.484375 Z M 4.382812 7.902344 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(52.941179%,41.176471%,18.82353%);fill-opacity:1;" d="M 19.933594 2.484375 L 19.933594 2.710938 C 18.839844 3.160156 18.617188 3.839844 19.496094 4.289062 L 18.617188 4.289062 L 17.960938 4.742188 L 19.496094 4.96875 L 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.828125 12.195312 L 10.515625 12.417969 L 10.734375 12.195312 L 10.734375 9.710938 L 10.515625 8.804688 C 13.609375 6.628906 16.75 4.519531 19.933594 2.484375 Z M 19.933594 2.484375 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(47.450981%,36.470589%,16.862746%);fill-opacity:1;" d="M 21.027344 7.453125 C 21.464844 7 21.464844 6.546875 21.464844 5.644531 L 21.464844 7.226562 L 21.246094 7.453125 L 20.371094 8.128906 C 17.839844 9.550781 15.203125 10.757812 12.484375 11.742188 L 11.828125 12.195312 L 10.515625 12.417969 L 10.734375 12.195312 L 11.171875 12.195312 L 12.046875 11.96875 C 15.042969 10.46875 18.035156 8.960938 21.027344 7.453125 Z M 21.027344 7.453125 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(87.450981%,71.764708%,40.784314%);fill-opacity:1;" d="M 1.753906 6.097656 C 5.445312 4.722656 9.167969 3.441406 12.921875 2.257812 L 14.238281 2.484375 L 15.550781 2.933594 L 16.648438 3.160156 C 17.523438 3.160156 17.960938 2.933594 18.179688 2.710938 L 18.617188 2.257812 L 19.058594 2.03125 C 19.714844 1.582031 20.152344 1.582031 20.808594 1.804688 C 21.246094 2.03125 21.246094 2.484375 20.808594 2.710938 L 19.496094 3.160156 L 18.179688 3.613281 L 19.058594 4.289062 C 19.855469 4.75 20.660156 5.203125 21.464844 5.644531 C 21.6875 5.871094 21.246094 6.324219 20.589844 6.773438 C 17.578125 8.257812 14.511719 9.613281 11.390625 10.839844 C 10.734375 11.066406 9.855469 10.839844 8.980469 10.613281 C 6.472656 9.3125 3.988281 7.957031 1.535156 6.546875 L 1.535156 6.097656 Z M 1.753906 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 2.628906 7.226562 L 2.410156 7.226562 L 4.820312 6.097656 L 6.351562 4.742188 L 8.105469 3.839844 C 9.554688 3.546875 11.015625 3.320312 12.484375 3.160156 C 12.921875 3.160156 13.582031 2.933594 14.019531 2.484375 L 14.457031 2.484375 C 12.945312 3.640625 11.078125 4.203125 9.199219 4.066406 C 8.105469 4.066406 7.230469 4.289062 6.570312 4.742188 L 5.039062 6.097656 C 4.601562 6.546875 3.722656 7 2.628906 7.226562 Z M 2.628906 7.226562 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 5.257812 7 C 4.601562 7 3.941406 7.226562 3.503906 7.675781 L 3.285156 7.675781 C 4.5625 6.878906 5.976562 6.34375 7.449219 6.097656 L 10.296875 5.417969 L 11.609375 4.742188 L 12.484375 4.066406 L 14.675781 2.484375 L 14.894531 2.710938 L 12.921875 4.289062 L 10.953125 5.644531 C 9.855469 6.324219 7.886719 6.773438 5.257812 7 Z M 1.972656 6.324219 L 3.066406 5.871094 L 4.160156 5.195312 L 6.132812 4.515625 L 5.476562 4.96875 L 3.722656 6.097656 L 2.191406 6.773438 L 1.972656 6.773438 L 1.535156 6.546875 Z M 9.855469 3.160156 L 11.609375 2.710938 L 13.363281 2.257812 L 13.582031 2.257812 C 13.144531 2.710938 12.484375 2.933594 11.828125 2.933594 Z M 18.617188 7.675781 C 19.277344 6.546875 20.152344 5.871094 21.246094 5.417969 L 21.464844 5.644531 C 20.808594 5.871094 20.152344 6.324219 19.714844 7 Z M 9.199219 7.902344 C 10.078125 7.902344 10.953125 7.675781 12.046875 7 L 14.019531 5.417969 L 15.550781 4.066406 C 16.210938 3.613281 16.648438 3.160156 17.304688 3.160156 L 18.179688 2.710938 L 20.589844 1.804688 L 20.808594 1.804688 L 20.589844 2.03125 L 18.398438 2.933594 L 16.210938 3.839844 L 14.457031 5.417969 C 13.800781 6.324219 13.144531 6.773438 12.703125 7 C 12.265625 7.453125 11.390625 7.902344 10.078125 8.128906 L 7.886719 8.582031 L 6.570312 9.257812 L 5.914062 8.804688 L 7.230469 8.355469 Z M 16.867188 6.097656 C 17.304688 5.195312 17.960938 4.742188 18.839844 4.289062 L 19.496094 4.515625 L 17.742188 5.871094 L 15.992188 7.902344 C 15.113281 8.582031 14.457031 9.03125 13.582031 9.257812 L 10.953125 9.710938 L 9.417969 10.613281 L 8.761719 10.386719 L 10.515625 9.484375 L 12.703125 9.03125 C 14.382812 8.582031 15.851562 7.542969 16.867188 6.097656 Z M 16.867188 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(79.215688%,64.313728%,33.333334%);fill-opacity:1;" d="M 6.789062 7.675781 C 5.914062 7.675781 5.257812 7.902344 4.601562 8.355469 L 4.160156 8.128906 C 4.820312 7.675781 5.914062 7.226562 7.230469 7.226562 L 10.953125 6.097656 C 12.046875 5.644531 12.921875 4.96875 13.582031 4.289062 L 15.332031 2.710938 L 15.992188 2.933594 L 14.019531 4.515625 L 12.265625 5.871094 L 9.636719 7.226562 Z M 20.152344 4.742188 L 20.589844 4.96875 L 18.839844 6.324219 C 18.179688 6.773438 17.742188 7.453125 17.304688 8.355469 C 14.960938 9.164062 12.625 9.992188 10.296875 10.839844 L 12.703125 9.933594 L 14.894531 9.03125 C 15.992188 8.582031 17.304688 7.453125 18.617188 5.644531 Z M 13.144531 7.453125 C 14.671875 6.238281 16.203125 5.035156 17.742188 3.839844 C 17.960938 3.386719 19.058594 2.933594 21.027344 2.03125 L 21.027344 2.484375 C 20.402344 2.570312 19.804688 2.800781 19.277344 3.160156 L 18.179688 3.613281 L 18.398438 3.839844 L 15.769531 6.324219 L 13.582031 8.128906 L 10.515625 8.804688 C 9.417969 9.03125 8.761719 9.484375 8.105469 9.933594 L 7.449219 9.710938 C 9.289062 8.8125 11.191406 8.058594 13.144531 7.453125 Z M 6.570312 5.871094 L 6.570312 5.644531 L 6.789062 5.195312 L 7.230469 4.515625 L 8.761719 4.289062 L 10.515625 4.289062 C 10.734375 4.515625 10.734375 4.742188 10.296875 4.96875 L 8.761719 5.644531 L 7.449219 5.871094 L 7.449219 5.195312 L 8.324219 4.742188 L 9.199219 4.515625 L 9.417969 4.742188 L 8.761719 5.195312 L 8.980469 4.96875 L 8.324219 4.96875 L 8.105469 5.195312 C 7.886719 5.417969 8.105469 5.417969 8.324219 5.417969 L 9.199219 5.195312 L 9.855469 4.742188 L 9.855469 4.515625 L 8.761719 4.515625 L 7.449219 4.96875 L 6.789062 5.417969 Z M 6.570312 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(41.960785%,25.098041%,14.117648%);fill-opacity:1;" d="M 9.855469 3.160156 L 11.171875 2.710938 L 12.265625 3.839844 L 14.238281 5.417969 L 15.550781 7 L 16.210938 8.582031 L 15.992188 9.484375 L 15.332031 9.710938 L 14.894531 9.710938 L 14.894531 9.257812 L 15.113281 9.03125 L 15.332031 8.582031 L 14.675781 7.675781 L 14.457031 7.453125 L 14.457031 7.226562 L 14.238281 6.773438 L 14.019531 6.773438 C 13.304688 7.003906 12.570312 7.15625 11.828125 7.226562 L 11.171875 7.226562 L 10.734375 6.097656 L 10.078125 4.289062 Z M 9.855469 3.160156 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(47.843137%,47.843137%,47.843137%);fill-opacity:1;" d="M 11.171875 7 L 11.390625 5.871094 L 12.265625 4.96875 L 13.582031 4.96875 L 14.894531 5.195312 L 15.113281 5.871094 L 14.894531 6.097656 L 12.921875 6.773438 L 11.609375 7 Z M 11.171875 7 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(59.215689%,59.215689%,59.215689%);fill-opacity:1;" d="M 12.265625 6.097656 L 12.046875 6.546875 L 12.265625 7 L 11.171875 7 L 11.390625 6.324219 Z M 12.265625 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(92.54902%,92.54902%,92.54902%);fill-opacity:1;" d="M 9.855469 1.582031 L 10.734375 1.804688 L 12.921875 2.933594 C 13.75 3.667969 14.488281 4.5 15.113281 5.417969 L 14.894531 5.417969 L 14.019531 4.289062 L 12.484375 2.710938 L 10.734375 1.804688 Z M 9.855469 1.582031 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(78.039217%,78.039217%,78.039217%);fill-opacity:1;" d="M 9.855469 1.582031 L 10.078125 1.582031 L 10.515625 1.804688 C 10.609375 3.429688 11.140625 4.992188 12.046875 6.324219 L 11.171875 7 L 10.953125 6.546875 C 10.421875 5.246094 10.054688 3.882812 9.855469 2.484375 Z M 9.855469 1.582031 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(89.411765%,89.411765%,89.411765%);fill-opacity:1;" d="M 10.078125 3.386719 L 10.078125 2.933594 L 10.734375 2.933594 L 10.734375 3.160156 Z M 9.855469 2.03125 L 10.515625 2.03125 L 10.515625 2.710938 L 9.855469 2.710938 Z M 11.828125 6.097656 L 11.171875 6.773438 L 10.953125 6.324219 L 11.609375 5.871094 Z M 10.296875 4.515625 L 10.078125 3.839844 L 10.734375 3.613281 L 10.953125 4.066406 Z M 11.390625 5.195312 L 10.734375 5.644531 L 10.515625 4.96875 L 10.953125 4.515625 Z M 11.390625 5.195312 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(56.470591%,56.470591%,56.470591%);fill-opacity:1;" d="M 14.238281 6.546875 L 14.019531 6.324219 L 14.019531 5.871094 L 14.675781 5.871094 L 15.113281 6.097656 L 15.113281 6.324219 L 14.894531 6.546875 Z M 14.238281 6.546875 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(70.19608%,70.19608%,70.19608%);fill-opacity:1;" d="M 14.019531 5.871094 L 14.238281 5.644531 L 14.894531 5.644531 L 15.113281 5.871094 L 15.113281 6.097656 L 14.238281 6.097656 Z M 14.019531 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(55.686277%,35.686275%,17.254902%);fill-opacity:1;" d="M 14.238281 6.773438 L 14.238281 6.097656 L 14.894531 6.097656 L 15.550781 6.773438 L 16.429688 8.128906 L 16.429688 8.582031 L 15.769531 9.484375 C 15.113281 9.710938 14.675781 9.710938 14.894531 9.257812 L 15.113281 8.582031 L 15.113281 8.128906 L 14.894531 7.675781 L 14.675781 7.453125 L 14.457031 6.773438 Z M 14.238281 6.773438 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(43.137255%,27.058825%,13.725491%);fill-opacity:1;" d="M 15.769531 8.355469 L 16.210938 7.675781 L 16.429688 8.128906 L 16.429688 8.582031 C 16.429688 9.03125 16.210938 9.484375 15.769531 9.484375 L 14.894531 9.484375 L 14.894531 9.03125 L 15.113281 8.582031 L 15.332031 8.355469 Z M 15.769531 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(64.313728%,44.705883%,26.666668%);fill-opacity:1;" d="M 14.675781 6.324219 L 14.457031 6.097656 L 14.457031 5.871094 L 15.113281 5.871094 L 15.769531 6.097656 L 16.429688 7.226562 L 16.429688 8.582031 C 15.992188 8.804688 15.550781 9.03125 15.113281 8.804688 L 15.113281 8.355469 L 15.550781 7.902344 L 15.332031 7.453125 L 14.894531 7.226562 L 14.675781 6.773438 Z M 14.675781 6.324219 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.113281 8.128906 L 15.550781 7.902344 L 15.332031 8.355469 L 15.332031 8.804688 L 15.113281 8.804688 Z M 14.457031 5.871094 L 14.894531 6.546875 L 15.332031 7.453125 L 15.113281 7.453125 L 14.894531 6.773438 L 14.894531 6.546875 L 14.675781 6.546875 L 14.238281 6.097656 Z M 16.429688 7.675781 L 16.429688 7.453125 C 16.648438 7.902344 16.648438 8.355469 16.210938 8.582031 Z M 16.429688 7.675781 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 14.894531 6.097656 L 15.332031 6.546875 L 15.550781 6.773438 L 15.550781 7 L 15.769531 7.453125 L 15.769531 8.128906 L 15.550781 8.804688 L 15.550781 7 L 14.675781 5.871094 Z M 14.894531 6.097656 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.550781 6.324219 L 15.992188 6.546875 L 16.210938 7.226562 L 16.210938 8.128906 L 15.992188 8.804688 L 15.992188 6.546875 C 15.695312 6.324219 15.402344 6.101562 15.113281 5.871094 L 15.332031 5.871094 Z M 15.550781 6.324219 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.113281 5.871094 L 15.332031 6.324219 L 15.550781 6.773438 L 15.769531 7.226562 L 15.992188 7.453125 L 15.992188 8.128906 L 15.769531 8.804688 L 15.769531 7 L 15.550781 6.773438 L 15.332031 6.324219 L 14.894531 5.871094 Z M 15.332031 5.871094 L 15.769531 6.324219 L 15.992188 6.546875 L 15.550781 6.324219 Z M 15.332031 5.871094 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(43.137255%,27.058825%,13.725491%);fill-opacity:1;" d="M 16.210938 8.355469 C 15.992188 8.582031 15.769531 8.804688 15.550781 8.582031 L 15.332031 8.355469 L 15.992188 8.128906 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(69.411767%,69.411767%,69.411767%);fill-opacity:1;" d="M 16.210938 8.355469 L 15.992188 8.582031 L 15.332031 8.582031 L 15.769531 8.355469 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(49.019608%,49.019608%,49.019608%);fill-opacity:1;" d="M 16.210938 8.355469 L 15.992188 8.582031 L 15.332031 8.582031 L 15.769531 8.582031 L 15.992188 8.355469 Z M 16.210938 8.355469 "></path><path style=" stroke:none;fill-rule:evenodd;fill:rgb(76.078433%,52.156866%,30.980393%);fill-opacity:1;" d="M 15.992188 6.773438 L 15.769531 6.773438 L 15.992188 7 L 15.992188 6.773438 L 15.992188 7 L 15.769531 7 L 15.769531 6.773438 Z M 15.992188 6.773438 "></path></g></svg><time datetime="PT3M">3m</time></div>"#,
                    r#"<div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time"><svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="24px" height="23px" viewBox="0 0 24 23" version="1.1"><g id="surface1"><path style=" stroke:none;fill-rule:nonzero;fill:rgb(62.745098%,64.705882%,65.882353%);fill-opacity:1;" d="M 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 Z M 4.636719 10.984375 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(76.862745%,76.862745%,76.862745%);fill-opacity:1;" d="M 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 Z M 19.289062 9.953125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.710938 7.8125 L 13.914062 7.8125 C 14.945312 7.8125 15.828125 8.476562 16.269531 9.363281 C 16.417969 9.730469 16.785156 9.953125 17.226562 9.953125 L 19.289062 9.953125 C 18.992188 8.699219 17.964844 7.8125 16.710938 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 22.160156 9.953125 L 20.320312 9.953125 C 20.097656 8.183594 18.550781 6.78125 16.710938 6.78125 L 15.535156 6.78125 L 15.3125 5.898438 C 15.09375 5.160156 14.503906 4.644531 13.765625 4.644531 L 11.191406 4.644531 C 10.453125 4.644531 9.792969 5.160156 9.644531 5.898438 L 9.421875 6.78125 L 7.214844 6.78125 C 5.449219 6.78125 3.902344 8.183594 3.605469 9.953125 L 1.765625 9.953125 C 1.03125 9.953125 0.441406 10.542969 0.441406 11.277344 C 0.441406 11.5 0.441406 11.722656 0.589844 11.941406 L 1.25 13.269531 C 1.546875 13.785156 2.0625 14.152344 2.648438 14.152344 L 3.605469 14.152344 L 3.605469 20.417969 C 3.605469 21.597656 4.492188 22.558594 5.667969 22.558594 L 18.257812 22.558594 C 19.4375 22.558594 20.394531 21.597656 20.394531 20.417969 L 20.394531 14.152344 L 21.351562 14.152344 C 21.9375 14.152344 22.453125 13.859375 22.75 13.269531 L 23.410156 11.867188 C 23.558594 11.648438 23.558594 11.5 23.558594 11.277344 C 23.558594 10.542969 22.894531 9.953125 22.160156 9.953125 M 3.605469 13.121094 L 2.648438 13.121094 C 2.429688 13.121094 2.28125 12.972656 2.136719 12.753906 L 1.472656 11.5 L 1.472656 11.277344 C 1.472656 11.132812 1.621094 10.984375 1.765625 10.984375 L 3.605469 10.984375 Z M 10.75 6.117188 C 10.75 5.972656 10.96875 5.75 11.265625 5.75 L 13.839844 5.75 C 14.0625 5.75 14.28125 5.898438 14.355469 6.117188 L 14.503906 6.78125 L 10.601562 6.78125 Z M 7.214844 7.8125 L 16.710938 7.8125 C 17.964844 7.8125 18.992188 8.699219 19.289062 9.953125 L 4.710938 9.953125 C 4.933594 8.699219 5.964844 7.8125 7.214844 7.8125 M 19.363281 13.636719 L 19.363281 20.417969 C 19.363281 21.007812 18.847656 21.527344 18.257812 21.527344 L 5.667969 21.527344 C 5.078125 21.527344 4.636719 21.007812 4.636719 20.417969 L 4.636719 10.984375 L 19.363281 10.984375 Z M 22.453125 11.5 L 21.71875 12.828125 C 21.644531 12.972656 21.496094 13.121094 21.277344 13.121094 L 20.394531 13.121094 L 20.394531 11.058594 L 22.160156 11.058594 C 22.308594 11.058594 22.453125 11.207031 22.453125 11.351562 L 22.453125 11.5 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(92.54902%,94.117647%,97.647059%);fill-opacity:1;" d="M 7.804688 17.324219 C 8.089844 17.324219 8.320312 17.554688 8.320312 17.839844 C 8.320312 18.125 8.089844 18.355469 7.804688 18.355469 C 7.519531 18.355469 7.289062 18.125 7.289062 17.839844 C 7.289062 17.554688 7.519531 17.324219 7.804688 17.324219 M 7.804688 16.808594 C 7.4375 16.808594 7.214844 16.585938 7.214844 16.21875 L 7.214844 13.121094 C 7.214844 12.753906 7.4375 12.53125 7.804688 12.53125 C 8.097656 12.53125 8.320312 12.753906 8.320312 13.121094 L 8.320312 16.21875 C 8.320312 16.585938 8.097656 16.808594 7.804688 16.808594 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(54.509804%,69.411765%,81.960784%);fill-opacity:1;" d="M 16.195312 12.015625 L 16.195312 19.902344 C 16.195312 20.492188 15.679688 21.007812 15.09375 21.007812 L 6.699219 21.007812 C 6.183594 21.007812 5.667969 20.492188 5.667969 19.902344 L 5.667969 12.015625 C 5.667969 11.5 5.226562 10.984375 4.636719 10.984375 L 4.636719 20.417969 C 4.636719 21.007812 5.078125 21.527344 5.667969 21.527344 L 18.257812 21.527344 C 18.847656 21.527344 19.363281 21.007812 19.363281 20.417969 L 19.363281 10.984375 L 17.226562 10.984375 C 16.636719 10.984375 16.195312 11.5 16.195312 12.015625 M 8.246094 7.8125 L 7.214844 7.8125 C 5.964844 7.8125 4.933594 8.699219 4.710938 9.953125 L 4.933594 9.953125 C 5.375 9.953125 5.742188 9.730469 5.890625 9.363281 C 6.332031 8.476562 7.214844 7.8125 8.246094 7.8125 "></path><path style=" stroke:none;fill-rule:nonzero;fill:rgb(0.392157%,26.666667%,38.823529%);fill-opacity:1;" d="M 18.773438 5.75 C 18.625 5.75 18.480469 5.675781 18.40625 5.527344 C 18.183594 5.308594 18.257812 4.9375 18.40625 4.792969 C 18.699219 4.644531 18.773438 4.421875 18.773438 4.128906 C 18.773438 3.90625 18.699219 3.6875 18.480469 3.539062 C 18.257812 3.316406 18.257812 3.023438 18.40625 2.800781 C 18.625 2.582031 18.992188 2.507812 19.140625 2.726562 C 19.582031 3.097656 19.878906 3.613281 19.878906 4.128906 C 19.878906 4.71875 19.582031 5.234375 19.140625 5.601562 L 18.773438 5.75 M 18.773438 1.03125 C 19.058594 1.03125 19.289062 1.261719 19.289062 1.546875 C 19.289062 1.832031 19.058594 2.0625 18.773438 2.0625 C 18.488281 2.0625 18.257812 1.832031 18.257812 1.546875 C 18.257812 1.261719 18.488281 1.03125 18.773438 1.03125 M 16.710938 5.75 L 16.269531 5.527344 C 16.050781 5.308594 16.121094 4.9375 16.34375 4.792969 C 16.5625 4.644531 16.710938 4.421875 16.710938 4.128906 C 16.710938 3.90625 16.5625 3.6875 16.417969 3.539062 C 15.902344 3.097656 15.679688 2.652344 15.679688 2.0625 C 15.679688 1.472656 15.902344 1.03125 16.417969 0.589844 C 16.5625 0.367188 16.933594 0.441406 17.152344 0.664062 C 17.300781 0.8125 17.300781 1.179688 17.078125 1.402344 C 16.785156 1.546875 16.710938 1.769531 16.710938 2.0625 C 16.710938 2.285156 16.785156 2.507812 17.007812 2.652344 C 17.519531 3.097656 17.742188 3.613281 17.742188 4.128906 C 17.742188 4.71875 17.519531 5.234375 17.007812 5.601562 L 16.710938 5.75 "></path></g></svg><time datetime="PT0S">0s</time></div>"#,
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
                &res,
                &[
                    r#"<div class="text-error">Invalid JSON: key must be a string at line 1 column 2</div>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_post_payload_with_properties1_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

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
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

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
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

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
        use models::{reports::ViewReport, user::User};

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
            let server = build_server_logged_in(config.clone()).await?;

            let res = server.post(BASE_URI).form(&valid_preview_form()).await;

            res.assert_status_ok();
            res.assert_header(axum_htmx::HX_REDIRECT, "/recipes/1");
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let reports = ViewReport::fetch_all(&state.mm, 1, user_id).await?;
            assert_eq!(reports.len(), 1);
            Ok(())
        }

        #[tokio::test]
        async fn test_post_payload_double_insert_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;
            let _ = server.post(BASE_URI).form(&valid_preview_form()).await;

            let res = server.post(BASE_URI).form(&valid_preview_form()).await;

            res.assert_status_conflict();
            assert_ws_message(
                &mut ws_server,
                r#"{"headers": {"HX-Trigger": "refreshReports"}}"#,
            )
            .await;
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Recipe exists.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }
    }
}
