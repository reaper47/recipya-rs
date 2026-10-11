use l10n::Messages;
use maud::{DOCTYPE, Markup, PreEscaped, html};

use config::AutologinState;
use models::data::Data;
use models::settings::UserSettingDetails;
use models::view::ViewMode;

use super::core::{head, toast, toast_sse};
use super::icons;
use crate::shopping::{render_shopping_list_actions, render_shopping_list_mobile};
use crate::templates::pagination::pagination;

/// Renders the authentication layout template.
pub fn auth(title: &str, content: &Markup, messages: &Messages) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" class="h-full dark:bg-gray-900" {
            (head(title))
            body .h-full.grid.place-content-center {
                (content)
            }
            (toast(messages))
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
    messages: &Messages,
    user_settings: &UserSettingDetails,
) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" .h-full {
            (head(title))
            body class="min-h-screen flex flex-col" hx-ext="sse" sse-connect="/sse" _=(PreEscaped(format!("on load call initTheme('{}', '{}')", user_settings.default_theme, user_settings.selected_theme))) {
                div sse-swap="message" style="display:none" {}
                div class="drawer lg:drawer-open" _="on htmx:afterSwap[target is #content] from body set #side-drawer-nav.checked to false" {
                    input #side-drawer-nav type="checkbox" .drawer-toggle;

                    div class="drawer-content flex flex-col min-h-screen" {
                        header class="navbar bg-base-200 shadow-sm print:hidden shrink-0" {
                            div .navbar-start {
                                @if data.is_authenticated {
                                    label for="side-drawer-nav" class="btn btn-ghost btn-circle lg:hidden" aria-label=(messages.nav_sidebar_aria_label()) {
                                        svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor" class="my-1.5 inline-block size-4" {
                                            path d="M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z" {}
                                            path d="M9 4v16" {}
                                            path d="M14 10l2 2l-2 2" {}
                                        }
                                    }
                                }
                                a class="hidden btn btn-ghost text-lg md:flex" style="padding-left: 0"
                                  hx-get=[data.is_authenticated.then_some("/")]
                                  hx-push-url=[data.is_authenticated.then_some("true")]
                                  hx-target=[data.is_authenticated.then_some("#content")]
                                  href=[if data.is_authenticated { None } else { Some("/") }]
                                  hx-swap="innerHTML transition:true" {
                                    img src="/data/images/Icon/android-chrome-192x192.png" alt=(messages.main_logo_alt()) style="width: 2rem";
                                    (messages.main_logo_title())
                                }
                            }
                            div #navbar-center .navbar-center {
                                @if data.is_authenticated {
                                    @if path != "/admin" || path != "/cookbooks" || path != "/recipes/add" || path != "/recipes/add/manual" {
                                        div #navbar-actions {
                                            @if path == "/" || path == "/recipes" {
                                                (render_recipe_button(false, messages))
                                            } @else if path.starts_with("/shopping") && !data.shopping.as_ref().is_none_or(|s| s.shopping_lists.is_empty()) {
                                                @let list = data.shopping.as_ref().and_then(|s| s.selected_shopping_list.as_ref().map(|l| l.id)).unwrap_or_default();
                                                @let mode = data.shopping.as_ref().map_or(&ViewMode::Edit, |s| &s.selected_view_mode);

                                                (render_shopping_list_actions(false, mode, list, messages))
                                            } @else if path == "/cookbooks" {
                                                button #addcookbook
                                                    class="btn btn-primary btn-sm hover:btn-accent"
                                                    hx-post="/cookbooks"
                                                    hx-prompt=(messages.nav_sidebar_new_cookbook_prompt())
                                                    hx-target="#cookbooks-display"
                                                    hx-trigger="mousedown"
                                                    hx-swap="beforeend" {
                                                    (messages.nav_sidebar_add_cookbook())
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div .navbar-end {
                                @if data.is_authenticated {
                                    button title=(messages.avatar_menu_button_title())
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
                                                    (messages.avatar_menu_update_available())
                                                }
                                            }
                                            div class="bg-neutral text-neutral-content w-10 rounded-full" {
                                                span #user-initials {
                                                    "A"
                                                }
                                            }
                                        }
                                    }
                                    div #avatar-menu popover
                                        style="inset: unset; top: 3.5rem; right: 0.5rem;"
                                        class="rounded-box z-10 shadow bg-base-200"
                                        _="on click if me.matches(':popover-open') then me.hidePopover()" {
                                        ul tabindex="0" .menu {
                                            li onclick="document.activeElement?.blur()" {
                                                a href="/reports" hx-get="/reports" hx-target="#content" hx-push-url="true" {
                                                    (icons::flag())
                                                    (messages.reports())
                                                }
                                            }
                                            div class="divider m-0" {}
                                            li onclick="document.activeElement?.blur()" {
                                                a href="https://recipya.musicavis.ca/docs" target="_blank" {
                                                    (icons::book_open())
                                                    (messages.avatar_menu_guide())
                                                }
                                            }
                                            li class="cursor-pointer" _="on click open #settings-dialog" {
                                                a hx-get="/settings" hx-target="#settings-dialog-content" {
                                                    (icons::cog_6_tooth())
                                                    (messages.settings())
                                                }
                                            }
                                            @if data.states.autologin == AutologinState::Off {
                                                div class="divider m-0" {}
                                                li {
                                                    form method="post" action="/auth/logout" .w-full {
                                                        button type="submit" class="flex cursor-pointer gap-2" {
                                                            (icons::arrow_right_start_on_rectangle())
                                                            (messages.logout())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    dialog #settings-dialog .modal {
                                        div class="toast-container-dialog toast toast-top toast-end hidden z-20 cursor-default" {}
                                        div class="modal-box p-1 max-w-lg w-[95%] h-3/5 md:h-[unset] md:max-w-3xl" {
                                            form method="dialog" {
                                                button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" {
                                                    "✕"
                                                }
                                            }
                                            h4 class="font-semibold text-lg p-4" {
                                                (messages.settings())
                                            }
                                            div #settings-dialog-content {
                                                p class="grid place-items-center p-12" {
                                                    (messages.settings_dialog_content_loading())
                                                }
                                            }
                                        }
                                        form method="dialog" .modal-backdrop {
                                            button .cursor-auto {}
                                        }
                                    }
                                } @else {
                                    a href="/auth/login" class="btn btn-ghost" {
                                        (messages.login())
                                    }
                                    a href="/auth/register" class="btn btn-ghost" {
                                        (messages.signup())
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
                                (pagination(p, messages))
                            }
                        }
                    }

                    @if data.is_authenticated {
                        div class="drawer-side z-40 is-drawer-close:overflow-visible" {
                            label for="side-drawer-nav" class="drawer-overlay" aria-label=(messages.nav_sidebar_close()) {}
                                div class="flex flex-col items-start bg-base-200 flex-1 min-h-0 h-full" {
                                    (render_nav(path, messages))
                                    @if let Some(shopping) = &data.shopping {
                                        (render_shopping_list_mobile(shopping, messages))
                                    } @else {
                                       (render_empty_nav_extra_content())
                                    }
                                }
                        }
                    }
                }

                div #fullscreen-loader .htmx-indicator {}
                (toast_sse("", "", false, messages))
            }
        }
    }
}

/// Renders the button to go the add recipe page.
pub(super) fn render_recipe_button(is_oob_swap: bool, messages: &Messages) -> Markup {
    html! {
        button
            id=(if is_oob_swap { "navbar-actions" } else { "add-recipe" })
            class="btn btn-outline btn-sm sm:btn-sm hover:btn-accent"
            hx-swap-oob=[is_oob_swap.then_some("true")]
            hx-get="/recipes/add"
            hx-target="#content"
            hx-trigger="mousedown"
            hx-push-url="true"
            hx-swap="innerHTML transition:true" {
            (messages.add_recipe_page_title())
        }
    }
}

/// Renders the desktop navigation sidebar.
pub(super) fn render_nav(path: &str, messages: &Messages) -> Markup {
    html! {
        ul class="menu w-full" {
            li #recipes-sidebar-recipes
                class={
                    "sidebar-item lg:is-drawer-close:tooltip lg:is-drawer-close:tooltip-right"
                    @if path.starts_with("/recipes") { " bg-secondary-content dark:bg-secondary" }
                }
                data-tip=(messages.recipes())
                hx-get="/recipes"
                hx-target="#content"
                hx-trigger="mousedown"
                hx-push-url="true"
                hx-swap="innerHTML transition:true" {
                a {
                    (icons::utensils())
                    span class="is-drawer-close:hidden" {
                        (messages.recipes())
                    }
                }
            }
            li #recipes-sidebar-cookbooks
                class="sidebar-item lg:is-drawer-close:tooltip lg:is-drawer-close:tooltip-right"
                data-tip=(messages.cookbooks())
                hx-get="/cookbooks"
                hx-target="#content"
                hx-trigger="mousedown"
                hx-push-url="true"
                hx-swap="innerHTML transition:true"
                _="on mousedown call alert('Not implemented yet')" {
                    a {
                    (icons::book_open())
                    span class="is-drawer-close:hidden" {
                        (messages.cookbooks())
                    }
                }
            }
            li #recipes-sidebar-shopping
                class={
                    "sidebar-item lg:is-drawer-close:tooltip lg:is-drawer-close:tooltip-right"
                    @if path.starts_with("/shopping") { " bg-secondary-content dark:bg-secondary" }
                }
                data-tip=(messages.shopping())
                hx-get="/shopping/lists"
                hx-target="#content"
                hx-trigger="mousedown"
                hx-push-url="true"
                hx-swap="innerHTML transition:true" {
                    a {
                    (icons::shopping_cart())
                    span class="is-drawer-close:hidden" {
                        (messages.shopping())
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
