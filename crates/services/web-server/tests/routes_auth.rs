mod support;

use crate::support::{
    assert::assert_html,
    server::{build_server_anonymous, build_server_logged_in},
    test_db::*,
};
use axum::http::StatusCode;
use lib_web::handlers::handlers_auth::LoginForm;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod tests_change_password {
    use super::*;
    use lib_web::handlers::handlers_auth::ChangePasswordForm;
    use support::server::build_server_ws;

    const BASE_URI: &str = "/auth/change-password";

    fn a_change_password_form() -> ChangePasswordForm {
        ChangePasswordForm {
            password: "12345678".to_string(),
            new_password: "123456789".to_string(),
            new_password_confirm: "123456789".to_string(),
        }
    }

    #[tokio::test]
    async fn test_post_change_password_err_must_be_logged_in() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server.post(BASE_URI).await;

        pretty_assertions::assert_eq!(res.status_code(), StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(res.header("Location"), "/auth/login");
        Ok(())
    }

    #[tokio::test]
    async fn test_post_change_password_err_form_invalid() -> Result<()> {
        let db = TestDb::new().await;
        let (server, mut ws_server) = build_server_ws(db.state()).await?;

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
    async fn test_post_change_password_err_password_same_as_new() -> Result<()> {
        let db = TestDb::new().await;
        let (server, mut ws_server) = build_server_ws(db.state()).await?;

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
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_post_change_err_password_cannot_update_if_autologin() -> Result<()> {
        std::env::set_var("SERVICE_AUTOLOGIN", "true");
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.state()).await?;

        let res = server.post(BASE_URI).form(&a_change_password_form()).await;

        std::env::remove_var("SERVICE_AUTOLOGIN");
        res.assert_status_forbidden();
        Ok(())
    }

    #[tokio::test]
    async fn test_post_change_password_ok() -> Result<()> {
        let db = TestDb::new().await;
        let (server, mut ws_server) = build_server_ws(db.state()).await?;

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
    use lib_utils::time::now_utc_plus_sec_str;

    const BASE_URI: &str = "/auth/confirm";

    #[tokio::test]
    async fn test_confirm_err_missing_token() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_confirm_err_invalid_token() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;
        let mut token = get_token(db.state().mm).await;
        token.exp = now_utc_plus_sec_str(-100.);

        let res = server
            .get(format!("{BASE_URI}?token={}", &token).as_str())
            .await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_confirm_err_user_not_exist() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;
        let mut token = get_token(db.state().mm).await;
        token.ident = "dont@exist.com".to_string();

        let res = server
            .get(format!("{BASE_URI}?token={}", &token).as_str())
            .await;

        res.assert_status_bad_request();
        Ok(())
    }

    #[tokio::test]
    async fn test_confirm_ok() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;
        let token = get_token(db.state().mm).await;

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

mod tests_forgot_password {
    use super::*;
    use lib_utils::time::now_utc_plus_sec_str;
    use lib_web::handlers::{
        handlers_auth::{ForgotPasswordForm, ForgotPasswordResetForm},
        KEY_HX_REDIRECT, KEY_HX_TRIGGER,
    };

    const BASE_URI: &str = "/auth/forgot-password";
    const URI_RESET: &str = "/auth/forgot-password/reset";

    #[tokio::test]
    async fn test_get_forgot_password_ok_anonymous() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status_ok();
        assert_html(
            res,
            vec![
                r#"<title hx-swap-oob="true">Forgot Password | Recipya</title>"#,
                r#"<input required type="email" placeholder="Enter your email address" class="input input-bordered w-full" name="email">"#,
                r#"<button class="btn btn-primary btn-block btn-sm">Reset password</button>"#,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_forgot_password_err_authenticated_no_access() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.state()).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(res.header("Location"), "/");
        Ok(())
    }

    #[tokio::test]
    async fn test_post_forgot_password_err_cannot_when_authenticated() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.state()).await?;

        let res = server.post(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(res.header("Location"), "/");
        Ok(())
    }

    #[tokio::test]
    async fn test_post_forgot_password_ok() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

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
    async fn test_get_forgot_password_reset_err_no_token() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server.get(URI_RESET).await;

        res.assert_status_bad_request();
        assert_html(res, vec![]);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_forgot_password_reset_err_invalid_token() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;
        let mut token = get_token(db.state().mm).await;
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
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;
        let token = get_token(db.state().mm).await;

        let res = server.get(&format!("{URI_RESET}?token={token}")).await;

        res.assert_status_ok();
        assert_html(
            res,
            vec![
                r#"<title hx-swap-oob="true">Reset Password | Recipya</title>"#,
                r#"<input name="user-id" type="hidden" value="2">"#,
                r#"<input required type="password" placeholder="Enter your new password" class="input input-bordered w-full" name="password">"#,
                r#"<input required type="password" placeholder="Retype your password" class="input input-bordered w-full" name="password-confirm">"#,
                r#"<button class="btn btn-primary btn-block btn-sm">Change</button>"#,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_forgot_password_reset_err_invalid() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server
            .post(URI_RESET)
            .form(&ForgotPasswordResetForm {
                user_id: 1,
                password: "12345678".to_string(),
                confirm_password: "123456789".to_string(),
            })
            .await;

        res.assert_status_bad_request();
        pretty_assertions::assert_eq!(
            res.header(KEY_HX_TRIGGER),
            r#"{"showMessageHtmx":{"type":"toast","message":"Password is invalid","status":"alert-info","title":"Operation Successful"}}"#
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_forgot_password_reset_ok() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server
            .post(URI_RESET)
            .form(&ForgotPasswordResetForm {
                user_id: 1,
                password: "12345678".to_string(),
                confirm_password: "12345678".to_string(),
            })
            .await;

        res.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(
            res.header(KEY_HX_TRIGGER),
            r#"{"showMessageHtmx":{"type":"toast","message":"Your password has been updated.","status":"alert-info","title":"Operation Successful"}}"#
        );
        pretty_assertions::assert_eq!(res.header(KEY_HX_REDIRECT), "/auth/login");
        Ok(())
    }
}

mod tests_login {
    use super::*;
    use lib_auth::token::Token;
    use lib_utils::time::{OffsetDateTime, Rfc3339};
    use lib_web::handlers::KEY_HX_TRIGGER;
    use lib_web::utils::token::AUTH_TOKEN;
    use support::assert::assert_not_in_html;

    const BASE_URI: &str = "/auth/login";

    fn a_login_form() -> LoginForm {
        LoginForm {
            email: "test@example.com".to_string(),
            password: "12345678".to_string(),
            remember_me: None,
        }
    }

    #[tokio::test]
    async fn test_get_login_page_ok() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::OK);
        assert_html(
            res,
            vec![
                r#"<form class="card w-80 sm:w-96 bg-base-100 shadow-xl" hx-post="/auth/login" action="/auth/login" method="post"><div class="card-body">"#,
                r#"<h2 class="card-title underline self-center">Log In</h2>"#,
                r#"<label class="form-control w-full"><div class="label"><span class="label-text font-semibold">Email</span></div><input class="input input-bordered w-full" required type="email" placeholder="Enter your email address" name="email" value=""></label>"#,
                r#"<label class="form-control w-full"><div class="label"><span class="label-text font-semibold">Password</span></div><input class="input input-bordered w-full" required type="password" placeholder="Enter your password" name="password" value=""></label>"#,
                r#"<div class="form-control grid place-content-center"><label class="label cursor-pointer gap-2"><span class="label-text">Remember me</span><input class="checkbox checkbox-primary" type="checkbox" name="remember_me" value="true"></label>"#,
                r#"<div class="card-actions justify-end"><button class="btn btn-primary btn-block btn-sm">Log In</button></div><div class="grid place-content-center text-center gap-2"><div><p class="text-center">Don't have an account?</p><a class="btn btn-sm btn-block btn-outline" href="/auth/register">Sign Up</a></div><a class="btn btn-sm btn-ghost" href="/auth/forgot-password">Forgot your password?</a></div>"#,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_login_ok() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server.post(BASE_URI).form(&a_login_form()).await;

        res.assert_status(StatusCode::SEE_OTHER);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_get_login_ok_hide_signup_button_when_registration_disabled() -> Result<()> {
        std::env::set_var("SERVICE_NO_SIGNUPS", "true");
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server.get(BASE_URI).await;

        std::env::remove_var("SERVICE_NO_SIGNUPS");
        res.assert_status(StatusCode::OK);
        assert_not_in_html(
            res,
            vec![
                r#"<div class="card-actions justify-end"><button class="btn btn-primary btn-block btn-sm">Log In</button></div><div class="grid place-content-center text-center gap-2"><div><p class="text-center">Don't have an account?</p><a class="btn btn-sm btn-block btn-outline" href="/auth/register">Sign Up</a></div><a class="btn btn-sm btn-ghost" href="/auth/forgot-password">Forgot your password?</a></div>"#,
            ],
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_login_err_invalid_email() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server
            .post(BASE_URI)
            .form(&LoginForm {
                email: "hello".to_string(),
                password: "12345678".to_string(),
                remember_me: None,
            })
            .await;

        res.assert_status(StatusCode::BAD_REQUEST);
        pretty_assertions::assert_eq!(
            res.header(KEY_HX_TRIGGER),
            r#"{"showMessageHtmx":{"type":"toast","message":"Credentials are invalid.","status":"alert-error","title":"Operation Failed"}}"#
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_err_invalid_password() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server
            .post(BASE_URI)
            .form(&LoginForm {
                email: "hello@example.com".to_string(),
                password: "123".to_string(),
                remember_me: None,
            })
            .await;

        res.assert_status(StatusCode::BAD_REQUEST);
        pretty_assertions::assert_eq!(
            res.header(KEY_HX_TRIGGER),
            r#"{"showMessageHtmx":{"type":"toast","message":"Credentials are invalid.","status":"alert-error","title":"Operation Failed"}}"#
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_login_ok_redirect_to_home_when_logged_in() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.state()).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(
            res.header("Location"),
            "/",
            "Location should be set to home"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_get_login_ok_redirect_to_index_when_autologin() -> Result<()> {
        std::env::set_var("SERVICE_AUTOLOGIN", "true");
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server.get(BASE_URI).await;

        std::env::remove_var("SERVICE_AUTOLOGIN");
        res.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(
            res.header("Location"),
            "/",
            "Location should be set to login"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_login_ok_remember_me_checked() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server
            .post(BASE_URI)
            .form(&LoginForm {
                email: "test@example.com".to_string(),
                password: "12345678".to_string(),
                remember_me: Some(true),
            })
            .await;

        res.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(
            res.header("Location"),
            "/",
            "Location should be set to login"
        );
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
    async fn test_get_login_ok_remember_me_checked() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;
        server
            .post(BASE_URI)
            .form(&LoginForm {
                email: "test@example.com".to_string(),
                password: "12345678".to_string(),
                remember_me: Some(true),
            })
            .await;

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(
            res.header("Location"),
            "/",
            "Location should be set to login"
        );
        Ok(())
    }
}

mod tests_logout {
    use super::*;
    use lib_auth::token::Token;
    use lib_web::utils::token::AUTH_TOKEN;

    const BASE_URI: &str = "/auth/logout";

    #[tokio::test]
    async fn test_post_logout_ok() -> Result<()> {
        Ok(())
    }

    #[tokio::test]
    async fn test_post_logout_err_user_already_logged_out() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.state()).await?;

        let res: axum_test::TestResponse = server.post(BASE_URI).await;
        let res_get = server.get("/auth/login").await;

        res.assert_status(StatusCode::SEE_OTHER);
        res_get.assert_status_ok();
        let token: std::result::Result<Token, _> =
            res.maybe_cookie(AUTH_TOKEN).unwrap().to_string().parse();
        assert!(token.is_err(), "auth token should be deleted");
        Ok(())
    }

    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_post_logout_err_cannot_logout_when_autologin() -> Result<()> {
        std::env::set_var("SERVICE_AUTOLOGIN", "true");
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.state()).await?;

        let res = server.post(BASE_URI).await;

        std::env::remove_var("SERVICE_AUTOLOGIN");
        res.assert_status_forbidden();
        Ok(())
    }

    #[tokio::test]
    async fn test_post_logout_ok_remember_me_user_deletes_token() -> Result<()> {
        Ok(())
    }
}

mod tests_register {
    use super::*;
    use lib_core::{ctx::Ctx, model::user::UserBmc};
    use lib_web::handlers::handlers_auth::RegisterForm;
    use lib_web::handlers::KEY_HX_TRIGGER;

    const BASE_URI: &str = "/auth/register";

    fn a_register_form() -> RegisterForm {
        RegisterForm {
            email: "new_user@example.com".to_string(),
            password: "12345678".to_string(),
            password_confirm: "12345678".to_string(),
        }
    }

    #[tokio::test]
    async fn test_get_register_ok_redirect_to_home_when_logged_in() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.state()).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(
            res.header("Location"),
            "/",
            "Location should be set to home"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_register_ok() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;
        let form = a_register_form();

        let res = server.post(BASE_URI).form(&form).await;

        res.assert_status(StatusCode::SEE_OTHER);
        assert!(
            UserBmc::first_by_email(&Ctx::root_ctx(), &db.state().mm, &form.email)
                .await?
                .is_some(),
            "should have user in database"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_register_err_when_user_already_registered() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let _res = server.post(BASE_URI).form(&a_register_form()).await;
        let res = server.post(BASE_URI).form(&a_register_form()).await;

        res.assert_status(StatusCode::INTERNAL_SERVER_ERROR);
        pretty_assertions::assert_eq!(
            res.header(KEY_HX_TRIGGER),
            r#"{"showMessageHtmx":{"type":"toast","message":"An error occurred during registration.","status":"alert-error","title":"Operation Failed"}}"#
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_register_ok_redirect_to_home_when_autologin() -> Result<()> {
        std::env::set_var("SERVICE_AUTOLOGIN", "true");
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;
        let fx_form = a_register_form();

        let res_get = server.get(BASE_URI).await;
        let res_post = server.post(BASE_URI).form(&fx_form).await;

        std::env::remove_var("SERVICE_AUTOLOGIN");
        res_get.assert_status(StatusCode::SEE_OTHER);
        res_post.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(
            res_get.header("Location"),
            "/",
            "Location should point to home"
        );
        pretty_assertions::assert_eq!(
            res_post.header("Location"),
            "/",
            "Location should point to home"
        );
        let user =
            UserBmc::first_by_email(&Ctx::root_ctx(), &db.state().mm, &fx_form.email).await?;
        assert!(user.is_none(), "user should not have been registered");
        Ok(())
    }

    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_register_err_cannot_register_when_no_signups() -> Result<()> {
        std::env::set_var("SERVICE_NO_SIGNUPS", "true");
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;
        let form = a_register_form();

        let res_get = server.post(BASE_URI).form(&form).await;
        let res_post = server.post(BASE_URI).form(&form).await;

        std::env::remove_var("SERVICE_NO_SIGNUPS");
        res_get.assert_status(StatusCode::SEE_OTHER);
        res_get.assert_status(StatusCode::SEE_OTHER);
        pretty_assertions::assert_eq!(
            res_get.header("Location"),
            "/auth/login",
            "Location should point to home"
        );
        pretty_assertions::assert_eq!(
            res_post.header("Location"),
            "/auth/login",
            "Location should point to home"
        );
        let user = UserBmc::first_by_email(&Ctx::root_ctx(), &db.state().mm, &form.email).await?;
        assert!(user.is_none(), "user should not have been registered");
        Ok(())
    }

    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_get_err_register_cannot_access_register_when_no_signups() -> Result<()> {
        std::env::set_var("SERVICE_NO_SIGNUPS", "true");
        let db = TestDb::new().await;
        let server = build_server_anonymous(db.state()).await?;

        let res = server.get(BASE_URI).await;

        std::env::remove_var("SERVICE_NO_SIGNUPS");
        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(
            res.header("Location"),
            "/auth/login",
            "Location should be set to login"
        );
        Ok(())
    }
}
