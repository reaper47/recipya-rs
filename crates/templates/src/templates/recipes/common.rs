use crate::recipes::render_favourite_button;
use crate::templates::helpers::cut_string;
use crate::templates::icons::icon_bars_3;
use config::DataDir;
use maud::{Markup, PreEscaped, html};
use models::data::Data;
use models::recipe::{Keyword, ToolRecipe};
use serde_json::json;
use std::sync::Arc;
use support::fs::FsSupport;

pub(super) fn add_tool(tool: Option<&ToolRecipe>) -> Markup {
    html! {
        li class="pb-2" {
            div class="grid grid-flow-col items-center" {
                label class="flex gap-1" {
                    div class="inline-block h-4 cursor-move handle mt-1" {
                        (icon_bars_3())
                    }
                    input type="text" name="tool" placeholder="1 frying pan" class="input input-bordered input-sm w-full"
                        value=(
                            tool
                                .map(|t| format!("{} {}", t.quantity, t.name))
                                .unwrap_or_default()
                        )
                        _="on keydown if event.key is 'Enter' halt the event then call addItem(event)";
                }
                div class="ml-2 flex gap-2" {
                    button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)" {
                        "+"
                    }
                    button type="button"
                        class="delete-button btn btn-square btn-sm btn-outline btn-error"
                        _=(PreEscaped("on click
                            if (closest <ol/>).childElementCount > 1
                                remove closest <li/>
                            else
                                set input to (closest <li/>).querySelector('input') then
                                set input.value to '' then
                                input.focus()")) { "-" }
                }
            }
        }
    }
}

pub(super) fn add_ingredient(name: &str) -> Markup {
    html! {
        li .pb-2 {
            div class="grid grid-flow-col items-center" {
                label class="flex gap-1" {
                    div class="inline-block h-4 cursor-move handle mt-1" {
                        (icon_bars_3())
                    }
                    input required type="text" name="ingredient" value=(name)
                            placeholder="1 cup of chopped onions"
                            class="input input-bordered input-sm w-full"
                            _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))";
                }
                div class="ml-2 flex gap-2" {
                    button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)" {
                        "+"
                    }
                    button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error"
                        _=(PreEscaped("on click
                            if (closest <ol/>).childElementCount > 1
                                remove closest <li/>
                            else
                                set input to (closest <li/>).querySelector('input') then
                                set input.value to '' then
                                input.focus()")) { "-" }
                }
            }
        }
    }
}

pub(super) fn add_instruction(name: &str) -> Markup {
    html! {
        li class="pt-2 md:pl-0" {
            div .flex {
                label class="w-11/12" {
                    textarea required name="instruction" rows="4" class="textarea textarea-bordered w-full"
                        placeholder="Mix all ingredients together"
                        _="on keydown if event.ctrlKey and event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))" {
                        (name)
                    }
                }
                div class="grid gap-2 ml-2" {
                    button type="button" class="btn btn-square btn-sm btn-outline btn-success" title="Shortcut: CTRL + Enter" onclick="addItem(event)" {
                        "+"
                    }
                    button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error"
                        _=(PreEscaped("on click
                            if (closest <ol/>).childElementCount > 1
                                remove closest <li/>
                            else
                                set input to (closest <li/>).querySelector('textarea') then
                                set input.value to '' then
                                input.focus()")) { "-" }
                    div class="h-4 cursor-move handle grid place-content-center" {
                        (icon_bars_3())
                    }
                }
            }
        }
    }
}

/// Renders a list of recipes.
pub fn list_recipes(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    path: &str,
    data: &Data,
    data_dir: &DataDir,
) -> Markup {
    let is_tile_deletable = data.searchbar.clone().unwrap_or_default().is_favourites
        && path.starts_with("/recipes/search");

    html! {
        article #list-recipes
                class="grid gap-4 p-4 text-sm place-items-center grid-cols-1 sm:grid-cols-2 md:m-auto md:max-w-7xl md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 md:text-base"
                data-layout="with-aside" {
            @for view in data.recipes.iter() {
                @let recipe = &view.recipe_details.recipe;
                @let details = &view.recipe_details;

                section class="card-side card-compact card-border bg-base-200 shadow-lg indicator w-full h-full flex flex-col sm:card" {
                    span class="hidden sm:block" {
                        (category_badge(&details.category, false))
                    }
                    figure class="relative cursor-pointer" hx-get=(format!("/recipes/{}", recipe.id)) hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true" {
                        img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full"
                            src=(match details.all_images().first() {
                                Some(&first_image) => {
                                    if !details.all_images().is_empty() && fs_support.is_file_exists(first_image, &data_dir.images, ".webp") {
                                        format!("/data/images/thumbnails/{first_image}.webp")
                                    } else {
                                        "/data/images/Placeholders/placeholder.recipe.webp".into()
                                    }
                                },
                                None => {
                                    "/data/images/Placeholders/placeholder.recipe.webp".into()
                                }
                            })
                            alt=(format!("Image of the {} recipe", recipe.name));

                        div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex" {
                            p class="p-2 text-sm" {
                                @match &recipe.description {
                                    Some(description) => (cut_string(description, 127)),
                                    None => ""
                                }
                            }
                        }
                        (render_favourite_button(recipe.id, recipe.is_favourite, is_tile_deletable, false))
                    }
                    div class="card-body h-full flex flex-col gap-2" {
                        h2 class="font-semibold line-clamp-2" {
                            (details.recipe.name)
                        }
                        div class="h-5" {
                            @if recipe.rating.is_some() {
                                (rating(&format!("rating-{}", recipe.id), recipe.rating, "rating-sm", true))
                            }
                        }
                        div class="max-h-16 overflow-y-auto" {
                            div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row" {
                                span class="sm:hidden" {
                                    (category_badge(&details.category, true))
                                }
                                @for kw in details.keywords.iter() {
                                    span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer"
                                        hx-get="/recipes/search" hx-target="#list-recipes"
                                        hx-push-url="true" hx-swap="innerHTML show:window:top transition:true"
                                        hx-vals=(json!({
                                            "q": format!(r#""tag": {kw}"#)
                                        }))
                                        _=(format!("on click put 'tag:{}' into #search-recipes.value", kw)) {
                                        (kw)
                                    }
                                }
                            }
                        }
                        div class="card-actions mt-auto" {
                            button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get=(format!("/recipes/{}", recipe.id))
                            hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" {
                                "View"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn category_badge(category: &str, is_inside_card: bool) -> Markup {
    html! {
        @if !category.contains(":") {
            span class={
                    "badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral"
                    @if !is_inside_card { " indicator-item indicator-center" }
                }
                hx-get="/recipes/search"
                hx-target="#list-recipes"
                hx-push-url="true"
                hx-swap="innerHTML show:window:top transition:true"
                hx-vals=(json!({
                    "q": format!(r#""cat": {category}"#)
                }))
                _=(format!("on click put \"cat:{category}\" into #search-recipes.value")) {
                (category)
            }
        } @else {
            span class={
                "badge badge-primary select-none cursor-pointer"
                @if !is_inside_card { " indicator-item indicator-center" }
            } {
                @for (i, sub_cat) in category.split(":").enumerate() {
                    @if i > 0 {
                        ":"
                    }
                    span class="hover:bg-neutral"
                        hx-get="/recipes/search" hx-target="#list-recipes"
                        hx-push-url="true" hx-swap="innerHTML show:window:top transition:true"
                        hx-vals=(json!({
                            "q": format!(r#"cat:{sub_cat}"#)
                        }))
                        _=(format!("on click put 'cat:{sub_cat}' into #search-recipes.value")) {
                        (sub_cat)
                    }
                }
            }
        }
    }
}

pub(super) fn rating(name: &str, value: Option<i16>, size: &str, is_ro: bool) -> Markup {
    let value = value.unwrap_or(0);

    if is_ro {
        html! {
            div class=(format!("rating {size}").trim()) {
                div class="mask mask-star-2" aria-label="1 star" aria-current=[if value == 1 { Some("true") } else { None }]  {}
                div class="mask mask-star-2" aria-label="2 star" aria-current=[if value == 2 { Some("true") } else { None }] {}
                div class="mask mask-star-2" aria-label="3 star" aria-current=[if value == 3 { Some("true") } else { None }]  {}
                div class="mask mask-star-2" aria-label="4 star" aria-current=[if value == 4 { Some("true") } else { None }] {}
                div class="mask mask-star-2" aria-label="5 star" aria-current=[if value == 5 { Some("true") } else { None }] {}
            }
        }
    } else {
        html! {
            div class="rating" {
                input type="radio" name=(name) class="rating-hidden" value="" aria-label="clear" checked[value == 0];
                input type="radio" name=(name) class="mask mask-star-2" value="1" aria-label="1 star" checked[value == 1];
                input type="radio" name=(name) class="mask mask-star-2" value="2" aria-label="2 star"  checked[value == 2];
                input type="radio" name=(name) class="mask mask-star-2" value="3" aria-label="3 star" checked[value == 3];
                input type="radio" name=(name) class="mask mask-star-2" value="4" aria-label="4 star" checked[value == 4];
                input type="radio" name=(name) class="mask mask-star-2" value="5" aria-label="5 star" checked[value == 5];
            }
        }
    }
}

pub(super) fn recipe_keyword_empty(keywords: Vec<Keyword>) -> Markup {
    html! {
        div #hidden-keyword class="hidden badge badge-sm badge-soft p-3 pr-0" {
            input type="hidden" name="keyword" value="";
            span class="select-none" {}
            button type="button" class="btn btn-xs btn-ghost" _=(PreEscaped("on click remove closest <div/>")) { "X" }
        }
        div #empty-keyword class="badge badge-sm badge-soft p-3 pr-0"
            _="on keydown if event.key is 'Enter' halt the event then addKeyword(event)" {
            label {
                input #new-keyword type="text" placeholder="New keyword" class="input input-ghost input-xs w-[16ch] focus:outline-none" autocomplete="off" list="keywords";
                datalist #keywords {
                    @for k in keywords {
                        option { (k.name) }
                    }
                }
            }
            button type="button" class="btn btn-xs btn-ghost" _="on click addKeyword(event)" {
                (PreEscaped("&#10003;"))
            }
        }
    }
}
