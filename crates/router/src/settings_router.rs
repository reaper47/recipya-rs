use axum::routing::{get, post};
use axum::{Router, middleware};

use app::state::AppState;

use crate::handlers::settings::{
    export_data_handler, export_data_post_handler, set_default_theme_handler,
    set_nutrition_source_handler, set_selected_theme_handler, settings_handler,
};
use crate::middleware::mw_auth::{mw_only_admin, mw_refresh_token};

/// Defines the routes for endpoints related to the settings module.
pub fn settings_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(settings_handler))
        .route(
            "/export-data",
            get(export_data_handler).post(export_data_post_handler),
        )
        .route("/nutrition/source", post(set_nutrition_source_handler))
        .route(
            "/theme-default",
            post(set_default_theme_handler)
                .layer(middleware::from_fn_with_state(state.clone(), mw_only_admin)),
        )
        .route("/theme-selected", post(set_selected_theme_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_refresh_token,
        ))
}

#[cfg(test)]
mod tests {
    use axum::http::{Method, StatusCode};

    use testing::utils::{
        TestDb, assert_html, assert_must_be_logged_in, assert_not_in_html, assert_ws_message,
        build_server_logged_in, build_server_ws, build_server_ws_other_user, create_app_state,
        insert_other_user,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_settings {
        use super::*;

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
                &res,
                vec![
                    r#"<div id="settings-recipes""#,
                    r#"<div id="settings-data""#,
                    r#"<div id="settings-account""#,
                    r#"<div id="settings-about""#,
                ],
            );
            assert_not_in_html(
                &res,
                vec![
                    r#"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-admin">"#,
                    r#"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-server">"#,
                    r#"<a class="setting-tab" _="on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-connections">"#,
                    r#"<div id="settings-admin""#,
                    r#"<div id="settings-server""#,
                    r#"<div id="settings-connections""#,
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
                &res,
                vec![
                    r#"<table class="table table-xs"><thead><tr><th></th><th>Setting</th><th>Environment</th><th>Value</th></tr></thead><tbody>"#,
                    r"<td>Host</td><td>SMTP_HOST</td><td>smtp.gmail.com</td></tr>",
                    r"<tr><th></th><td>From</td><td>SMTP_FROM_EMAIL</td><td>demo@demo.com</td></tr>",
                    r"<tr><th></th><td>Username</td><td>SMTP_USERNAME</td><td>demo@demo.com</td></tr>",
                    r"<tr><th></th><td>Password</td><td>SMTP_PASSWORD</td><td>Not displayed</td></tr></tbody></table></div>",
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
                &res,
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
                    r#"<div id="settings-recipes" class="p-3 md:h-[50vh] overflow-y-auto">"#,
                    r#"<div id="settings-connections" class="p-3 overflow-y-auto max-h-96 hidden">"#,
                    r#"<div id="settings-server" class="hidden p-3 md:max-h-96">"#,
                    r#"<div id="settings-data" class="hidden p-3">"#,
                    r#"<div id="settings-admin" class="hidden p-3 md:max-h-96">"#,
                    r#"<div id="settings-account" class="hidden p-3 md:max-h-96">"#,
                    r#"<div id="settings-about" class="hidden p-3 md:p-0 md:pr-4 hidden">"#,
                ],
            );
            Ok(())
        }
    }

    mod tests_export {
        use models::{
            Recipe, recipe::structs::test_utils::a_complete_recipe_for_create, user::User,
        };

        use super::*;

        const BASE_URI: &str = "/settings/export-data";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await?;
            assert_must_be_logged_in(Method::POST, BASE_URI).await
        }

        mod tests_get {
            use super::*;

            #[tokio::test]
            async fn test_no_recipes_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let (server, mut ws_server) = build_server_ws(config).await?;

                let res = server.get(BASE_URI).await;

                res.assert_status_not_found();
                assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"No recipes found for export.","status":"alert-warning","title":"Attention"}}"# ).await;
                Ok(())
            }

            #[tokio::test]
            async fn test_with_recipes_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let server = build_server_logged_in(config.clone()).await?;
                let recipe1 = a_complete_recipe_for_create();
                let mut recipe2 = a_complete_recipe_for_create();
                recipe2.name = "Taco Tuesday".to_string();
                recipe2.category = Some("Meat".to_string());
                let state = create_app_state(config).await;
                let user_id = User::all(&state.mm).await?[0].id;
                let _ = Recipe::create(&state.mm, user_id, &recipe1).await?;
                let _ = Recipe::create(&state.mm, user_id, &recipe2).await?;

                let res = server.get(BASE_URI).await;

                res.assert_status_ok();
                assert_html(
                    &res,
                    vec![
                        r#"<form class="card bg-base-100 shadow-sm min-w-[50vw]" hx-post="/settings/export-data" hx-swap="none"><div class="card-body"><h3 class="mb-1 grid grid-flow-col"><label class="input input-sm"><svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor"><circle cx="11" cy="11" r="8"></circle><path d="m21 21-4.3-4.3"></path></g></svg><input type="search" placeholder="Search a recipe" _="on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"></label><select required name="type" class="[display:ruby] md:block select select-sm w-fit place-self-end"><optgroup label="Recipes"><option value="json" selected>JSON</option><option value="pdf">PDF</option></optgroup></select></h3>"#,
                        r#"<div class="overflow-auto h-[50vh]"><table class="table table-zebra table-sm"><thead><tr class="text-center"><th class="py-1 text-left"><label><input type="checkbox" name="recipe-ids" class="checkbox" value="0" _="on change set &lt;input.checkbox-recipe-id/&gt;'s checked to my checked"></label></th><th class="py-1 text-left">Name</th><th class="py-1">Favourite</th><th class="py-1">Rating</th><th class="py-1">Page</th><th class="py-1">Source</th></tr></thead><tbody id="search-result"#,
                        r#"<tr><td class="py-1"><label><input type="checkbox" name="recipe-ids" class="checkbox-recipe-id checkbox" value="1"></label></td><td class="py-1">Best Chinese Kale</td><td class="py-1 text-center"></td><td class="py-1 text-center">4/5</td><td class="py-1 text-center"><a class="link" href="//recipes/1" target="_blank">View</a></td><td class="py-1 text-center"><a class="link" href="https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/" target="_blank">Visit</a></td></tr>"#,
                        r#"<tr><td class="py-1"><label><input type="checkbox" name="recipe-ids" class="checkbox-recipe-id checkbox" value="2"></label></td><td class="py-1">Taco Tuesday</td><td class="py-1 text-center"></td><td class="py-1 text-center">4/5</td><td class="py-1 text-center"><a class="link" href="//recipes/2" target="_blank">View</a></td><td class="py-1 text-center"><a class="link" href="https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/" target="_blank">Visit</a></td></tr></tbody></table></div><div class="card-actions justify-end"><button type="button" class="btn btn-sm" onclick="this.closest('dialog').close()">Cancel</button><button type="submit" class="btn btn-sm"><svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" fill="black" viewBox="0 0 24 24" stroke="currentColor"><path d="M16 11v5H2v-5H0v5a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-5z"></path><path d="m9 14 5-6h-4V0H8v8H4z"></path></svg></button></div></div></form>"#,
                    ],
                );
                Ok(())
            }
        }
    }

    mod tests_nutrition {
        use models::{nutrition::NutritionDataSource, settings::UserSettingDetails, user::User};

        use crate::schemas::settings::NutritionSourcePayload;

        use super::*;

        const BASE_URI: &str = "/settings/nutrition/source";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, BASE_URI).await
        }

        #[tokio::test]
        async fn test_post_set_same_nutrition_source_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let users = User::all(&state.mm).await?;

            let res = server
                .post(BASE_URI)
                .form(&NutritionSourcePayload {
                    nutrition_source: NutritionDataSource::USDAFoodDataCentral.to_string(),
                })
                .await;

            res.assert_status(StatusCode::NO_CONTENT);
            let got = UserSettingDetails::get(&state.mm, users[0].id).await?;
            pretty_assertions::assert_eq!(
                got.nutrition_source,
                NutritionDataSource::USDAFoodDataCentral
            );
            Ok(())
        }
    }

    mod tests_themes {
        use models::{
            settings::{Theme, UserSettingDetails},
            user::User,
        };

        use crate::schemas::settings::ThemePayload;

        use super::*;

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
            let users = User::all(&state.mm).await?;

            let res = server
                .post(&format!("{BASE_URI}-default"))
                .form(&ThemePayload {
                    theme: Theme::Aqua.to_string(),
                })
                .await;

            res.assert_status(StatusCode::NO_CONTENT);
            let got1 = UserSettingDetails::get(&state.mm, users[0].id).await?;
            let got2 = UserSettingDetails::get(&state.mm, other_user.id).await?;
            pretty_assertions::assert_eq!(got1.default_theme, Theme::Aqua);
            pretty_assertions::assert_eq!(got2.default_theme, Theme::Aqua);
            Ok(())
        }

        #[tokio::test]
        async fn test_set_selected_theme_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let users = User::all(&state.mm).await?;

            let res = server
                .post(&format!("{BASE_URI}-selected"))
                .form(&ThemePayload {
                    theme: Theme::Winter.to_string(),
                })
                .await;

            res.assert_status(StatusCode::NO_CONTENT);
            let got = UserSettingDetails::get(&state.mm, users[0].id).await?;
            pretty_assertions::assert_eq!(got.selected_theme, Theme::Winter);
            Ok(())
        }
    }
}
