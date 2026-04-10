use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};

use app::state::AppState;

use crate::handlers::general::{
    download_handler, fetch_handler, index_handler, search_suggestions_handler, upload_note_image,
    user_initials_handler, ws_handler,
};
use crate::middleware::mw_auth::mw_refresh_token;

/// Defines the routes for general endpoints of the web application.
pub fn general_routes(state: &AppState) -> Router<AppState> {
    let protected = Router::new()
        .route("/download", get(download_handler))
        .route("/fetch", get(fetch_handler))
        .route("/search-suggestions", get(search_suggestions_handler))
        .route(
            "/upload/note-image",
            post(upload_note_image).layer(DefaultBodyLimit::max(10 * 1024 * 1024)),
        )
        .route("/user-initials", get(user_initials_handler))
        .route("/ws", get(ws_handler))
        .layer(from_fn_with_state(state.clone(), mw_refresh_token));

    Router::new()
        .route("/", get(index_handler))
        .merge(protected)
}

#[cfg(test)]
mod tests {
    use axum::http::Method;
    use axum_test::multipart::{MultipartForm, Part};
    use serde_json::json;
    use test_db::TestDb;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in, create_app_state};
    use uuid::Uuid;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_fetch {
        use super::*;

        fn url(target: &str) -> String {
            format!("/fetch?url={target}")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &url("https://www.example.com")).await
        }

        #[tokio::test]
        async fn test_missing_url_param_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_invalid_url_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("not-a-url")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_non_http_scheme_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("ftp://example.com/file.txt")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_file_scheme_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("file:///etc/passwd")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_loopback_ip_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("http://127.0.0.1/admin")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_private_ip_10_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("http://10.0.0.1/internal")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_private_ip_172_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("http://172.16.0.1/internal")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_private_ip_192_168_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("http://192.168.1.1/internal")).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_cloud_metadata_ip_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .get(&url("http://169.254.169.254/latest/meta-data"))
                .await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_unreachable_url_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url("http://192.0.2.1/")).await;

            res.assert_status_bad_request();
            Ok(())
        }
    }

    mod tests_index {
        use super::*;

        use config::Config;

        const BASE_URI: &str = "/";

        #[tokio::test]
        async fn test_get_index_redirect_to_recipes_when_autologin_ok() -> Result<()> {
            let config = Some(Config {
                is_autologin: true,
                ..Default::default()
            });
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }
    }

    mod tests_download {
        use std::path::PathBuf;

        use models::{
            download::{Download, DownloadForCreate},
            user::User,
        };

        use super::*;

        fn url(token: Uuid) -> String {
            format!("/download?token={token}")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &url(Uuid::new_v4())).await
        }

        #[tokio::test]
        async fn test_token_not_found_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&url(Uuid::new_v4())).await;

            res.assert_status_not_found();
            Ok(())
        }

        #[tokio::test]
        async fn test_download_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let server = build_server_logged_in(config).await?;
            let user_id = User::all(&state.mm).await?[0].id;
            let file_path = "/tmp/test_export.zip".to_string();
            tokio::fs::write(&file_path, b"some zip bytes").await?;
            let token = Uuid::new_v4();
            Download::create(
                &state.mm,
                DownloadForCreate::new(user_id, token, PathBuf::from(&file_path)),
            )
            .await?;

            let res = server.get(&url(token)).await;

            res.assert_status_ok();
            res.assert_header("content-type", "application/zip");
            res.assert_header(
                "content-disposition",
                "attachment; filename=\"recipya-data-export.zip\"",
            );
            assert!(Download::find_by_token(&state.mm, token).await?.is_none());
            assert!(!tokio::fs::try_exists(&file_path).await?);
            Ok(())
        }

        #[tokio::test]
        async fn test_download_file_missing_on_disk_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let server = build_server_logged_in(config).await?;
            let user = User::all(&state.mm).await?[0].clone();
            let token = Uuid::new_v4();
            Download::create(
                &state.mm,
                DownloadForCreate {
                    user_id: user.id,
                    token,
                    file_path: PathBuf::from("/tmp/nonexistent_export.zip"),
                },
            )
            .await?;

            let res = server.get(&url(token)).await;

            res.assert_status_internal_server_error();
            Ok(())
        }
    }

    mod tests_search_suggestions {
        use config::Config;
        use models::{Recipe, user::User};
        use test_models::a_complete_recipe_for_create;

        use super::*;

        fn url(term: &str) -> String {
            format!("/search-suggestions?q={term}")
        }

        #[tokio::test]
        async fn test_categories_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("cat:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'cat:dinner' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">dinner</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_cuisines_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let _ = insert_basic_recipe(config).await;
            let (mut recipe, _) = a_complete_recipe_for_create();
            recipe.name = "Test Recipe".into();
            recipe.cuisine = Some("Italian".into());
            let user = User::all(&state.mm).await?[0].clone();
            let _ = Recipe::create(&state.mm, user.id, &recipe).await?;

            let res = server.get(&url("cui:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'cui:italian' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">italian</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'cui:thai' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">thai</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_ingredients_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("ing:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'ing:blue spinach' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">blue spinach</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'ing:cinnamon' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">cinnamon</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'ing:lemon juice' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">lemon juice</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'ing:top quality chicken filet' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">top quality chicken filet</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_keywords_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("kw:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'kw:tofu' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">tofu</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'kw:vegetarian' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">vegetarian</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_tools_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("tool:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'tool:frying pan' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">frying pan</a></li><li><a tabindex="0" _="on click set #search-recipes.value to 'tool:wok' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">wok</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_sources_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("src:")).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(
                res.text()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" "),
                r#"<li><a tabindex="0" _="on click set #search-recipes.value to 'src:www.allrecipes.com' then add .hidden to #search-suggestions-menu then call #search-recipes.focus() then set #search-recipes.selectionStart to #search-recipes.value.length then set #search-recipes.selectionEnd to #search-recipes.value.length">www.allrecipes.com</a></li>"#
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_invalid_term_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let _ = insert_basic_recipe(config).await;

            let res = server.get(&url("source:")).await;

            res.assert_status_ok();
            assert!(res.text().is_empty());
            Ok(())
        }

        async fn insert_basic_recipe(config: Config) -> Result<()> {
            let state = create_app_state(config.clone()).await;
            let user = User::all(&state.mm).await?[0].clone();
            let (recipe, _) = a_complete_recipe_for_create();
            let _ = Recipe::create(&state.mm, user.id, &recipe).await?;
            Ok(())
        }
    }

    mod tests_upload_note_image {
        use std::io::Cursor;

        use image::{ImageBuffer, Rgb};
        use reqwest::StatusCode;
        use serde_json::Value;

        use super::*;

        const BASE_URI: &str = "/upload/note-image";

        fn create_form(is_add_image: bool, is_image_valid: bool) -> MultipartForm {
            let mut form = MultipartForm::new();

            if is_add_image {
                let image = Uuid::new_v4();

                if is_image_valid {
                    let img: ImageBuffer<Rgb<u8>, _> =
                        ImageBuffer::from_pixel(1, 1, Rgb([255, 0, 0]));

                    let mut buf = Vec::new();
                    img.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Jpeg)
                        .unwrap();

                    form = form.add_part(
                        "image",
                        Part::bytes(buf)
                            .file_name(format!("{image}.jpg"))
                            .mime_type("image/jpeg"),
                    );
                } else {
                    let image = Uuid::new_v4();
                    form = form.add_part(
                        "image",
                        Part::file_name(Part::text(image.to_string()), format!("{image}.jpg")),
                    );
                }
            }

            form
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, BASE_URI).await
        }

        #[tokio::test]
        async fn test_no_image_in_form_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .post(BASE_URI)
                .multipart(create_form(false, false))
                .await;

            res.assert_status_bad_request();
            let body: Value = res.json();
            assert_eq!(body, json!({"error": "noFileGiven"}));
            Ok(())
        }

        #[tokio::test]
        async fn test_image_type_not_supported_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .post(BASE_URI)
                .multipart(create_form(true, false))
                .await;

            res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
            let body: Value = res.json();
            assert_eq!(body, json!({"error": "typeNotAllowed"}));
            Ok(())
        }

        #[tokio::test]
        async fn test_image_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .post(BASE_URI)
                .multipart(create_form(true, true))
                .await;

            res.assert_status_ok();
            Ok(())
        }
    }

    mod tests_user_initials {
        use axum::http::Method;

        use test_utils::{assert_must_be_logged_in, build_server_logged_in};

        use super::*;

        const BASE_URI: &str = "/user-initials";

        #[tokio::test]
        async fn test_get_user_initials_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_get_user_initials_logged_in_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            pretty_assertions::assert_eq!(res.text(), "T");
            Ok(())
        }
    }
}
