use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};

use app::state::AppState;

use crate::{
    handlers::shopping::{
        shopping_list_item_post_handler, shopping_lists_handler, shopping_lists_post_handler,
    },
    middleware::mw_auth::mw_refresh_token,
};

/// Defines the shopping routes for the application.
#[allow(clippy::literal_string_with_formatting_args)]
pub fn shopping_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/lists",
            get(shopping_lists_handler).post(shopping_lists_post_handler),
        )
        .route("/lists/{:id}/item", post(shopping_list_item_post_handler))
        .layer(from_fn_with_state(state.clone(), mw_refresh_token))
}

#[cfg(test)]
mod tests {
    use reqwest::Method;

    use models::{shopping::ShoppingList, user::User};
    use test_db::TestDb;
    use test_fixtures::{assert_html, assert_ws_message};
    use test_utils::{
        assert_must_be_logged_in, build_server_logged_in, build_server_ws, create_app_state,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_lists {
        use axum_htmx::HX_PROMPT;

        use super::*;

        const BASE_URI: &str = "/shopping/lists";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_user_has_no_lists_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r##"<div id="shopping-list-container"><div class="grid grid-flow-col gap-2 place-items-center"><p class="text-center font-semibold text-lg underline">Shopping Lists</p><button class="btn btn-xs btn-square btn-ghost" hx-post="/shopping/list" hx-prompt="Name of the new shopping list:" hx-target="#shopping-list-menu" hx-swap="beforeend"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" width="24px" height="24px" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line></svg></button></div>"##,
                    r#"<ul id="shopping-list-menu" class="menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0 h-full"></ul></div></aside>"#,
                    r#"<div class="order-1 divider my-0 md:order-2 md:divider-horizontal md:mx-0"></div>"#,
                    r#"<div class="order-0 flex-1 overflow-y-auto min-h-0 md:order-3 max-h-[94vh]"><div id="shopping-list-view-pane"><p class="p-4">← Create your first shopping list to get started.</p></div>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_new_list_title_empty_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;

            let res = server
                .post(BASE_URI)
                .add_header(HX_PROMPT, String::new())
                .await;

            res.assert_status_bad_request();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"List title must not be empty.","status":"alert-warning","title":"Attention"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_new_list_title_valid_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;

            let res = server.post(BASE_URI).add_header(HX_PROMPT, "Costco").await;

            res.assert_status_ok();
            let lists = ShoppingList::get_all(&state.mm, user_id).await?;
            assert_eq!(lists.len(), 1);
            assert_eq!(lists[0].name, "Costco".to_string());
            assert_html(&res, &["Fuck"]);
            Ok(())
        }

        #[tokio::test]
        async fn test_new_list_title_duplicate_ok() -> Result<()> {
            todo!()
        }

        #[tokio::test]
        async fn test_user_has_lists_ok() -> Result<()> {
            todo!()
        }
    }

    mod tests_list_items {
        use reqwest::StatusCode;
        use uuid::Uuid;

        use crate::schemas::shopping::ListItemPayload;

        use super::*;

        fn base_uri(list_id: Uuid) -> String {
            format!("/shopping/lists/{}/item", list_id)
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, &base_uri(Uuid::new_v4())).await
        }

        #[tokio::test]
        async fn test_item_empty_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;

            let res = server
                .post(&base_uri(list_id))
                .form(&ListItemPayload {
                    item: String::new(),
                    quantity: None,
                    ..Default::default()
                })
                .await;

            res.assert_status_bad_request();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Item name must not be empty.","status":"alert-warning","title":"Attention"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_item_duplicate_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let _ = server
                .post(&base_uri(list_id))
                .form(&ListItemPayload {
                    item: "Spaghetti".into(),
                    quantity: None,
                    ..Default::default()
                })
                .await;

            let res = server
                .post(&base_uri(list_id))
                .form(&ListItemPayload {
                    item: "Spaghetti".into(),
                    quantity: Some("500g".into()),
                    ..Default::default()
                })
                .await;

            res.assert_status(StatusCode::CONFLICT);
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Item already exists in the label.","status":"alert-warning","title":"Attention"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_item_valid_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;

            let res = server
                .post(&base_uri(list_id))
                .form(&ListItemPayload {
                    item: "Spaghetti".into(),
                    quantity: Some("500g".into()),
                    ..Default::default()
                })
                .await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<li class="list-row grid grid-cols-[1fr_auto]"><div class="grid gap-1 min-w-0"><label class="label"><input class="checkbox" type="checkbox">Spaghetti (500g)</label></div><div class="flex gap-1"><button class="btn join-item btn-sm"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button><button class="btn join-item btn-sm"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button><button class="btn join-item btn-sm cursor-grab h-full"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3 7.5 7.5 3m0 0L12 7.5M7.5 3v13.5m13.5 0L16.5 21m0 0L12 16.5m4.5 4.5V7.5"></svg></button></div></li>"#,
                ],
            );
            Ok(())
        }
    }
}
