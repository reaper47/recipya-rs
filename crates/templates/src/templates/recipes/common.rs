use std::path::Path;
use std::sync::Arc;

use maud::{Markup, PreEscaped, html};
use models::recipe::structs::recipe::Keyword;
use models::recipe::structs::tool::ToolRecipe;
use serde_json::json;

use config::DataDir;
use models::data::Data;
use support::fs::FsSupport;

use crate::recipes::render_favourite_button;
use crate::templates::helpers::cut_string;
use crate::templates::icons::{
    icon_arrow_uturn_left, icon_arrow_uturn_right, icon_arrows_right_left, icon_arrows_up_down,
    icon_check, icon_crop, icon_magnifying_glass_minus, icon_magnifying_glass_plus, icon_move_thin,
    icon_pencil, icon_plus, icon_trash, icon_x_circle, icon_x_mark,
};

pub(super) fn add_tool(tool: Option<&ToolRecipe>) -> Markup {
    html! {
        li class="pb-2" data-drag-row {
            div class="grid grid-flow-col items-center" {
                label class="flex gap-1" {
                    div class="inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl" {
                        data-drageable draggable="true" data-drag-handle {
                            "⠿"
                        }
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
                    button type="button" class="btn btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)" {
                        (icon_plus())
                        "Add"
                    }
                    button type="button"
                        class="delete-button btn btn-square btn-sm btn-outline btn-error"
                        _=(PreEscaped("on click
                            if (closest <ol/>).childElementCount > 1
                                remove closest <li/>
                            else
                                set input to (closest <li/>).querySelector('input') then
                                set input.value to '' then
                                input.focus()")) { (icon_trash()) }
                }
            }
        }
    }
}

pub(super) fn add_ingredient(name: &str) -> Markup {
    html! {
        (add_section("ingredient", None, None))
        (add_ingredient_without_section(name, None))
    }
}

pub(super) fn add_ingredient_without_section(name: &str, section: Option<&str>) -> Markup {
    let input_name =
        section.map_or_else(|| "ingredient".to_string(), |s| format!("ingredient<>{s}"));

    html! {
        li .pb-2 data-drag-row {
            div class="grid grid-flow-col items-center" {
                label class="flex gap-1" {
                    div class="inline-flex size-6 cursor-grab handle mt-1 items-center justify-center text-2xl"
                        data-drageable draggable="true" data-drag-handle {
                        "⠿"
                    }
                    input required type="text" name=(input_name) value=(name)
                            placeholder="1 cup of chopped onions"
                            class="input input-sm"
                            _="on keydown if event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))";
                }
                div class="ml-2 flex gap-2" {
                    button type="button" class="btn btn-sm btn-outline btn-success" title="Shortcut: Enter" onclick="addItem(event)" {
                        (icon_plus())
                        "Add"
                    }
                    button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error"
                        onclick="deleteItem(this, 'ingredient')" {
                            (icon_trash())
                        }
                }
            }
        }
    }
}

pub(super) fn add_instruction(name: &str, section_base_class: Option<&str>) -> Markup {
    html! {
        (add_section("instruction", None, section_base_class))
        (add_instruction_without_section(name, None))
    }
}

pub(super) fn add_instruction_without_section(name: &str, section: Option<&str>) -> Markup {
    let textarea_name = section.map_or_else(
        || "instruction".to_string(),
        |s| format!("instruction<>{s}"),
    );

    html! {
        li class="flex items-start gap-2 pt-2 md:pl-0 [counter-increment:steps] before:content-[counter(steps)_'.'] before:pt-2 before:font-medium" data-drag-row  {
            div class="w-full bg-base-300" data-drageable draggable="true" {
                label class="w-11/12" {
                    textarea required name=(textarea_name) rows="4" class="textarea textarea-bordered rounded-none w-full"
                        placeholder="Mix all ingredients together"
                        _="on keydown if event.ctrlKey and event.key is 'Enter' halt the event then call addItem(event) end on paste call pasteText(me,event.clipboardData.getData('text/plain'))" {
                        (name)
                    }
                }
                div class="grid gap-2 p-2 grid-flow-col" {
                    div class="inline-flex size-6 cursor-grab mt-1 items-center justify-center text-2xl" data-drag-handle {
                        "⠿"
                    }
                    div class="flex gap-2 justify-end" {
                        button type="button" class="btn btn-sm btn-outline btn-success" title="Shortcut: CTRL + Enter" onclick="addItem(event)" {
                            (icon_plus())
                            "Add"
                        }
                        button type="button" class="delete-button btn btn-square btn-sm btn-outline btn-error"
                            onclick="deleteItem(this, 'instruction')" {
                                (icon_trash())
                            }
                    }
                }
            }
        }
    }
}

pub(super) fn add_section(section: &str, id: Option<&str>, base_class: Option<&str>) -> Markup {
    html! {
        li id=[id] class=[base_class] data-drag-row {
            div class="flex items-center" data-drageable draggable="true" {
                div class="inline-flex size-6 cursor-grab items-center justify-center text-2xl" data-drag-handle {
                    "⠿"
                }
                div class="divider flex-1" {
                    div class="flex gap-2" {
                        btn class="btn btn-xs btn-outline"
                            _=(format!("on click
                                make an <input/> called newInput
                                set newInput.type to 'text'
                                set newInput.name to 'section-{section}'
                                set newInput.placeholder to 'Section name'
                                set newInput.className to 'input input-sm w-fit'
                                set list to closest <ol/>
                                set sectionName to '{section}'
                                put newInput before me
                                remove me
                                js(newInput, list, sectionName)
                                    newInput.addEventListener('focusout', function() {{ renumberSections(list, sectionName) }})
                                end
                                newInput.focus()")) {
                            "Add section"
                        }
                        btn class="btn btn-xs btn-square"
                            _=(format!("on click
                                set list to closest <ol/>
                                set sectionName to '{section}'
                                remove closest <li/>
                                call renumberSections(list, sectionName)")) {
                            (icon_x_circle())
                        }
                    }
                }
            }
        }
    }
}

/// Renders a list of recipes.
pub fn list_recipes(
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
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
                        img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full w-full"
                            src=(match details.all_images().first() {
                                Some(&first_image) => {
                                    if !details.all_images().is_empty() && fs_support.is_file_exists(first_image, &data_dir.images.root, ".webp") {
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
                                (render_rating(&format!("rating-{}", recipe.id), recipe.rating, Some(RatingSize::Small), true, None))
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
        @if !category.contains(':') {
            span class={
                    "badge badge-primary select-none cursor-pointer badge-sm p-2 m-1 sm:badge-md sm:m-0 hover:bg-neutral"
                    @if !is_inside_card { " indicator-item indicator-center" }
                }
                hx-get="/recipes/search"
                hx-target="#list-recipes"
                hx-push-url="true"
                hx-swap="innerHTML show:window:top transition:true"
                hx-vals=(json!({
                    "q": format!("cat:{category}")
                }))
                _=(format!("on click put \"cat:{category}\" into #search-recipes.value")) {
                (category)
            }
        } @else {
            span class={
                "badge badge-primary select-none cursor-pointer"
                @if !is_inside_card { " indicator-item indicator-center" }
            } {
                @for (i, sub_cat) in category.split(':').enumerate() {
                    @if i > 0 {
                        ":"
                    }
                    span class="hover:bg-neutral"
                        hx-get="/recipes/search" hx-target="#list-recipes"
                        hx-push-url="true" hx-swap="innerHTML show:window:top transition:true"
                        hx-vals=(json!({
                            "q": format!(r"cat:{sub_cat}")
                        }))
                        _=(format!("on click put 'cat:{sub_cat}' into #search-recipes.value")) {
                        (sub_cat)
                    }
                }
            }
        }
    }
}

pub(super) fn init_recipe_form_js() -> Markup {
    html! {
        (PreEscaped(r##"<script>
            document.addEventListener("htmx:afterSettle", function handler() {
                if (document.querySelector("#ingredients-list")) {
                    initRecipeFormJS();
                    document.removeEventListener("htmx:afterSettle", handler);
                }
            });
        </script>"##))
    }
}

const RATING_MIN: i16 = 1;
const RATING_MAX: i16 = 5;

pub(super) enum RatingSize {
    Small,
}

impl RatingSize {
    pub const fn to_class<'a>(&self) -> &'a str {
        match self {
            Self::Small => "rating-sm",
        }
    }
}

pub(super) fn render_rating(
    name: &str,
    value: Option<i16>,
    size: Option<RatingSize>,
    is_readonly: bool,
    form_id: Option<&str>,
) -> Markup {
    let value = value.unwrap_or(0).clamp(0, RATING_MAX);
    let size = size.map(|v| v.to_class()).unwrap_or_default();

    if is_readonly {
        render_readonly_rating(size, value)
    } else {
        render_writable_rating(size, name, value, form_id)
    }
}

fn render_readonly_rating(size: &str, value: i16) -> Markup {
    html! {
        div class={"rating " (size)} {
            @for i in RATING_MIN..=RATING_MAX {
                div class="mask mask-star-2"
                    aria-label={(i) " star"}
                    aria-current=[if value == i { Some("true") } else { None }] {}
            }
        }
    }
}

fn render_writable_rating(size: &str, name: &str, value: i16, form_id: Option<&str>) -> Markup {
    html! {
        div class=(format!("rating {size}").trim()) {
            input type="radio" name=(name) class="rating-hidden" value="" aria-label="clear" checked[value == 0] form=[form_id];
            @for i in RATING_MIN..=RATING_MAX {
                input type="radio" name=(name) class="mask mask-star-2" value=(i) aria-label={(i) " star"} checked[value == i] form=[form_id];
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

pub(super) fn render_media_editor(image_num: usize, image_src: &str) -> Markup {
    html! {
        label id=(format!("media-{image_num}")) class={
            "block"
            @if image_num > 1 { " hidden" }
        } {
            div class={
                "cropper-wrap mb-2 w-full max-h-[39rem] relative overflow-hidden"
                @if image_src.is_empty() { " hidden" }
            } {
                img src=(image_src) alt=(format!("Image #{image_num} of the recipe")) class="block w-full h-full object-contain";
                @if !image_src.is_empty() {
                    @let name = if Path::new(image_src).extension().is_some_and(|ext| ext.eq_ignore_ascii_case("webp")) {
                        "media-existing-image"
                    } else {
                        "media-existing-video"
                    };
                    input type="hidden" name=(name) value=(image_src);
                }
            }

            span class="grid gap-1" {
                div class={
                    "mr-1 image-selector p-4"
                    @if !image_src.is_empty() { " hidden" }
                } {
                    input type="file" accept="image/*,video/*" name="media" class="file-input file-input-sm file-input-bordered w-full max-w-sm"
                          _=(PreEscaped("on dragover or dragenter halt the event then set the target's style.background to 'lightgray'
                             on dragleave or drop set the target's style.background to ''
                             on drop or change
                             make an FileReader called reader
                             if event.dataTransfer
                                get event.dataTransfer.files[0]
                             else
                                get event.target.files[0]
                             end
                             if it.type.startsWith('video')
                                put `<video controls class='object-cover mb-2 w-full max-h-[39rem]' src='${window.URL.createObjectURL(it)}'</video>` after previous <img/> then
                                add .hidden to previous <img/>
                             else
                                set {src: window.URL.createObjectURL(it)} on previous <img/>
                             end
                             set root to the closest <label/>
                             remove .hidden from the first <.cropper-wrap/> in root
                             remove .hidden from the first <.image-actions/> in root
                             remove .hidden from the first <.main-toolbox/> in root
                             add .hidden to the first <.image-selector/> in root
                             add .hidden to the first <.edit-toolbox/> in root"));

                    div class="divider" { "OR" }
                    span class="hidden input-error" {}
                    div class="justify-center flex join" {
                        input type="url" placeholder="Enter the URL of an image" class="input input-sm join-item";
                        button type="button" class="btn btn-sm join-item" hx-get="/fetch" hx-vals="js:{url: event.target.previousElementSibling.value}" hx-swap="none" _="on htmx:afterRequest
                          if event.detail.successful then
                            set a to first in event.target.parentElement.parentElement.children then
                            call updateMediaFromFetch(a, event.detail.xhr.responseURL)
                            set the value of the previous <input/> to ''
                          end" { "Fetch" }
                    }
                    div _="on load if not navigator.clipboard hide me" {
                       div class="divider" { "OR" }
                       button type="button" class="btn btn-sm" onclick="pasteImage(event)" { "Paste copied image" }
                    }
                }
                div class={
                    "image-actions"
                    @if image_src.is_empty() { " hidden" }
                } {
                    div class={
                        "main-toolbox flex gap-2 justify-between"
                        @if image_src.is_empty() { " hidden" }
                    } {
                       button type="button" class="btn btn-sm" onclick="editImage(event)" {
                          (icon_pencil(false))
                          "Edit"
                       }
                       button type="button" class="btn btn-sm btn-error" onclick="deleteMedia(event)" {
                          (icon_trash())
                          "Delete"
                       }
                    }
                    div class={
                        "edit-toolbox flex items-center w-full"
                        @if !image_src.is_empty() { " hidden" }
                    } {
                       div class="join" {
                          button type="button" title="Move" class="btn join-item btn-square btn-sm" _=(PreEscaped("on click remove .btn-active from next <button/> then add .btn-active then call window.imageEditor.setHandle('move')")) {
                             (icon_move_thin())
                          }
                          button type="button" title="Crop" class="btn join-item btn-square btn-sm btn-active" _=(PreEscaped("on click remove .btn-active from previous <button/> then add .btn-active then call window.imageEditor.setHandle('select')")) {
                             (icon_crop())
                          }
                          button type="button" title="Zoom in" class="btn join-item btn-square btn-sm" _="on click call window.imageEditor.zoom(0.05)" {
                             (icon_magnifying_glass_plus())
                          }
                          button type="button" title="Zoom out" class="btn join-item btn-square btn-sm" _="on click call window.imageEditor.zoom(-0.05)" {
                             (icon_magnifying_glass_minus())
                          }
                          button type="button" title="Rotate left" class="btn join-item btn-square btn-sm" _="on click call window.imageEditor.rotate('-90deg')" {
                             (icon_arrow_uturn_left())
                          }
                          button type="button" title="Rotate right" class="btn join-item btn-square btn-sm" _="on click call window.imageEditor.rotate('90deg')" {
                             (icon_arrow_uturn_right())
                          }
                          button type="button" title="Flip horizontal" class="btn join-item btn-square btn-sm" _="on click call window.imageEditor.scale(-1, 1)" {
                             (icon_arrows_right_left())
                          }
                          button type="button" title="Flip vertical" class="btn join-item btn-square btn-sm" _="on click call window.imageEditor.scale(1, -1)" {
                             (icon_arrows_up_down())
                          }
                       }
                       div class="join ml-auto" {
                          button type="button" title="Cancel" class="btn join-item btn-square btn-sm" onclick="cancelCropper(event)" {
                             (icon_x_mark())
                          }
                          button type="button" title="Apply" class="btn join-item btn-square btn-sm" onclick="applyCropper(event)" {
                             (icon_check())
                          }
                       }
                    }
                }
            }
        }
    }
}

pub(super) fn nutrition_table_header() -> Markup {
    html! {
        thead {
            tr {
                th {
                    select class="select select-sm" onchange="filterNutritionRows(this, this.value)" {
                        option value="per-100g" { "Nutrition (per 100g)" }
                        option value="per-serving" { "Nutrition (per serving)" }
                    }
                }
            }
        }
    }
}

pub(super) fn format_nutrition(value: Option<f64>, unit: &str) -> String {
    match value {
        None => "-".to_string(),
        Some(v) if v < 0.005 => "-".to_string(),
        Some(v) if v < 1.0 => format!("{v:.2}{unit}"),
        Some(v) => format!("{v:.0}{unit}"),
    }
}
