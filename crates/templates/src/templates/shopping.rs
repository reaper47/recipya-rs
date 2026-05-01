use maud::{Markup, html};
use uuid::Uuid;

use models::{
    data::{Data, PaginationData, ShoppingData},
    settings::UserSettingDetails,
    shopping::{ShoppingListDetails, ShoppingListItemDetails},
};

use crate::templates::{
    icons::{
        icon_arrows_up_down, icon_carrot, icon_check, icon_pencil, icon_plus, icon_plus_circle,
        icon_scale, icon_share, icon_trash,
    },
    layouts,
    pagination::pagination,
};

/// Renders the searchbar.
pub fn lists_index(path: &str, data: &Data, user_setting: &UserSettingDetails) -> Markup {
    let content = render_lists_index(data);

    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" { "Shopping Lists | Recipya" }
            span #data-layout data-layout="with-aside" hx-swap-oob="true" {}
            (content)
            (pagination(&PaginationData::hidden()))
        } @else {
            (layouts::main("Shopping Lists", path, data, &content, user_setting, false))
            (pagination(&PaginationData::hidden()))
        }
    }
}

fn render_lists_index(data: &Data) -> Markup {
    html! {
        @if !matches!(&data.share, Some(share) if share.is_shared) {
             dialog #share-dialog .modal {
                div class="modal-box w-4/5 sm:w-96" {
                    div #share-dialog-result {}
                    div class="modal-action block mt-4" {
                        form method="dialog" {
                            button class="btn btn-block btn-outline btn-sm" {
                                "Close"
                            }
                        }
                    }
                }
            }
        }

        @let Some(shopping) = data.shopping.as_ref() else {
            return html! {
                p { "No shopping data provided." }
            };
        };

        div #shopping-lists-index class="flex flex-col-reverse md:flex-row h-full" {
            aside class="relative max-h-full text-center pt-1 pl-2" {
                input #selected-shopping-list-id type="hidden" name="selected"
                      value=(shopping.selected_shopping_list.as_ref().map(|r| r.id).unwrap_or_default());
                div #shopping-list-container {
                    (render_shopping_lists_list(shopping))
                }
            }
            div class="order-1 divider my-0 md:order-2 md:divider-horizontal md:mx-0" {}
            div class="order-0 flex-1 overflow-y-auto min-h-0 md:order-3 max-h-[94vh]" {
                div #shopping-list-view-pane class="p-4 text-center grid" {
                    div class="grid" {
                        @if shopping.shopping_lists.is_empty() {
                            p class="text-left" {
                                "← Create your first shopping list to get started."
                            }
                            div id=[if data.is_hx_request { Some("navbar-actions") } else { None }]
                                hx-swap-oob=[if data.is_hx_request { Some("true") } else { None }]
                                class="join border border-gray-700 mb-2 w-fit" {}
                        } @else {
                            @match shopping.selected_shopping_list {
                                Some(ref list) => {
                                    (shopping_list_title(list.id, &list.name))
                                    (item_sections(list))

                                    @if data.is_hx_request {
                                        (render_shopping_list_actions(data.is_hx_request, list.id))
                                    }
                                }
                                None => {
                                    p class="text-left" {
                                        "← Select a shopping list to view its items."
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn item_sections(list: &ShoppingListDetails) -> Markup {
    html! {
        div class="min-w-[33rem] place-self-center" {
            @if list.items.is_empty() {
                details open {
                    (render_label("No label", list.id, 1))
                    ol class="list bg-base-100 rounded-box shadow-md" {
                        (new_shopping_list_item(list.id, None))
                    }
                }
            } @else {
                @let items = list.items_per_label();
                @for (label, items) in items {
                    details open {
                        summary class="text-left cursor-default" {
                            (label)
                            button class="btn join-item btn-square btn-sm ml-2 mb-1"
                                hx-target="closest summary"
                                hx-swap="outerHTML"
                                hx-get=(format!("/shopping/lists/{}/labels/{}/edit", list.id, items.first().map_or(1, |i| i.label_id))) {
                                (icon_pencil(false))
                            }
                        }
                        ol class="list bg-base-100 rounded-box shadow-md" {
                            @for item in items {
                                (render_shopping_list_item(list.id, item, false))
                            }
                            (new_shopping_list_item(list.id, Some(label)))
                        }
                    }
                }
            }
        }
    }
}

/// Renders the shopping list.
pub fn render_shopping_list(list: &ShoppingListDetails) -> Markup {
    html! {
        div {
            (shopping_list_title(list.id, &list.name))
            div class="grid" {
                (item_sections(list))
            }
        }
    }
}

fn render_shopping_lists_list(shopping: &ShoppingData) -> Markup {
    let shopping_lists = shopping.shopping_lists.as_slice();
    let selected = shopping.selected_shopping_list.as_ref();

    html! {
        div class="grid grid-flow-col gap-2 place-items-center" {
            p class="text-center font-semibold text-lg underline" {
                "Shopping Lists"
            }
            button class="btn btn-xs btn-square btn-ghost"
                hx-post="/shopping/lists"
                hx-prompt="Name of the new shopping list:"
                hx-target="#shopping-lists"
                hx-swap="afterbegin"
                hx-on:htmx:after-request="Array.from(document.getElementById('shopping-lists').children).forEach((item) => item.classList.remove('bg-base-300')); document.getElementById('shopping-lists').firstElementChild.classList.add('bg-base-300')" {
                (icon_plus_circle())
            }
        }
        ul #shopping-lists class={
            "menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0"
            @if shopping_lists.len() < 10 { " h-full" }
        } {
            @for (idx, list) in shopping_lists.iter().enumerate() {
                li id=(format!("shopping-list-sidebar-{}", list.id))
                    class=[selected.as_ref().map_or(idx == 0, |l| l.id == list.id).then_some("bg-base-300")]
                    hx-get=(format!("/shopping/lists/{}", list.id))
                    hx-target="#shopping-list-view-pane"
                    hx-push-url="false"
                    hx-trigger="mousedown"
                    hx-on:mousedown=(format!(
                        "document.querySelectorAll('#shopping-lists li').forEach((el) => el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-shopping-list-id').value = '{}';", list.id
                    )) {
                    div class="flex justify-between items-center gap-2 w-full" {
                        div class="min-w-0" {
                            p class="font-bold text-sm" {
                                (list.name)
                            }
                        }
                        div class="flex flex-col items-end gap-1 shrink-0" {
                            (render_shopping_list_item_count(list.id, list.num_items, false))
                        }
                    }
                }
            }
        }
    }
}

/// Renders the new shopping list.
pub fn render_new_shopping_list<T: AsRef<str>>(list_id: Uuid, title: T) -> Markup {
    html! {
        li id=(format!("shopping-list-sidebar-{list_id}"))
            class="bg-base-300"
            hx-get=(&format!("/shopping/lists/{list_id}"))
            hx-target="#shopping-list-view-pane"
            hx-push-url="false"
            hx-trigger="mousedown"
            hx-on:mousedown=(format!(
                "document.querySelectorAll('#shopping-lists li').forEach((el) => el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-shopping-list-id').value = '{list_id}';"
            )) {
            div class="flex justify-between items-center gap-2 w-full" {
                div class="min-w-0" {
                    p class="font-bold text-sm" {
                        (title.as_ref())
                    }
                }
                div class="flex flex-col items-end gap-1 shrink-0" {
                    (render_shopping_list_item_count(list_id, 0, false))
                }
            }
        }
        div #shopping-list-view-pane hx-swap-oob="innerHTML" {
            div {
                h1 class="text-2xl font-bold underline p-2" {
                    (title.as_ref())
                    span class="ml-2" {
                        button class="btn join-item btn-square btn-sm"
                            hx-target="closest h1"
                            hx-swap="outerHTML"
                            hx-get=(format!("/shopping/lists/{list_id}/edit")) {
                            (icon_pencil(false))
                        }
                    }
                }
                div class="grid" {
                    div class="min-w-[33rem] place-self-center" {
                        details open {
                            (render_label("No label", list_id, 1))
                            ol class="list bg-base-100 rounded-box shadow-md" {
                                (new_shopping_list_item(list_id, None))
                            }
                        }
                    }
                }
            }
        }
        (render_shopping_list_actions(true, list_id))
    }
}

pub fn render_shopping_list_item_count(list_id: Uuid, num_items: i64, is_swap_oob: bool) -> Markup {
    html! {
        div id=(format!("shopping-list-item-count-{list_id}"))
            class="badge badge-xs badge-primary"
            hx-swap-oob=[if is_swap_oob { Some("true") } else { None }] {
            (num_items)
        }
    }
}

/// Render shopping list actions.
pub(super) fn render_shopping_list_actions(is_oob_swap: bool, list_id: Uuid) -> Markup {
    html! {
        div id=[if is_oob_swap { Some("navbar-actions") } else { None }]
            hx-swap-oob=[if is_oob_swap { Some("true") } else { None }]
            class="join border border-gray-700 mb-2 w-fit" {
            button class="btn join-item" {
                "Toggle Recipes"
            }
            button class="btn join-item" {
                "View (default)"
            }
            button class="btn join-item" {
                "Copy"
            }
            button class="btn join-item" {
                "Export"
            }
            button class="btn join-item" {
                "Upload to app"
            }
            button title="Share list"
                class="btn join-item"
                hx-post=(format!("/shopping/lists/{list_id}/share"))
                hx-target="#share-dialog-result"
                hx-push-url="false"
                _="on htmx:afterRequest from me
                    if event.detail.successful
                        if navigator.canShare
                            set name to document.querySelector('[itemprop=name]').textContent then
                            set data to {title: name, text: name, url: document.querySelector('#share-dialog-result input').value} then
                            call navigator.share(data)
                        else
                            call #share-dialog.showModal()
                    end" {
                (icon_share())
            }
            button title="Delete list" class="btn join-item" hx-delete=(format!("/shopping/lists/{list_id}")) hx-confirm="Are you sure you wish to delete this list?" {
                (icon_trash())
            }
        }
    }
}

fn new_shopping_list_item(list_id: Uuid, label: Option<&str>) -> Markup {
    shopping_list_item(list_id, label, None)
}

/// Renders a new shopping list item form as an HTML list item.
pub fn shopping_list_item<T: AsRef<str>>(
    list_id: Uuid,
    label: Option<T>,
    item: Option<&ShoppingListItemDetails>,
) -> Markup {
    let ingredient = item
        .as_ref()
        .map(|i| i.ingredient.as_str())
        .unwrap_or_default();

    let quantity = item
        .as_ref()
        .map(|i| i.quantity.as_deref().unwrap_or_default())
        .unwrap_or_default();

    let label = html! {
        div class="grid gap-1 min-w-0" {
            label class="input input-sm" {
                (icon_carrot())
                input required autofocus type="text" placeholder="Surloin steak" name="item" value=(ingredient);
            }
            label class="input input-sm" {
                (icon_scale())
                input type="text" placeholder="500g (optional)" name="quantity" value=(quantity);
            }
            @if let Some(label) = label.map(|l| l.as_ref().to_string()) {
                input type="hidden" name="label" value=(label);
            }
        }
    };

    let action = |icon: Markup| {
        html! {
            div class="grid grid-flow-col gap-1 place-self-end" {
                div class="grid grid-col gap-2 w-12" {
                    button class="btn join-item btn-sm" {
                        (icon)
                    }
                }
            }
        }
    };

    html! {
        li class="list-row grid grid-cols-[1fr_auto]" {
            @if let Some(item) = item {
                form class="contents"
                    hx-put=(format!("/shopping/lists/{list_id}/items/{}", item.id))
                    hx-target="closest li"
                    hx-swap="outerHTML"
                    hx-on--after-request="if(event.detail.successful) { this.reset(); this.querySelector('input').focus(); }" {
                    (label)
                    (action(icon_check()))
                }
            } @else {
                form class="contents"
                    hx-post=(format!("/shopping/lists/{list_id}/items"))
                    hx-target="closest li"
                    hx-swap="beforebegin"
                    hx-on--after-request="if(event.detail.successful) { this.reset(); this.querySelector('input').focus(); }" {
                    (label)
                    (action(icon_plus()))
                }
            }
        }
    }
}

/// Renders the shopping list title.
pub fn render_shopping_list_title<T: AsRef<str>>(
    list_id: Uuid,
    title: T,
    num_items: i64,
) -> Markup {
    let title = title.as_ref();

    html! {
        (shopping_list_title(list_id, title))

        li id=(format!("shopping-list-sidebar-{list_id}"))
            class="bg-base-300"
            hx-swap-oob="true"
            hx-get=(format!("/shopping/lists/{list_id}"))
            hx-target="#shopping-list-view-pane"
            hx-push-url="false"
            hx-trigger="mousedown"
            hx-on:mousedown=(format!(
                "document.querySelectorAll('#shopping-lists li').forEach((el) => el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-shopping-list-id').value = '{list_id}'"
            )) {
            div class="flex justify-between items-center gap-2 w-full" {
                div class="min-w-0" {
                    p class="font-bold text-sm" {
                        (title)
                    }
                }
                div class="flex flex-col items-end gap-1 shrink-0" {
                    div class="badge badge-xs badge-primary" {
                        (num_items)
                    }
                }
            }
        }
    }
}

fn shopping_list_title<T: AsRef<str>>(list_id: Uuid, title: T) -> Markup {
    html! {
        h1 class="text-2xl font-bold underline p-2" {
            (title.as_ref())
            span class="ml-2" {
                button class="btn join-item btn-square btn-sm"
                    hx-target="closest h1"
                    hx-swap="outerHTML"
                    hx-get=(format!("/shopping/lists/{}/edit", list_id)) {
                    (icon_pencil(false))
                }
            }
        }
    }
}

/// Renders the shopping list title edit form.
pub fn render_shopping_list_title_edit<T: AsRef<str>>(list_id: Uuid, title: T) -> Markup {
    html! {
        form hx-put=(format!("/shopping/lists/{list_id}")) hx-swap="outerHTML" {
            h1 class="text-2xl font-bold underline p-2" {
                input required autofocus type="text" name="name" class="input input-lg text-center mr-1" value=(title.as_ref()) _="on load wait 50ms then call me.select()";
                span class="ml-2" {
                    button class="btn join-item btn-square btn-lg" {
                        (icon_check())
                    }
                }
            }
        }
    }
}

/// Renders a shopping list item with a count.
pub fn render_shopping_list_item_with_count(
    list_id: Uuid,
    item: &ShoppingListItemDetails,
    num_items: i64,
    readonly: bool,
) -> Markup {
    html! {
        (render_shopping_list_item(list_id, item, readonly))
        (render_shopping_list_item_count(list_id, num_items, true))
    }
}

/// Renders a shopping list item as an HTML list item.
pub fn render_shopping_list_item(
    list_id: Uuid,
    item: &ShoppingListItemDetails,
    readonly: bool,
) -> Markup {
    html! {
        li class="list-row grid grid-cols-[1fr_auto]" {
            div class="grid gap-1 min-w-0" {
                label class="label text-base-content" {
                    input class="checkbox peer" type="checkbox" checked[item.is_checked] checked[item.is_checked];
                    span class="peer-checked:line-through peer-checked:opacity-50 transition-all" {
                        @if let Some(q) = item.quantity.as_ref() && !q.is_empty() {
                            (format!("{} ({q})", item.ingredient))
                        } @else {
                            (format!("{}", item.ingredient))
                        }
                    }
                }
            }
            @if !readonly {
                div class="flex gap-1" {
                    button class="btn join-item btn-square btn-sm"
                        hx-target="closest li"
                        hx-swap="outerHTML"
                        hx-get=(format!("/shopping/lists/{list_id}/items/{}/edit", item.id)) {
                        (icon_pencil(false))
                    }
                    button class="btn join-item btn-square btn-sm"
                        hx-target="closest li"
                        hx-swap="delete"
                        hx-delete=(format!("/shopping/lists/{list_id}/items/{}", item.id)) {
                        (icon_trash())
                    }
                    button class="btn join-item btn-square btn-sm cursor-grab h-full" {
                        (icon_arrows_up_down())
                    }
                }
            }
        }
    }
}

/// Renders the label.
pub fn render_label<T: AsRef<str>>(label: T, list_id: Uuid, label_id: i64) -> Markup {
    html! {
        summary class="text-left cursor-default" {
            (label.as_ref())
            button class="btn join-item btn-square btn-sm ml-2 mb-1"
                hx-target="closest summary"
                hx-swap="outerHTML"
                hx-get=(format!("/shopping/lists/{list_id}/labels/{label_id}/edit")) {
                (icon_pencil(false))
            }
        }
    }
}

/// Renders the edit label form.
pub fn render_label_edit<T: AsRef<str>>(label: T, list_id: Uuid, label_id: i64) -> Markup {
    html! {
        summary class="text-left cursor-default" {
            input autofocus type="text" name="name" class="input input-sm" value=(label.as_ref()) _="on load wait 50ms then call me.select()";
            span {
                button class="btn join-item btn-square btn-sm ml-2 mb-1"
                    hx-include="closest summary"
                    hx-target="closest summary"
                    hx-swap="outerHTML"
                    hx-put=(format!("/shopping/lists/{list_id}/labels/{label_id}")) {
                    (icon_check())
                }
            }
        }
    }
}

pub fn render_view_shopping_list_details<T: AsRef<str>>(
    path: T,
    data: &Data,
    user_settings: &UserSettingDetails,
) -> Markup {
    let content = data.shopping.as_ref().map_or_else(
        || {
            html! {
                "No shopping list available."
            }
        },
        |shopping| {
            shopping
                .selected_shopping_list
                .as_ref()
                .map(shopping_list_view_mode)
                .unwrap_or_default()
        },
    );

    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" { "Shopping Lists | Recipya" }
            span #data-layout data-layout="no-aside" hx-swap-oob="true" {}
            (content)
            (pagination(&PaginationData::hidden()))
        } @else {
            (layouts::main("Shopping Lists", path.as_ref(), data, &content, user_settings, true))
            (pagination(&PaginationData::hidden()))
        }
    }
}

fn shopping_list_view_mode(list: &ShoppingListDetails) -> Markup {
    html! {
        div .p-2 {
            h1 class="text-center text-2xl font-bold underline p-2" {
                (list.name)
            }
            div class="grid" {
                @if list.items.is_empty() {
                    div class="place-self-center" {
                        p .text-center {
                            "Shopping list has no items."
                        }
                    }
                } @else {
                    div class="min-w-full sm:min-w-[33vw] place-self-center" {
                        @let items = list.items_per_label();
                        @for (label, items) in items {
                            details open {
                                summary class="text-left cursor-default" {
                                    (label)
                                }
                                ol class="list bg-base-100 rounded-box shadow-md" {
                                    @for item in items {
                                        (render_shopping_list_item(list.id, item,  true))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
