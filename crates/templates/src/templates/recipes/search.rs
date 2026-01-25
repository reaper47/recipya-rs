use std::sync::Arc;

use config::DataDir;
use maud::{Markup, html};
use models::data::Data;
use models::settings::UserSettingDetails;
use serde_json::json;
use support::fs::FsSupport;

use crate::recipes::common::list_recipes;
use crate::search::{render_search_favourites_button, search_help, searchbar};
use crate::templates::layouts;
use crate::templates::pagination::pagination;

/// Renders search results for recipes.
pub fn search_results(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    path: &str,
    data: Data,
    data_dir: DataDir,
    user_setting: UserSettingDetails,
) -> Markup {
    if data.is_hx_request {
        let is_fav = data
            .searchbar
            .as_ref()
            .map(|sb| sb.is_favourites)
            .unwrap_or(false);

        html! {
            (list_recipes(fs_support, path, &data, &data_dir))
            (render_search_favourites_button(is_fav, true))
            @if let Some(p) = &data.pagination {
                (pagination(p))
            }
        }
    } else {
        let content: Markup = html! {
            (search_bar(&data))
            (list_recipes(fs_support, path, &data, &data_dir))
        };

        layouts::main("Recipes", path, &data, content, user_setting, false)
    }
}

/// Renders the searchbar component.
pub fn search_bar(data: &Data) -> Markup {
    html! {
        div class="flex flex-col pb-2" {
            section class="grid w-full max-w-xl mx-auto px-4 pt-4" {
                search {
                    form
                        class="flex w-full"
                        hx-get="/recipes/search"
                        hx-vals=(
                            if let Some(p) = &data.pagination {
                                json!({
                                    "page": p.search.current_page
                                }).to_string()
                            } else {
                                String::new()
                            }
                        )
                        hx-target="#list-recipes"
                        hx-swap="outerHTML"
                        hx-push-url="true"
                        hx-trigger="submit, change target:.sort-option" {
                        @if let Some(s) = &data.searchbar {
                            (searchbar(s))
                        }
                    }
                }
            }
        }
        (search_help())
    }
}
