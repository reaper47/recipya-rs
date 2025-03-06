use maud::{html, Markup, DOCTYPE};

use crate::server::templates::core::{head, toast, toast_ws};
use crate::server::templates::data::Data;
use crate::server::templates::icons::{
    icon_arrow_right_start_on_rectangle, icon_book_open, icon_building_library, icon_cog_6_tooth,
    icon_flag, icon_pencil,
};

/// Renders the authentication layout template.
pub fn auth(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" class="h-full bg-indigo-100 dark:bg-gray-800" {
            (head(title))
            body .h-full.grid.place-content-center {
                (content)
            }
            (toast())
        }
    }
}

/// Renders the main layout template.
pub fn main(title: &str, data: &Data, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" class="h-full" _="on htmx:afterSwap
		        if location.pathname is '/recipes' or location.pathname is '/' then
                    add .active to first <button/> in mobile_nav then
                    remove .active from last <button/> in mobile_nav then
                    remove .md:hidden from desktop_nav then
                    remove .hidden from mobile_nav then
                    remove .active from first <a/> in recipes_sidebar_cookbooks then
                    add .active to first <a/> in recipes_sidebar_recipes
                else if location.pathname.startsWith('/cookbooks') then
                    add .active to last <button/> in mobile_nav then
                    remove .active from first <button/> in mobile_nav then
                    remove .md:hidden from desktop_nav then
                    remove .hidden from mobile_nav then
                    remove .active from first <a/> in recipes_sidebar_recipes then
                    add .active to first <a/> in recipes_sidebar_cookbooks
                else if location.pathname is '/settings' or location.pathname.startsWith('/recipes/add') then
                    add .md:hidden to desktop_nav then
                    add .hidden to mobile_nav
                end" {
            (head(title))
            body class="min-h-full" hx-ext="ws" ws-connect="/ws" {
                header class="navbar bg-base-200 shadow-sm print:hidden" {
                    div class="navbar-start" {
                        a class="btn btn-ghost text-lg" style="padding-left: 0"
                         hx-get=@if data.is_authenticated { "/" }
                          hx-push-url=@if data.is_authenticated { "true" }
                          hx-target=@if data.is_authenticated { "#content" }
                          href=@if !data.is_authenticated { "/" } {
                            img src="/static/android-chrome-192x192.png" alt="Logo" style="width: 2rem";
                            "Recipya"
                        }
                    }
                    div class="navbar-center" {
                        @if data.is_authenticated {
                            div id="content-title" class="font-semibold hidden md:block md:text-xl" {
                                (title)
                            }
                            button
                                id="add_recipe"
                                class="btn btn-primary btn-sm hover:btn-accent"
                                hx-get="/recipes/add"
                                hx-target="#content"
                                hx-trigger="mousedown"
                                hx-push-url="true" {
                                "Add recipe"
                            }
                            button
                                id="add_cookbook"
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
                    div class="navbar-end" {
                        @if data.is_authenticated {
                            button title="Open avatar menu"
                                   popovertarget="avatar_menu"
                                   popovertargetaction="toggle"
                                   class={
                                       @if data.about.is_update_available { "indicator" }
                                   }
                                   hx-get="/user-initials"
                                   hx-trigger="load"
                                   hx-target="#user-initials" {
                                div tabindex="0"
                                    role="button"
                                    class={
                                       "btn btn-ghost btn-circle avatar placeholder"
                                       @if data.about.is_update_available { " indicator" }
                                    } {
                                    @if data.about.is_update_available {
                                        span class="indicator-item indicator-start badge badge-sm badge-secondary z-30" {
                                            "New update"
                                        }
                                    }
                                    div class="bg-neutral text-neutral-content w-10 rounded-full" {
                                        span id="user-initials" {
                                            "A"
                                        }
                                    }
                                }
                            }
                            div id="avatar_menu"
                                popover
                                style="inset: unset; top: 3.5rem; right: 0.5rem;"
                                class="rounded-box z-10 shadow bg-base-200"
                                _="on click if me.matches(':popover-open') then me.hidePopover()" {
                                ul tabindex="0" class="menu" {
                                    @if data.is_admin {
                                        li onclick="document.activeElement?.blur()" {
                                            a href="/admin" hx-get="/admin" hx-target="#content" hx-push-url="true" {
                                                (icon_building_library())
                                                "Admin"
                                            }
                                        }
                                    }
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
                                    li class="cursor-pointer" onclick="settings_dialog.showModal()" {
                                        a hx-get="/settings" hx-target="#settings_dialog_content" {
                                            (icon_cog_6_tooth())
                                            "Settings"
                                        }
                                    }
                                    @if !data.is_autologin {
                                        div class="divider m-0" {}
                                        li {
                                            a hx-post="/auth/logout" {
                                                (icon_arrow_right_start_on_rectangle())
                                                "Log out"
                                            }
                                        }
                                    }
                                }
                            }
                            dialog id="settings_dialog" class="modal" {
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
                                    div id="settings_dialog_content" {
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
                    div id="fullscreen-loader" class="htmx-indicator" {}
                    main class="inline-flex w-full" {
                        @if data.is_authenticated {
                            aside id="desktop_nav" class="hidden md:block" {
                                ul class="menu w-full menu-sm bg-base-300 rounded-box h-full" style="border-radius: 0" {
                                    li id="recipes_sidebar_recipes" hx-get="/recipes" hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap-oob="true" hx-swap="innerHTML transition:true" {
                                        a class="tooltip tooltip-right active" data-tip="Recipes" {
                                            (icon_pencil())
                                        }
                                    }
                                    li id="recipes_sidebar_cookbooks"
                                       hx-get="/cookbooks"
                                       hx-target="#content"
                                       hx-trigger="mousedown"
                                       hx-push-url="true"
                                       hx-swap-oob="true"
                                       hx-swap="innerHTML transition:true" {
                                         a class="tooltip tooltip-right" data-tip="Cookbooks" {
                                            (icon_book_open())
                                        }
                                    }
                                }
                            }
                            aside id="mobile_nav" class="dock dock-sm md:hidden z-20" {
                                button hx-get="/recipes" hx-target="#content" hx-push-url="true" hx-swap-oob="true" hx-swap="innerHTML transition:true" {
                                    "Recipes"
                                }
                                button hx-get="/cookbooks" hx-target="#content" hx-push-url="true" hx-swap-oob="true" hx-swap="innerHTML transition:true" {
                                    "Cookbooks"
                                }
                            }
                        }
                        div id="content" class="min-h-[92.5vh] w-full" {
                            (content)
                        }
                    }
                }
                (toast())
                (toast_ws("", "", false))
            }
        }
    }
}
