use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{delete, get, post, put},
};

use app::state::AppState;

use crate::{
    handlers::shopping::{
        shopping_list_copy_handler, shopping_list_delete_handler, shopping_list_export_handler,
        shopping_list_handler, shopping_list_item_delete_handler, shopping_list_item_edit_handler,
        shopping_list_item_post_handler, shopping_list_item_put_handler,
        shopping_list_item_toggle_handler, shopping_list_items_positions_put_handler,
        shopping_list_label_put_handler, shopping_list_labels_new_handler,
        shopping_list_labels_post_handler, shopping_list_print_handler, shopping_list_put_handler,
        shopping_list_share_post_handler, shopping_lists_handler, shopping_lists_post_handler,
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
        .route("/lists/{:list_id}/copy", get(shopping_list_copy_handler))
        .route(
            "/lists/{:list_id}/export",
            get(shopping_list_export_handler),
        )
        .route("/lists/{:list_id}/print", get(shopping_list_print_handler))
        .route(
            "/lists/{:list_id}/share",
            post(shopping_list_share_post_handler),
        )
        .route(
            "/lists/{:list_id}/labels",
            post(shopping_list_labels_post_handler),
        )
        .route(
            "/lists/{:list_id}/labels/new",
            get(shopping_list_labels_new_handler),
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
            "/lists/{:list_id}/items/positions",
            put(shopping_list_items_positions_put_handler),
        )
        .route(
            "/lists/{:list_id}/items/{:item_id}",
            delete(shopping_list_item_delete_handler).put(shopping_list_item_put_handler),
        )
        .route(
            "/lists/{:list_id}/items/{:item_id}/edit",
            get(shopping_list_item_edit_handler),
        )
        .route(
            "/lists/{:list_id}/items/{:item_id}/toggle",
            post(shopping_list_item_toggle_handler),
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
            notes: None,
            recipe_id: None,
        }
    }

    fn a_meat_item() -> ShoppingListItemForCreate {
        ShoppingListItemForCreate::new(
            Some("1 cup".into()),
            "chicken",
            Some("Meat".into()),
            Some("new notes".into()),
            None,
        )
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
                    r#"<div class="order-0 flex-1 overflow-y-auto min-h-0 md:order-3 max-h-[94vh]"><div id="shopping-list-view-pane" class="p-4 text-center grid"><div class="grid"><p class="text-left">← Create your first shopping list to get started.</p><div class="join border border-gray-700 mb-2 w-fit"></div>"#,
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
                        r##"<li id="shopping-list-sidebar-{list_id}" class="bg-base-300" hx-get="/shopping/lists/{list_id}" hx-target="#shopping-list-view-pane" hx-push-url="false" hx-trigger="mousedown" hx-on:mousedown="document.querySelectorAll('#shopping-lists li').forEach((el) =&gt; el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-shopping-list-id').value = '{list_id}';"><div class="flex justify-between items-center gap-2 w-full"><div class="min-w-0"><p class="font-bold text-sm">Costco</p></div><div class="flex flex-col items-end gap-1 shrink-0"><div id="shopping-list-item-count-{list_id}" class="badge badge-xs badge-primary">0</div></div></div></li><div id="shopping-list-view-pane" hx-swap-oob="innerHTML"><div><div><h1 class="text-2xl font-bold underline p-2">Costco<span class="ml-2"><button class="btn join-item btn-square btn-sm" _="on click add .hidden to closest &lt;h1/&gt; then remove .hidden from next &lt;form/&gt; from closest &lt;h1/&gt; then call (next &lt;input/&gt; from closest &lt;h1/&gt;).select()"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></span></h1><form class="hidden" hx-put="/shopping/lists/{list_id}" hx-swap="outerHTML"><h1 class="text-2xl font-bold underline p-2"><input type="text" required autofocus name="name" class="input input-lg text-center mr-1" value="Costco" list="labels" autocomplete="off"><span class="ml-2"><button class="btn join-item btn-square btn-lg"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button></span></h1></form></div><div class="grid"><div class="min-w-[33rem] place-self-center">"##
                    ),
                    &format!(
                        r#"<details open><summary id="label-1" class="text-left cursor-default"><span>No label<button class="btn join-item btn-square btn-sm ml-2 mb-1" _="on click add .hidden to closest &lt;span/&gt; then remove .hidden from next &lt;span/&gt; from closest &lt;span/&gt; then add .inline-flex to next &lt;span/&gt; from closest &lt;span/&gt; then call (next &lt;input/&gt; from closest &lt;span/&gt;).select()"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></span><span class="hidden text-left cursor-default "><form class="flex" hx-target="closest summary" hx-swap="outerHTML" hx-put="/shopping/lists/{list_id}/labels/1"><input autofocus type="text" required name="name" class="input input-sm" value="No label" list="labels" autocomplete="off" _="on load wait 50ms then call me.select()"><span><button class="btn join-item btn-square btn-sm ml-2 mb-1"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button></span></form></span></summary><ol class="list bg-base-100 rounded-box shadow-md"><li class="list-row grid grid-cols-[1fr_auto]"><form class="contents" hx-post="/shopping/lists/{list_id}/items" hx-target="closest li" hx-swap="beforebegin" hx-on--after-request="if(event.detail.successful) {{ this.reset(); this.querySelector('input').focus(); }}"><div class="grid gap-1 min-w-0"><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" height="20" width="22.5" fill="currentColor"><path d="M453.1 27.3L440.9 39.4C409.7 70.6 409.7 121.3 440.9 152.5C456.5 168.1 472.1 183.7 487.8 199.4C519 230.6 569.7 230.6 600.9 199.4L613 187.3C619.2 181.1 619.2 170.9 613 164.7L600.9 152.6C569.7 121.4 519 121.4 487.8 152.6C519 121.4 519 70.7 487.8 39.5L475.7 27.3C469.5 21.1 459.3 21.1 453.1 27.3zM331.6 160C286.4 160 244.5 180.4 216.6 214.3L273.3 271C282.7 280.4 282.7 295.6 273.3 304.9C263.9 314.2 248.7 314.3 239.4 304.9L191.6 257.2L67.2 530.8C61.7 542.9 64.3 557.2 73.7 566.7C83.1 576.2 97.4 578.7 109.6 573.2L251.2 508.8L207.4 465C198 455.6 198 440.4 207.4 431.1C216.8 421.8 232 421.7 241.3 431.1L297.8 487.6L393.1 444.3C446.2 420.2 480.3 367.2 480.3 308.8C480.3 226.6 413.7 160 331.5 160z"></svg><input required autofocus type="text" placeholder="Surloin steak" name="item" value=""></label><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v17.25m0 0c-1.472 0-2.882.265-4.185.75M12 20.25c1.472 0 2.882.265 4.185.75M18.75 4.97A48.416 48.416 0 0 0 12 4.5c-2.291 0-4.545.16-6.75.47m13.5 0c1.01.143 2.01.317 3 .52m-3-.52 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.988 5.988 0 0 1-2.031.352 5.988 5.988 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L18.75 4.971Zm-16.5.52c.99-.203 1.99-.377 3-.52m0 0 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.989 5.989 0 0 1-2.031.352 5.989 5.989 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L5.25 4.971Z"></svg><input type="text" placeholder="500g (optional)" name="quantity" value=""></label><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m18.375 12.739-7.693 7.693a4.5 4.5 0 0 1-6.364-6.364l10.94-10.94A3 3 0 1 1 19.5 7.372L8.552 18.32m.009-.01-.01.01m5.699-9.941-7.81 7.81a1.5 1.5 0 0 0 2.112 2.13"></svg><input type="text" placeholder="Notes (optional)" name="notes" value="" autocomplete="off"></label></div><div class="grid grid-flow-col gap-1 place-self-end"><div class="grid grid-col gap-2 w-12"><button class="btn join-item btn-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15"></svg></button></div></div></form></li></ol></details>"#
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
                        r##"<ul id="shopping-lists" class="menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0 h-full"><li id="shopping-list-sidebar-{list_id}" class="bg-base-300" hx-get="/shopping/lists/{list_id}" hx-target="#shopping-list-view-pane" hx-push-url="false" hx-trigger="mousedown" hx-on:mousedown="document.querySelectorAll('#shopping-lists li').forEach((el) =&gt; el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-shopping-list-id').value = '{list_id}';"><div class="flex justify-between items-center gap-2 w-full"><div class="min-w-0"><p class="font-bold text-sm">Test</p></div><div class="flex flex-col items-end gap-1 shrink-0"><div id="shopping-list-item-count-{list_id}" class="badge badge-xs badge-primary">1</div></div></div></li></ul>"##
                    ),
                    r#"<div class="order-1 divider my-0 md:order-2 md:divider-horizontal md:mx-0"></div><div class="order-0 flex-1 overflow-y-auto min-h-0 md:order-3 max-h-[94vh]"><div id="shopping-list-view-pane" class="p-4 text-center grid">"#,
                    &format!(
                        r#"<div><h1 class="text-2xl font-bold underline p-2">Test<span class="ml-2"><button class="btn join-item btn-square btn-sm" _="on click add .hidden to closest &lt;h1/&gt; then remove .hidden from next &lt;form/&gt; from closest &lt;h1/&gt; then call (next &lt;input/&gt; from closest &lt;h1/&gt;).select()"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></span></h1><form class="hidden" hx-put="/shopping/lists/{list_id}" hx-swap="outerHTML"><h1 class="text-2xl font-bold underline p-2"><input type="text" required autofocus name="name" class="input input-lg text-center mr-1" value="Test" list="labels" autocomplete="off"><span class="ml-2"><button class="btn join-item btn-square btn-lg"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button></span></h1></form></div>"#
                    ),
                    &format!(
                        r#"<div class="min-w-[33rem] place-self-center"><details open><summary id="label-1" class="text-left cursor-default"><span>No label<button class="btn join-item btn-square btn-sm ml-2 mb-1" _="on click add .hidden to closest &lt;span/&gt; then remove .hidden from next &lt;span/&gt; from closest &lt;span/&gt; then add .inline-flex to next &lt;span/&gt; from closest &lt;span/&gt; then call (next &lt;input/&gt; from closest &lt;span/&gt;).select()"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></span><span class="hidden text-left cursor-default "><form class="flex" hx-target="closest summary" hx-swap="outerHTML" hx-put="/shopping/lists/{list_id}/labels/1"><input autofocus type="text" required name="name" class="input input-sm" value="No label" list="labels" autocomplete="off" _="on load wait 50ms then call me.select()"><span><button class="btn join-item btn-square btn-sm ml-2 mb-1"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button></span></form></span></summary><ol id="shopping-list-items-container-Nolabel" class="list bg-base-100 rounded-box shadow-md"><li class="list-row grid grid-cols-[1fr_auto]" data-item-id="1" data-drag-row><div class="grid grid-flow-col" data-drageable draggable="true"><div class="grid gap-1 min-w-0"><label class="label text-base-content"><input type="checkbox" class="checkbox peer" hx-post="/shopping/lists/{list_id}/items/1/toggle"><div class="text-left [input:checked~&amp;]:line-through [input:checked~&amp;]:opacity-50 transition-all"><p>Spaghetti</p></div></label></div><div class="flex gap-1 place-content-end"><button class="btn join-item btn-square btn-sm [li:has(input:checked)_&amp;]:hidden transition-all" hx-target="closest li" hx-swap="outerHTML" hx-get="/shopping/lists/{list_id}/items/1/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button><button class="btn join-item btn-square btn-sm [li:has(input:checked)_&amp;]:opacity-50 transition-all" hx-target="closest li" hx-swap="delete" hx-delete="/shopping/lists/{list_id}/items/1"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button><div class="inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl [li:has(input:checked)_&amp;]:hidden transition-all" data-drag-handle>⠿</div></div></div></li><li class="list-row grid grid-cols-[1fr_auto]"><form class="contents" hx-post="/shopping/lists/{list_id}/items" hx-target="closest li" hx-swap="beforebegin" hx-on--after-request="if(event.detail.successful) {{ this.reset(); this.querySelector('input').focus(); }}"><div class="grid gap-1 min-w-0"><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" height="20" width="22.5" fill="currentColor"><path d="M453.1 27.3L440.9 39.4C409.7 70.6 409.7 121.3 440.9 152.5C456.5 168.1 472.1 183.7 487.8 199.4C519 230.6 569.7 230.6 600.9 199.4L613 187.3C619.2 181.1 619.2 170.9 613 164.7L600.9 152.6C569.7 121.4 519 121.4 487.8 152.6C519 121.4 519 70.7 487.8 39.5L475.7 27.3C469.5 21.1 459.3 21.1 453.1 27.3zM331.6 160C286.4 160 244.5 180.4 216.6 214.3L273.3 271C282.7 280.4 282.7 295.6 273.3 304.9C263.9 314.2 248.7 314.3 239.4 304.9L191.6 257.2L67.2 530.8C61.7 542.9 64.3 557.2 73.7 566.7C83.1 576.2 97.4 578.7 109.6 573.2L251.2 508.8L207.4 465C198 455.6 198 440.4 207.4 431.1C216.8 421.8 232 421.7 241.3 431.1L297.8 487.6L393.1 444.3C446.2 420.2 480.3 367.2 480.3 308.8C480.3 226.6 413.7 160 331.5 160z"></svg><input required autofocus type="text" placeholder="Surloin steak" name="item" value=""></label><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v17.25m0 0c-1.472 0-2.882.265-4.185.75M12 20.25c1.472 0 2.882.265 4.185.75M18.75 4.97A48.416 48.416 0 0 0 12 4.5c-2.291 0-4.545.16-6.75.47m13.5 0c1.01.143 2.01.317 3 .52m-3-.52 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.988 5.988 0 0 1-2.031.352 5.988 5.988 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L18.75 4.971Zm-16.5.52c.99-.203 1.99-.377 3-.52m0 0 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.989 5.989 0 0 1-2.031.352 5.989 5.989 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L5.25 4.971Z"></svg><input type="text" placeholder="500g (optional)" name="quantity" value=""></label><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m18.375 12.739-7.693 7.693a4.5 4.5 0 0 1-6.364-6.364l10.94-10.94A3 3 0 1 1 19.5 7.372L8.552 18.32m.009-.01-.01.01m5.699-9.941-7.81 7.81a1.5 1.5 0 0 0 2.112 2.13"></svg><input type="text" placeholder="Notes (optional)" name="notes" value="" autocomplete="off"></label><input type="hidden" name="label" value="No label"></div><div class="grid grid-flow-col gap-1 place-self-end"><div class="grid grid-col gap-2 w-12"><button class="btn join-item btn-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15"></svg></button></div></div></form></li></ol></details>"#
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
            assert_must_be_logged_in(Method::GET, &base_uri(Uuid::new_v4())).await?;
            assert_must_be_logged_in(Method::PUT, &base_uri(Uuid::new_v4())).await?;
            assert_must_be_logged_in(Method::DELETE, &base_uri(Uuid::new_v4())).await
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
                    r#"<div><h1 class="text-2xl font-bold underline p-2">New Title<span class="ml-2"><button class="btn join-item btn-square btn-sm" _="on click add .hidden to closest &lt;h1/&gt; then remove .hidden from next &lt;form/&gt; from closest &lt;h1/&gt; then call (next &lt;input/&gt; from closest &lt;h1/&gt;).select()"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></span></h1>"#,
                    &format!(
                        r#"<form class="hidden" hx-put="/shopping/lists/{list_id}" hx-swap="outerHTML"><h1 class="text-2xl font-bold underline p-2"><input type="text" required autofocus name="name" class="input input-lg text-center mr-1" value="New Title" list="labels" autocomplete="off"><span class="ml-2"><button class="btn join-item btn-square btn-lg"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button></span></h1></form></div>"#
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

    mod tests_list_share {
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;

        use models::shopping::ShareShoppingList;
        use repository::schema;

        use super::*;
        use crate::recipes_router::params::ShareRecipeForm;

        fn base_uri(list_id: Uuid) -> String {
            format!("/shopping/lists/{list_id}/share")
        }

        async fn get_first_shared(
            state: AppState,
            list_id: Uuid,
            user_id: Uuid,
        ) -> ShareShoppingList {
            let mut conn = state
                .mm
                .pool
                .get()
                .await
                .expect("a connection from the pool");

            schema::shares_shopping_lists::table
                .filter(
                    schema::shares_shopping_lists::list_id
                        .eq(list_id)
                        .and(schema::shares_shopping_lists::user_id.eq(user_id)),
                )
                .first::<ShareShoppingList>(&mut conn)
                .await
                .expect("a share shopping list must have been fetched")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, &base_uri(Uuid::new_v4())).await
        }

        #[tokio::test]
        async fn test_default_expires_at_time_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;

            let res = server
                .post(&base_uri(list_id))
                .form(&ShareRecipeForm::new(None))
                .await;

            let share = get_first_shared(state, list_id, user_id).await;
            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    &format!(
                        r#"<label><input class="input" type="url" value="http://localhost:8078/shared/sl/{}" readonly="readonly"></label>"#,
                        share.link
                    ),
                    &format!(
                        r#"<button class="btn btn-neutral" id="copy-button" title="Copy to clipboard" onClick="copyToClipboard('http://localhost:8078/shared/sl/{}')">Copy</button>"#,
                        share.link
                    ),
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_custom_expires_at_time_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let expires_at = (chrono::Utc::now() + chrono::Duration::days(31)).naive_utc();

            let res = server
                .post(&base_uri(list_id))
                .form(&ShareRecipeForm::new(Some(expires_at.to_string())))
                .await;

            res.assert_status_ok();
            let share = get_first_shared(state, list_id, user_id).await;
            assert_html(
                &res,
                &[
                    &format!(
                        r#"<label><input class="input" type="url" value="http://localhost:8078/shared/sl/{}" readonly="readonly"></label>"#,
                        share.link
                    ),
                    &format!(
                        r#"<button class="btn btn-neutral" id="copy-button" title="Copy to clipboard" onClick="copyToClipboard('http://localhost:8078/shared/sl/{}')">Copy</button>"#,
                        share.link
                    ),
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_invalid_expires_at_time_defaults_to_7_days_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let now = chrono::Utc::now().naive_utc();

            let res = server
                .post(&base_uri(list_id))
                .form(&ShareRecipeForm::new(Some("hello".into())))
                .await;

            res.assert_status_ok();
            let share = get_first_shared(state, list_id, user_id).await;
            pretty_assertions::assert_eq!(
                share.expires_at.signed_duration_since(now).num_days(),
                7
            );
            Ok(())
        }

        #[tokio::test]
        #[tracing_test::traced_test]
        async fn test_share_twice_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let expires_at = (chrono::Utc::now() + chrono::Duration::days(31)).naive_utc();
            let _ = server
                .post(&base_uri(list_id))
                .form(&ShareRecipeForm::new(Some(expires_at.to_string())))
                .await;

            let res = server
                .post(&base_uri(list_id))
                .form(&ShareRecipeForm::new(Some(expires_at.to_string())))
                .await;

            res.assert_status_ok();
            let share = get_first_shared(state, list_id, user_id).await;
            assert_eq!(share.list_id, list_id);
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
                    &format!(
                        r#"<li class="list-row grid grid-cols-[1fr_auto]" data-item-id="1" data-drag-row><div class="grid grid-flow-col" data-drageable draggable="true"><div class="grid gap-1 min-w-0"><label class="label text-base-content"><input type="checkbox" class="checkbox peer" hx-post="/shopping/lists/{}/items/1/toggle"><div class="text-left [input:checked~&amp;]:line-through [input:checked~&amp;]:opacity-50 transition-all"><p>Spaghetti (500g)</p></div></label></div><div class="flex gap-1 place-content-end">"#,
                        c.list_id
                    ),
                    &format!(
                        r#"<button class="btn join-item btn-square btn-sm [li:has(input:checked)_&amp;]:hidden transition-all" hx-target="closest li" hx-swap="outerHTML" hx-get="/shopping/lists/{}/items/1/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button>"#,
                        c.list_id
                    ),
                    &format!(
                        r#"<button class="btn join-item btn-square btn-sm [li:has(input:checked)_&amp;]:opacity-50 transition-all" hx-target="closest li" hx-swap="delete" hx-delete="/shopping/lists/{}/items/1"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button>"#,
                        c.list_id
                    ),
                    &format!(
                        r#"<div class="inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl [li:has(input:checked)_&amp;]:hidden transition-all" data-drag-handle>⠿</div></div></div></li><div id="shopping-list-item-count-{}" class="badge badge-xs badge-primary" hx-swap-oob="true">1</div>"#,
                        c.list_id
                    ),
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
            let count = ShoppingList::items_count(&state.mm, list_id)
                .await
                .unwrap_or_default();
            assert_eq!(count, 0);
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
                    position: None,
                    notes: Some("new notes".into()),
                })
                .await;

            res.assert_status_ok();
            let item = ShoppingList::get_item(&state.mm, list_id, item_id, user_id).await?;
            assert_eq!(item.ingredient, "Apples");
            assert_eq!(item.quantity, Some("50".into()));
            assert_html(
                &res,
                &[
                    &format!(
                        r#"<li class="list-row grid grid-cols-[1fr_auto]" data-item-id="1" data-drag-row><div class="grid grid-flow-col" data-drageable draggable="true"><div class="grid gap-1 min-w-0"><label class="label text-base-content"><input type="checkbox" class="checkbox peer" hx-post="/shopping/lists/{list_id}/items/1/toggle"><div class="text-left [input:checked~&amp;]:line-through [input:checked~&amp;]:opacity-50 transition-all"><p>Apples (50)</p><p class="text-xs font-light">new notes</p></div></label></div><div class="flex gap-1 place-content-end">"#
                    ),
                    &format!(
                        r#"<button class="btn join-item btn-square btn-sm [li:has(input:checked)_&amp;]:hidden transition-all" hx-target="closest li" hx-swap="outerHTML" hx-get="/shopping/lists/{list_id}/items/1/edit"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button>"#
                    ),
                    &format!(
                        r#"<button class="btn join-item btn-square btn-sm [li:has(input:checked)_&amp;]:opacity-50 transition-all" hx-target="closest li" hx-swap="delete" hx-delete="/shopping/lists/{list_id}/items/1"><svg xmlns="http://www.w3.org/2000/svg" class="size-6 hover:text-red-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"></path></svg></button>"#
                    ),
                    &format!(
                        r#"<div class="inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl [li:has(input:checked)_&amp;]:hidden transition-all" data-drag-handle>⠿</div></div></div></li><div id="shopping-list-item-count-{list_id}" class="badge badge-xs badge-primary" hx-swap-oob="true">1</div>"#
                    ),
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
                    &format!(
                        r#"<li class="list-row grid grid-cols-[1fr_auto]"><form class="contents" hx-put="/shopping/lists/{list_id}/items/1" hx-target="closest li" hx-swap="outerHTML" hx-on--after-request="if(event.detail.successful) {{ this.reset(); this.querySelector('input').focus(); }}"><div class="grid gap-1 min-w-0"><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" height="20" width="22.5" fill="currentColor"><path d="M453.1 27.3L440.9 39.4C409.7 70.6 409.7 121.3 440.9 152.5C456.5 168.1 472.1 183.7 487.8 199.4C519 230.6 569.7 230.6 600.9 199.4L613 187.3C619.2 181.1 619.2 170.9 613 164.7L600.9 152.6C569.7 121.4 519 121.4 487.8 152.6C519 121.4 519 70.7 487.8 39.5L475.7 27.3C469.5 21.1 459.3 21.1 453.1 27.3zM331.6 160C286.4 160 244.5 180.4 216.6 214.3L273.3 271C282.7 280.4 282.7 295.6 273.3 304.9C263.9 314.2 248.7 314.3 239.4 304.9L191.6 257.2L67.2 530.8C61.7 542.9 64.3 557.2 73.7 566.7C83.1 576.2 97.4 578.7 109.6 573.2L251.2 508.8L207.4 465C198 455.6 198 440.4 207.4 431.1C216.8 421.8 232 421.7 241.3 431.1L297.8 487.6L393.1 444.3C446.2 420.2 480.3 367.2 480.3 308.8C480.3 226.6 413.7 160 331.5 160z"></svg><input required autofocus type="text" placeholder="Surloin steak" name="item" value="Spaghetti"></label><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v17.25m0 0c-1.472 0-2.882.265-4.185.75M12 20.25c1.472 0 2.882.265 4.185.75M18.75 4.97A48.416 48.416 0 0 0 12 4.5c-2.291 0-4.545.16-6.75.47m13.5 0c1.01.143 2.01.317 3 .52m-3-.52 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.988 5.988 0 0 1-2.031.352 5.988 5.988 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L18.75 4.971Zm-16.5.52c.99-.203 1.99-.377 3-.52m0 0 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.989 5.989 0 0 1-2.031.352 5.989 5.989 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L5.25 4.971Z"></svg><input type="text" placeholder="500g (optional)" name="quantity" value=""></label><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m18.375 12.739-7.693 7.693a4.5 4.5 0 0 1-6.364-6.364l10.94-10.94A3 3 0 1 1 19.5 7.372L8.552 18.32m.009-.01-.01.01m5.699-9.941-7.81 7.81a1.5 1.5 0 0 0 2.112 2.13"></svg><input type="text" placeholder="Notes (optional)" name="notes" value="" autocomplete="off"></label><input type="hidden" name="label" value="No label"></div><div class="grid grid-flow-col gap-1 place-self-end"><div class="grid grid-col gap-2 w-12">"#
                    ),
                    r#"<button class="btn join-item btn-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button>"#,
                ],
            );
            Ok(())
        }
    }

    mod tests_shopping_list_labels {
        use models::shopping::ShoppingListDetails;

        use crate::schemas::shopping::ListPayload;

        use super::*;

        fn base_uri(list_id: Uuid, label_id: i64) -> String {
            format!("/shopping/lists/{list_id}/labels/{label_id}")
        }

        fn base_uri_labels(list_id: Uuid) -> String {
            format!("/shopping/lists/{list_id}/labels")
        }

        fn base_uri_new(list_id: Uuid) -> String {
            format!("/shopping/lists/{list_id}/labels/new")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, &base_uri(Uuid::new_v4(), 1)).await?;
            assert_must_be_logged_in(Method::GET, &base_uri_new(Uuid::new_v4())).await
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
                    r#"<summary id="label-1" class="text-left cursor-default"><span><button class="btn join-item btn-square btn-sm ml-2 mb-1" _="on click add .hidden to closest &lt;span/&gt; then remove .hidden from next &lt;span/&gt; from closest &lt;span/&gt; then add .inline-flex to next &lt;span/&gt; from closest &lt;span/&gt; then call (next &lt;input/&gt; from closest &lt;span/&gt;).select()"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></span><span class="hidden text-left cursor-default "><form class="flex" hx-target="closest summary" hx-swap="outerHTML" hx-put="/shopping/lists/{list_id}/labels/1"><input autofocus type="text" required name="name" class="input input-sm" value="" list="labels" autocomplete="off" _="on load wait 50ms then call me.select()"><span><button class="btn join-item btn-square btn-sm ml-2 mb-1"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button></span></form></span></summary>"#
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
                    r#"<summary id="label-3" class="text-left cursor-default"><span>Veggies<button class="btn join-item btn-square btn-sm ml-2 mb-1" _="on click add .hidden to closest &lt;span/&gt; then remove .hidden from next &lt;span/&gt; from closest &lt;span/&gt; then add .inline-flex to next &lt;span/&gt; from closest &lt;span/&gt; then call (next &lt;input/&gt; from closest &lt;span/&gt;).select()"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></span><span class="hidden text-left cursor-default "><form class="flex" hx-target="closest summary" hx-swap="outerHTML" hx-put="/shopping/lists/{list_id}/labels/3"><input autofocus type="text" required name="name" class="input input-sm" value="Veggies" list="labels" autocomplete="off" _="on load wait 50ms then call me.select()"><span><button class="btn join-item btn-square btn-sm ml-2 mb-1"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button></span></form></span></summary>"#
                )],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_shopping_list_label_get_new_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;

            let res = server.get(&base_uri_new(list_id)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[&format!(
                    r#"<form hx-post="/shopping/lists/{list_id}/labels" hx-target="closest div.divider" hx-swap="outerHTML"><input type="text" autofocus required name="name" class="input input-sm gap-1" list="labels" autocomplete="off"><button type="submit" class="btn btn-sm btn-ghost"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"></svg></button></form>"#
                )],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_create_new_label_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;

            let res = server
                .post(&base_uri_labels(list_id))
                .form(&ListPayload::new("Costco"))
                .await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    &format!(
                        r#"<details open><summary id="label-1" class="text-left cursor-default"><span>Costco<button class="btn join-item btn-square btn-sm ml-2 mb-1" _="on click add .hidden to closest &lt;span/&gt; then remove .hidden from next &lt;span/&gt; from closest &lt;span/&gt; then add .inline-flex to next &lt;span/&gt; from closest &lt;span/&gt; then call (next &lt;input/&gt; from closest &lt;span/&gt;).select()"><svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg></button></span><span class="hidden text-left cursor-default "><form class="flex" hx-target="closest summary" hx-swap="outerHTML" hx-put="/shopping/lists/{list_id}/labels/1"><input autofocus type="text" required name="name" class="input input-sm" value="Costco" list="labels" autocomplete="off" _="on load wait 50ms then call me.select()"><span><button class="btn join-item btn-square btn-sm ml-2 mb-1"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5"></svg></button></span></form></span></summary>"#
                    ),
                    &format!(
                        r#"<ol id="shopping-list-items-container-Costco" class="list bg-base-100 rounded-box shadow-md"><li class="list-row grid grid-cols-[1fr_auto]"><form class="contents" hx-post="/shopping/lists/{list_id}/items" hx-target="closest li" hx-swap="beforebegin" hx-on--after-request="if(event.detail.successful) {{ this.reset(); this.querySelector('input').focus(); }}"><div class="grid gap-1 min-w-0"><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" height="20" width="22.5" fill="currentColor"><path d="M453.1 27.3L440.9 39.4C409.7 70.6 409.7 121.3 440.9 152.5C456.5 168.1 472.1 183.7 487.8 199.4C519 230.6 569.7 230.6 600.9 199.4L613 187.3C619.2 181.1 619.2 170.9 613 164.7L600.9 152.6C569.7 121.4 519 121.4 487.8 152.6C519 121.4 519 70.7 487.8 39.5L475.7 27.3C469.5 21.1 459.3 21.1 453.1 27.3zM331.6 160C286.4 160 244.5 180.4 216.6 214.3L273.3 271C282.7 280.4 282.7 295.6 273.3 304.9C263.9 314.2 248.7 314.3 239.4 304.9L191.6 257.2L67.2 530.8C61.7 542.9 64.3 557.2 73.7 566.7C83.1 576.2 97.4 578.7 109.6 573.2L251.2 508.8L207.4 465C198 455.6 198 440.4 207.4 431.1C216.8 421.8 232 421.7 241.3 431.1L297.8 487.6L393.1 444.3C446.2 420.2 480.3 367.2 480.3 308.8C480.3 226.6 413.7 160 331.5 160z"></svg><input required autofocus type="text" placeholder="Surloin steak" name="item" value=""></label><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v17.25m0 0c-1.472 0-2.882.265-4.185.75M12 20.25c1.472 0 2.882.265 4.185.75M18.75 4.97A48.416 48.416 0 0 0 12 4.5c-2.291 0-4.545.16-6.75.47m13.5 0c1.01.143 2.01.317 3 .52m-3-.52 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.988 5.988 0 0 1-2.031.352 5.988 5.988 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L18.75 4.971Zm-16.5.52c.99-.203 1.99-.377 3-.52m0 0 2.62 10.726c.122.499-.106 1.028-.589 1.202a5.989 5.989 0 0 1-2.031.352 5.989 5.989 0 0 1-2.031-.352c-.483-.174-.711-.703-.59-1.202L5.25 4.971Z"></svg><input type="text" placeholder="500g (optional)" name="quantity" value=""></label><label class="input input-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m18.375 12.739-7.693 7.693a4.5 4.5 0 0 1-6.364-6.364l10.94-10.94A3 3 0 1 1 19.5 7.372L8.552 18.32m.009-.01-.01.01m5.699-9.941-7.81 7.81a1.5 1.5 0 0 0 2.112 2.13"></svg><input type="text" placeholder="Notes (optional)" name="notes" value="" autocomplete="off"></label><input type="hidden" name="label" value="Costco"></div><div class="grid grid-flow-col gap-1 place-self-end"><div class="grid grid-col gap-2 w-12"><button class="btn join-item btn-sm"><svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15"></svg></button></div></div></form></li></ol></details>"#
                    ),
                    &format!(
                        r#"<div class="divider"><div class="grid grid-flow-col gap-2 w-full"><btn class="btn btn-sm btn-outline" hx-get="/shopping/lists/{list_id}/labels/new" hx-swap="outerHTML">Add label</btn></div></div>"#
                    ),
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_create_new_label_duplicate_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let _ = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

            let res = server
                .post(&base_uri_labels(list_id))
                .form(&ListPayload::new(a_meat_item().label.unwrap()))
                .await;

            res.assert_status_conflict();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Label already exists in the shopping list.","status":"alert-warning","title":"Attention"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_create_new_label_empty_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;

            let res = server
                .post(&base_uri_labels(list_id))
                .form(&ListPayload::default())
                .await;

            res.assert_status_bad_request();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Label name cannot be empty.","status":"alert-warning","title":"Attention"}}"# ).await;
            Ok(())
        }
    }

    mod tests_shopping_list_print {
        use super::*;

        fn base_uri(list_id: Uuid) -> String {
            format!("/shopping/lists/{list_id}/print")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri(Uuid::new_v4())).await
        }

        #[tokio::test]
        async fn test_print_no_items() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;

            let res = server.get(&base_uri(list_id)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Print View</title></head><body class="p-8">"#,
                    r#"<h1 style="text-align: center; text-decoration: underline;">Test</h1><div><p>Shopping list has no items.</p>"#,
                    "<script>window.onload = function() { window.print(); window.close(); }</script></body></html>",
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_print_with_items_no_labels_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let _ = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;

            let res = server.get(&base_uri(list_id)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Print View</title></head><body class="p-8">"#,
                    r#"<h1 style="text-align: center; text-decoration: underline;">Test</h1><div><ul><li style="list-style-type: none;"><label style="display: flex;"><input class="checkbox" type="checkbox" style="margin-right: .5rem;"><div><p style="margin-bottom: 0.25rem; margin-top: 0.25rem;">Spaghetti</p></div></label></li></ul></div>"#,
                    "<script>window.onload = function() { window.print(); window.close(); }</script></body></html>",
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_print_with_items_with_labels_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let _ = ShoppingList::add_item(&state.mm, list_id, a_meat_item(), user_id).await?;

            let res = server.get(&base_uri(list_id)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Print View</title></head><body class="p-8">"#,
                    r#"<h1 style="text-align: center; text-decoration: underline;">Test</h1><div><details open><summary>Meat</summary><ul><li style="list-style-type: none;"><label style="display: flex;"><input class="checkbox" type="checkbox" style="margin-right: .5rem;"><div><p style="margin-bottom: 0.25rem; margin-top: 0.25rem;">chicken (1 cup)</p><p style="font-size: 0.75rem; line-height: 1.2; font-weight: 300; margin: 0;">new notes</p></div></label></li></ul></details>"#,
                    r"<script>window.onload = function() { window.print(); window.close(); }</script></body></html>",
                ],
            );
            Ok(())
        }
    }

    mod tests_shopping_list_copy {
        use mime_guess::mime::TEXT_PLAIN_UTF_8;
        use reqwest::header::CONTENT_TYPE;

        use super::*;

        fn base_uri(list_id: Uuid, format: Option<&str>) -> String {
            format!(
                "/shopping/lists/{list_id}/copy?format={}",
                format.unwrap_or_default()
            )
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri(Uuid::new_v4(), Some("text"))).await
        }

        #[tokio::test]
        async fn test_copy_no_items_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;

            let res = server.get(&base_uri(list_id, Some("text"))).await;

            res.assert_status_internal_server_error();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Shopping list is empty.","status":"alert-warning","title":"Attention"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_copy_list_with_items_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let _ = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;

            let res = server.get(&base_uri(list_id, Some("text"))).await;

            res.assert_status_ok();
            res.assert_header(CONTENT_TYPE, TEXT_PLAIN_UTF_8.to_string());
            res.assert_text("Test\n----\n\n- Spaghetti\n");
            Ok(())
        }
    }

    mod tests_shopping_list_export {
        use axum_htmx::HX_TRIGGER;
        use models::download::Download;

        use super::*;

        fn base_uri(list_id: Uuid, format: Option<&str>) -> String {
            format!(
                "/shopping/lists/{list_id}/export?format={}",
                format.unwrap_or_default()
            )
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri(Uuid::new_v4(), Some("text"))).await
        }

        #[tokio::test]
        async fn test_export_no_items_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let (server, mut ws_server) = build_server_ws(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;

            let res = server.get(&base_uri(list_id, Some("text"))).await;

            res.assert_status_internal_server_error();
            assert_ws_message(&mut ws_server, r#"{"showMessageHtmx":{"type":"toast","message":"Shopping list is empty.","status":"alert-warning","title":"Attention"}}"# ).await;
            Ok(())
        }

        #[tokio::test]
        async fn test_export_list_with_items_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let _ = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;

            let res = server.get(&base_uri(list_id, Some("text"))).await;

            res.assert_status_ok();
            let hx_trigger = res.header(HX_TRIGGER);
            let hx_str = hx_trigger.to_str()?;
            assert!(hx_str.contains("downloadReady"));
            assert!(hx_str.contains("/download?token="));
            let token_str = hx_str
                .split_once("/download?token=")
                .and_then(|(_, after)| after.split('"').next())
                .expect("HX-Trigger should contain a download token");
            let token = token_str.parse::<uuid::Uuid>()?;
            let dl = Download::find_by_token(&state.mm, token).await?.unwrap();
            assert_eq!(dl.user_id, user_id);
            assert_eq!(dl.file_path, "/tmp/Test.txt");
            Ok(())
        }
    }

    mod tests_shopping_list_items_positions {
        use std::collections::HashMap;

        use super::*;

        fn base_uri(list_id: Uuid) -> String {
            format!("/shopping/lists/{list_id}/items/positions")
        }

        #[tokio::test]
        async fn test_sl_pos_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::PUT, &base_uri(Uuid::new_v4())).await
        }

        #[tokio::test]
        async fn test_sl_pos_update_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let item1 = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;
            let mut item_c = an_item_c();
            item_c.ingredient = "Cheese".into();
            let item2 = ShoppingList::add_item(&state.mm, list_id, item_c, user_id).await?;
            let payload = HashMap::from([(item1.id, 1), (item2.id, 0)]);

            let res = server.put(&base_uri(list_id)).form(&payload).await;

            res.assert_status_ok();
            let item1_u = ShoppingList::get_item(&state.mm, list_id, item1.id, user_id).await?;
            let item2_u = ShoppingList::get_item(&state.mm, list_id, item2.id, user_id).await?;
            assert_eq!(item1_u.position, 1);
            assert_eq!(item2_u.position, 0);
            Ok(())
        }
    }

    mod tests_toggle_item {
        use super::*;

        fn base_uri(list_id: Uuid, item_id: i64) -> String {
            format!("/shopping/lists/{list_id}/items/{item_id}/toggle")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::POST, &base_uri(Uuid::new_v4(), 1)).await
        }

        #[tokio::test]
        async fn test_toggle_once_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let item = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;

            let res = server.post(&base_uri(list_id, item.id)).await;

            res.assert_status_ok();
            let item = ShoppingList::get_item(&state.mm, list_id, item.id, user_id).await?;
            assert!(item.is_checked);
            Ok(())
        }

        #[tokio::test]
        async fn test_toggle_twice_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            let list_id = ShoppingList::create(&state.mm, "Test", user_id).await?;
            let item = ShoppingList::add_item(&state.mm, list_id, an_item_c(), user_id).await?;

            let _ = server.post(&base_uri(list_id, item.id)).await;
            let res = server.post(&base_uri(list_id, item.id)).await;

            res.assert_status_ok();
            let item = ShoppingList::get_item(&state.mm, list_id, item.id, user_id).await?;
            assert!(!item.is_checked);
            Ok(())
        }
    }
}
