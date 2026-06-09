use maud::{Markup, PreEscaped, html};
use uuid::Uuid;

use models::{
    data::{Data, PaginationData, ShoppingData},
    settings::UserSettingDetails,
    shopping::{ShoppingList, ShoppingListDetails, ShoppingListItemDetails},
    view::ViewMode,
};

use crate::templates::{
    common::cancel_submit_form_actions,
    icons::{
        icon_arrow_down_tray, icon_carrot, icon_check, icon_check_circle, icon_clipboard_document,
        icon_paper_clip, icon_pencil, icon_plus, icon_plus_circle, icon_printer, icon_scale,
        icon_share, icon_trash,
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
            (content)
            (pagination(&PaginationData::hidden()))
        } @else {
            (layouts::main("Shopping Lists", path, data, &content, user_setting))
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
                div #shopping-list-container .hidden.md:block {
                    (render_shopping_lists_list(shopping, "shopping-lists"))
                }
            }
            div class="hidden md:flex order-1 divider my-0 md:order-2 md:divider-horizontal md:mx-0" {}
            div class="order-0 flex-1 overflow-y-auto min-h-0 md:order-3 max-h-[94vh]" {
                div #shopping-list-view-pane class="p-4 text-center grid" {
                    div class="grid" {
                        @if shopping.shopping_lists.is_empty() {
                            p class="text-left" {
                                "← Create your first shopping list to get started."
                            }
                            div id=[if data.is_hx_request { Some("navbar-actions") } else { None }]
                                hx-swap-oob=[if data.is_hx_request { Some("true") } else { None }]
                                hidden=[if data.is_hx_request { None } else { Some("") }]
                                class="join border border-gray-700 mb-2 w-fit" {}
                        } @else {
                            @match shopping.selected_shopping_list {
                                Some(ref list) => {
                                    (match shopping.selected_view_mode {
                                        ViewMode::Edit | ViewMode::Print => render_shopping_list_view_edit(list, data.is_hx_request),
                                        ViewMode::View => render_shopping_list_view_view(list, data.is_hx_request),
                                    })

                                    @if data.is_hx_request {
                                        (render_shopping_list_actions(data.is_hx_request, &shopping.selected_view_mode, list.id))
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

        @if let Some(labels) = &shopping.labels {
            datalist id="labels" {
                @for label in labels {
                    option { (label) }
                }
            }
        }

        @if data.is_hx_request {
            (render_shopping_list_nav(shopping))
        }
    }
}

pub(super) fn render_shopping_list_nav(shopping: &ShoppingData) -> Markup {
    html! {
        div #navbar-extra-content class="lg:hidden w-full p-2 bg-base-100 flex-1 min-h-0" hx-swap-oob="true" {
            div .divider.my-0 {}
            (render_shopping_lists_list(shopping, "shopping-lists-nav"))
            div .divider.my-0 {}
            @if let Some(list) = &shopping.selected_shopping_list {
                @let list_id = list.id;
                div .grid.gap-1 {
                    (action_view_button(&shopping.selected_view_mode, list_id))
                    (action_export_button())
                    (action_print_button(list_id))
                    (action_share_button(list_id))
                    (action_delete_button(list_id))
                }
            }
        }
    }
}

fn render_shopping_lists_list(shopping: &ShoppingData, container_id: &str) -> Markup {
    let shopping_lists = shopping.shopping_lists.as_slice();
    let selected = shopping.selected_shopping_list.as_ref();

    html! {
        div class="grid grid-flow-col gap-2 place-items-center" {
            p class="text-center font-semibold text-lg underline" {
                "Shopping Lists"
            }
            button type="button"
                class="btn btn-xs btn-square btn-ghost"
                hx-post="/shopping/lists"
                hx-prompt="Name of the new shopping list:"
                hx-target=(format!("#{container_id}"))
                hx-swap="afterbegin"
                hx-on:htmx:after-request="Array.from(document.getElementById('shopping-lists').children).forEach((item) => item.classList.remove('bg-base-300')); document.getElementById('shopping-lists').firstElementChild.classList.add('bg-base-300')" {
                (icon_plus_circle())
            }
        }
        ul id=(container_id) class={
            "menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0"
            @if shopping_lists.len() < 10 { " md:h-full" }
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

/// Renders the shopping list items section for a given label.
pub fn render_shopping_list_items<T: AsRef<str>>(
    label: T,
    list_id: Uuid,
    items: &[&ShoppingListItemDetails],
    is_add_new_label: bool,
) -> Markup {
    let label = label.as_ref();
    let label_no_spaces = label
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    html! {
        details open {
            (render_label(label, list_id, items.first().map_or(1, |i| i.label_id)))
            ol id=(format!("shopping-list-items-container-{label_no_spaces}"))
                class="list bg-base-100 rounded-box shadow-md"
                data-drag-list
                data-list-id=(list_id) {
                @for item in items {
                    (render_shopping_list_item(list_id, item, false))
                }
                (new_shopping_list_item(list_id, Some(label)))
            }
        }
        @if is_add_new_label {
            (add_label(list_id))
        }
    }
}

/// Renders the shopping list in edit mode.
pub fn render_shopping_list_view_edit(list: &ShoppingListDetails, is_hx_request: bool) -> Markup {
    html! {
        div {
            (shopping_list_title(list.id, &list.name))
            div class="grid" {
                (item_sections(list))
            }
        }
        @if is_hx_request {
            (render_shopping_list_actions(true, &ViewMode::Edit, list.id))
        }
    }
}

fn item_sections(list: &ShoppingListDetails) -> Markup {
    html! {
        div class="w-full max-w-[33rem] place-self-center" {
            @if list.items.is_empty() {
                details open {
                    (render_label("No label", list.id, 1))
                    ol #shopping-list-items-container class="list bg-base-100 rounded-box shadow-md" {
                        (new_shopping_list_item(list.id, None))
                    }
                }
            } @else {
                @let items = list.items_per_label();
                @for (label, items) in items {
                    (render_shopping_list_items(label, list.id, items.as_slice(), false))
                }
            }
            (add_label(list.id))
        }
    }
}

/// Renders the new shopping list.
pub fn render_new_shopping_list<T: AsRef<str>>(
    selected_mode: &ViewMode,
    list_id: Uuid,
    title: T,
) -> Markup {
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
                (shopping_list_title(list_id, title))
                div class="grid" {
                    div class="w-full max-w-[33rem] place-self-center" {
                        details open {
                            (render_label("No label", list_id, 1))
                            ol class="list bg-base-100 rounded-box shadow-md" {
                                (new_shopping_list_item(list_id, None))
                            }
                        }
                        (add_label(list_id))
                    }
                }
            }
        }
        (render_shopping_list_actions(true, selected_mode, list_id))
    }
}

pub fn render_shopping_list_item_count(list_id: Uuid, num_items: i64, is_swap_oob: bool) -> Markup {
    html! {
        div id=(format!("shopping-list-item-count-{list_id}"))
            class="badge badge-xs badge-primary"
            hx-swap-oob=[is_swap_oob.then_some("true")] {
            (num_items)
        }
    }
}

/// Render shopping list actions.
#[allow(clippy::too_many_lines)]
pub(super) fn render_shopping_list_actions(
    is_oob_swap: bool,
    selected_view_mode: &ViewMode,
    list_id: Uuid,
) -> Markup {
    html! {
        div id=[is_oob_swap.then_some("navbar-actions")]
            hx-swap-oob=[is_oob_swap.then_some("true")] {
            div class="hidden md:block join border border-gray-700 mb-2 w-fit" {
                (action_view_button(selected_view_mode, list_id))
                // TODO: Implement upload to apps (todoist)
                // button class="btn join-item" {
                //     "Upload to app"
                // }
                (action_export_button())
                (action_print_button(list_id))
                (action_share_button(list_id))
                (action_delete_button(list_id))
            }
        }

        div #shopping-list-copy-popover
            class="dropdown dropdown-right w-full md:w-auto rounded-box bg-base-100 shadow-sm"
            popover  {
            ul class="list bg-base-200 rounded-box shadow-md" {
                li class="list-row p-0 pl-4" {
                    p .place-content-center {
                        "Text"
                    }
                    div class="place-self-end grid grid-flow-col" {
                        button title="Copy" class="btn btn-square btn-ghost"
                            hx-get=(format!("/shopping/lists/{list_id}/copy?format=text"))
                            hx-swap="none"
                            hx-on--after-request="copyHtmxResponseToClipboard(event)"
                            _="on click call #shopping-list-copy-popover.hidePopover()" {
                            (icon_clipboard_document())
                        }
                        form title="Download"
                            hx-get=(format!("/shopping/lists/{list_id}/export?format=text"))
                            hx-swap="none"
                            hx-on:download-ready="document.getElementById('shopping-list-copy-popover').hidePopover(); window.location.href = event.detail.url;" {
                                button title="Download" class="btn btn-square btn-ghost" {
                                    (icon_arrow_down_tray())
                                }
                            }
                    }
                }
                li class="list-row p-0 pl-4" {
                    p .place-content-center {
                        "Markdown"
                    }
                    div class="place-self-end grid grid-flow-col" {
                        button title="Copy" class="btn btn-square btn-ghost"
                            hx-get=(format!("/shopping/lists/{list_id}/copy?format=markdown"))
                            hx-swap="none"
                            hx-on--after-request="copyHtmxResponseToClipboard(event)"
                            _="on click call #shopping-list-copy-popover.hidePopover()" {
                            (icon_clipboard_document())
                        }
                        form title="Download"
                            hx-get=(format!("/shopping/lists/{list_id}/export?format=markdown"))
                            hx-swap="none"
                            hx-on:download-ready="document.getElementById('shopping-list-copy-popover').hidePopover(); window.location.href = event.detail.url;" {
                                button title="Download" class="btn btn-square btn-ghost" {
                                    (icon_arrow_down_tray())
                                }
                            }
                    }
                }
                li class="list-row p-0 pl-4" {
                    p .place-content-center {
                        "PDF"
                    }
                    div class="place-self-end grid grid-flow-col" {
                        form title="Download"
                            hx-get=(format!("/shopping/lists/{list_id}/export?format=pdf"))
                            hx-swap="none"
                            hx-on:download-ready="document.getElementById('shopping-list-copy-popover').hidePopover(); window.location.href = event.detail.url;" {
                                button title="Download" class="btn btn-square btn-ghost" {
                                    (icon_arrow_down_tray())
                                }
                            }
                    }
                }
            }
        }
    }
}

fn action_view_button(selected_mode: &ViewMode, list_id: Uuid) -> Markup {
    html! {
        div class="dropdown w-full md:w-auto" {
            div tabindex="0" role="button" class="btn btn-sm btn-wide md:btn-md md:join-item md:w-auto" {
                "View Mode"
                span class="hidden md:inline mb-1" { "⌄" }
                span class="inline md:hidden" { "→" }
            }
            form tabindex="0"
                class="menu dropdown-content bg-base-200 w-32 text-lg pr-2 left-full top-0 md:left-0 md:top-full"
                hx-get=(format!("/shopping/lists/{list_id}/view"))
                hx-target="#shopping-list-view-pane"
                hx-trigger="change"
                hx-swap="innerHTML transition:true"
                onchange="setTimeout(() => document.activeElement.blur(), 25)" {
                fieldset class="fieldset flex" {
                    label class="label cursor-pointer text-inherit w-full p-2" {
                        input type="radio"
                            name="mode"
                            autocomplete="off"
                            class="radio radio-sm view-option"
                            value="edit"
                            checked=[if selected_mode == &ViewMode::Edit { Some("") } else { None }];

                        span class="ml-1" {
                            "Edit"
                        }
                    }
                }
                fieldset class="fieldset flex" {
                    label class="label cursor-pointer text-inherit w-full p-2" {
                        input type="radio"
                            name="mode"
                            autocomplete="off"
                            class="radio radio-sm view-option"
                            value="view"
                            checked=[if selected_mode == &ViewMode::View { Some("") } else { None }];

                        span class="ml-1" {
                            "View"
                        }
                    }
                }
            }
        }
    }
}

fn action_export_button() -> Markup {
    html! {
        button type="button" class="btn btn-sm btn-wide md:btn-md md:join-item md:w-auto"
            style="anchor-name:--anchor-copy-list"
            popovertarget="shopping-list-copy-popover" {
            "Export"
            span class="hidden md:inline mb-1" { "⌄" }
            span class="inline md:hidden" { "→" }
        }
    }
}

fn action_print_button(list_id: Uuid) -> Markup {
    html! {
        a #print-shopping-list-button
            title="Print list"
            class="btn btn-sm btn-wide md:btn-md md:join-item md:w-auto"
            href=(format!("/shopping/lists/{list_id}/print"))
            target="_blank" {
            span .hidden.md:block {
                (icon_printer())
            }
            span .md:hidden { "Print" }
        }
    }
}

fn action_share_button(list_id: Uuid) -> Markup {
    html! {
        button type="button" title="Share list"
            class="btn btn-sm btn-wide md:btn-md md:join-item md:w-auto"
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
                        open #share-dialog
                end" {
            span .hidden.md:block {
                (icon_share())
            }
            span .md:hidden {
                "Share"
            }
        }
    }
}

fn action_delete_button(list_id: Uuid) -> Markup {
    html! {
        button type="button" title="Delete list"
            class="btn btn-sm btn-wide md:btn-md md:join-item md:w-auto"
            hx-delete=(format!("/shopping/lists/{list_id}")) hx-confirm="Are you sure you wish to delete this list?" {
            span .hidden.md:block {
                (icon_trash())
            }
            span .md:hidden {
                "Delete"
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

    let notes = item
        .as_ref()
        .map(|i| i.notes.as_deref().unwrap_or_default())
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
            label class="input input-sm" {
                (icon_paper_clip())
                input type="text" placeholder="Notes (optional)" name="notes" value=(notes) autocomplete="off";
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
        div {
            h1 class="text-2xl font-bold underline p-2" {
                (title.as_ref())
                span class="ml-2" {
                    button type="button" class="btn join-item btn-square btn-sm"
                        _="on click add .hidden to closest <h1/> then remove .hidden from next <form/> from closest <h1/> then call (next <input/> from closest <h1/>).select()" {
                        (icon_pencil(false))
                    }
                }
            }
            form class="hidden" hx-put=(format!("/shopping/lists/{list_id}")) hx-swap="outerHTML" {
                h1 class="text-2xl font-bold underline p-2" {
                    input type="text" required
                        name="name"
                        class="input input-lg text-center mr-1"
                        value=(title.as_ref())
                        list="labels"
                        autocomplete="off";

                    span class="ml-2" {
                        button class="btn join-item btn-square btn-lg" {
                            (icon_check())
                        }
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
        li class="list-row grid grid-cols-[1fr_auto]" data-item-id=(item.id) data-drag-row {
            div class="grid grid-flow-col" data-drageable draggable="true" {
                div class="grid gap-1 min-w-0" {
                    label class="label text-base-content" {
                        input type="checkbox"
                            class="checkbox peer"
                            hx-post=(format!("/shopping/lists/items/{}/toggle", item.id))
                            checked[item.is_checked];

                        (render_list_item_details(item, &ViewMode::Edit))
                    }
                }
                @if !readonly {
                    div class="flex gap-1 place-content-end" {
                        button class="btn join-item btn-square btn-sm [li:has(input:checked)_&]:hidden transition-all"
                            hx-target="closest li"
                            hx-swap="outerHTML"
                            hx-get=(format!("/shopping/lists/{list_id}/items/{}/edit", item.id)) {
                            (icon_pencil(false))
                        }
                        button class="btn join-item btn-square btn-sm [li:has(input:checked)_&]:opacity-50 transition-all"
                            hx-target="closest li"
                            hx-swap="delete"
                            hx-delete=(format!("/shopping/lists/{list_id}/items/{}", item.id)) {
                            (icon_trash())
                        }
                        div class="inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl [li:has(input:checked)_&]:hidden transition-all" data-drag-handle {
                            "⠿"
                        }
                    }
                }
            }
        }
    }
}

/// Renders the label.
pub fn render_label<T: AsRef<str>>(label: T, list_id: Uuid, label_id: i64) -> Markup {
    let id = format!("label-{label_id}");

    html! {
        summary id=(id) class="text-left cursor-default" {
            // View mode
            span {
                (label.as_ref())
                button class="btn join-item btn-square btn-sm ml-2 mb-1"
                    _="on click
                        add .hidden to closest <span/>
                        remove .hidden from next <span/> from closest <span/>
                        add .inline-flex to next <span/> from closest <span/>
                        call (next <input/> from closest <span/>).select()
                        call (next <input/> from closest <span/>).focus()" {
                    (icon_pencil(false))
                }
            }

            // Edit mode
            span class="hidden text-left cursor-default " {
                form .flex hx-target="closest summary" hx-swap="outerHTML" hx-put=(format!("/shopping/lists/{list_id}/labels/{label_id}")) {
                    input type="text"
                        required
                        name="name"
                        class="input input-sm"
                        value=(label.as_ref())
                        list="labels"
                        autocomplete="off";

                    span {
                        button class="btn join-item btn-square btn-sm ml-2 mb-1" {
                            (icon_check())
                        }
                    }
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
                .map(|l| render_shopping_list_view_view(l, data.is_hx_request))
                .unwrap_or_default()
        },
    );

    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" { "Shopping Lists | Recipya" }
            (content)
            (pagination(&PaginationData::hidden()))
        } @else {
            (layouts::main("Shopping Lists", path.as_ref(), data, &content, user_settings))
            (pagination(&PaginationData::hidden()))
        }
    }
}

/// Renders the shopping list view in view mode.
pub fn render_shopping_list_view_view(list: &ShoppingListDetails, is_hx_request: bool) -> Markup {
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
        @if is_hx_request {
            (render_shopping_list_actions(true, &ViewMode::View, list.id))
        }
    }
}

fn add_label(list_id: Uuid) -> Markup {
    html! {
        div .divider {
            div class="grid grid-flow-col gap-2 w-full" {
                button class="btn btn-sm btn-outline" hx-get=(format!("/shopping/lists/{list_id}/labels/new")) hx-swap="outerHTML" {
                    "Add label"
                }
            }
        }
    }
}

pub fn render_label_new(list_id: Uuid) -> Markup {
    html! {
        form hx-post=(format!("/shopping/lists/{list_id}/labels")) hx-target="closest div.divider" hx-swap="outerHTML" {
            input type="text"
                autofocus
                required
                name="name"
                class="input input-sm gap-1"
                list="labels"
                autocomplete="off";

            button type="submit" class="btn btn-sm btn-ghost" {
                (icon_check_circle())
            }
        }
    }
}

pub fn render_shopping_list_print_mode(list: &ShoppingListDetails) -> Markup {
    let render_list = |items: &[&ShoppingListItemDetails]| {
        html! {
            ul {
                @for item in items {
                    li style="list-style-type: none;" {
                        label style="display: flex;" {
                            input class="checkbox" type="checkbox" style="margin-right: .5rem;";
                            (render_list_item_details(item, &ViewMode::Print))
                        }
                    }
                }
            }
        }
    };

    html! {
        h1 style="text-align: center; text-decoration: underline;" {
            (list.name)
        }
        div {
            @if list.items.is_empty() {
                p {
                    "Shopping list has no items."
                }
            } @else {
                @let items = list.items_per_label();
                @for (label, items) in items {
                    @if label == "No label" {
                        (render_list(items.as_slice()))
                    } @else {
                        details open {
                            summary {
                                (label)
                            }
                            (render_list(items.as_slice()))
                        }
                    }
                }
            }
        }
    }
}

fn render_list_item_details(item: &ShoppingListItemDetails, view: &ViewMode) -> Markup {
    let div_class = match view {
        ViewMode::Edit | ViewMode::View => Some(
            "text-left [input:checked~&]:line-through [input:checked~&]:opacity-50 transition-all",
        ),
        ViewMode::Print => None,
    };

    let p_style = match view {
        ViewMode::Edit | ViewMode::View => None,
        ViewMode::Print => Some("margin-bottom: 0.25rem; margin-top: 0.25rem;"),
    };

    let notes_class = match view {
        ViewMode::Edit | ViewMode::View => Some("text-xs font-light mt-1"),
        ViewMode::Print => None,
    };

    let notes_style = match view {
        ViewMode::Edit | ViewMode::View => None,
        ViewMode::Print => Some(
            "font-size: 0.75rem; line-height: 1.2; font-weight: 300; margin: 0; margin-top: 0.25rem",
        ),
    };

    html! {
        div class=[div_class] {
            @if let Some(q) = item.quantity.as_ref() && !q.is_empty() {
                p style=[p_style] {
                    (format!("{} ({q})", item.ingredient))
                }
            } @else {
                p style=[p_style] {
                    (format!("{}", item.ingredient))
                }
            }
            @if let Some(notes) = item.notes.as_deref() {
                p class=[notes_class] style=[notes_style] { (notes) }
            }
            @if let Some(recipe) = &item.recipe {
                p class=[notes_class] style=[notes_style] {
                    "For "
                    a href=(format!("/recipes/{}", recipe.id)) target="_blank" class="link" { (recipe.name) }
                }
            }
        }
    }
}

/// Stores the components of an ingredient row.
pub struct AddShoppingIngredient {
    pub name: String,
    pub quantity: Option<String>,
    pub notes: Option<String>,
}

/// Renders the ingredient selector to shopping list dialog.
pub fn render_recipe_add_shopping_dialog() -> Markup {
    html! {
        dialog #add-to-shopping-list-dialog .modal {}
        script type="text/hyperscript" {
            (PreEscaped(r#"
                behavior OnMasterToggled
                    on masterToggled
                        js(me) return me.closest('tr').querySelector('input.checkbox-ingredient').checked end
                        if it
                            remove @disabled from me
                        else
                            add @disabled to me
                        end
                    end
                    on change
                        if I match .with-quantity
                            if my.checked
                                set my previousElementSibling's value to 'true'
                            else
                                set my previousElementSibling's value to 'false'
                            end

                            js(me) return me.closest('tr').querySelector('input[name="quantities"]') end
                            if my.checked
                                remove .opacity-50 from it
                            else
                                add .opacity-50 to it
                            end
                        end
                    end
                end
            "#))
        }
    }
}

/// Renders the ingredient selection content for the add to shopping list dialog.
pub fn render_recipe_add_shopping_dialog_content(
    recipe_id: i64,
    shopping_lists: Vec<ShoppingList>,
    ingredients: Vec<AddShoppingIngredient>,
) -> Markup {
    html! {
        form class="card bg-base-100 shadow-sm"
            hx-post=(format!("/shopping/recipes/{recipe_id}/ingredients"))
            hx-indicator="#add-ingredients-submit-button-spinner"
            hx-on--after-request="document.querySelector('#add-to-shopping-list-dialog').close()" {
            div class="card-body" {
                h3 class="mb-1 grid grid-flow-col" {
                    p .text-lg { "Add ingredients to shopping list" }
                    @if shopping_lists.is_empty() {
                        input type="text" required class="input input-sm max-w-sm" placeholder="New shopping list name" name="list" value="";
                    } @else {
                        select #add-to-shopping-list-select class="select" name="list" required {
                          option disabled { "Pick a shopping list" }
                          @for list in shopping_lists {
                              option value=(list.id) { (&list.name) }
                          }
                        }
                    }
                }
                div class="overflow-auto h-[50vh]" {
                    table class="table table-zebra table-sm" {
                        thead {
                            tr .text-center {
                                th class="py-1 text-left" {
                                    input type="checkbox" checked class="checkbox"
                                        _="on change set <input.checkbox-ingredient/>'s checked to my checked then send masterToggled to <input.ingredient-element/> then call checkDataSubmit('checkbox-ingredient', 'add-ingredients-submit-button')";
                                }
                                th .py-1 { "Ingredient" }
                                th .py-1 { "Quantity" }
                                th .py-1 { "Notes" }
                                th .py-1 { "Include" br; "Quantity" }
                            }
                        }
                        tbody {
                            @for ing in ingredients {
                                tr {
                                    td .py-1 {
                                        input type="checkbox" checked class="checkbox checkbox-ingredient"
                                            _="on change or change from <input.master-checkbox/> for el in <input.ingredient-element/> in closest <tr/> toggle @disabled on el end then call checkDataSubmit('checkbox-ingredient', 'add-ingredients-submit-button')";
                                    }
                                    td class="max-w-[30ch] py-1" {
                                        input type="hidden" name="ingredients" value=(ing.name) .ingredient-element _="install OnMasterToggled";
                                        (ing.name)
                                    }
                                    td .py-1.text-center {
                                        input type="text" name="quantities" placeholder="1 cup" class="input input-sm max-w-sm ingredient-element" value=(ing.quantity.unwrap_or_else(|| "-".into())) _="install OnMasterToggled";
                                    }
                                    td .py-1.text-center {
                                        input type="text" name="notes" placeholder="large" class="input input-sm max-w-sm ingredient-element" value=(ing.notes.unwrap_or_else(|| "-".into())) _="install OnMasterToggled";
                                    }
                                    td class="py-1 text-center select-none" {
                                        input type="hidden" .ingredient-element name="with-quantity" value="true" _="install OnMasterToggled";
                                        input type="checkbox" checked class="checkbox ingredient-element with-quantity" _="install OnMasterToggled";
                                    }
                                }
                            }
                        }
                    }
                }
                (cancel_submit_form_actions(&html! { "Submit" }, "add-ingredients-submit-button", false))
            }
        }
    }
}
