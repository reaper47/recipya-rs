use maud::{Markup, html};

use models::{
    data::{Data, ShoppingData},
    settings::UserSettingDetails,
};

use crate::templates::{icons::icon_plus_circle, layouts};

/// Renders the searchbar.
pub fn lists_index(path: &str, data: &Data, user_setting: &UserSettingDetails) -> Markup {
    let content = render_lists_index(data);

    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" { "Shopping Lists | Recipya" }
            span #data-layout data-layout="with-aside" hx-swap-oob="true" {}
            (content)
        } @else {
            (layouts::main("Shopping Lists", path, data, &content, user_setting, false))
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
                div #shopping-list-view-pane {
                    @if shopping.shopping_lists.is_empty() {
                        p class="p-4" {
                            "← Create your first shopping list to get started."
                        }
                    } @else {
                        // @let report = data.selected.as_ref().unwrap_or_else(|| data.reports.first().unwrap());
                        // (render_report(&report.report_type.primary, &report.report_logs))
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
                hx-post="/shopping/list"
                hx-prompt="Name of the new shopping list:"
                hx-target="#shopping-list-menu"
                hx-swap="beforeend" {
                (icon_plus_circle())
            }
        }
        ul #shopping-list-menu class={
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
                            p class="font-bold text-sm truncate" {
                                (list.name)
                            }
                            // p class="text-sm" {
                            //     "Execution time: " (report.format_duration())
                            // }
                        }
                        div class="flex flex-col items-end gap-1 shrink-0" {
                            div class="badge badge-xs badge-primary" {
                                // (report.report_type.primary.name)
                            }
                        }
                    }
                }
            }
        }
    }
}
