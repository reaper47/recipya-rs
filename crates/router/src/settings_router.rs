use axum::routing::{get, post};
use axum::{Router, middleware};
use serde::{Deserialize, Serialize};

use app::state::AppState;

use crate::handlers::settings::{
    set_default_theme_handler, set_selected_theme_handler, settings_handler,
};
use crate::middleware::mw_auth;
use crate::middleware::mw_auth::mw_only_admin;

/// Represents the payload for setting themes.
#[derive(Deserialize, Serialize)]
pub struct ThemePayload {
    pub theme: String,
}

/// Defines the routes for endpoints related to the settings module.
pub(super) fn settings_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(settings_handler))
        .route(
            "/theme-default",
            post(set_default_theme_handler).layer(middleware::from_fn(mw_only_admin)),
        )
        .route("/theme-selected", post(set_selected_theme_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_auth::mw_ctx_require,
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use testing::utils::{
        TestDb, assert_html, assert_must_be_logged_in, assert_not_in_html, build_server_logged_in,
        build_server_ws, build_server_ws_other_user, create_app_state, insert_other_user,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_settings {
        use super::*;
        use axum::http::Method;

        const BASE_URI: &str = "/settings";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_server_and_conn_tabs_not_displayed_when_not_admin_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, _) = build_server_ws_other_user(config.clone(), "demo@demo.com").await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                res.clone(),
                vec![
                    r#"<div id="settings-recipes""#,
                    r#"<div id="settings-data""#,
                    r#"<div id="settings-account""#,
                    r#"<div id="settings-about""#,
                ],
            );
            assert_not_in_html(
                res,
                vec![
                    r##"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-admin">"##,
                    r##"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-server">"##,
                    r##"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-connections">"##,
                    r##"<div id="settings-admin""##,
                    r##"<div id="settings-server""##,
                    r##"<div id="settings-connections""##,
                ],
            )?;
            Ok(())
        }

        #[tokio::test]
        async fn test_demo_sees_fake_connection_data_ok() -> Result<()> {
            let (_test_db, mut config) = TestDb::new(None).await?;
            config.is_demo = true;
            let server = build_server_logged_in(config.clone()).await?;
            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r##"<input name="email.from" type="email" placeholder="SMTP email" value="demo@demo.com" autocomplete="off" class="input input-sm">"##,
                    r##"<input name="email.host" type="text" placeholder="smtp.gmail.com" value="smtp.gmail.com" autocomplete="off" class="input input-bordered input-sm">"##,
                    r##"<input name="email.username" type="email" placeholder="email@example.com" value="demo@demo.com" autocomplete="off" class="input input-bordered input-sm">"##,
                    r##"<input name="email.password" type="password" placeholder="SMTP password or app password" value="demo-password" autocomplete="off" class="input input-bordered input-sm">"##,
                    // TODO: Add Azure OCR key and endpoint
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_app_update_available_ok() -> Result<()> {
            // TODO: Write this test once the update available functionality is implemented.
            /*let (_test_db, mut config): (_, config::Config) = TestDb::new(None).await?;
            config.is_demo = true;
            let server = build_server_logged_in(config.clone()).await?;
            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(res, vec![
                r##"<div><p class="font-semibold">Recipya Version</p><p class="text-sm mt-2">v1.3.0 (update available)</p><p class="text-xs">Last checked: 0001-01-01<br>Last updated: 0001-01-01<br><br>Read the <a class="link" href="https://recipya.musicavis.ca/about/changelog/v1.3.0" target="_blank">release notes</a></p></div><div class="flex flex-row self-start"><button class="btn btn-sm" hx-get="/update" hx-swap="none" hx-indicator="#fullscreen-loader">Update</button></div></div>"##,
            ]);*/
            Ok(())
        }

        #[tokio::test]
        async fn test_display_settings_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                res.clone(),
                vec![
                    r#"<div class="flex flex-col menu-sm sm:flex-row sm:menu-md">"#,
                    r#"<ul class="menu menu-horizontal flex-nowrap overflow-x-auto w-full sm:overflow-x-clip sm:w-48 sm:menu-vertical" _="on click remove .menu-active from .setting-tab then add .menu-active to closest <a/> to event.target">"#,
                    r#"<a class="setting-tab menu-active" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-recipes">"#,
                    r#"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-connections">"#,
                    r#"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-data">"#,
                    r#"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-server">"#,
                    r#"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-admin">"#,
                    r#"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-account">"#,
                    r#"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-about">"#,
                    r#"<div id="settings-blocks" class="w-full md:h-[50vh] md:max-h-[50vh]" style="padding-right: 1rem">"#,
                    r#"<div id="settings-recipes" class="p-3 md:max-h-96 overflow-y-auto">"#,
                    r#"<div id="settings-connections" class="p-3 overflow-y-auto max-h-96 hidden">"#,
                    r#"<div id="settings-server" class="hidden p-3 md:max-h-96">"#,
                    r#"<div id="settings-data" class="hidden p-3">"#,
                    r#"<div id="settings-admin" class="hidden p-3 md:max-h-96">"#,
                    r##"<div id="settings-account" class="hidden p-3 md:max-h-96">"##,
                    r##"<div id="settings-about" class="hidden p-3 md:p-0 md:pr-4 hidden">"##,
                ],
            );
            Ok(())
        }
    }

    mod tests_themes {
        use super::*;
        use axum::http::{Method, StatusCode};
        use models::settings::{Theme, UserSettingDetails};

        const BASE_URI: &str = "/settings/theme";

        #[tokio::test]
        async fn test_post_change_password_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, &format!("{BASE_URI}-default")).await?;
            assert_must_be_logged_in(Method::POST, &format!("{BASE_URI}-selected")).await?;
            Ok(())
        }

        #[tokio::test]
        async fn test_set_default_theme_must_be_admin_err() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, _) = build_server_ws_other_user(config.clone(), "demo@demo.com").await?;

            let res = server
                .post(&format!("{BASE_URI}-default"))
                .form(&ThemePayload {
                    theme: Theme::Aqua.to_string(),
                })
                .await;

            res.assert_status(StatusCode::FORBIDDEN);
            Ok(())
        }

        #[tokio::test]
        async fn test_set_default_theme_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config.clone()).await;
            let other_user = insert_other_user(config, "slava@ukraini.ua").await?;

            let res = server
                .post(&format!("{BASE_URI}-default"))
                .form(&ThemePayload {
                    theme: Theme::Aqua.to_string(),
                })
                .await;

            res.assert_status(StatusCode::NO_CONTENT);
            let got1 = UserSettingDetails::get_settings(&state.mm, 1).await?;
            let got2 = UserSettingDetails::get_settings(&state.mm, other_user.id).await?;
            pretty_assertions::assert_eq!(got1.default_theme, Theme::Aqua);
            pretty_assertions::assert_eq!(got2.default_theme, Theme::Aqua);
            Ok(())
        }

        #[tokio::test]
        async fn test_set_selected_theme_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;

            let res = server
                .post(&format!("{BASE_URI}-selected"))
                .form(&ThemePayload {
                    theme: Theme::Winter.to_string(),
                })
                .await;

            res.assert_status(StatusCode::NO_CONTENT);
            let got = UserSettingDetails::get_settings(&state.mm, 1).await?;
            pretty_assertions::assert_eq!(got.selected_theme, Theme::Winter);
            Ok(())
        }
    }
}
