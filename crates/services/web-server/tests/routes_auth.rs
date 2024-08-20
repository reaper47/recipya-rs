mod support;

use crate::support::{
    assert::assert_html,
    server::{build_server, build_server_logged_in},
};
use axum::http::StatusCode;
use lib_web::handlers::handlers_auth::LoginForm;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(test)]
pub mod test_db {
    use std::sync::atomic::AtomicU32;

    use diesel::{sql_query, Connection, PgConnection, RunQueryDsl};
    use lib_core::{
        ctx::Ctx,
        model::{
            store::Pool,
            user::{UserBmc, UserForCreate},
            ModelManager,
        },
    };

    static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);

    #[derive(Clone)]
    pub struct TestDb {
        default_db_url: String,
        name: String,
        pub pool: Pool,
    }

    impl TestDb {
        pub async fn new() -> Self {
            let default_db_url = &std::env::var("DATABASE_URL").unwrap();
            let mut conn = PgConnection::establish(default_db_url).unwrap();

            let name = format!(
                "test_db_{}_{}",
                std::process::id(),
                TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            );

            sql_query(format!("CREATE DATABASE {};", name))
                .execute(&mut conn)
                .unwrap();

            let mut url = url::Url::parse(default_db_url).unwrap();
            url.set_path(&name);

            let pool = lib_core::model::store::new_db_pool(url.as_str())
                .await
                .unwrap();

            UserBmc::create(
                &Ctx::root_ctx(),
                &ModelManager {
                    db: pool.clone(),
                    email: None,
                },
                UserForCreate {
                    email: "test@example.com".to_string(),
                    password_clear: "12345678".to_string(),
                },
            )
            .await
            .unwrap();

            Self {
                default_db_url: default_db_url.to_string(),
                name,
                pool,
            }
        }

        pub fn mm(&self) -> ModelManager {
            ModelManager {
                db: self.pool.clone(),
                email: None,
            }
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            /*if thread::panicking() {
                println!("TestDb leaking database {}", self.name);
                return;
            }*/
            let mut conn = PgConnection::establish(&self.default_db_url).unwrap();
            sql_query(format!(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
                self.name
            ))
            .execute(&mut conn)
            .unwrap();
            sql_query(format!("DROP DATABASE {}", self.name))
                .execute(&mut conn)
                .unwrap();
        }
    }
}

#[cfg(test)]
mod tests_login {
    use lib_auth::token::Token;
    use lib_utils::time::{OffsetDateTime, Rfc3339};
    use lib_web::utils::token::AUTH_TOKEN;
    use support::assert::assert_not_in_html;
    use test_db::TestDb;

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
    async fn test_get_login_page_ok() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server(db.mm()).await.unwrap();

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
        Ok(())
    }

    #[tokio::test]
    async fn test_post_login_ok() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server(db.mm()).await?;

        let res = server.post(BASE_URI).form(&a_login_form()).await;

        res.assert_status(StatusCode::SEE_OTHER);
        Ok(())
    }

    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_get_login_ok_hide_signup_button_when_registration_disabled() -> Result<()> {
        std::env::set_var("SERVICE_NO_SIGNUPS", "true");
        let db = TestDb::new().await;
        let server = build_server(db.mm()).await.unwrap();

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
        let server = build_server(db.mm()).await.unwrap();

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
        Ok(())
    }

    #[tokio::test]
    async fn test_post_err_invalid_password() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server(db.mm()).await.unwrap();

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
        Ok(())
    }

    #[tokio::test]
    async fn test_get_login_ok_redirect_to_home_when_logged_in() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server_logged_in(db.mm()).await?;

        let res = server.get(BASE_URI).await;

        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(
            res.header("Location"),
            "/",
            "Location should be set to login"
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore = "run manually because the lib_core::config() cannot reload"]
    async fn test_get_login_ok_redirect_to_index_when_autologin() -> Result<()> {
        std::env::set_var("SERVICE_AUTOLOGIN", "true");
        let db = TestDb::new().await;
        let server = build_server(db.mm()).await?;

        let res = server.get(BASE_URI).await;

        std::env::remove_var("SERVICE_AUTOLOGIN");
        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(
            res.header("Location"),
            "/",
            "Location should be set to login"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_post_login_ok_remember_me_checked() -> Result<()> {
        let db = TestDb::new().await;
        let server = build_server(db.mm()).await?;

        let res = server
            .post(BASE_URI)
            .form(&LoginForm {
                email: "test@example.com".to_string(),
                password: "12345678".to_string(),
                remember_me: Some(true),
            })
            .await;

        res.assert_status(StatusCode::SEE_OTHER);
        assert_eq!(
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
        let server = build_server(db.mm()).await?;
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
        assert_eq!(
            res.header("Location"),
            "/",
            "Location should be set to login"
        );
        Ok(())
    }
}

/*
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
*/
