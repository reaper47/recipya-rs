use axum::routing::{delete, get, post};
use axum::{Router, middleware};
use serde::{Deserialize, Serialize};
use validator::Validate;

use app::state::AppState;

use crate::handlers::admin::{
    add_user_handler, delete_user_handler, update_user_form_handler, update_user_handler,
    user_row_handler,
};
use crate::middleware::mw_auth::{mw_ctx_require, mw_only_admin};

#[derive(Deserialize)]
pub struct UserRowParams {
    #[serde(rename = "row-index")]
    pub row_index: usize,
}

#[derive(Default, Validate, Deserialize, Serialize)]
pub struct UpdatePasswordForm {
    #[serde(rename = "new-password")]
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub new_password: String,
    #[serde(rename = "new-password-confirm")]
    #[validate(must_match(other = "new_password"))]
    pub new_password_confirm: String,
    #[serde(rename = "row-index")]
    pub row_index: usize,
}

/// Defines the routes for endpoints related to the administrator module.
pub(super) fn admin_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/user", post(add_user_handler))
        .route(
            "/user/{:id}",
            delete(delete_user_handler)
                .patch(update_user_handler)
                .get(update_user_form_handler),
        )
        .route("/user/{:id}/row", get(user_row_handler))
        .layer(middleware::from_fn(mw_only_admin))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            mw_ctx_require,
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Method;
    use models::user::User;
    use reqwest::StatusCode;
    use testing::utils::{
        TEST_USER_EMAIL, TestDb, assert_html, assert_must_be_logged_in, assert_ws_message,
        build_server_logged_in, build_server_ws, build_server_ws_other_user, create_app_state,
        insert_other_user,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_user {
        use super::*;
        use crate::auth_router::RegisterForm;

        const BASE_URI: &str = "/admin/user";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, BASE_URI).await
        }

        #[tokio::test]
        async fn test_must_be_admin_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, _) = build_server_ws_other_user(config.clone(), "demo@demo.com").await?;

            let res = server.post(BASE_URI).await;

            res.assert_status_forbidden();
            Ok(())
        }

        #[tokio::test]
        async fn test_add_user_password_invalid_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&RegisterForm {
                    email: TEST_USER_EMAIL.into(),
                    password: "atremendousloser".into(),
                    password_confirm: "abigbigloser".into(),
                })
                .await;

            res.assert_status(StatusCode::BAD_REQUEST);
            assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"Email is invalid or passwords do not match.","status":"alert-error","title":"Operation Failed"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_add_user_email_exists_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config).await?;

            let res = server
                .post(BASE_URI)
                .form(&RegisterForm {
                    email: TEST_USER_EMAIL.into(),
                    password: "atremendousloser".into(),
                    password_confirm: "atremendousloser".into(),
                })
                .await;

            res.assert_status(StatusCode::CONFLICT);
            assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"A user with this email exists.","status":"alert-error","title":"Operation Failed"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_add_user_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;

            let res = server
                .post(BASE_URI)
                .form(&RegisterForm {
                    email: "trumpisa@loser.com".into(),
                    password: "atremendousloser".into(),
                    password_confirm: "atremendousloser".into(),
                })
                .await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r##"<tr id="user-row-3"><th>3</th><td>trumpisa@loser.com</td><td></td><td class="grid grid-flow-col gap-2"><button type="button" class="btn btn-ghost btn-square btn-xs" hx-get="/admin/user/3" hx-target="#user-row-3" hx-swap="outerHTML" hx-vals="{&quot;row-index&quot;: &quot;3&quot;}"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button><button type="submit" class="btn btn-ghost btn-square btn-xs" hx-delete="/admin/user/3" hx-confirm="Are you sure you want to delete this user? This action is irreversible."><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button></td></tr>"##,
                    r##"<tr id="new-row-user"><th>4</th><td><input id="email" type="email" required placeholder="Email" class="input input-sm" name="email" autocomplete="off" hx-post="/admin/user" hx-target="#new-row-user" hx-swap="outerHTML" hx-include="#email,#password,#confirm-password" hx-trigger="keydown[key=='Enter']" _="on htmx:afterRequest call document.activeElement.blur()"></td><td><input id="password" type="password" required placeholder="Enter password" class="input input-sm mb-1" name="password" autocomplete="off" hx-post="/admin/user" hx-target="#new-row-user" hx-swap="outerHTML" hx-include="#email,#password,#confirm-password" hx-trigger="keydown[key=='Enter']" _="on htmx:afterRequest call document.activeElement.blur()"><input id="confirm-password" type="password" required placeholder="Retype password" class="input input-sm" name="password-confirm" autocomplete="off" hx-post="/admin/user" hx-target="#new-row-user" hx-swap="outerHTML" hx-include="#email,#password,#confirm-password" hx-trigger="keydown[key=='Enter']" _="on htmx:afterRequest call document.activeElement.blur()"></td><td><button class="btn btn-ghost btn-square btn-xs" hx-post="/admin/user" hx-target="#new-row-user" hx-swap="outerHTML" hx-include="#email,#password,#confirm-password"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" width="24px" height="24px" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line></svg></button></td></tr>"##,
                ],
            );
            let state = create_app_state(config.clone()).await;
            let user = User::get_user_by_email(&state.mm, "trumpisa@loser.com").await?;
            assert!(user.is_some());
            Ok(())
        }
    }

    mod tests_users {
        use super::*;

        fn base_uri(id: u32) -> String {
            format!("/admin/user/{id}")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::DELETE, &base_uri(1)).await?;
            assert_must_be_logged_in(Method::PATCH, &base_uri(1)).await?;
            assert_must_be_logged_in(Method::GET, &base_uri(2)).await
        }

        mod tests_delete {
            use super::*;

            #[tokio::test]
            async fn test_must_be_admin_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let (server, _) =
                    build_server_ws_other_user(config.clone(), "demo@demo.com").await?;

                let res = server.delete(&base_uri(2)).await;

                res.assert_status_forbidden();
                Ok(())
            }

            #[tokio::test]
            async fn test_cannot_delete_admin_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let (server, mut ws_server) = build_server_ws(config).await?;

                let res = server.delete(&base_uri(1)).await;

                res.assert_status_forbidden();
                assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"Cannot delete an admin.","status":"alert-error","title":"Operation Failed"}}"# ).await;
                Ok(())
            }

            #[tokio::test]
            async fn test_delete_user_not_exist_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let (server, mut ws_server) = build_server_ws(config).await?;

                let res = server.delete(&base_uri(5)).await;

                res.assert_status_internal_server_error();
                assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"Failed to delete user.","status":"alert-error","title":"Operation Failed"}}"# ).await;
                Ok(())
            }

            #[tokio::test]
            async fn test_delete_user_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let (server, mut ws_server) = build_server_ws(config.clone()).await?;
                let user2 = insert_other_user(config.clone(), "slava@ukraini.ua").await?;

                let res = server.delete(&base_uri(user2.id.try_into().unwrap())).await;

                res.assert_status_ok();
                assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"User deleted.","status":"alert-info","title":"Operation Successful"}}"# ).await;
                let state = create_app_state(config.clone()).await;
                let user = User::get_user_by_id(&state.mm, user2.id).await;
                if user.is_ok() && user.unwrap().is_some() {
                    panic!("User should be deleted");
                }
                Ok(())
            }
        }

        mod tests_patch {
            use super::*;

            #[tokio::test]
            async fn test_must_be_admin_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let (server, _) =
                    build_server_ws_other_user(config.clone(), "demo@demo.com").await?;

                let res = server.patch(&base_uri(2)).await;

                res.assert_status_forbidden();
                Ok(())
            }

            #[tokio::test]
            async fn test_invalid_payload_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let (server, mut ws_server) = build_server_ws(config).await?;

                let res = server
                    .patch(&base_uri(1))
                    .form(&UpdatePasswordForm {
                        new_password: "".into(),
                        new_password_confirm: "test".into(),
                        row_index: 0,
                    })
                    .await;

                res.assert_status_bad_request();
                assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"Payload cannot be empty.","status":"alert-error","title":"Operation Failed"}}"# ).await;
                Ok(())
            }

            #[tokio::test]
            async fn test_user_does_not_exist_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let (server, mut ws_server) = build_server_ws(config).await?;
                let password = "a big beautiful bill";

                let res = server
                    .patch(&base_uri(99))
                    .form(&UpdatePasswordForm {
                        new_password: password.into(),
                        new_password_confirm: password.into(),
                        row_index: 0,
                    })
                    .await;

                res.assert_status_internal_server_error();
                assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"Failed to update user password.","status":"alert-error","title":"Operation Failed"}}"# ).await;
                Ok(())
            }

            #[tokio::test]
            async fn test_update_success_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let (server, mut ws_server) = build_server_ws(config.clone()).await?;
                let user2 = insert_other_user(config, "slava@ukraini.ua").await?;
                let password = "a big beautiful bill";

                let res = server
                    .patch(&base_uri(user2.id.try_into().unwrap()))
                    .form(&UpdatePasswordForm {
                        new_password: password.into(),
                        new_password_confirm: password.into(),
                        row_index: 0,
                    })
                    .await;

                res.assert_status_ok();
                assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"User password updated.","status":"alert-info","title":"Operation Successful"}}"# ).await;
                assert_html(
                    res,
                    vec![
                        r##"<tr id="user-row-3"><th>0</th><td>slava@ukraini.ua</td><td></td><td class="grid grid-flow-col gap-2"><button type="button" class="btn btn-ghost btn-square btn-xs" hx-get="/admin/user/3" hx-target="#user-row-3" hx-swap="outerHTML" hx-vals="{&quot;row-index&quot;: &quot;0&quot;}"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button><button type="submit" class="btn btn-ghost btn-square btn-xs" hx-delete="/admin/user/3" hx-confirm="Are you sure you want to delete this user? This action is irreversible."><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button></td></tr>"##,
                    ],
                );
                Ok(())
            }
        }

        mod tests_get {
            use super::*;

            #[tokio::test]
            async fn test_must_be_admin_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let (server, _) =
                    build_server_ws_other_user(config.clone(), "demo@demo.com").await?;

                let res = server.get(&base_uri(3)).await;

                res.assert_status_forbidden();
                Ok(())
            }

            #[tokio::test]
            async fn test_user_does_not_exist_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let (server, mut ws_server) = build_server_ws(config).await?;

                let res = server.get(&format!("{}?row-index=1", base_uri(99))).await;

                res.assert_status_internal_server_error();
                assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"User not found.","status":"alert-error","title":"Operation Failed"}}"# ).await;
                Ok(())
            }

            #[tokio::test]
            async fn test_valid_ok() -> Result<()> {
                let (_test_db, config) = TestDb::new(None).await?;
                let server = build_server_logged_in(config).await?;

                let res = server.get(&format!("{}?row-index=1", base_uri(1))).await;

                res.assert_status_ok();
                assert_html(
                    res,
                    vec![
                        r##"<tr id="user-row-1"><th>2</th><td>test@test.com</td><td><input type="hidden" name="row-index" value="1"><input id="password" type="password" required placeholder="New password" class="input input-sm mb-1" name="new-password" autocomplete="off" hx-post="/admin/user" hx-target="#new-row-user" hx-swap="outerHTML" hx-include="#email,#password,#confirm-password" hx-trigger="keydown[key=='Enter']" _="on htmx:afterRequest call document.activeElement.blur()"><input id="confirm-password" type="password" required placeholder="Retype password" class="input input-sm" name="new-password-confirm" autocomplete="off" hx-post="/admin/user" hx-target="#new-row-user" hx-swap="outerHTML" hx-include="#email,#password,#confirm-password" hx-trigger="keydown[key=='Enter']" _="on htmx:afterRequest call document.activeElement.blur()"></td><td class="grid grid-flow-col gap-2"><button type="button" class="btn btn-ghost btn-square btn-xs hover:text-green-600" hx-patch="/admin/user/1" hx-target="#user-row-1" hx-swap="outerHTML" hx-include="closest tr"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"></svg></button><button type="button" class="btn btn-ghost btn-square btn-xs hover:text-red-600" hx-get="/admin/user/1/row" hx-target="#user-row-1" hx-swap="outerHTML" hx-vals="{&quot;row-index&quot;: &quot;1&quot;}"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"></path></svg></button></td></tr>"##,
                    ],
                );
                Ok(())
            }
        }
    }

    mod tests_user_row {
        use super::*;

        fn base_uri(id: u32, row_id: usize) -> String {
            format!("/admin/user/{id}/row?row-index={row_id}")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri(2, 1)).await
        }

        #[tokio::test]
        async fn test_must_be_admin_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, _) = build_server_ws_other_user(config.clone(), "demo@demo.com").await?;

            let res = server.get(&base_uri(2, 1)).await;

            res.assert_status_forbidden();
            Ok(())
        }

        #[tokio::test]
        async fn test_user_not_exist_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;

            let res = server.get(&base_uri(6, 3)).await;

            res.assert_status_internal_server_error();
            assert_ws_message(&mut ws_server,r#"{"showMessageHtmx":{"type":"toast","message":"User not found.","status":"alert-error","title":"Operation Failed"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_valid_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(&base_uri(1, 3)).await;

            res.assert_status_ok();
            assert_html(
                res,
                vec![
                    r##"<tr id="user-row-1"><th>3</th><td>test@test.com</td><td></td><td class="grid grid-flow-col gap-2"><button type="button" class="btn btn-ghost btn-square btn-xs" hx-get="/admin/user/1" hx-target="#user-row-1" hx-swap="outerHTML" hx-vals="{&quot;row-index&quot;: &quot;3&quot;}"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button><button type="submit" class="btn btn-ghost btn-square btn-xs" hx-delete="/admin/user/1" hx-confirm="Are you sure you want to delete this user? This action is irreversible."><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button></td></tr>"##,
                ],
            );
            Ok(())
        }
    }
}
