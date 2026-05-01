use std::sync::Arc;

use config::DataDir;
use maud::{Markup, html};
use models::data::Data;
use models::settings::UserSettingDetails;
use support::fs::FsSupport;

use crate::recipes::common::list_recipes;
use crate::recipes::search_bar;
use crate::templates::layouts::{self, render_recipe_button};
use crate::templates::pagination::pagination;

/// Renders the index page of recipes.
pub fn index(
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
    path: &str,
    data: &Data,
    data_dir: &DataDir,
    user_setting: &UserSettingDetails,
) -> Markup {
    if data.is_hx_request {
        html! {
            (render_index(fs_support, path, data, data_dir))
            (render_recipe_button(true))
        }
    } else {
        layouts::main(
            "Recipes",
            path,
            data,
            &render_index(fs_support, path, data, data_dir),
            user_setting,
            false,
        )
    }
}

fn render_index(
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
    path: &str,
    data: &Data,
    data_dir: &DataDir,
) -> Markup {
    if data.recipes.is_empty() {
        html! {
            span #data-layout data-layout="with-aside" hx-swap-oob="true" {}

            div class="grid place-content-center text-sm h-full text-center md:text-base" {
                div class="p-4 md:p-0" {
                    p class="pb-2" {
                        "Your recipe collection looks a bit empty at the moment."
                    }
                    p {
                        "Why not start adding recipes by clicking the "
                        a class="underline font-semibold cursor-pointer" hx-get="/recipes/add" hx-target="#content" hx-push-url="true" { "Add recipe" }
                        " button at the top?"
                    }
                }
            }
        }
    } else {
        html! {
            span #data-layout data-layout="with-aside" hx-swap-oob="true" {}

            (search_bar(data))
            div #list-recipes class="min-h-0" {
                (list_recipes(fs_support, path, data, data_dir))
            }
            @if data.is_hx_request && let Some(p) = &data.pagination {
                (pagination(p))
            }
        }
    }
}
