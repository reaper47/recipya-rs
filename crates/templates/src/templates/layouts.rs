use maud::{DOCTYPE, Markup, PreEscaped, html};

use super::core::{head, toast, toast_ws};
use super::icons::{
    icon_arrow_right_start_on_rectangle, icon_book_open, icon_cog_6_tooth, icon_flag, icon_pencil,
};
use crate::templates::pagination::pagination;
use models::data::Data;
use models::settings::UserSettingDetails;

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
pub fn main(
    title: &str,
    path: &str,
    data: &Data,
    content: Markup,
    user_settings: UserSettingDetails,
    is_hide_nav: bool,
) -> Markup {
    let data_layout = if is_hide_nav {
        "no-aside"
    } else {
        "with-aside"
    };

    html! {
        (DOCTYPE)
        html lang="en" class="h-full" {
            (head(title))
            body class="min-h-screen flex flex-col" hx-ext="ws" ws-connect="/ws"
                 _=(PreEscaped(format!("on load call initTheme('{}', '{}')", user_settings.default_theme, user_settings.selected_theme))) {
                span #data-layout data-layout=(data_layout) {}

                header class="navbar bg-base-200 shadow-sm print:hidden shrink-0" {
                    div class="navbar-start" {
                        a class="btn btn-ghost text-lg" style="padding-left: 0"
                          hx-get=@if data.is_authenticated { "/" }
                          hx-push-url=@if data.is_authenticated { "true" }
                          hx-target=@if data.is_authenticated { "#content" }
                          href=@if !data.is_authenticated { "/" } {
                            img src="/data/images/Icon/android-chrome-192x192.png" alt="Logo" style="width: 2rem";
                            "Recipya"
                        }
                    }
                    div class="navbar-center" {
                        @if data.is_authenticated {
                            // TODO: Check where to use this.
                            //div #content-title class="font-semibold hidden md:block md:text-xl" {
                            //    (title)
                            //}

                            @if path != "/admin" || path != "/cookbooks" || path != "/recipes/add" || path != "/recipes/add/manual" {
                                @if path == "/" || path == "/recipes" {
                                    (render_recipe_button())
                                } @else {
                                    (render_recipe_button())
                                }

                                @if path == "/cookbooks" {
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
                                    li class="cursor-pointer" onclick="document.querySelector('#settings-dialog').showModal()" {
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
                div #fullscreen-loader class="htmx-indicator" {}
                main class="flex w-full flex-1 min-h-0" {
                    @if data.is_authenticated {
                        (render_nav(path))
                    }
                    div #content class="flex-1 min-h-0" {
                        (content)
                    }
                }
                @if let Some(p) = &data.pagination {
                    (pagination(p))
                }
                (toast())
                (toast_ws("", "", false))
            }
        }
    }
}

/// Renders the button to go the add recipe page.
pub(super) fn render_recipe_button() -> Markup {
    html! {
        button
            #add-recipe
            class="btn btn-primary btn-sm hover:btn-accent"
            hx-get="/recipes/add"
            hx-target="#content"
            hx-trigger="mousedown"
            hx-push-url="true" {
            "Add recipe"
        }
    }
}

/// Renders the desktop navigation sidebar.
pub(super) fn render_nav(path: &str) -> Markup {
    html! {
        aside #desktop-nav {
            ul class="menu w-full menu-sm bg-base-300 rounded-box h-full gap-1" style="border-radius: 0" {
                li #recipes-sidebar-recipes
                    class={
                        "rounded-lg"
                        @if path == "/recipes" || path == "/" { " bg-secondary" }
                    }
                    hx-get="/recipes"
                    hx-target="#content"
                    hx-trigger="mousedown"
                    hx-push-url="true"
                    hx-swap="innerHTML transition:true" {
                    a class="tooltip tooltip-right active" data-tip="Recipes" {
                        (icon_pencil(false))
                    }
                }
                li #recipes-sidebar-cookbooks
                   hx-get="/cookbooks"
                   hx-target="#content"
                   hx-trigger="mousedown"
                   hx-push-url="true"
                   hx-swap="innerHTML transition:true" {
                     a class="tooltip tooltip-right" data-tip="Cookbooks" {
                        (icon_book_open())
                    }
                }
            }
        }

        aside #mobile-nav class="dock dock-sm md:hidden z-20" {
            button hx-get="/recipes" hx-target="#content" hx-push-url="true" hx-swap="innerHTML transition:true" {
                "Recipes"
            }
            button hx-get="/cookbooks" hx-target="#content" hx-push-url="true" hx-swap="innerHTML transition:true" {
                "Cookbooks"
            }
        }
    }
}
