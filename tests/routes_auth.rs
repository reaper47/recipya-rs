use std::sync::Arc;

use axum::http::StatusCode;

use helpers::*;

mod helpers;

/*
#[cfg(test)]
mod login {
    use recipya::app::App;
    use recipya::model::ModelManager;
    use recipya::models::payloads::LoginForm;

    use super::*;

    const BASE_URI: &str = "/auth/login";

    fn a_login_form() -> LoginForm {
        LoginForm {
            email: "test@example.com".to_string(),
            password: "12345678".to_string(),
            remember_me: None,
        }
    }

    #[tokio::test]
    async fn success() {
        let server = build_server(
            Arc::new(App::new_test()),
            ModelManager::new().await.unwrap(),
        );

        let res = server.post(BASE_URI).form(&a_login_form()).await;

        res.assert_status(StatusCode::SEE_OTHER);
    }

    #[tokio::test]
    async fn get_login_page() {
        let server = build_server(
            Arc::new(App::new_test()),
            ModelManager::new().await.unwrap(),
        );

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::OK);
        assert_html(
            res,
            vec![
                r#"<form class="card w-80 sm:w-96 bg-base-100 shadow-xl" action="/auth/login" method="post" hx-post="/auth/login"><div class="card-body">"#,
                r#"<h2 class="card-title underline self-center">Log In</h2>"#,
                r#"<label class="form-control w-full"><div class="label"><span class="label-text font-semibold">Email</span></div><input class="input input-bordered w-full" required type="email" placeholder="Enter your email address" name="email" value=""></label>"#,
                r#"<label class="form-control w-full"><div class="label"><span class="label-text font-semibold">Password</span></div><input class="input input-bordered w-full" required type="password" placeholder="Enter your password" name="password" value=""></label>"#,
                r#"<div class="form-control grid place-content-center"><label class="label cursor-pointer gap-2"><span class="label-text">Remember me</span><input class="checkbox checkbox-primary" type="checkbox" name="remember_me" value="true"></label>"#,
                r#"<div class="card-actions justify-end"><button class="btn btn-primary btn-block btn-sm">Log In</button></div><div class="grid place-content-center text-center gap-2"><div><p class="text-center">Don't have an account?</p><a class="btn btn-sm btn-block btn-outline" href="/auth/register">Sign Up</a></div><a class="btn btn-sm btn-ghost" href="/auth/forgot-password">Forgot your password?</a></div>"#,
            ],
        );
    }

    #[tokio::test]
    async fn hide_signup_button_when_registration_disabled() {
        let mut app = App::new_test();
        app.config.server.is_no_signups = true;
        let server = build_server(Arc::new(app), ModelManager::new().await.unwrap());

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::OK);
        assert_not_in_html(
            res,
            vec![
                r#"<div class="card-actions justify-end"><button class="btn btn-primary btn-block btn-sm">Log In</button></div><div class="grid place-content-center text-center gap-2"><div><p class="text-center">Don't have an account?</p><a class="btn btn-sm btn-block btn-outline" href="/auth/register">Sign Up</a></div><a class="btn btn-sm btn-ghost" href="/auth/forgot-password">Forgot your password?</a></div>"#,
            ],
        );
    }

    #[tokio::test]
    async fn invalid_email() {
        let server = build_server(
            Arc::new(App::new_test()),
            ModelManager::new().await.unwrap(),
        );

        let res = server
            .post(BASE_URI)
            .form(&LoginForm {
                email: "hello".to_string(),
                password: "12345678".to_string(),
                remember_me: None,
            })
            .await;

        res.assert_status(StatusCode::BAD_REQUEST);
        res.assert_text("Field 'email': Invalid email address");
    }

    #[tokio::test]
    async fn invalid_password() {
        let server = build_server(
            Arc::new(App::new_test()),
            ModelManager::new().await.unwrap(),
        );

        let res = server
            .post(BASE_URI)
            .form(&LoginForm {
                email: "hello@example.com".to_string(),
                password: "123".to_string(),
                remember_me: None,
            })
            .await;

        res.assert_status(StatusCode::BAD_REQUEST);
        res.assert_text("Field 'password': Password must be at least 8 characters long");
    }

    #[tokio::test]
    async fn redirect_to_home_when_logged_in() {
        let server = build_server_logged_in(
            Arc::new(App::new_test()),
            ModelManager::new().await.unwrap(),
        );

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(
            res.header("Location"),
            "/",
            "Location should be set to login"
        );
    }

    #[tokio::test]
    async fn redirect_to_index_when_autologin() {
        let mut app = App::new_test();
        app.config.server.is_autologin = true;
        let server = build_server_logged_in(Arc::new(app), ModelManager::new().await.unwrap());

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(
            res.header("Location"),
            "/",
            "Location should be set to login"
        );
    }
}

#[cfg(test)]
mod register {
    use std::sync::Arc;

    use reqwest::StatusCode;

    use recipya::model::ModelManager;
    use recipya::web::KEY_HX_TRIGGER;
    use recipya::{app::App, models::payloads::RegisterForm};

    use crate::helpers::{build_server, build_server_logged_in};

    const BASE_URI: &str = "/auth/register";

    fn a_register_form() -> RegisterForm {
        RegisterForm {
            email: "test@example.com".to_string(),
            password: "12345678".to_string(),
            password_confirm: "12345678".to_string(),
        }
    }

    #[tokio::test]
    async fn success() {
        let app = Arc::new(App::new_test());
        let server = build_server(Arc::clone(&app), ModelManager::new().await.unwrap());
        let form = a_register_form();

        let res = server.post(BASE_URI).form(&form).await;

        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(res.header("HX-Redirect"), "/auth/login");
        assert!(
            app.repository
                .users()
                .await
                .iter()
                .any(|u| u.email == form.email),
            "Expected to find user"
        );
    }

    #[tokio::test]
    async fn cannot_register_when_no_signups() {
        let mut app = App::new_test();
        app.config.server.is_no_signups = true;
        let app = Arc::new(app);
        let server = build_server(Arc::clone(&app), ModelManager::new().await.unwrap());
        let original_num_users = app.repository.users().await.len();

        let res = server.post(BASE_URI).form(&a_register_form()).await;

        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(
            res.header("Location"),
            "/auth/login",
            "Location should be set to login"
        );
        assert_eq!(
            original_num_users,
            app.repository.users().await.len(),
            "No new user should be registered"
        );
    }

    #[tokio::test]
    async fn cannot_access_register_when_no_signups() {
        let mut app = App::new_test();
        app.config.server.is_no_signups = true;
        let server = build_server(Arc::new(app), ModelManager::new().await.unwrap());

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(
            res.header("Location"),
            "/auth/login",
            "Location should be set to login"
        );
    }

    #[tokio::test]
    async fn redirect_to_home_when_register_and_autologin() {
        let mut app = App::new_test();
        app.config.server.is_autologin = true;
        let app = Arc::new(app);
        let server = build_server(Arc::clone(&app), ModelManager::new().await.unwrap());
        let original_num_users = app.repository.users().await.len();

        let res = server.post(BASE_URI).form(&a_register_form()).await;

        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(res.header("Location"), "/");
        assert_eq!(
            original_num_users,
            app.repository.users().await.len(),
            "No new user should be registered"
        )
    }

    #[tokio::test]
    async fn redirect_to_home_when_logged_in() {
        let server = build_server_logged_in(
            Arc::new(App::new_test()),
            ModelManager::new().await.unwrap(),
        );

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(res.header("Location"), "/", "Location should point to home");
    }

    #[tokio::test]
    async fn redirect_to_home_when_autologin() {
        let mut app = App::new_test();
        app.config.server.is_autologin = true;
        let server = build_server(Arc::new(app), ModelManager::new().await.unwrap());

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(res.header("Location"), "/", "Location should point to home");
    }

    #[tokio::test]
    async fn fails_when_user_already_registered() {
        let app = Arc::new(App::new_test());
        let num_users = app.repository.users().await.len();
        let server = build_server(Arc::clone(&app), ModelManager::new().await.unwrap());

        let _res = server.post(BASE_URI).form(&a_register_form()).await;
        let res = server.post(BASE_URI).form(&a_register_form()).await;

        res.assert_status(StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(res.header(KEY_HX_TRIGGER), "{}");
        assert_eq!(
            num_users + 1,
            app.repository.users().await.len(),
            "Only one user should be registered"
        );
    }
}
*/
