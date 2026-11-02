use std::sync::Arc;

use l10n::Messages;

use config::DataDir;
use maud::{Markup, html};
use models::data::Data;
use models::settings::UserSettingDetails;
use support::fs::FsSupport;

use crate::recipes::common::list_recipes;
use crate::recipes::search_bar;
use crate::shopping::master_toggled_behaviour;
use crate::templates::helpers::{Replace, inject_markup_in_message};
use crate::templates::icons;
use crate::templates::layouts::{self, render_empty_nav_extra_content, render_recipe_button};
use crate::templates::pagination::pagination;

/// Renders the index page of recipes.
pub fn index(
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
    path: &str,
    data: &Data,
    data_dir: &DataDir,
    user_setting: &UserSettingDetails,
    messages: &Messages,
) -> Markup {
    if data.is_hx_request {
        html! {
            (render_index(fs_support, path, data, data_dir, messages))
            (render_empty_nav_extra_content())
            (render_recipe_button(true, messages))
        }
    } else {
        layouts::main(
            &messages.recipes(),
            path,
            data,
            &render_index(fs_support, path, data, data_dir, messages),
            messages,
            user_setting,
        )
    }
}

fn render_index(
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
    path: &str,
    data: &Data,
    data_dir: &DataDir,
    messages: &Messages,
) -> Markup {
    if data.recipes.is_empty() {
        let add_recipe_page_title = messages.add_recipe_page_title();
        let button_markup = html! {
            a class="underline font-semibold cursor-pointer" hx-get="/recipes/add" hx-target="#content" hx-push-url="true" {
                (add_recipe_page_title)
            }
        };

        html! {
            div class="grid place-content-center text-sm h-full text-center md:text-base" {
                div class="p-4 md:p-0" {
                    p .pb-2 {
                        (messages.recipe_page_empty_collection())
                    }
                    p {
                        (inject_markup_in_message(
                            &messages.recipe_page_collection_subtext(add_recipe_page_title.to_string()),
                            &[(&add_recipe_page_title, button_markup)], &Replace::All)
                        )
                    }
                }
            }
        }
    } else {
        html! {
            div #list-recipes-wrapper class="group/select" data-select="false" {
                (search_bar(data, messages))
                div #list-recipes .min-h-0  {
                    (list_recipes(fs_support, path, data, data_dir, messages))
                }
                @if data.is_hx_request && let Some(p) = &data.pagination {
                    (pagination(p, messages))
                }
                div class="grid invisible z-20 bg-base-200 fixed top-0 right-0 left-0 duration-300 group-data-[select=true]/select:block min-h-[4rem] opacity-0 transition-[opacity,visibility] group-data-[select=true]/select:visible group-data-[select=true]/select:opacity-100" {
                    p #num-recipes-selected class="text-sm mr-4 text-end" {
                        span { "0" } " " (messages.selected())
                    }
                    div #select-recipes-actions class="flex justify-center gap-2" {
                        button popovertarget="select-recipes-shopping"
                            class="btn btn-sm btn-soft" disabled autocomplete="off"
                            hx-get="/shopping/recipes/ingredients"
                            hx-include="#list-recipes input[name='ids']"
                            hx-target="#select-recipes-shopping" {
                            (icons::shopping_cart())
                            (messages.shopping_list_single_title())
                        }
                        button class="btn btn-sm btn-soft" disabled autocomplete="off" onclick="alert('Not implemented yet')" {
                            (icons::book_open())
                            (messages.cookbook())
                        }
                        button popovertarget="select-recipes-categorise" class="btn btn-sm btn-soft" disabled autocomplete="off" {
                            (icons::tag())
                            (messages.categorise())
                        }
                        button class="btn btn-sm btn-error btn-square"
                            disabled
                            hx-post="/recipes/bulk"
                            hx-include="#list-recipes input[name='ids']"
                            hx-confirm=(messages.delete_recipes_confirm()) {
                            (icons::trash())
                        }
                    }
                }
                (dialog_select_recipes_categorise(messages))
                div #select-recipes-shopping popover class="place-self-center" {
                    p {
                        "CAN YOU EAT SOME MOTHERFUCKING BULLSHIT YOU FUCKER LEAVE ME ALONE AND DIE IN THE DEPTHS OF THE FUCKING RIVER OF STYX"
                    }
                }
                (master_toggled_behaviour())
            }
        }
    }
}

fn dialog_select_recipes_categorise(messages: &Messages) -> Markup {
    html! {
        div #select-recipes-categorise popover class="place-self-center min-w-[15%]" {
            div class="card" {
                form class="card-body contents" style="padding: 0"
                    enctype="application/x-www-form-urlencoded"
                    hx-post="/recipes/bulk/categorise"
                    hx-on::after-request="if (event.detail.successful) this.closest('[popover]').hidePopover()"
                    hx-include="#list-recipes input[name='ids']" {
                    div class="card-actions justify-between" {
                        h2 .card-title {
                            (messages.categorise_recipes_title())
                        }
                        button class="btn btn-square btn-sm" onclick="document.getElementById('select-recipes-categorise').hidePopover()" {
                            (icons::x_mark())
                        }
                    }
                    p class="text-sm mb-2" {
                        (messages.categorise_recipes_description())
                    }
                    div class="flex justify-between" {
                        div class="mb-2 w-full" {
                            fieldset .fieldset {
                                input #category list="select-categories" type="text" list="categories" name="name"
                                    class="input input-sm w-11/12" placeholder=(messages.search_help_breakfast())
                                    autocomplete="off" value="" autofocus required;
                                datalist #select-categories hx-get="/recipes/categories" hx-trigger="load" {}
                            }
                        }
                        button class="btn btn-primary btn-sm" {
                            (messages.action_update())
                        }
                    }
                }
            }
        }
    }
}
