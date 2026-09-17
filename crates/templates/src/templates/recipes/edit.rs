use std::sync::Arc;

use maud::{Markup, PreEscaped, html};

use config::DataDir;
use models::data::{Data, ViewRecipe};
use models::recipe::structs::recipe::{Category, Keyword};
use models::recipe::structs::section::SectionComponents;
use models::settings::UserSettingDetails;
use support::fs::FsSupport;

use crate::recipes::common::{
    add_ingredient, add_ingredient_without_section, add_instruction,
    add_instruction_without_section, add_section, add_tool, init_recipe_form_js,
    nutrition_table_header, recipe_keyword_empty, render_media_editor, render_rating,
};
use crate::templates::icons::{
    icon_cooking_pot, icon_cutting_board, icon_information_circle, icon_plus_circle, icon_x_circle,
};
use crate::templates::layouts;
use crate::{Error, Result};

/// Renders the edit recipe page.
pub fn edit_recipe(
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
    mut data: Data,
    data_dir: &DataDir,
    user_setting: &UserSettingDetails,
    categories: Vec<Category>,
    keywords: Vec<Keyword>,
) -> Result<Markup> {
    let view = data
        .recipes
        .pop()
        .ok_or("Must have at least one recipe.")
        .map_err(|_| Error::NoRecipe)?;

    let path = format!("/{}/edit", view.recipe_details.recipe.id);

    let page_title = format!("Edit {}", view.recipe_details.recipe.name);

    Ok(html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" {
                (page_title) " | Recipya"
            }
            (render_edit_recipe(fs_support, &view, data_dir, categories, keywords))
        } @else {
            (layouts::main(&page_title, &path, &data, &render_edit_recipe(fs_support, &view, data_dir, categories, keywords), user_setting))
        }
        (init_recipe_form_js())
    })
}

fn render_edit_recipe(
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
    view: &ViewRecipe,
    data_dir: &DataDir,
    categories: Vec<Category>,
    keywords: Vec<Keyword>,
) -> Markup {
    let recipe_id = view.recipe_details.recipe.id;

    html! {
        section .p-2 {
            div class="flex justify-center" {
                div class="card card-border bg-base-100 w-full border-gray-700 xl:w-[72rem]" {
                    form .card-body.contents style="padding: 0" enctype="multipart/form-data" hx-put=(&format!("/recipes/{recipe_id}/edit")) hx-indicator="#fullscreen-loader" {
                        (render_title(view))
                        div {
                            div class="grid md:grid-flow-col md:grid-cols-6" {
                                div #media-container class="grid grid-flow-col w-full text-center grid-cols-7 md:col-span-3 md:border-r dark:border-gray-700" {
                                    div class="buttons-container flex flex-col gap-1 p-1" {
                                        @if view.recipe_details.videos.is_empty() && view.recipe_details.recipe.image.is_none() {
                                            button #media-button-1 type="button" class="btn btn-sm btn-ghost btn-active" onclick="switchMedia(event)" {
                                                "Media 1"
                                            }
                                        } @else {
                                            @for i in 0..view.recipe_details.num_media() {
                                                button id=(format!("media-button-{}", i+1)) type="button" class={
                                                    "btn btn-sm btn-ghost"
                                                    @if i == 0 { " btn-active" }
                                                } onclick="switchMedia(event)" {
                                                    (format!("Media {}", i + 1))
                                                }
                                            }
                                        }
                                        button #add-media-button type="button" class="btn btn-sm btn-ghost" onclick="addMedia(event)" {
                                            (icon_plus_circle())
                                            "Add"
                                        }
                                    }
                                    (render_media(view, fs_support, data_dir))
                                }
                                div class="grid grid-cols-3 col-span-3 text-sm md:grid-flow-row md:grid-rows-4" style="grid-template-rows: auto" {
                                    div class="grid grid-flow-col border-gray-700 col-span-6 py-2 print:border-none" {
                                        div class="flex justify-center items-center" {
                                            (render_rating("rating", view.recipe_details.recipe.rating, None, false, None))
                                        }
                                    }
                                    div class="grid col-span-6 pb-2 md:grid-cols-3 md:pb-0 md:border-gray-700 md:border-t" {
                                        div class="grid grid-flow-col grid-cols-3 gap-2 border-b border-t border-gray-700 px-2 md:col-span-2 md:border-b-0 md:border-r md:border-t-0 md:px-0" {
                                            div class="col-span-2 border-r border-gray-700 pb-2 px-2" {
                                                (render_categories(view, categories))
                                            }
                                            div class="col-span-1 pb-2" {
                                                (render_yield(view))
                                            }
                                        }
                                        div class="relative px-2 pb-2 md:pr-0" {
                                            (render_source(view))
                                        }
                                    }
                                    div class="border-gray-700 border-y col-span-6 md:grid-cols-3" {
                                        (render_keywords(view, keywords))
                                    }
                                    div class="grid grid-flow-col col-span-6 py-1 border-b md:border-gray-700" {
                                        div class="contents grid grid-flow-col" {
                                            (render_times(view))
                                        }
                                    }
                                    div class="grid grid-flow-col col-span-6" {
                                        div class="grid grid-flow-col col-span-6 border-gray-700" {
                                            textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea w-full h-full resize-none rounded-none focus:outline-none" {
                                                (view.recipe_details.recipe.description.as_ref().map_or(String::new(), ToString::to_string))
                                            }
                                        }
                                         div class="grid grid-flow-col col-span-6 border-gray-700 overflow-x-auto" {
                                            (render_nutrition(view))
                                        }
                                    }
                                }
                            }
                        }
                        div #ingredients-instructions-container class="md:border-t grid text-sm md:grid-flow-col md:col-span-6 dark:border-gray-700" {
                            div class="col-span-6 px-2 py-2 border-y md:col-span-2 md:border-r md:border-y-0 dark:border-gray-700" {
                                (render_tools(view))
                                div .divider {}
                                (render_ingredients(view))
                            }
                            div class="col-span-6 px-6 py-2 border-gray-700 md:rounded-bl-none md:col-span-4" {
                                (render_instructions(view))
                            }
                        }
                        @let notes = view.recipe_details.recipe.notes.as_ref().map_or(String::new(), ToString::to_string);
                        div class="col-span-6 dark:border-gray-700" data-notes=(notes) data-textarea-id="notes" _="on load call initNotes(me.dataset.textareaId, me.dataset.notes)" {
                             textarea #notes name="notes" placeholder="Write some notes about the recipe..." rows="8" class="textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none" {}
                        }
                        div class="card-actions justify-end" {
                            button class="btn btn-primary btn-block btn-sm" { "Submit" }
                        }
                    }
                }
            }
        }
    }
}

fn render_categories(view: &ViewRecipe, categories: Vec<Category>) -> Markup {
    html! {
        fieldset .fieldset {
            label .label for="category" { "Category" }
            input #category type="text" list="categories" name="category"
                class="input input-sm w-11/12" placeholder="Breakfast"
                autocomplete="off"
                value=(view.recipe_details.category);
            datalist id="categories" {
                @for c in categories {
                    option { (c.name) }
                }
            }
        }
    }
}

fn render_ingredients(view: &ViewRecipe) -> Markup {
    html! {
        (add_section("ingredient", Some("section-base-ingredient"), Some("hidden")))
        h2 class="font-semibold text-center pb-2" {
            span .underline { "Ingredients" }
            sup .text-red-600 { "*" }
        }
        ol #ingredients-list class="pl-4" {
            @let ingredients = &view.recipe_details.ingredients;
            @if !ingredients.is_empty() {
                @match ingredients {
                    SectionComponents::Grouped(section_items) => {
                        @for section in section_items.iter() {
                            li .list-none {
                                div .divider {
                                    div class="grid grid-flow-col gap-2 w-full" {
                                        input type="text"
                                            name="section-ingredient"
                                            placeholder="Section name"
                                            class="input input-sm w-fit"
                                            value=(section.title)
                                            onfocusout="renumberSections(this.closest('ol'), 'ingredient')";
                                        btn class="btn btn-xs btn-square" onclick="deleteSection(this, 'ingredient')" {
                                            (icon_x_circle())
                                        }
                                    }
                                }
                            }
                            @for ing in section.items.iter() {
                                (add_ingredient_without_section(&ing.text, Some(&section.title)))
                            }
                        }
                    },
                    SectionComponents::Flat(items) => {
                        @for ing in items.iter() {
                            (add_ingredient(&ing.text))
                        }
                    },
                }
            } @else {
                (add_ingredient(""))
            }
        }
    }
}

fn render_instructions(view: &ViewRecipe) -> Markup {
    html! {
        (add_section("instruction", Some("section-base-instruction"), Some("hidden")))
        h2 class="font-semibold text-center pb-2" {
            span .underline { "Instructions" }
            sup .text-red-600 { "*" }
        }
        ol #instructions-list class="grid list-decimal" {
            @let instructions = &view.recipe_details.instructions;

            @if !instructions.is_empty() {
                 @match instructions {
                    SectionComponents::Grouped(section_items) => {
                        @for section in section_items.iter() {
                            li .list-none data-drag-row {
                                div class="flex items-center" data-drageable draggable="true" {
                                    div class="inline-flex size-6 cursor-grab items-center justify-center text-2xl" data-drag-handle {
                                        "⠿"
                                    }
                                    div class="divider flex-1" {
                                        div class="flex gap-2" {
                                            input type="text"
                                                name="section-instruction"
                                                placeholder="Section name"
                                                class="input input-sm w-fit"
                                                value=(section.title)
                                                onfocusout="renumberSections(this.closest('ol'), 'instruction')";
                                            btn class="btn btn-xs btn-square" onclick="deleteSection(this, 'instruction')" {
                                                (icon_x_circle())
                                            }
                                        }
                                    }
                                }
                            }
                            @for ins in section.items.iter() {
                                (add_instruction_without_section(&ins.text, Some(&section.title)))
                            }
                        }
                    },
                    SectionComponents::Flat(items) => {
                        @for ins in items.iter() {
                            (add_instruction(&ins.text, Some("list-none")))
                        }
                    },
                }
            } @else {
                (add_instruction("", None))
            }
        }
    }
}

fn render_keywords(view: &ViewRecipe, keywords: Vec<Keyword>) -> Markup {
    html! {
        div class="p-4 flex gap-2 flex-wrap" {
            @for kw in view.recipe_details.keywords.iter() {
                div class="badge badge-sm badge-neutral p-3 pr-0" {
                    input type="hidden" name="keyword" value=(kw);
                    span class="select-none" { (kw) }
                    button type="button" class="btn btn-xs btn-ghost" _=(PreEscaped("on click remove closest <div/>")) { "X" }
                }
            }
            (recipe_keyword_empty(keywords))
        }
    }
}

fn render_media(
    view: &ViewRecipe,
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
    data_dir: &DataDir,
) -> Markup {
    const EXT_IMAGE: &str = ".webp";
    const EXT_VIDEO: &str = ".webm";

    html! {
        div #media .col-span-6 {
            @if view.recipe_details.videos.is_empty() && view.recipe_details.recipe.image.is_none() {
                (render_media_editor(1, ""))
            } @else {
                @for (idx, &image) in view.recipe_details.all_images().iter().enumerate() {
                    @let image_exists = fs_support.is_file_exists(image, &data_dir.images.root, EXT_IMAGE);
                    @let image_src = if image_exists {
                        &format!("/data/images/{image}{EXT_IMAGE}")
                    } else {
                        ""
                    };

                    (render_media_editor(idx+1, image_src))
                }
                @for (idx, video) in view.recipe_details.videos.iter().enumerate() {
                    @let video_exists = fs_support.is_file_exists(video.video, &data_dir.videos, EXT_VIDEO);
                    @let video_url = format!("/data/videos/{}{EXT_VIDEO}", video.video);
                    @let num_images = view.recipe_details.num_images();

                    label id=(format!("media-{}", idx+1+num_images)) class={
                        @if num_images > 0 || idx > 0 { "hidden" }
                    } {
                        img src="" alt="" class="mb-2";
                        @if video_exists {
                            video controls class="mb-2" src=(format!("/data/videos/{}{EXT_VIDEO}", video.video)) type="video/webm" {}
                        }
                        span class="grid gap-1 max-w-sm" style="margin: auto auto 0.25rem;" {
                            div class="mr-1 hidden" {
                                input type="file" accept="image/*,video/*" name="media"
                                    class="file-input file-input-sm file-input-bordered w-full max-w-sm"
                                    value=(if video_exists {
                                        &video_url
                                    } else {
                                        ""
                                    })
                                    _=(PreEscaped("on dragover or dragenter halt the event then set the target's style.background to 'lightgray'
                                          on dragleave or drop set the target's style.background to ''
                                          on drop or change
                                            make an FileReader called reader then
                                            if event.dataTransfer
                                                get event.dataTransfer.files[0]
                                            else
                                                get event.target.files[0]
                                            end then
                                            if it.type.startsWith('video')
                                                put `<video controls class='object-cover mb-2 w-full max-h-[39rem]' src='${window.URL.createObjectURL(it)}'></video>` after previous <img/> then
                                                add .hidden to previous <img/>
                                            else
                                                set {src: window.URL.createObjectURL(it)} on previous <img/>
                                            end then
                                            remove .hidden from me.parentElement.parentElement.querySelectorAll('button') then
                                            add .hidden to the parentElement of me"));
                                div .divider { "OR" }
                                span class="hidden input-error" {}
                                div class="justify-center flex join" {
                                    input type="url" placeholder="Enter the URL of an image" class="input input-sm join-item";
                                    button type="button" class="btn btn-sm join-item"
                                        hx-get="/fetch"
                                        hx-vals="js:{url: event.target.previousElementSibling.value}"
                                        hx-swap="none"
                                        _="on htmx:afterRequest
                                            if event.detail.successful then
                                                set a to first in event.target.parentElement.parentElement.children then
                                                call updateMediaFromFetch(a, event.detail.xhr.responseURL)
                                                set the value of the previous <input/> to ''
                                            end" { "Fetch" }
                                }
                                div _="on load if not navigator.clipboard hide me" {
                                    div .divider { "OR" }
                                    button type="button" class="btn btn-sm" onclick="pasteImage(event)" {
                                        "Paste copied image"
                                    }
                                }
                            }
                            button type="button" class="btn btn-sm btn-error btn-outline" onclick="deleteMedia(event)" {
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_nutrition(view: &ViewRecipe) -> Markup {
    html! {
        table class="table table-zebra table-xs" {
            (nutrition_table_header())
            tbody {
                @let format_nutrition = |value: Option<f64>, unit: &str| -> String {
                    value.map_or_else(|| "-".into(), |v| if v < 1.0 {
                        format!("{v:.2}{unit}")
                    } else {
                        format!("{v:.0}{unit}")
                    })
                };

                @let nutrition = view.recipe_details.nutrition.per_100g.as_ref();
                @for (name, name_attr, placeholder, value) in [
                    ("Calories", "calories-per-100g", "368kcal", format_nutrition(nutrition.and_then(|n| n.calories_kcal.map(Into::into)), " kcal")),
                    ("Total carbs", "total-carbohydrates-per-100g", "35g", format_nutrition(nutrition.and_then(|n| n.total_carbohydrates), "g")),
                    ("Sugars", "sugars-per-100g", "3g", format_nutrition(nutrition.and_then(|n| n.sugars_g), "g")),
                    ("Protein", "protein-per-100g", "21g", format_nutrition(nutrition.and_then(|n| n.protein_g), "g")),
                    ("Total fat", "total-fat-per-100g", "15g", format_nutrition(nutrition.and_then(|n| n.total_fat_g), "g")),
                    ("Saturated fat", "saturated-fat-per-100g", "1.8g", format_nutrition(nutrition.and_then(|n| n.saturated_fat_g), "g")),
                    ("Unsaturated fat", "unsaturated-fat-per-100g", "1.8g", format_nutrition(nutrition.and_then(|n| n.unsaturated_fat_g), "g")),
                    ("Trans fat", "trans-fat-per-100g", "1.8g", format_nutrition(nutrition.and_then(|n| n.trans_fat_g), "g")),
                    ("Cholesterol", "cholesterol-per-100g", "1.1mg", format_nutrition(nutrition.and_then(|n| n.cholesterol_mg), "mg")),
                    ("Sodium", "sodium-per-100g", "100mg", format_nutrition(nutrition.and_then(|n| n.sodium_mg), "mg")),
                    ("Fiber", "fiber-per-100g", "8g", format_nutrition(nutrition.and_then(|n| n.fiber_g), "g")),
                ] {
                    tr data-nutrition-type="per-100g" {
                        td { (name) }
                        td {
                            label {
                                input type="text" name=(name_attr) autocomplete="off" placeholder=(placeholder) class="input input-xs max-w-24" value=(value);
                            }
                        }
                    }
                }

                @let nutrition = view.recipe_details.nutrition.per_serving.as_ref();
                @for (name, name_attr, placeholder, value) in [
                    ("Serving size", "serving-size", "1/4 cup (45g)", nutrition.map_or_else(|| "-", |nutrition| nutrition.serving_size.as_ref())),
                    ("Calories", "calories-per-serving", "368kcal", &format_nutrition(nutrition.and_then(|n| n.nutrition.calories_kcal.map(Into::into)), " kcal")),
                    ("Total carbs", "total-carbohydrates-per-serving", "35g", &format_nutrition(nutrition.and_then(|n| n.nutrition.total_carbohydrates), "g")),
                    ("Sugars", "sugars-per-serving", "3g", &format_nutrition(nutrition.and_then(|n| n.nutrition.sugars_g), "g")),
                    ("Protein", "protein-per-serving", "21g", &format_nutrition(nutrition.and_then(|n| n.nutrition.protein_g), "g")),
                    ("Total fat", "total-fat-per-serving", "15g", &format_nutrition(nutrition.and_then(|n| n.nutrition.total_fat_g), "g")),
                    ("Saturated fat", "saturated-fat-per-serving", "1.8g", &format_nutrition(nutrition.and_then(|n| n.nutrition.saturated_fat_g), "g")),
                    ("Unsaturated fat", "unsaturated-fat-per-serving", "1.8g", &format_nutrition(nutrition.and_then(|n| n.nutrition.unsaturated_fat_g), "g")),
                    ("Trans fat", "trans-fat-per-serving", "1.8g", &format_nutrition(nutrition.and_then(|n| n.nutrition.trans_fat_g), "g")),
                    ("Cholesterol", "cholesterol-per-serving", "1.1mg", &format_nutrition(nutrition.and_then(|n| n.nutrition.cholesterol_mg), "mg")),
                    ("Sodium", "sodium-per-serving", "100mg", &format_nutrition(nutrition.and_then(|n| n.nutrition.sodium_mg), "mg")),
                    ("Fiber", "fiber-per-serving", "8g", &format_nutrition(nutrition.and_then(|n| n.nutrition.fiber_g), "g")),
                ] {
                    tr data-nutrition-type="per-serving" .hidden {
                        td { (name) }
                        td {
                            label {
                                input type="text" name=(name_attr) autocomplete="off" placeholder=(placeholder) class="input input-xs max-w-24" value=(value);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_source(view: &ViewRecipe) -> Markup {
    html! {
        fieldset .fieldset {
            label .label for="source" { "Source" }
            input #source type="text" placeholder="Source" name="source"
                class="input input-sm w-11/12"
                value=(view.recipe_details.recipe.source.as_str());
        }
        button type="button" class="tooltip tooltip-left absolute top-1 right-1"
            _="on click toggle .tooltip-open"
            data-tip="The source can be a website, name of a cookbook, a relative or friend, a magazine, etc." {
            (icon_information_circle(false))
        }
    }
}

fn render_times(view: &ViewRecipe) -> Markup {
    html! {
        div class="flex justify-self-center items-center gap-1 cursor-default" {
            span data-tip="Prep time" class="tooltip tooltip-left" {
                (icon_cutting_board())
            }
            label {
                input type="text" name="time-prep"
                    value=(if view.formatted_times.prep_edit.is_empty() { "00:15:00" } else { &view.formatted_times.prep_edit })
                    class="input input-sm max-w-24 html-duration-picker";
            }
        }
        div class="flex justify-self-center items-center gap-1 cursor-default" {
            span data-tip="Cook time" class="tooltip tooltip-left" {
                (icon_cooking_pot())
            }
            label {
                input type="text" name="time-cook"
                    value=(if view.formatted_times.cook_edit.is_empty() { "00:15:00" } else { &view.formatted_times.cook_edit })
                    class="input input-sm max-w-24 html-duration-picker";
            }
        }
    }
}

fn render_title(view: &ViewRecipe) -> Markup {
    let title = &view.recipe_details.recipe.name;

    html! {
        h2 class="card-title place-content-center rounded-t-2xl" {
            label .w-full {
                input required type="text" name="title" placeholder="Title of the recipe*"
                    autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200"
                    value=(title);
            }
        }
    }
}

fn render_tools(view: &ViewRecipe) -> Markup {
    html! {
        h2 class="font-semibold text-center pb-2" {
            span .underline { "Tools" }
        }
        ol #tools-list class="pl-4" {
            @if !view.recipe_details.tools.is_empty() {
                @for tool in &view.recipe_details.tools {
                    (add_tool(Some(tool)))
                }
            } @else {
                (add_tool(None))
            }
        }
    }
}

fn render_yield(view: &ViewRecipe) -> Markup {
    html! {
        fieldset .fieldset {
            label .label for="servings" { "Servings" }
            input #servings type="number" min="1" name="yield"
                value=(view.recipe_details.recipe.r#yield.to_string())
                class="input input-sm w-11/12";
        }
    }
}
