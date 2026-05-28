use maud::{DOCTYPE, Markup, PreEscaped, html};

use models::data::Data;
use models::settings::UserSettingDetails;

use super::core::{head, toast, toast_ws};
use super::icons::{
    icon_arrow_right_start_on_rectangle, icon_book_open, icon_cog_6_tooth, icon_flag,
    icon_shopping_cart,
};
use crate::shopping::{render_shopping_list_actions, render_shopping_list_nav};
use crate::templates::icons::icon_utensils;
use crate::templates::pagination::pagination;

/// Renders the authentication layout template.
pub fn auth(title: &str, content: &Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" class="h-full dark:bg-gray-900" {
            (head(title))
            body .h-full.grid.place-content-center {
                (content)
            }
            (toast())
            script {
                "window.addEventListener('load', function() {
                    initTheme('system', 'system');
                });"
            }
        }
    }
}

/// Renders the main layout template.
#[allow(clippy::too_many_lines)]
pub fn main(
    title: &str,
    path: &str,
    data: &Data,
    content: &Markup,
    user_settings: &UserSettingDetails,
) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" class="h-full" {
            (head(title))
            body class="min-h-screen flex flex-col" hx-ext="ws" ws-connect="/ws"
                 _=(PreEscaped(format!("on load call initTheme('{}', '{}')", user_settings.default_theme, user_settings.selected_theme))) {
                div class="drawer lg:drawer-open" _="on htmx:afterSwap[target is #content] from body set #side-drawer-nav.checked to false" {
                    input #side-drawer-nav type="checkbox" class="drawer-toggle";

                    div class="drawer-content flex flex-col min-h-screen" {
                        header class="navbar bg-base-200 shadow-sm print:hidden shrink-0" {
                            div class="navbar-start" {
                                @if data.is_authenticated {
                                    label for="side-drawer-nav" class="btn btn-ghost btn-circle lg:hidden" aria-label="Open menu sidebar" {
                                        svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor" class="my-1.5 inline-block size-4" {
                                            path d="M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z" {}
                                            path d="M9 4v16" {}
                                            path d="M14 10l2 2l-2 2" {}
                                        }
                                    }
                                }
                                a class="hidden btn btn-ghost text-lg md:flex" style="padding-left: 0"
                                  hx-get=[if data.is_authenticated { Some("/") } else { None }]
                                  hx-push-url=[if data.is_authenticated { Some("true") } else { None }]
                                  hx-target=[if data.is_authenticated { Some("#content") } else { None }]
                                  href=[if data.is_authenticated { None } else { Some("/") }]
                                  hx-swap="innerHTML transition:true" {
                                    img src="/data/images/Icon/android-chrome-192x192.png" alt="Logo" style="width: 2rem";
                                    "Recipya"
                                }
                            }
                            div #navbar-center class="navbar-center" {
                                @if data.is_authenticated {
                                    @if path != "/admin" || path != "/cookbooks" || path != "/recipes/add" || path != "/recipes/add/manual" {
                                        div #navbar-actions {
                                            @if path == "/" || path == "/recipes" {
                                                (render_recipe_button(false))
                                            } @else if path.starts_with("/shopping") && !data.shopping.as_ref().is_none_or(|s| s.shopping_lists.is_empty()) {
                                                (render_shopping_list_actions(false, data
                                                    .shopping
                                                    .as_ref()
                                                    .and_then(|s| s.selected_shopping_list.as_ref().map(|l| l.id))
                                                    .unwrap_or_default()))
                                            } @else if path == "/cookbooks" {
                                                button
                                                    #addcookbook
                                                    class="btn btn-primary btn-sm hover:btn-accent"
                                                    hx-post="/cookbooks"
                                                    hx-prompt="Enter the name of your cookbook"
                                                    hx-target="#cookbooks-display"
                                                    hx-trigger="mousedown"
                                                    hx-swap="beforeend" {
                                                    "Add cookbook"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div class="navbar-end" {
                                @if data.is_authenticated {
                                    button title="Open avatar menu"
                                           popovertarget="avatar-menu"
                                           popovertargetaction="toggle"
                                           class={
                                               @if data.about.is_update_available { "indicator" }
                                           }
                                           hx-get="/user-initials"
                                           hx-trigger="load"
                                           hx-target="#user-initials" {
                                        div tabindex="0" role="button" class={
                                               "btn btn-ghost btn-circle avatar avatar-placeholder"
                                               @if data.about.is_update_available { " indicator" }
                                            } {
                                            @if data.about.is_update_available {
                                                span class="indicator-item indicator-start badge badge-sm badge-secondary z-30" {
                                                    "New update"
                                                }
                                            }
                                            div class="bg-neutral text-neutral-content w-10 rounded-full" {
                                                span #user-initials {
                                                    "A"
                                                }
                                            }
                                        }
                                    }
                                    div #avatar-menu
                                        popover
                                        style="inset: unset; top: 3.5rem; right: 0.5rem;"
                                        class="rounded-box z-10 shadow bg-base-200"
                                        _="on click if me.matches(':popover-open') then me.hidePopover()" {
                                        ul tabindex="0" class="menu" {
                                            li onclick="document.activeElement?.blur()" {
                                                a href="/reports" hx-get="/reports" hx-target="#content" hx-push-url="true" {
                                                    (icon_flag())
                                                    "Reports"
                                                }
                                            }
                                            div class="divider m-0" {}
                                            li onclick="document.activeElement?.blur()" {
                                                a href="https://recipya.musicavis.ca/docs" target="_blank" {
                                                    (icon_book_open())
                                                    "Guide"
                                                }
                                            }
                                            li class="cursor-pointer" _="on click open #settings-dialog" {
                                                a hx-get="/settings" hx-target="#settings-dialog-content" {
                                                    (icon_cog_6_tooth())
                                                    "Settings"
                                                }
                                            }
                                            @if !data.is_autologin {
                                                div class="divider m-0" {}
                                                li {
                                                    form method="post" action="/auth/logout" class="w-full" {
                                                        button type="submit" class="flex cursor-pointer gap-2" {
                                                            (icon_arrow_right_start_on_rectangle())
                                                            "Log out"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    dialog #settings-dialog class="modal" {
                                        div class="toast-container-dialog toast toast-top toast-end hidden z-20 cursor-default" {}
                                        div class="modal-box p-1 max-w-lg w-[95%] h-3/5 md:h-[unset] md:max-w-3xl" {
                                            form method="dialog" {
                                                button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" {
                                                    "✕"
                                                }
                                            }
                                            h4 class="font-semibold text-lg p-4" {
                                                "Settings"
                                            }
                                            div #settings-dialog-content {
                                                p class="grid place-items-center p-12" {
                                                    "Content is loading..."
                                                }
                                            }
                                        }
                                        form method="dialog" class="modal-backdrop" {
                                            button class="cursor-auto" {}
                                        }
                                    }
                                } @else {
                                    a href="/auth/login" class="btn btn-ghost" {
                                        "Log In"
                                    }
                                    a href="/auth/register" class="btn btn-ghost" {
                                        "Sign Up"
                                    }
                                }
                            }
                        }

                        main class="flex w-full flex-1 min-h-0 overflow-hidden" {
                            div #content class="flex-1 pb-0" {
                                (content)
                            }
                        }

                        footer #pagination-anchor class="footer footer-center bg-base-200 p-2 gap-2 mt-auto shrink-0" style="grid-auto-flow: row;" {
                            @if let Some(p) = &data.pagination {
                                (pagination(p))
                            }
                        }
                    }

                    @if data.is_authenticated {
                        div class="drawer-side z-40 is-drawer-close:overflow-visible" {
                            label for="side-drawer-nav" class="drawer-overlay" aria-label="Close sidebar" {}
                                div class="flex flex-col items-start bg-base-200 flex-1 min-h-0 h-full" {
                                    (render_nav(path))
                                    @if let Some(shopping) = &data.shopping {
                                        (render_shopping_list_nav(shopping))
                                    } @else {
                                       (render_empty_nav_extra_content())
                                    }
                                }
                        }
                    }
                }

                div #fullscreen-loader class="htmx-indicator" {}
                (toast())
                (toast_ws("", "", false))
            }
        }
    }
}

/// Renders the button to go the add recipe page.
pub(super) fn render_recipe_button(is_oob_swap: bool) -> Markup {
    html! {
        button
            id=(if is_oob_swap { "navbar-actions" } else { "add-recipe" })
            class="btn btn-outline btn-sm sm:btn-sm hover:btn-accent"
            hx-swap-oob=[if is_oob_swap { Some("true") } else { None }]
            hx-get="/recipes/add"
            hx-target="#content"
            hx-trigger="mousedown"
            hx-push-url="true"
            hx-swap="innerHTML transition:true" {
            "Add recipe"
        }
    }
}

/// Renders the desktop navigation sidebar.
pub(super) fn render_nav(path: &str) -> Markup {
    html! {
        ul class="menu w-full" {
            li #recipes-sidebar-recipes
                class="sidebar-item lg:is-drawer-close:tooltip lg:is-drawer-close:tooltip-right"
                data-tip="Recipes"
                hx-get="/recipes"
                hx-target="#content"
                hx-trigger="mousedown"
                hx-push-url="true"
                hx-swap="innerHTML transition:true" {
                a {
                    (icon_utensils())
                    span class="is-drawer-close:hidden" {
                        "Recipes"
                    }
                }
            }
            li #recipes-sidebar-cookbooks
                class="sidebar-item lg:is-drawer-close:tooltip lg:is-drawer-close:tooltip-right"
                data-tip="Cookbooks"
                hx-get="/cookbooks"
                hx-target="#content"
                hx-trigger="mousedown"
                hx-push-url="true"
                hx-swap="innerHTML transition:true"
                _="on mousedown call alert('Not implemented yet')" {
                    a {
                    (icon_book_open())
                    span class="is-drawer-close:hidden" {
                        "Cookbooks"
                    }
                }
            }
            li #recipes-sidebar-shopping
                class={
                    "sidebar-item lg:is-drawer-close:tooltip lg:is-drawer-close:tooltip-right"
                    @if path.starts_with("/shopping") { " bg-secondary-content" }
                }
                data-tip="Shopping"
                hx-get="/shopping/lists"
                hx-target="#content"
                hx-trigger="mousedown"
                hx-push-url="true"
                hx-swap="innerHTML transition:true" {
                    a {
                    (icon_shopping_cart())
                    span class="is-drawer-close:hidden" {
                        "Shopping"
                    }
                }
            }
        }
    }
}

pub(super) fn render_empty_nav_extra_content() -> Markup {
    html! {
         div #navbar-extra-content .hidden hx-swap-oob="true" {}
    }
}
