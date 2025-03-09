use axum::routing::{delete, get, post};
use axum::{middleware, Router};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::core::model::user::UserForCreate;
use crate::server::router::handlers::auth::{
    change_password_post_handler, confirm_handler, forgot_password_handler,
    forgot_password_post_handler, forgot_password_reset_handler,
    forgot_password_reset_post_handler, login_handler, login_post_handler, logout_post_handler,
    register_handler, register_post_handler, user_delete_handler,
};
use crate::server::router::middleware::mw_auth;
use crate::server::AppState;

#[derive(Default, Validate, Deserialize, Serialize)]
pub struct ChangePasswordForm {
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub new_password: String,
    #[validate(must_match(other = "new_password"))]
    pub new_password_confirm: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct ForgotPasswordForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct ForgotPasswordResetForm {
    pub user_id: i64,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}

/// A form structure used for logging in a user.
#[derive(Default, Validate, Deserialize, Serialize)]
pub struct LoginForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    pub remember_me: Option<bool>,
}

impl LoginForm {
    /// Returns whether the user has opted to be remembered across sessions.
    pub fn is_remember_me(&self) -> bool {
        self.remember_me.unwrap_or(false)
    }
}

#[derive(Default, Validate, Deserialize, Serialize)]
pub struct RegisterForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub password_confirm: String,
}

impl RegisterForm {
    /// Converts the form to a UserForCreate.
    pub fn to_user(&self) -> UserForCreate {
        UserForCreate {
            email: self.email.clone(),
            password_clear: self.password.clone(),
        }
    }
}

/// Defines the authentication-related routes for the web application.
pub(super) fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/change-password",
            post(change_password_post_handler).layer(middleware::from_fn(mw_auth::mw_ctx_require)),
        )
        .route("/confirm", get(confirm_handler))
        .route(
            "/forgot-password",
            get(forgot_password_handler)
                .post(forgot_password_post_handler)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    mw_auth::mw_redirect_if_authenticated,
                )),
        )
        .route(
            "/forgot-password/reset",
            get(forgot_password_reset_handler).post(forgot_password_reset_post_handler),
        )
        .route(
            "/login",
            get(login_handler)
                .post(login_post_handler)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    mw_auth::mw_redirect_if_authenticated,
                )),
        )
        .route("/logout", post(logout_post_handler))
        .route(
            "/register",
            get(register_handler).post(register_post_handler).layer(
                middleware::from_fn_with_state(
                    state.clone(),
                    mw_auth::mw_redirect_if_authenticated,
                ),
            ),
        )
        .route(
            "/user",
            delete(user_delete_handler).layer(middleware::from_fn(mw_auth::mw_ctx_require)),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::config::Config;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_change_password {
        use super::*;
        use axum::http::StatusCode;

        use crate::server::test_utils::{assert_must_be_logged_in, build_server_logged_in, build_server_ws, TestDb};

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
            assert_must_be_logged_in(BASE_URI).await
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
            let _ = ws_server.receive_message().await;
            ws_server
                .assert_receive_text_contains(r#"{"showMessageWs":{"type":"toast","message":"Passwords do not match.","status":"alert-info","title":"Operation Failed"}}"#)
                .await;
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
            let _ = ws_server.receive_message().await;
            ws_server
                .assert_receive_text_contains(r#"{"showMessageWs":{"type":"toast","message":"New password cannot be the same as the current.","status":"alert-info","title":"Operation Failed"}}"#)
                .await;
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
            let _ = ws_server.receive_message().await;
            ws_server
                .assert_receive_text_contains(r#"{"showMessageWs":{"type":"toast","message":"Your password has been updated.","status":"alert-info","title":"Operation Successful"}}"#)
                .await;
            Ok(())
        }
    }

    mod tests_confirm {
        use super::*;
        use crate::core::support::time::now_utc_plus_sec_str;
        use crate::server::test_utils::{assert_html, build_server_anonymous, get_token, TestDb};

        const BASE_URI: &str = "/auth/confirm";

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
            let state = AppState::new(config).await?;
            let mut token = get_token(state.mm).await?;
            token.exp = now_utc_plus_sec_str(-100.);

            let res = server
                .get(format!("{BASE_URI}?token={}", &token).as_str())
                .await;

            res.assert_status_bad_request();
            Ok(())
        }

        #[tokio::test]
        async fn test_get_confirm_user_not_exist_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config.clone()).await?;
            let state = AppState::new(config).await?;
            let mut token = get_token(state.mm).await?;
            token.ident = "dont@exist.com".to_string();

            let res = server
                .get(format!("{BASE_URI}?token={}", &token).as_str())
                .await;

            res.assert_status_not_found();
            Ok(())
        }

        #[tokio::test]
        async fn test_get_confirm_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config.clone()).await?;
            let state = AppState::new(config).await?;
            let token = get_token(state.mm).await?;

            let res = server
                .get(format!("{BASE_URI}?token={}", &token).as_str())
                .await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r#"<title hx-swap-oob="true">Success | Recipya</title>"#,
                    r#"Your account has been confirmed."#,
                ],
            );
            Ok(())
        }
    }

    mod tests_delete_user {
        use super::*;

        use crate::core::model::user::User;
        use crate::server::test_utils::{assert_must_be_logged_in, build_server_logged_in, build_server_ws, build_server_ws_other_user, TestDb, TEST_USER_EMAIL};
        use axum::http::StatusCode;

        const BASE_URI: &str = "/auth/user";

        #[tokio::test]
        async fn test_delete_user_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(BASE_URI).await
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
            let _ = ws_server.receive_message().await;
            ws_server
                .assert_receive_text_contains(r#"{"showMessageWs":{"type":"toast","message":"Trump is Putin's lap dog. Remove him from office!","status":"alert-info","title":"Operation Failed"}}"#).await;
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
            let _ = ws_server.receive_message().await;
            ws_server
                .assert_receive_text_contains(r#"{"showMessageWs":{"type":"toast","message":"This account cannot be deleted.","status":"alert-info","title":"Operation Failed"}}"#).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_user_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = AppState::new(config).await?;

            let res = server.delete(BASE_URI).await;

            res.assert_status_see_other();
            res.assert_header(axum_htmx::headers::HX_REDIRECT, "/");
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
        use super::*;

        use crate::core::support::time::now_utc_plus_sec_str;
        use crate::server::test_utils::{
            assert_html, build_server_anonymous, build_server_logged_in, get_token, TestDb,
        };

        const BASE_URI: &str = "/auth/forgot-password";
        const URI_RESET: &str = "/auth/forgot-password/reset";

        #[tokio::test]
        async fn test_get_forgot_password_anonymous_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                res,
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

            let res = server.post(BASE_URI).await;

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
                res,
                vec![
                    r#"<h2 class="card-title underline self-center">Password Reset Requested</h2>"#,
                    r#"An email with instructions on how to reset your password has been sent to you. Please check your inbox and follow the provided steps to regain access to your account."#,
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
            assert_html(res, vec![]);
            Ok(())
        }

        #[tokio::test]
        async fn test_get_forgot_password_reset_err_invalid_token() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_anonymous(config.clone()).await?;
            let state = AppState::new(config).await?;
            let mut token = get_token(state.mm).await?;
            token.exp = now_utc_plus_sec_str(-100.);

            let res = server.get(&format!("{URI_RESET}?token={token}")).await;

            res.assert_status_bad_request();
            assert_html(
                res,
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
            let state = AppState::new(config).await?;
            let token = get_token(state.mm).await?;

            let res = server.get(&format!("{URI_RESET}?token={token}")).await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r#"<title hx-swap-oob="true">Reset Password | Recipya</title>"#,
                    r#"<input name="user-id" type="hidden" value="3">"#,
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
            let server = build_server_anonymous(config).await?;

            let res = server
                .post(URI_RESET)
                .form(&ForgotPasswordResetForm {
                    user_id: 1,
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
            let server = build_server_anonymous(config).await?;

            let res = server
                .post(URI_RESET)
                .form(&ForgotPasswordResetForm {
                    user_id: 1,
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
            Ok(())
        }
    }

    mod tests_login {
        use super::*;

        use std::default::Default;
        use time::format_description::well_known::Rfc3339;
        use time::OffsetDateTime;

        use crate::core::auth::token::Token;
        use crate::core::support::token::AUTH_TOKEN;
        use crate::server::test_utils::{
            assert_html, assert_not_in_html, build_server_anonymous, build_server_logged_in, TestDb,
            TEST_USER_EMAIL, TEST_USER_PASSWORD,
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
                res,
                vec![
                    r#"<form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-post="/auth/login" action="/auth/login" method="post"><div class="card-body">"#,
                    r#"<h2 class="card-title underline self-center">Log In</h2>"#,
                    r#"<fieldset class="fieldset"><label class="label" for="email">Email</label><input id="email" type="email" required placeholder="Enter your email address" class="input" name="email" value=""></fieldset>"#,
                    r#"<fieldset class="fieldset"><label class="label block" for="password">Password<a class="btn btn-sm btn-ghost float-right" href="/auth/forgot-password">Forgot your password?</a></label><input id="password" type="password" required placeholder="Enter your password" class="input" name="password" value=""></fieldset>"#,
                    r#"<fieldset class="fieldset p-4 bg-base-100 border border-base-300 rounded-box w-64 grid self-center mb-2"><legend class="fieldset-legend">Login options</legend><label class="fieldset-label"><input name="remember-me" type="checkbox" checked="checked" class="checkbox" checked="checked">Remember me</label></fieldset>"#,
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
                res,
                vec![
                    r#"<fieldset class="fieldset"><label class="label" for="email">Email</label><input id="email" type="email" required placeholder="Enter your email address" class="input" name="email" value="demo@demo.com"></fieldset>"#,
                    r#"<fieldset class="fieldset"><label class="label block" for="password">Password<a class="btn btn-sm btn-ghost float-right" href="/auth/forgot-password">Forgot your password?</a></label><input id="password" type="password" required placeholder="Enter your password" class="input" name="password" value="demo"></fieldset>"#,
                    r#"<fieldset class="fieldset p-4 bg-base-100 border border-base-300 rounded-box w-64 grid self-center mb-2"><legend class="fieldset-legend">Login options</legend><label class="fieldset-label"><input name="remember-me" type="checkbox" checked="checked" class="checkbox" checked="checked">Remember me</label></fieldset>"#,
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
                res,
                vec![r#"<a class="btn btn-sm btn-block btn-outline" href="/auth/register">Sign Up</a>"#],
            ).await?;
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
            let token: Token = res.cookie(AUTH_TOKEN).value().to_string().parse()?;
            let token_expire = OffsetDateTime::parse(&token.exp, &Rfc3339)?;
            let now = OffsetDateTime::now_utc();
            assert!(
                (token_expire - now).whole_days() >= 30,
                "expiration time should be a month"
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
            let token: Token = res.cookie(AUTH_TOKEN).value().to_string().parse()?;
            let token_expire = OffsetDateTime::parse(&token.exp, &Rfc3339)?;
            let now = OffsetDateTime::now_utc();
            assert!(
                (token_expire - now).whole_days() >= 30,
                "expiration time should be a month"
            );
            Ok(())
        }
    }

    mod tests_logout {
        use super::*;

        use crate::core::auth::token::Token;
        use crate::core::model::user::User;
        use crate::core::support::token::AUTH_TOKEN;
        use crate::server::test_utils::{build_server_logged_in, TestDb, TEST_USER_EMAIL};

        const BASE_URI: &str = "/auth/logout";

        #[tokio::test]
        async fn test_post_logout_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;

            let res = server.post(BASE_URI).await;

            res.assert_status_see_other();
            res.assert_header("Location", "/");
            pretty_assertions::assert_eq!(res.cookie(AUTH_TOKEN).value(), "");
            let state = AppState::new(config).await?;
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
            let token: std::result::Result<Token, _> = res_post
                .maybe_cookie(AUTH_TOKEN)
                .unwrap()
                .to_string()
                .parse();
            assert!(token.is_err(), "auth token should be deleted");
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
        use super::*;
        use crate::core::model::user::User;

        use crate::server::test_utils::{build_server_anonymous, build_server_logged_in, TestDb};

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
            let state = AppState::new(config).await?;
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
            let state = AppState::new(config).await?;
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
            let state = AppState::new(config).await?;
            let user = User::get_user_by_email(&state.mm, &a_form.email).await?;
            assert!(user.is_none(), "user should not have been registered");
            Ok(())
        }
    }
}
