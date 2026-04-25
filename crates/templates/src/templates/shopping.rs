use maud::{Markup, html};

use models::{
    data::{Data, PaginationData, ShoppingData},
    settings::UserSettingDetails,
    shopping::ShoppingListItemDetails,
};
use uuid::Uuid;

use crate::templates::{
    icons::{
        icon_arrows_up_down, icon_carrot, icon_check, icon_pencil, icon_plus_circle, icon_scale,
        icon_trash,
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
                        } @else {
                            @match shopping.selected_shopping_list {
                                Some(ref list) => {
                                    div {
                                        (shopping_list_actions())
                                    }
                                    h1 class="text-2xl font-bold underline p-2" {
                                        (list.name)
                                    }
                                    div class="min-w-[30vw] place-self-center" {
                                        @if list.items.is_empty() {
                                            details open {
                                                summary class="text-left" {
                                                    "No label"
                                                }
                                                ol class="list bg-base-100 rounded-box shadow-md" {
                                                    (new_shopping_list_item(list.id, None))
                                                }
                                            }
                                        } @else {
                                            @let items = list.items_per_label();
                                            @for (label, items) in items {
                                                details open {
                                                    summary class="text-left" {
                                                        (label)
                                                    }
                                                    ol class="list bg-base-100 rounded-box shadow-md" {
                                                        @for item in items {
                                                            (render_shopping_list_item(item))
                                                        }
                                                        (new_shopping_list_item(list.id, Some(label)))
                                                    }
                                                }
                                            }
                                        }
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
                li class=[selected.as_ref().map_or(idx == 0, |l| l.id == list.id).then_some("bg-base-300")]
                    hx-get=(format!("/shopping/lists/{}", list.id))
                    hx-target="#shopping-list-view-pane"
                    hx-push-url="false"
                    hx-trigger="mousedown"
                    hx-on:mousedown=(format!(
                        "document.querySelectorAll('#shopping-list-menu li').forEach((el) => el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-shopping-list-id').value = '{}';", list.id
                    )) {
                    div class="flex justify-between items-center gap-2 w-full" {
                        div class="min-w-0" {
                            p class="font-bold text-sm" {
                                (list.name)
                            }
                        }
                        div class="flex flex-col items-end gap-1 shrink-0" {
                            div class="badge badge-xs badge-primary" {
                                (list.num_items)
                            }
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
        li class="bg-base-300"
            hx-get=(&format!("/shopping/lists/{list_id}"))
            hx-target="#shopping-list-view-pane"
            hx-push-url="false"
            hx-trigger="mousedown"
            hx-on:mousedown=(format!(
                "document.querySelectorAll('#shopping-list-menu li').forEach((el) => el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-shopping-list-id').value = '{list_id}';"
            )) {
            div class="flex justify-between items-center gap-2 w-full" {
                div class="min-w-0" {
                    p class="font-bold text-sm" {
                        (title.as_ref())
                    }
                }
                div class="flex flex-col items-end gap-1 shrink-0" {
                    div class="badge badge-xs badge-primary" {
                        (0)
                    }
                }
            }
        }
        div #shopping-list-view-pane hx-swap-oob="innerHTML" {
            div {
                (shopping_list_actions())
                h1 class="text-2xl font-bold underline p-2" {
                    (title.as_ref())
                }
                div class="grid" {
                    div class="min-w-[30vw] place-self-center" {
                        details open {
                            summary class="text-left" {
                                "No label"
                            }
                            ol class="list bg-base-100 rounded-box shadow-md" {
                                (new_shopping_list_item(list_id, None))
                            }
                        }
                    }
                }
            }
        }
    }
}

fn shopping_list_actions() -> Markup {
    html! {
        div class="join border border-gray-700 mb-2 w-fit" {
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
                "Share"
            }
            button class="btn join-item" {
                "Export"
            }
            button class="btn join-item" {
                "Upload to app"
            }
            button class="btn join-item" {
                "Delete"
            }
        }
    }
}

fn new_shopping_list_item(list_id: Uuid, label: Option<&str>) -> Markup {
    html! {
        li class="list-row grid grid-cols-[1fr_auto]" {
            div class="grid gap-1 min-w-0" {
                label class="input input-sm" {
                    (icon_carrot())
                    input required type="text" placeholder="Surloin steak" name="item";
                }
                label class="input input-sm" {
                    (icon_scale())
                    input type="text" placeholder="500g (optional)" name="quantity";
                }
                @if let Some(label) = label {
                    input type="hidden" name="label" value=(label);
                }
            }
            div class="grid grid-flow-col gap-1 place-self-end" {
                div class="grid grid-col gap-2 w-12" {
                    button class="btn join-item btn-sm"
                        hx-post=(format!("/shopping/lists/{list_id}/item"))
                        hx-include="closest li"
                        hx-target="closest li"
                        hx-swap="beforebegin" {
                        (icon_check())
                    }
                    button class="btn join-item btn-sm" {
                        (icon_trash())
                    }
                }
            }
        }
    }
}

/// Renders a shopping list item as an HTML list item.
pub fn render_shopping_list_item(item: &ShoppingListItemDetails) -> Markup {
    html! {
        li class="list-row grid grid-cols-[1fr_auto]" {
            div class="grid gap-1 min-w-0" {
                label class="label" {
                    input class="checkbox" type="checkbox" checked[item.is_checked] checked[item.is_checked];
                    @if let Some(q) = item.quantity.as_ref() && !q.is_empty() {
                        (format!("{} ({q})", item.ingredient))
                    } @else {
                        (format!("{}", item.ingredient))
                    }
                }
            }
            // div class="grid grid-flow-col gap-1" {
            //     div class="grid grid-col gap-2" {
            //         button class="btn join-item btn-sm" {
            //             (icon_pencil(false))
            //         }
            //         button class="btn join-item btn-sm" {
            //             (icon_trash())
            //         }
            //     }
            //     button class="btn join-item btn-sm cursor-grab h-full" {
            //         (icon_arrows_up_down())
            //     }
            // }
            div class="flex gap-1" {
                button class="btn join-item btn-sm" {
                    (icon_pencil(false))
                }
                button class="btn join-item btn-sm" {
                    (icon_trash())
                }
                button class="btn join-item btn-sm cursor-grab h-full" {
                    (icon_arrows_up_down())
                }
            }
        }
    }
}
