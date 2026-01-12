use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};

use app::state::AppState;

use crate::handlers::general::{
    index_handler, upload_note_image, user_initials_handler, ws_handler,
};

/// Defines the routes for general endpoints of the web application.
pub(super) fn general_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index_handler))
        .route(
            "/upload/note-image",
            post(upload_note_image).layer(DefaultBodyLimit::max(10 * 1024 * 1024)),
        )
        .route("/user-initials", get(user_initials_handler))
        .route("/ws", get(ws_handler))
}

#[cfg(test)]
mod tests {
    use axum::http::Method;
    use axum_test::multipart::{MultipartForm, Part};
    use serde_json::json;
    use testing::utils::{TestDb, assert_must_be_logged_in, build_server_logged_in};
    use uuid::Uuid;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

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

        use testing::utils::{TestDb, assert_must_be_logged_in, build_server_logged_in};

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
