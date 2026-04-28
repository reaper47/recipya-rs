use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
};

use app::state::AppState;

use crate::{
    handlers::shopping::{
        shopping_list_delete_handler, shopping_list_edit_handler, shopping_list_handler,
        shopping_list_item_delete_handler, shopping_list_item_edit_handler,
        shopping_list_item_post_handler, shopping_list_item_put_handler,
        shopping_list_label_handler, shopping_list_label_put_handler, shopping_list_put_handler,
        shopping_lists_handler, shopping_lists_post_handler,
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
        .route(
            "/lists/{:list_id}",
            get(shopping_list_handler)
                .delete(shopping_list_delete_handler)
                .put(shopping_list_put_handler),
        )
        .route("/lists/{:list_id}/edit", get(shopping_list_edit_handler))
        .route(
            "/lists/{:list_id}/labels/{:label_id}/edit",
            get(shopping_list_label_handler),
        )
        .route(
            "/lists/{:list_id}/labels/{:label_id}",
            put(shopping_list_label_put_handler),
        )
        .route(
            "/lists/{:list_id}/items",
            post(shopping_list_item_post_handler),
        )
        .route(
            "/lists/{:list_id}/items/{:item_id}",
            delete(shopping_list_item_delete_handler).put(shopping_list_item_put_handler),
        )
        .route(
            "/lists/{:list_id}/items/{:item_id}/edit",
            get(shopping_list_item_edit_handler),
        )
        .layer(from_fn_with_state(state.clone(), mw_refresh_token))
}

#[cfg(test)]
mod tests {
    use app::state::AppState;
    use axum_test::TestServer;
    use reqwest::Method;
    use uuid::Uuid;

    use models::{
        shopping::{ShoppingList, ShoppingListItemForCreate},
        user::User,
    };
    use test_db::TestDb;
    use test_fixtures::{assert_html, assert_ws_message};
    use test_utils::{
        assert_must_be_logged_in, build_server_logged_in, build_server_ws, create_app_state,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    struct Components {
        #[allow(unused)]
        test_db: TestDb,
        state: AppState,
        user_id: Uuid,
        list_id: Uuid,
        server: TestServer,
    }

    impl Components {
        async fn setup_with_one_list() -> Result<Self> {
            let (test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;

            Ok(Self {
                test_db,
                state,
                user_id,
                list_id,
                server,
            })
        }
    }

    fn an_item_c() -> ShoppingListItemForCreate {
        ShoppingListItemForCreate {
            ingredient: "Spaghetti".into(),
            quantity: None,
            label: None,
            recipe_id: None,
        }
    }

    fn a_meat_item() -> ShoppingListItemForCreate {
        ShoppingListItemForCreate::new(Some("1 cup"), "chicken", Some("Meat".into()), None)
    }

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
                    r##"<div id="shopping-list-container"><div class="grid grid-flow-col gap-2 place-items-center"><p class="text-center font-semibold text-lg underline">Shopping Lists</p><button class="btn btn-xs btn-square btn-ghost" hx-post="/shopping/lists" hx-prompt="Name of the new shopping list:" hx-target="#shopping-lists" hx-swap="afterbegin" hx-on:htmx:after-request="Array.from(document.getElementById('shopping-lists').children).forEach((item) =&gt; item.classList.remove('bg-base-300')); document.getElementById('shopping-lists').firstElementChild.classList.add('bg-base-300')"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" width="24px" height="24px" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line></svg></button></div>"##,
                    r#"<ul id="shopping-lists" class="menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0 h-full"></ul></div></aside>"#,
                    r#"<div class="order-1 divider my-0 md:order-2 md:divider-horizontal md:mx-0"></div>"#,
                    r#"<div class="order-0 flex-1 overflow-y-auto min-h-0 md:order-3 max-h-[94vh]"><div id="shopping-list-view-pane" class="p-4 text-center grid"><div class="grid"><p class="text-left">← Create your first shopping list to get started.</p></div>"#,
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
            let list_id = lists[0].id;
            assert_html(
                &res,
                &[
                    &format!(
                        r##"<li id="shopping-list-sidebar-{list_id}" class="bg-base-300" hx-get="/shopping/lists/{list_id}" hx-target="#shopping-list-view-pane" hx-push-url="false" hx-trigger="mousedown" hx-on:mousedown="document.querySelectorAll('#shopping-lists li').forEach((el) =&gt; el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-shopping-list-id').value = '{list_id}';"><div class="flex justify-between items-center gap-2 w-full"><div class="min-w-0"><p class="font-bold text-sm">Costco</p></div><div class="flex flex-col items-end gap-1 shrink-0"><div class="badge badge-xs badge-primary">0</div></div></div></li><div id="shopping-list-view-pane" hx-swap-oob="innerHTML"><div><div class="join border border-gray-700 mb-2 w-fit"><button class="btn join-item">Toggle Recipes</button><button class="btn join-item">View (default)</button><button class="btn join-item">Copy</button><button class="btn join-item">Share</button><button class="btn join-item">Export</button><button class="btn join-item">Upload to app</button><button class="btn join-item" hx-delete="/shopping/lists/{list_id}" hx-confirm="Are you sure you wish to delete this list?">Delete</button></div><h1 class="text-2xl font-bold underline p-2">Costco<span class="ml-2"><button class="btn join-item btn-square btn-sm" hx-target="closest h1" hx-swap="outerHTML" hx-get="/shopping/lists/{list_id}/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></span></h1><div class="grid"><div class="min-w-[33rem] place-self-center">"##
                    ),
                    &format!(
                        r#"<details open><summary class="text-left cursor-default">No label<button class="btn join-item btn-square btn-sm ml-2 mb-1" hx-target="closest summary" hx-swap="outerHTML" hx-get="/shopping/lists/{list_id}/labels/1/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></summary><ol class="list bg-base-100 rounded-box shadow-md"><li class="list-row grid grid-cols-[1fr_auto]"><div class="grid gap-1 min-w-0"><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" height="20" width="22.5" fill="currentColor"><path d="M453.1 27.3L440.9 39.4C409.7 70.6 409.7 121.3 440.9 152.5C456.5 168.1 472.1 183.7 487.8 199.4C519 230.6 569.7 230.6 600.9 199.4L613 187.3C619.2 181.1 619.2 170.9 613 164.7L600.9 152.6C569.7 121.4 519 121.4 487.8 152.6C519 121.4 519 70.7 487.8 39.5L475.7 27.3C469.5 21.1 459.3 21.1 453.1 27.3zM331.6 160C286.4 160 244.5 180.4 216.6 214.3L273.3 271C282.7 280.4 282.7 295.6 273.3 304.9C263.9 314.2 248.7 314.3 239.4 304.9L191.6 257.2L67.2 530.8C61.7 542.9 64.3 557.2 73.7 566.7C83.1 576.2 97.4 578.7 109.6 573.2L251.2 508.8L207.4 465C198 455.6 198 440.4 207.4 431.1C216.8 421.8 232 421.7 241.3 431.1L297.8 487.6L393.1 444.3C446.2 420.2 480.3 367.2 480.3 308.8C480.3 226.6 413.7 160 331.5 160z"></svg><input required type="text" placeholder="Surloin steak" name="item" value=""></label><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v17.25m0 0c-1.472 0-2.882.265-4.185.75M12 20.25c1.472 0 2.882.265 4.185.75M18.75 4.97A48.416 48.416 0 0 0 12 4.5c-2.291 0-4.545.16-6.75.47m13.5 0c1.01.143 2.01.317 3 .52m-3-.52 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.988 5.988 0 0 1-2.031.352 5.988 5.988 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L18.75 4.971Zm-16.5.52c.99-.203 1.99-.377 3-.52m0 0 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.989 5.989 0 0 1-2.031.352 5.989 5.989 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L5.25 4.971Z"></svg><input type="text" placeholder="500g (optional)" name="quantity" value=""></label></div><div class="grid grid-flow-col gap-1 place-self-end"><div class="grid grid-col gap-2 w-12"><button class="btn join-item btn-sm" hx-post="/shopping/lists/{list_id}/items" hx-include="closest li" hx-target="closest li" hx-swap="beforebegin"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15"></svg></button></div></div></li></ol></details>"#
                    ),
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_user_has_lists_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let _ = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    &format!(
                        r#"<div id="shopping-lists-index" class="flex flex-col-reverse md:flex-row h-full"><aside class="relative max-h-full text-center pt-1 pl-2"><input id="selected-shopping-list-id" type="hidden" name="selected" value="{list_id}"><div id="shopping-list-container">"#
                    ),
                    r##"<div class="grid grid-flow-col gap-2 place-items-center"><p class="text-center font-semibold text-lg underline">Shopping Lists</p><button class="btn btn-xs btn-square btn-ghost" hx-post="/shopping/lists" hx-prompt="Name of the new shopping list:" hx-target="#shopping-lists" hx-swap="afterbegin" hx-on:htmx:after-request="Array.from(document.getElementById('shopping-lists').children).forEach((item) =&gt; item.classList.remove('bg-base-300')); document.getElementById('shopping-lists').firstElementChild.classList.add('bg-base-300')"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" width="24px" height="24px" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line></svg></button></div>"##,
                    &format!(
                        r##"<ul id="shopping-lists" class="menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0 h-full"><li id="shopping-list-sidebar-{list_id}" class="bg-base-300" hx-get="/shopping/lists/{list_id}" hx-target="#shopping-list-view-pane" hx-push-url="false" hx-trigger="mousedown" hx-on:mousedown="document.querySelectorAll('#shopping-lists li').forEach((el) =&gt; el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-shopping-list-id').value = '{list_id}';"><div class="flex justify-between items-center gap-2 w-full"><div class="min-w-0"><p class="font-bold text-sm">Test</p></div><div class="flex flex-col items-end gap-1 shrink-0"><div class="badge badge-xs badge-primary">1</div></div></div></li></ul>"##
                    ),
                    r#"<div class="order-1 divider my-0 md:order-2 md:divider-horizontal md:mx-0"></div><div class="order-0 flex-1 overflow-y-auto min-h-0 md:order-3 max-h-[94vh]"><div id="shopping-list-view-pane" class="p-4 text-center grid">"#,
                    &format!(
                        r#"<div class="grid"><div><div class="join border border-gray-700 mb-2 w-fit"><button class="btn join-item">Toggle Recipes</button><button class="btn join-item">View (default)</button><button class="btn join-item">Copy</button><button class="btn join-item">Share</button><button class="btn join-item">Export</button><button class="btn join-item">Upload to app</button><button class="btn join-item" hx-delete="/shopping/lists/{list_id}" hx-confirm="Are you sure you wish to delete this list?">Delete</button></div></div>"#
                    ),
                    &format!(
                        r#"<h1 class="text-2xl font-bold underline p-2">Test<span class="ml-2"><button class="btn join-item btn-square btn-sm" hx-target="closest h1" hx-swap="outerHTML" hx-get="/shopping/lists/{list_id}/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></span></h1><div class="min-w-[33rem] place-self-center"><details open><summary class="text-left cursor-default">No label<button class="btn join-item btn-square btn-sm ml-2 mb-1" hx-target="closest summary" hx-swap="outerHTML" hx-get="/shopping/lists/{list_id}/labels/1/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></summary><ol class="list bg-base-100 rounded-box shadow-md"><li class="list-row grid grid-cols-[1fr_auto]"><div class="grid gap-1 min-w-0"><label class="label text-base-content"><input class="checkbox" type="checkbox">Spaghetti</label></div><div class="flex gap-1"><button class="btn join-item btn-square btn-sm" hx-target="closest li" hx-swap="outerHTML" hx-get="/shopping/lists/{list_id}/items/1/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button><button class="btn join-item btn-square btn-sm" hx-target="closest li" hx-swap="delete" hx-delete="/shopping/lists/{list_id}/items/1"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button><button class="btn join-item btn-square btn-sm cursor-grab h-full"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3 7.5 7.5 3m0 0L12 7.5M7.5 3v13.5m13.5 0L16.5 21m0 0L12 16.5m4.5 4.5V7.5"></svg></button></div></li><li class="list-row grid grid-cols-[1fr_auto]"><div class="grid gap-1 min-w-0"><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" height="20" width="22.5" fill="currentColor"><path d="M453.1 27.3L440.9 39.4C409.7 70.6 409.7 121.3 440.9 152.5C456.5 168.1 472.1 183.7 487.8 199.4C519 230.6 569.7 230.6 600.9 199.4L613 187.3C619.2 181.1 619.2 170.9 613 164.7L600.9 152.6C569.7 121.4 519 121.4 487.8 152.6C519 121.4 519 70.7 487.8 39.5L475.7 27.3C469.5 21.1 459.3 21.1 453.1 27.3zM331.6 160C286.4 160 244.5 180.4 216.6 214.3L273.3 271C282.7 280.4 282.7 295.6 273.3 304.9C263.9 314.2 248.7 314.3 239.4 304.9L191.6 257.2L67.2 530.8C61.7 542.9 64.3 557.2 73.7 566.7C83.1 576.2 97.4 578.7 109.6 573.2L251.2 508.8L207.4 465C198 455.6 198 440.4 207.4 431.1C216.8 421.8 232 421.7 241.3 431.1L297.8 487.6L393.1 444.3C446.2 420.2 480.3 367.2 480.3 308.8C480.3 226.6 413.7 160 331.5 160z"></svg><input required type="text" placeholder="Surloin steak" name="item" value=""></label><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v17.25m0 0c-1.472 0-2.882.265-4.185.75M12 20.25c1.472 0 2.882.265 4.185.75M18.75 4.97A48.416 48.416 0 0 0 12 4.5c-2.291 0-4.545.16-6.75.47m13.5 0c1.01.143 2.01.317 3 .52m-3-.52 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.988 5.988 0 0 1-2.031.352 5.988 5.988 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L18.75 4.971Zm-16.5.52c.99-.203 1.99-.377 3-.52m0 0 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.989 5.989 0 0 1-2.031.352 5.989 5.989 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L5.25 4.971Z"></svg><input type="text" placeholder="500g (optional)" name="quantity" value=""></label><input type="hidden" name="label" value="No label"></div><div class="grid grid-flow-col gap-1 place-self-end"><div class="grid grid-col gap-2 w-12"><button class="btn join-item btn-sm" hx-post="/shopping/lists/{list_id}/items" hx-include="closest li" hx-target="closest li" hx-swap="beforebegin"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15"></svg></button></div></div></li></ol></details>"#
                    ),
                ],
            );
            Ok(())
        }
    }

    mod tests_list {
        use axum_htmx::HX_REDIRECT;

        use crate::schemas::shopping::ListPayload;

        use super::*;

        fn base_uri(list_id: Uuid) -> String {
            format!("/shopping/lists/{list_id}")
        }

        fn base_uri_edit(list_id: Uuid) -> String {
            format!("{}/edit", base_uri(list_id))
        }

        impl ListPayload {
            /// Creates a new `ListPayload` with the given name.
            pub fn new<T: AsRef<str>>(name: T) -> Self {
                Self {
                    name: name.as_ref().to_string(),
                }
            }
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri_edit(Uuid::new_v4())).await?;
            assert_must_be_logged_in(Method::GET, &base_uri(Uuid::new_v4())).await?;
            assert_must_be_logged_in(Method::PUT, &base_uri(Uuid::new_v4())).await?;
            assert_must_be_logged_in(Method::DELETE, &base_uri(Uuid::new_v4())).await
        }

        #[tokio::test]
        async fn test_get_edit_title_ok() -> Result<()> {
            let c = Components::setup_with_one_list().await?;

            let res = c.server.get(&base_uri_edit(c.list_id)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[&format!(
                    r#"<form hx-put="/shopping/lists/{}" hx-swap="outerHTML"><h1 class="text-2xl font-bold underline p-2"><input required type="text" name="name" class="input input-lg text-center mr-1" value="Test" span class="ml-2"><button class="btn join-item btn-square btn-lg"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button></input></h1></form>"#,
                    c.list_id
                )],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_put_title_empty_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;

            let res = server
                .put(&base_uri(list_id))
                .form(&ListPayload::default())
                .await;

            res.assert_status_bad_request();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Title cannot be empty.","status":"alert-error","title":"Operation Failed"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_put_title_new_ok() -> Result<()> {
            let c = Components::setup_with_one_list().await?;

            let res = c
                .server
                .put(&base_uri(c.list_id))
                .form(&ListPayload::new("New Title"))
                .await;

            res.assert_status_ok();
            let list = ShoppingList::get(&c.state.mm, c.list_id, c.user_id).await?;
            let list_id = list.id;
            assert_eq!(list.name, "New Title");
            assert_html(
                &res,
                &[
                    &format!(
                        r#"<h1 class="text-2xl font-bold underline p-2">New Title<span class="ml-2"><button class="btn join-item btn-square btn-sm" hx-target="closest h1" hx-swap="outerHTML" hx-get="/shopping/lists/{list_id}/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></span></h1>"#,
                    ),
                    &format!(
                        r##"<li id="shopping-list-sidebar-{list_id}" class="bg-base-300" hx-swap-oob="true" hx-get="/shopping/lists/{list_id}" hx-target="#shopping-list-view-pane" hx-push-url="false" hx-trigger="mousedown" hx-on:mousedown="document.querySelectorAll('#shopping-lists li').forEach((el) =&gt; el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-shopping-list-id').value = '{list_id}'"><div class="flex justify-between items-center gap-2 w-full"><div class="min-w-0"><p class="font-bold text-sm">New Title</p></div><div class="flex flex-col items-end gap-1 shrink-0"><div class="badge badge-xs badge-primary">0</div></div></div></li>"##
                    ),
                ],
            );
            Ok(())
        }

        #[tokio::test]
        #[tracing_test::traced_test]
        async fn test_put_title_same_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test2", user_id).await?;
            let _ = ShoppingList::create(&state.mm, "Test", user_id).await?;

            let res = server
                .put(&base_uri(list_id))
                .form(&ListPayload::new("Test"))
                .await;

            res.assert_status_bad_request();
            let list = ShoppingList::get(&state.mm, list_id, user_id).await?;
            assert_eq!(list.name, "Test2");
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Title already exists.","status":"alert-warning","title":"Attention"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_list_exists_ok() -> Result<()> {
            let c = Components::setup_with_one_list().await?;

            let res = c.server.delete(&base_uri(c.list_id)).await;

            res.assert_status_see_other();
            let lists = ShoppingList::get_all(&c.state.mm, c.user_id).await?;
            assert!(lists.is_empty());
            res.assert_header(HX_REDIRECT, "/shopping/lists");
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_list_not_exists_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;

            let res = server.delete(&base_uri(Uuid::new_v4())).await;

            res.assert_status_see_other();
            let lists = ShoppingList::get_all(&state.mm, user_id).await?;
            assert!(lists.is_empty());
            res.assert_header(HX_REDIRECT, "/shopping/lists");
            Ok(())
        }
    }

    mod tests_list_items {
        use models::shopping::ShoppingListDetails;
        use reqwest::StatusCode;
        use uuid::Uuid;

        use crate::schemas::shopping::ListItemPayload;

        use super::*;

        fn base_uri(list_id: Uuid) -> String {
            format!("/shopping/lists/{list_id}/items")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, &base_uri(Uuid::new_v4())).await?;
            assert_must_be_logged_in(Method::DELETE, &base_uri(Uuid::new_v4())).await
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
            let c = Components::setup_with_one_list().await?;

            let res = c
                .server
                .post(&base_uri(c.list_id))
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
                    r#"<li class="list-row grid grid-cols-[1fr_auto]"><div class="grid gap-1 min-w-0"><label class="label text-base-content"><input class="checkbox" type="checkbox">Spaghetti (500g)</label></div><div class="flex gap-1">"#,
                    &format!(
                        r#"<button class="btn join-item btn-square btn-sm" hx-target="closest li" hx-swap="outerHTML" hx-get="/shopping/lists/{}/items/1/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button>"#,
                        c.list_id
                    ),
                    &format!(
                        r#"<button class="btn join-item btn-square btn-sm" hx-target="closest li" hx-swap="delete" hx-delete="/shopping/lists/{}/items/1"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button>"#,
                        c.list_id
                    ),
                    r#"<button class="btn join-item btn-square btn-sm cursor-grab h-full"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3 7.5 7.5 3m0 0L12 7.5M7.5 3v13.5m13.5 0L16.5 21m0 0L12 16.5m4.5 4.5V7.5"></svg></button></div></li>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_item_not_found_ok() -> Result<()> {
            let c = Components::setup_with_one_list().await?;

            let res = c
                .server
                .delete(&format!("{}/{}", base_uri(c.list_id), 666))
                .await;

            res.assert_status_ok();
            let list = ShoppingListDetails::get(&c.state.mm, c.list_id, c.user_id).await?;
            assert!(list.items.is_empty());
            Ok(())
        }

        #[tokio::test]
        async fn test_delete_item_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let item = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;

            let res = server
                .delete(&format!("{}/{}", base_uri(list_id), item.id))
                .await;

            res.assert_status_ok();
            let list = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
            assert!(list.items.is_empty());
            Ok(())
        }

        #[tokio::test]
        async fn test_put_item_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let item = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;
            let item_id = item.id;

            let res = server
                .put(&format!("{}/{item_id}", base_uri(list_id)))
                .form(&ListItemPayload {
                    item: "Apples".into(),
                    quantity: Some("50".into()),
                    label: None,
                })
                .await;

            res.assert_status_ok();
            let item = ShoppingList::get_item(&state.mm, list_id, item_id, user_id).await?;
            assert_eq!(item.ingredient, "Apples");
            assert_eq!(item.quantity, Some("50".into()));
            assert_html(
                &res,
                &[
                    r#"<li class="list-row grid grid-cols-[1fr_auto]"><div class="grid gap-1 min-w-0"><label class="label text-base-content"><input class="checkbox" type="checkbox">Apples (50)</label></div><div class="flex gap-1">"#,
                    &format!(
                        r#"<button class="btn join-item btn-square btn-sm" hx-target="closest li" hx-swap="outerHTML" hx-get="/shopping/lists/{list_id}/items/{item_id}/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button>"#
                    ),
                    &format!(
                        r#"<button class="btn join-item btn-square btn-sm" hx-target="closest li" hx-swap="delete" hx-delete="/shopping/lists/{list_id}/items/{item_id}"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button>"#
                    ),
                    r#"<button class="btn join-item btn-square btn-sm cursor-grab h-full"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M3 7.5 7.5 3m0 0L12 7.5M7.5 3v13.5m13.5 0L16.5 21m0 0L12 16.5m4.5 4.5V7.5"></svg></button></div></li>"#,
                ],
            );
            Ok(())
        }
    }

    mod tests_lists_item_edit {
        use super::*;

        fn base_uri(list_id: Uuid, item_id: i64) -> String {
            format!("/shopping/lists/{list_id}/items/{item_id}/edit")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri(Uuid::new_v4(), 1)).await
        }

        #[tokio::test]
        async fn test_item_not_found_ok() -> Result<()> {
            let c = Components::setup_with_one_list().await?;

            let res = c.server.get(&base_uri(c.list_id, 5)).await;

            res.assert_status_not_found();
            Ok(())
        }

        #[tokio::test]
        async fn test_item_exists_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let item = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;

            let res = server.get(&base_uri(list_id, item.id)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<li class="list-row grid grid-cols-[1fr_auto]"><div class="grid gap-1 min-w-0"><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" height="20" width="22.5" fill="currentColor"><path d="M453.1 27.3L440.9 39.4C409.7 70.6 409.7 121.3 440.9 152.5C456.5 168.1 472.1 183.7 487.8 199.4C519 230.6 569.7 230.6 600.9 199.4L613 187.3C619.2 181.1 619.2 170.9 613 164.7L600.9 152.6C569.7 121.4 519 121.4 487.8 152.6C519 121.4 519 70.7 487.8 39.5L475.7 27.3C469.5 21.1 459.3 21.1 453.1 27.3zM331.6 160C286.4 160 244.5 180.4 216.6 214.3L273.3 271C282.7 280.4 282.7 295.6 273.3 304.9C263.9 314.2 248.7 314.3 239.4 304.9L191.6 257.2L67.2 530.8C61.7 542.9 64.3 557.2 73.7 566.7C83.1 576.2 97.4 578.7 109.6 573.2L251.2 508.8L207.4 465C198 455.6 198 440.4 207.4 431.1C216.8 421.8 232 421.7 241.3 431.1L297.8 487.6L393.1 444.3C446.2 420.2 480.3 367.2 480.3 308.8C480.3 226.6 413.7 160 331.5 160z"></svg><input required type="text" placeholder="Surloin steak" name="item" value="Spaghetti"></label><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v17.25m0 0c-1.472 0-2.882.265-4.185.75M12 20.25c1.472 0 2.882.265 4.185.75M18.75 4.97A48.416 48.416 0 0 0 12 4.5c-2.291 0-4.545.16-6.75.47m13.5 0c1.01.143 2.01.317 3 .52m-3-.52 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.988 5.988 0 0 1-2.031.352 5.988 5.988 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L18.75 4.971Zm-16.5.52c.99-.203 1.99-.377 3-.52m0 0 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.989 5.989 0 0 1-2.031.352 5.989 5.989 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L5.25 4.971Z"></svg><input type="text" placeholder="500g (optional)" name="quantity" value=""></label><input type="hidden" name="label" value="No label"></div><div class="grid grid-flow-col gap-1 place-self-end"><div class="grid grid-col gap-2 w-12">"#,
                    &format!(
                        r#"<button class="btn join-item btn-sm" hx-put="/shopping/lists/{list_id}/items/1" hx-include="closest li" hx-target="closest li" hx-swap="outerHTML"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button>"#
                    ),
                ],
            );
            Ok(())
        }
    }

    mod tests_shopping_list_labels {
        use models::shopping::ShoppingListDetails;

        use crate::schemas::shopping::ListPayload;

        use super::*;

        fn base_uri_edit(list_id: Uuid, label_id: i64) -> String {
            format!("{}/edit", base_uri(list_id, label_id))
        }

        fn base_uri(list_id: Uuid, label_id: i64) -> String {
            format!("/shopping/lists/{list_id}/labels/{label_id}")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri_edit(Uuid::new_v4(), 1)).await
        }

        #[tokio::test]
        #[tracing_test::traced_test]
        async fn test_get_edit_label_ok() -> Result<()> {
            let c = Components::setup_with_one_list().await?;

            let res = c.server.get(&base_uri_edit(c.list_id, 1)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[&format!(
                    r#"<summary class="text-left cursor-default"><input type="text" name="name" class="input input-sm" value="No label" span><button class="btn join-item btn-square btn-sm ml-2 mb-1" hx-include="closest summary" hx-target="closest summary" hx-swap="outerHTML" hx-put="/shopping/lists/{}/labels/1"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button></input></summary>"#,
                    c.list_id
                )],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_put_empty_label_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

            let res = server
                .put(&base_uri(list_id, item.label_id))
                .form(&ListPayload::default())
                .await;

            res.assert_status_ok();
            let list = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
            assert_eq!(list.items[0].label_id, 1);
            assert_eq!(list.items[0].label, "No label");
            assert_html(
                &res,
                &[&format!(
                    r#"<summary class="text-left cursor-default"><button class="btn join-item btn-square btn-sm ml-2 mb-1" hx-target="closest summary" hx-swap="outerHTML" hx-get="/shopping/lists/{list_id}/labels/1/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></summary>"#
                )],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_put_existing_label_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let label = a_meat_item().label.unwrap();
            let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

            let res = server
                .put(&base_uri(list_id, item.label_id))
                .form(&ListPayload::new(&label))
                .await;

            res.assert_status_ok();
            let list = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
            assert_eq!(list.items[0].label_id, 2);
            assert_eq!(list.items[0].label, label);
            Ok(())
        }

        #[tokio::test]
        async fn test_put_new_label_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let item = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

            let res = server
                .put(&base_uri(list_id, item.label_id))
                .form(&ListPayload::new("Veggies"))
                .await;

            res.assert_status_ok();
            let list = ShoppingListDetails::get(&state.mm, list_id, user_id).await?;
            assert_eq!(list.items[0].label_id, 3);
            assert_eq!(list.items[0].label, "Veggies");
            assert_html(
                &res,
                &[&format!(
                    r#"<summary class="text-left cursor-default">Veggies<button class="btn join-item btn-square btn-sm ml-2 mb-1" hx-target="closest summary" hx-swap="outerHTML" hx-get="/shopping/lists/{list_id}/labels/3/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></summary>"#
                )],
            );
            Ok(())
        }
    }
}
