use axum::routing::{delete, get, post};
use axum::{Router, middleware};

use app::state::AppState;

use crate::handlers::auth::{
    change_password_post_handler, forgot_password_handler, forgot_password_post_handler,
    forgot_password_reset_handler, forgot_password_reset_post_handler, login_handler,
    login_post_handler, logout_post_handler, register_handler, register_post_handler,
    user_delete_handler, verify_email_handler,
};
use crate::middleware::mw_auth::mw_refresh_token;

/// Defines the authentication-related routes for the web application.
pub fn auth_routes(state: &AppState) -> Router<AppState> {
    let protected = Router::new()
        .route("/change-password", post(change_password_post_handler))
        .route("/user", delete(user_delete_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_refresh_token,
        ));

    let public = Router::new()
        .route(
            "/forgot-password",
            get(forgot_password_handler).post(forgot_password_post_handler),
        )
        .route(
            "/forgot-password/reset",
            get(forgot_password_reset_handler).post(forgot_password_reset_post_handler),
        )
        .route("/login", get(login_handler).post(login_post_handler))
        .route("/logout", post(logout_post_handler))
        .route(
            "/register",
            get(register_handler).post(register_post_handler),
        )
        .route("/verify-email", get(verify_email_handler));

    Router::new().merge(public).merge(protected)
}

#[cfg(test)]
mod tests {
    use axum::http::{Method, StatusCode};
    use config::Config;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_change_password {
        use super::*;
        use crate::schemas::auth::ChangePasswordForm;

        use testing::utils::{
            TestDb, assert_must_be_logged_in, assert_ws_message, build_server_logged_in,
            build_server_ws,
        };

        const BASE_URI: &str = "/auth/change-password";

        fn a_change_password_form() -> ChangePasswordForm {
            ChangePasswordForm {
                password: "12345678".to_string(),
                new_password: "123456789".to_string(),
                new_password_confirm: "123456789".to_string(),
            }
        }

        #[tokio::test]
        async fn test_post_change_password_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, BASE_URI).await
        }

        #[tokio::test]
        async fn test_post_change_password_form_invalid_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&ChangePasswordForm {
                    password: "12345678".to_string(),
                    new_password: "123456789".to_string(),
                    new_password_confirm: "12345678".to_string(),
                })
                .await;

            res.assert_status_bad_request();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Passwords do not match.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_post_change_password_password_same_as_new_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&ChangePasswordForm {
                    password: "12345678".to_string(),
                    new_password: "12345678".to_string(),
                    new_password_confirm: "12345678".to_string(),
                })
                .await;

            res.assert_status_bad_request();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"New password cannot be the same as the current.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_post_change_password_cannot_update_if_autologin_ok() -> Result<()> {
            let config = Some(Config {
                is_autologin: true,
                ..Default::default()
            });
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.post(BASE_URI).form(&a_change_password_form()).await;

            res.assert_status_forbidden();
            Ok(())
        }

        #[tokio::test]
        async fn test_post_change_password_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server.post(BASE_URI).form(&a_change_password_form()).await;

            res.assert_status(StatusCode::NO_CONTENT);
            assert_ws_message(&mut ws_server, r#"{"showMessageWs":{"type":"toast","message":"Your password has been updated.","status":"alert-info","title":"Operation Successful"}}"#).await;
            Ok(())
        }
    }

    mod tests_confirm {
        use models::{
            tokens::{EmailVerificationToken, EmailVerificationTokenForCreate},
            user::User,
        };
        use testing::utils::{TestDb, assert_html, build_server_anonymous, create_app_state};

        use super::*;

        const BASE_URI: &str = "/auth/verify-email";

        #[tokio::test]
        async fn test_get_confirm_missing_token_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_get_confirm_invalid_token_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let entry = EmailVerificationToken::new(
                &state.mm,
                EmailVerificationTokenForCreate::new(user_id, 0),
            )
            .await?;
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            let res = server
                .get(format!("{BASE_URI}?token={}", entry.token).as_str())
                .await;

            res.assert_status(StatusCode::GONE);
            Ok(())
        }

        #[tokio::test]
        async fn test_get_confirm_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let entry = EmailVerificationToken::new(
                &state.mm,
                EmailVerificationTokenForCreate::new(user_id, 24),
            )
            .await?;

            let res = server
                .get(format!("{BASE_URI}?token={}", entry.token).as_str())
                .await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r#"<title hx-swap-oob="true">Success | Recipya</title>"#,
                    r"Your account has been verified.",
                ],
            );
            Ok(())
        }
    }

    mod tests_delete_user {
        use super::*;

        use axum::http::StatusCode;
        use models::user::User;
        use testing::utils::{
            TEST_USER_EMAIL, TestDb, assert_must_be_logged_in, assert_ws_message,
            build_server_logged_in, build_server_ws, build_server_ws_other_user, create_app_state,
        };

        const BASE_URI: &str = "/auth/user";

        #[tokio::test]
        async fn test_delete_user_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::DELETE, BASE_URI).await
        }

        #[tokio::test]
        async fn test_delete_user_demo_cannot_be_deleted_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(Some(Config {
                is_demo: true,
                ..Config::default()
            }))
            .await?;
            let (server, mut ws_server) =
                build_server_ws_other_user(config.clone(), "demo@demo.com").await?;

            let res = server.delete(BASE_URI).await;

            res.assert_status(StatusCode::FORBIDDEN);
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Trump is Putin's lap dog. Remove him from office!","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_user_cannot_delete_if_autologin_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(Some(Config {
                is_autologin: true,
                ..Config::default()
            }))
            .await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;

            let res = server.delete(BASE_URI).await;

            res.assert_status(StatusCode::FORBIDDEN);
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"This account cannot be deleted.","status":"alert-error","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_user_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;

            let res = server.delete(BASE_URI).await;

            res.assert_status_see_other();
            pretty_assertions::assert_eq!(
                User::get_user_by_email(&state.mm, TEST_USER_EMAIL)
                    .await?
                    .is_none(),
                true,
                "user should have been deleted"
            );
            Ok(())
        }
    }

    mod tests_forgot_password {
        use models::{
            tokens::{PasswordResetToken, PasswordResetTokenForCreate},
            user::User,
        };
        use testing::utils::{
            TestDb, assert_html, build_server_anonymous, build_server_logged_in, create_app_state,
        };

        use super::*;
        use crate::schemas::auth::{ForgotPasswordForm, ForgotPasswordResetForm};

        const BASE_URI: &str = "/auth/forgot-password";
        const URI_RESET: &str = "/auth/forgot-password/reset";

        #[tokio::test]
        async fn test_get_forgot_password_anonymous_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r#"<title hx-swap-oob="true">Forgot Password | Recipya</title>"#,
                    r#"<fieldset class="fieldset"><label class="label" for="email">Email</label><input id="email" type="email" required placeholder="Enter your email address" class="input" name="email"></fieldset>"#,
                    r#"<button class="btn btn-primary btn-block btn-sm">Reset password</button>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_forgot_password_authenticated_no_access_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }

        #[tokio::test]
        async fn test_post_forgot_password_cannot_when_authenticated_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&ForgotPasswordForm {
                    email: "not@exist.com".to_string(),
                })
                .await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }

        #[tokio::test]
        async fn test_post_forgot_password_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&ForgotPasswordForm {
                    email: "not@exist.com".to_string(),
                })
                .await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r#"<h2 class="card-title underline self-center">Password Reset Requested</h2>"#,
                    r"An email with instructions on how to reset your password has been sent to you. Please check your inbox and follow the provided steps to regain access to your account.",
                    r#"<a href="/" class="btn btn-primary btn-block btn-sm">Back Home</a>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_forgot_password_reset_no_token_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server.get(URI_RESET).await;

            res.assert_status_bad_request();
            assert_html(&res, vec![]);
            Ok(())
        }

        #[tokio::test]
        async fn test_get_forgot_password_reset_err_invalid_token() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let entry =
                PasswordResetToken::new(&state.mm, PasswordResetTokenForCreate::new(user_id, 0))
                    .await?;

            let res = server
                .get(&format!("{URI_RESET}?token={}", entry.token))
                .await;

            res.assert_status_bad_request();
            assert_html(
                &res,
                vec![
                    r#"<title hx-swap-oob="true">Token Expired | Recipya</title>"#,
                    "The token associated with the URL expired.",
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_forgot_password_reset_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let entry =
                PasswordResetToken::new(&state.mm, PasswordResetTokenForCreate::new(user_id, 1))
                    .await?;

            let res = server
                .get(&format!("{URI_RESET}?token={}", entry.token))
                .await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r#"<title hx-swap-oob="true">Reset Password | Recipya</title>"#,
                    &format!(
                        r#"<input type="hidden" name="token" value="{}">"#,
                        entry.token
                    ),
                    r#"<fieldset class="fieldset"><label class="label" for="password">New password</label><input id="password" type="password" required placeholder="Enter your new password" class="input" name="password"></fieldset>"#,
                    r#"<fieldset class="fieldset"><label class="label" for="confirm-password">Confirm password</label><input id="confirm-password" type="password" required placeholder="Retype your password" class="input" name="password-confirm"></fieldset>"#,
                    r#"<button class="btn btn-primary btn-block btn-sm">Change</button>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_post_forgot_password_reset_err_invalid() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let server = build_server_anonymous(config).await?;
            let user_id = User::all(&state.mm).await?[0].id;
            let entry =
                PasswordResetToken::new(&state.mm, PasswordResetTokenForCreate::new(user_id, 1))
                    .await?;

            let res = server
                .post(URI_RESET)
                .form(&ForgotPasswordResetForm {
                    token: entry.token,
                    password: "12345678".to_string(),
                    confirm_password: "123456789".to_string(),
                })
                .await;

            res.assert_status_bad_request();
            res.assert_header(
                axum_htmx::headers::HX_TRIGGER,
                r#"{"showMessageHtmx":{"type":"toast","message":"Password is invalid","status":"alert-info","title":"Operation Successful"}}"#,
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_post_forgot_password_reset_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let state = create_app_state(config.clone()).await;
            let server = build_server_anonymous(config).await?;
            let user_id = User::all(&state.mm).await?[0].id;
            let entry =
                PasswordResetToken::new(&state.mm, PasswordResetTokenForCreate::new(user_id, 1))
                    .await?;

            let res = server
                .post(URI_RESET)
                .form(&ForgotPasswordResetForm {
                    token: entry.token.clone(),
                    password: "12345678".to_string(),
                    confirm_password: "12345678".to_string(),
                })
                .await;

            res.assert_status_see_other();
            res.assert_header(
                axum_htmx::headers::HX_TRIGGER,
                r#"{"showMessageHtmx":{"type":"toast","message":"Your password has been updated.","status":"alert-info","title":"Operation Successful"}}"#,
            );
            res.assert_header(axum_htmx::headers::HX_REDIRECT, "/auth/login");
            assert!(
                PasswordResetToken::find_by_token(&state.mm, &entry.token)
                    .await?
                    .is_none()
            );
            Ok(())
        }
    }

    mod tests_login {
        use crate::schemas::auth::LoginForm;

        use super::*;

        use std::default::Default;
        use time::OffsetDateTime;

        use auth::token::{http::AUTH_TOKEN, jwt::validate_token};
        use testing::utils::{
            TEST_USER_EMAIL, TEST_USER_PASSWORD, TestDb, assert_html, assert_not_in_html,
            build_server_anonymous, build_server_logged_in,
        };

        const BASE_URI: &str = "/auth/login";

        fn a_login_form() -> LoginForm {
            LoginForm {
                email: TEST_USER_EMAIL.to_string(),
                password: TEST_USER_PASSWORD.to_string(),
                remember_me: None,
            }
        }

        #[tokio::test]
        async fn test_get_login_page_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r#"<form class="card w-80 sm:w-96 bg-base-100 shadow-xl" method="post" action="/auth/login"><div class="card-body">"#,
                    r#"<h2 class="card-title underline self-center">Log In</h2>"#,
                    r#"<fieldset class="fieldset"><label class="label" for="email">Email</label><input id="email" type="email" required placeholder="Enter your email address" class="input" name="email" value=""></fieldset>"#,
                    r#"<fieldset class="fieldset"><label class="label block" for="password">Password<a class="btn btn-sm btn-ghost float-right" href="/auth/forgot-password">Forgot your password?</a></label><input id="password" type="password" required placeholder="Enter your password" class="input" name="password" value=""></fieldset>"#,
                    r#"<label class="fieldset-label py-2"><input name="remember-me" type="checkbox" checked="checked" class="checkbox" checked="checked">Remember me</label>"#,
                    r#"<div class="card-actions justify-end"><button class="btn btn-primary btn-block btn-sm">Log In</button></div><div class="grid place-content-center text-center gap-2"><div><p class="text-center">Don't have an account?</p><a class="btn btn-sm btn-block btn-outline" href="/auth/register">Sign Up</a></div></div>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_login_page_demo_show_autologin_ok() -> Result<()> {
            let config = Some(Config {
                is_demo: true,
                ..Default::default()
            });
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_anonymous(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r#"<fieldset class="fieldset"><label class="label" for="email">Email</label><input id="email" type="email" required placeholder="Enter your email address" class="input" name="email" value="demo@demo.com"></fieldset>"#,
                    r#"<fieldset class="fieldset"><label class="label block" for="password">Password<a class="btn btn-sm btn-ghost float-right" href="/auth/forgot-password">Forgot your password?</a></label><input id="password" type="password" required placeholder="Enter your password" class="input" name="password" value="demo"></fieldset>"#,
                    r#"<label class="fieldset-label py-2"><input name="remember-me" type="checkbox" checked="checked" class="checkbox" checked="checked">Remember me</label>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_get_login_page_hide_signup_button_when_no_signups_ok() -> Result<()> {
            let config = Some(Config {
                is_no_signups: true,
                ..Default::default()
            });
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_anonymous(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_not_in_html(
                &res,
                vec![
                    r#"<a class="btn btn-sm btn-block btn-outline" href="/auth/register">Sign Up</a>"#,
                ],
            )?;
            Ok(())
        }

        #[tokio::test]
        async fn test_get_login_redirect_to_home_when_already_logged_in_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }

        #[tokio::test]
        async fn test_get_login_redirect_to_recipes_when_autologin_ok() -> Result<()> {
            let config = Some(Config {
                is_autologin: true,
                ..Default::default()
            });
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_anonymous(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }

        #[tokio::test]
        async fn test_get_login_remember_me_checked_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&LoginForm {
                    email: TEST_USER_EMAIL.to_string(),
                    password: TEST_USER_PASSWORD.to_string(),
                    remember_me: Some(true),
                })
                .await;

            res.assert_status_see_other();
            res.assert_header("Location", "/");
            let token = res.cookie(AUTH_TOKEN).value().to_string();
            let claims = validate_token(&token)?;
            let now =
                usize::try_from(OffsetDateTime::now_utc().unix_timestamp()).unwrap_or_default();
            assert!(
                (claims.exp - now) >= 15 * 60,
                "expiration time should be 15 minutes"
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_post_login_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server.post(BASE_URI).form(&a_login_form()).await;

            res.assert_status_see_other();
            Ok(())
        }

        #[tokio::test]
        async fn test_post_login_invalid_email_err() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&LoginForm {
                    email: "hello".to_string(),
                    password: TEST_USER_PASSWORD.to_string(),
                    remember_me: None,
                })
                .await;

            res.assert_status_bad_request();
            res.assert_header(
                axum_htmx::headers::HX_TRIGGER,
                r#"{"showMessageHtmx":{"type":"toast","message":"Credentials are invalid.","status":"alert-error","title":"Operation Failed"}}"#,
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_post_login_invalid_password_err() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&LoginForm {
                    email: "hello@example.com".to_string(),
                    password: "123".to_string(),
                    remember_me: None,
                })
                .await;

            res.assert_status_bad_request();
            res.assert_header(
                axum_htmx::headers::HX_TRIGGER,
                r#"{"showMessageHtmx":{"type":"toast","message":"Credentials are invalid.","status":"alert-error","title":"Operation Failed"}}"#,
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_post_login_remember_me_checked_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&LoginForm {
                    email: TEST_USER_EMAIL.to_string(),
                    password: TEST_USER_PASSWORD.to_string(),
                    remember_me: Some(true),
                })
                .await;

            res.assert_status_see_other();
            res.assert_header("Location", "/");
            let token = res.cookie(AUTH_TOKEN).value().to_string();
            let claims = validate_token(&token)?;
            let now =
                usize::try_from(OffsetDateTime::now_utc().unix_timestamp()).unwrap_or_default();
            assert!(
                (claims.exp - now) >= 15 * 60,
                "expiration time should be 15 minutes"
            );
            Ok(())
        }
    }

    mod tests_logout {
        use super::*;

        use auth::token::http::REFRESH_TOKEN;
        use models::user::User;
        use testing::utils::{TEST_USER_EMAIL, TestDb, build_server_logged_in, create_app_state};

        const BASE_URI: &str = "/auth/logout";

        #[tokio::test]
        async fn test_post_logout_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;

            let res = server.post(BASE_URI).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/");
            assert!(
                res.maybe_cookie(REFRESH_TOKEN).is_none(),
                "refresh token should be deleted"
            );
            let state = create_app_state(config).await;
            let user = User::get_user_by_email(&state.mm, TEST_USER_EMAIL)
                .await?
                .expect("Expected user");
            pretty_assertions::assert_eq!(user.is_remember_me, false);
            Ok(())
        }

        #[tokio::test]
        async fn test_post_logout_user_already_logged_in_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res_post = server.post(BASE_URI).await;
            let res_get = server.get("/auth/login").await;

            res_post.assert_status_see_other();
            res_get.assert_status_ok();
            assert!(
                res_post.maybe_cookie(REFRESH_TOKEN).is_none(),
                "refresh token should be deleted"
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_post_logout_cannot_logout_when_autologin_ok() -> Result<()> {
            let config = Some(Config {
                is_autologin: true,
                ..Default::default()
            });
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.post(BASE_URI).await;

            res.assert_status_forbidden();
            Ok(())
        }
    }

    mod tests_register {
        use crate::schemas::auth::{LoginForm, RegisterForm};

        use super::*;
        use models::user::User;

        use testing::utils::{
            TestDb, build_server_anonymous, build_server_logged_in, create_app_state,
        };

        const BASE_URI: &str = "/auth/register";

        fn a_register_form() -> RegisterForm {
            RegisterForm {
                email: "new_user@example.com".to_string(),
                password: "12345678".to_string(),
                password_confirm: "12345678".to_string(),
            }
        }

        #[tokio::test]
        async fn test_get_register_redirect_to_home_when_logged_in_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }

        #[tokio::test]
        async fn test_get_register_cannot_access_register_when_no_signups_ok() -> Result<()> {
            let config = Some(Config {
                is_no_signups: true,
                ..Default::default()
            });
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }

        #[tokio::test]
        async fn test_post_register_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config.clone()).await?;
            let form = a_register_form();

            let res = server.post(BASE_URI).form(&form).await;

            res.assert_status_see_other();
            let state = create_app_state(config).await;
            let user = User::get_user_by_email(&state.mm, &form.email).await?;
            assert!(user.is_some(), "should have user in database");
            Ok(())
        }

        #[tokio::test]
        async fn test_post_register_when_user_already_registered_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;
            let form = a_register_form();

            let _res = server.post(BASE_URI).form(&form).await;
            let _res = server
                .post("/auth/login")
                .form(&LoginForm {
                    email: form.email.clone(),
                    password: form.password.clone(),
                    remember_me: Some(false),
                })
                .await;
            let res = server.post(BASE_URI).form(&form).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/recipes");
            Ok(())
        }

        #[tokio::test]
        async fn test_register_redirect_to_home_when_autologin_ok() -> Result<()> {
            let config = Some(Config {
                is_autologin: true,
                ..Default::default()
            });
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let a_form = a_register_form();

            let res_get = server.get(BASE_URI).await;
            let res_post = server.post(BASE_URI).form(&a_form).await;

            res_get.assert_status_see_other();
            res_post.assert_status_see_other();
            res_get.assert_header("Location", "/recipes");
            res_post.assert_header("Location", "/recipes");
            let state = create_app_state(config).await;
            let user = User::get_user_by_email(&state.mm, &a_form.email).await?;
            assert!(user.is_none(), "user should not have been registered");
            Ok(())
        }

        #[tokio::test]
        async fn test_register_cannot_register_when_no_signups_ok() -> Result<()> {
            let config = Some(Config {
                is_no_signups: true,
                ..Default::default()
            });
            let (_test_db, config) = TestDb::new(config).await?;
            let server = build_server_anonymous(config.clone()).await?;
            let a_form = a_register_form();

            let res_get = server.post(BASE_URI).form(&a_form).await;
            let res_post = server.post(BASE_URI).form(&a_form).await;

            res_get.assert_status_see_other();
            res_get.assert_status_see_other();
            res_get.assert_header("Location", "/auth/login");
            res_post.assert_header("Location", "/auth/login");
            let state = create_app_state(config).await;
            let user = User::get_user_by_email(&state.mm, &a_form.email).await?;
            assert!(user.is_none(), "user should not have been registered");
            Ok(())
        }
    }
}
