use std::sync::Arc;

use fluent_static::Message;
use l10n::Messages;
use maud::{Markup, PreEscaped, html};

use config::DataDir;
use models::data::{Data, ViewRecipe};
use models::recipe::structs::nutrition::{Nutrition, NutritionPerServingDetails};
use models::recipe::structs::recipe::{Category, Keyword};
use models::recipe::structs::section::SectionComponents;
use models::settings::UserSettingDetails;
use support::fs::FsSupport;

use crate::recipes::common::{
    add_ingredient, add_ingredient_without_section, add_instruction,
    add_instruction_without_section, add_section, add_tool, format_nutrition, init_recipe_form_js,
    nutrition_table_header, recipe_keyword_empty, render_media_editor, render_rating,
};
use crate::templates::icons;
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
    messages: &Messages,
) -> Result<Markup> {
    let view = data
        .recipes
        .pop()
        .ok_or("Must have at least one recipe.")
        .map_err(|_| Error::NoRecipe)?;

    let path = format!("/{}/edit", view.recipe_details.recipe.id);
    let recipe_name = view.recipe_details.recipe.name.as_str();

    Ok(html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" {
                (messages.edit_page_tab_title(recipe_name))
            }
            (render_edit_recipe(fs_support, &view, data_dir, categories, keywords, messages))
        } @else {
            @let content = render_edit_recipe(fs_support, &view, data_dir, categories, keywords, messages);
            (layouts::main(&messages.edit_page_title(recipe_name), &path, &data, &content, messages, user_setting))
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
    messages: &Messages,
) -> Markup {
    let recipe_id = view.recipe_details.recipe.id;

    html! {
        section .p-2 {
            div class="flex justify-center" {
                div class="card card-border bg-base-100 w-full border-gray-700 xl:w-[72rem]" {
                    form .card-body.contents style="padding: 0" enctype="multipart/form-data" hx-put={ "/recipes/" (recipe_id) "/edit" } hx-indicator="#fullscreen-loader" {
                        (render_title(view, messages))
                        div {
                            div class="grid md:grid-flow-col md:grid-cols-6" {
                                div #media-container class="grid grid-flow-col w-full text-center grid-cols-7 md:col-span-3 md:border-r dark:border-gray-700" {
                                    div class="buttons-container flex flex-col gap-1 p-1" {
                                        @if view.recipe_details.videos.is_empty() && view.recipe_details.recipe.image.is_none() {
                                            button #media-button-1 type="button" class="btn btn-sm btn-ghost btn-active" onclick="switchMedia(event)" {
                                                (messages.recipe_page_media_num(1))
                                            }
                                        } @else {
                                            @for i in 0..view.recipe_details.num_media() {
                                                button id={ "media-button-" (i+1) } type="button" class={
                                                    "btn btn-sm btn-ghost"
                                                    @if i == 0 { " btn-active" }
                                                } onclick="switchMedia(event)" {
                                                    (messages.recipe_page_media_num(i + 1))
                                                }
                                            }
                                        }
                                        button #add-media-button type="button" class="btn btn-sm btn-ghost" onclick="addMedia(event)" {
                                            (icons::plus_circle())
                                            (messages.action_add())
                                        }
                                    }
                                    (render_media(view, fs_support, data_dir, messages))
                                }
                                div class="grid grid-cols-3 col-span-3 text-sm md:grid-flow-row md:grid-rows-4" style="grid-template-rows: auto" {
                                    div class="grid grid-flow-col border-gray-700 col-span-6 py-2 print:border-none" {
                                        div class="flex justify-center items-center" {
                                            (render_rating("rating", view.recipe_details.recipe.rating, None, false, None, messages))
                                        }
                                    }
                                    div class="grid col-span-6 pb-2 md:grid-cols-3 md:pb-0 md:border-gray-700 md:border-t" {
                                        div class="grid grid-flow-col grid-cols-3 gap-2 border-b border-t border-gray-700 px-2 md:col-span-2 md:border-b-0 md:border-r md:border-t-0 md:px-0" {
                                            div class="col-span-2 border-r border-gray-700 pb-2 px-2" {
                                                (render_categories(view, categories, messages))
                                            }
                                            div class="col-span-1 pb-2" {
                                                (render_yield(view, messages))
                                            }
                                        }
                                        div class="relative px-2 pb-2 md:pr-0" {
                                            (render_source(view, messages))
                                        }
                                    }
                                    div class="border-gray-700 border-y col-span-6 md:grid-cols-3" {
                                        (render_keywords(view, keywords, messages))
                                    }
                                    div class="grid grid-flow-col col-span-6 py-1 border-b md:border-gray-700" {
                                        div class="contents grid grid-flow-col" {
                                            (render_times(view, messages))
                                        }
                                    }
                                    div class="grid grid-flow-col col-span-6" {
                                        div class="grid grid-flow-col col-span-6 border-gray-700" {
                                            textarea name="description" placeholder=(messages.recipe_page_description_placeholder()) class="textarea w-full h-full resize-none rounded-none focus:outline-none" {
                                                (view.recipe_details.recipe.description.as_ref().map_or(String::new(), ToString::to_string))
                                            }
                                        }
                                         div class="grid grid-flow-col col-span-6 border-gray-700 overflow-x-auto" {
                                            (render_nutrition(view, messages))
                                        }
                                    }
                                }
                            }
                        }
                        div #ingredients-instructions-container class="md:border-t grid text-sm md:grid-flow-col md:col-span-6 dark:border-gray-700" {
                            div class="col-span-6 px-2 py-2 border-y md:col-span-2 md:border-r md:border-y-0 dark:border-gray-700" {
                                (render_tools(view, messages))
                                div .divider {}
                                (render_ingredients(view, messages))
                            }
                            div class="col-span-6 px-6 py-2 border-gray-700 md:rounded-bl-none md:col-span-4" {
                                (render_instructions(view, messages))
                            }
                        }
                        @let notes = view.recipe_details.recipe.notes.as_ref().map_or(String::new(), ToString::to_string);
                        div class="col-span-6 dark:border-gray-700" data-notes=(notes) data-textarea-id="notes" _="on load call initNotes(me.dataset.textareaId, me.dataset.notes)" {
                             textarea #notes name="notes" placeholder=(messages.recipe_page_notes_placeholder()) rows="8" class="textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none" {}
                        }
                        div class="card-actions justify-end" {
                            button class="btn btn-primary btn-block btn-sm" {
                                (messages.action_submit())
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_categories(view: &ViewRecipe, categories: Vec<Category>, messages: &Messages) -> Markup {
    html! {
        fieldset .fieldset {
            label .label for="category" {
                (messages.recipe_page_category())
            }
            input #category type="text" list="categories" name="category" class="input input-sm w-11/12" placeholder=(messages.search_help_breakfast()) autocomplete="off" value=(view.recipe_details.category);
            datalist #categories {
                @for c in categories {
                    option {
                        (c.name)
                    }
                }
            }
        }
    }
}

fn render_ingredients(view: &ViewRecipe, messages: &Messages) -> Markup {
    html! {
        (add_section("ingredient", Some("section-base-ingredient"), Some("hidden"), messages))
        h2 class="font-semibold text-center pb-2" {
            span .underline {
                (messages.recipe_page_ingredients())
            }
            sup .text-red-600 { "*" }
        }
        ol #ingredients-list .pl-4 {
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
                                            placeholder=(messages.recipe_page_section_name())
                                            class="input input-sm w-fit"
                                            value=(section.title)
                                            onfocusout="renumberSections(this.closest('ol'), 'ingredient')";
                                        btn class="btn btn-xs btn-square" onclick="deleteSection(this, 'ingredient')" {
                                            (icons::x_circle())
                                        }
                                    }
                                }
                            }
                            @for ing in section.items.iter() {
                                (add_ingredient_without_section(&ing.text, Some(&section.title), messages))
                            }
                        }
                    },
                    SectionComponents::Flat(items) => {
                        @for ing in items.iter() {
                            (add_ingredient(&ing.text, messages))
                        }
                    },
                }
            } @else {
                (add_ingredient("", messages))
            }
        }
    }
}

fn render_instructions(view: &ViewRecipe, messages: &Messages) -> Markup {
    html! {
        (add_section("instruction", Some("section-base-instruction"), Some("hidden"), messages))
        h2 class="font-semibold text-center pb-2" {
            span .underline {
                (messages.recipe_page_instructions())
            }
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
                                                placeholder=(messages.recipe_page_section_name())
                                                class="input input-sm w-fit"
                                                value=(section.title)
                                                onfocusout="renumberSections(this.closest('ol'), 'instruction')";
                                            btn class="btn btn-xs btn-square" onclick="deleteSection(this, 'instruction')" {
                                                (icons::x_circle())
                                            }
                                        }
                                    }
                                }
                            }
                            @for ins in section.items.iter() {
                                (add_instruction_without_section(&ins.text, Some(&section.title), messages))
                            }
                        }
                    },
                    SectionComponents::Flat(items) => {
                        @for ins in items.iter() {
                            (add_instruction(&ins.text, Some("list-none"), messages))
                        }
                    },
                }
            } @else {
                (add_instruction("", None, messages))
            }
        }
    }
}

fn render_keywords(view: &ViewRecipe, keywords: Vec<Keyword>, messages: &Messages) -> Markup {
    html! {
        div class="p-4 flex gap-2 flex-wrap" {
            @for kw in view.recipe_details.keywords.iter() {
                div class="badge badge-sm badge-neutral p-3 pr-0" {
                    input type="hidden" name="keyword" value=(kw);
                    span .select-none { (kw) }
                    button type="button" class="btn btn-xs btn-ghost" _=(PreEscaped("on click remove closest <div/>")) { "X" }
                }
            }
            (recipe_keyword_empty(keywords, messages))
        }
    }
}

fn render_media(
    view: &ViewRecipe,
    fs_support: &Arc<dyn FsSupport + Sync + Send>,
    data_dir: &DataDir,
    messages: &Messages,
) -> Markup {
    const EXT_IMAGE: &str = ".webp";
    const EXT_VIDEO: &str = ".webm";

    html! {
        div #media .col-span-6 {
            @if view.recipe_details.videos.is_empty() && view.recipe_details.recipe.image.is_none() {
                (render_media_editor(1, "", messages))
            } @else {
                @for (idx, &image) in view.recipe_details.all_images().iter().enumerate() {
                    @let image_exists = fs_support.is_file_exists(image, &data_dir.images.root, EXT_IMAGE);
                    @let image_src = if image_exists {
                        &format!("/data/images/{image}{EXT_IMAGE}")
                    } else {
                        ""
                    };

                    (render_media_editor(idx+1, image_src, messages))
                }
                @for (idx, video) in view.recipe_details.videos.iter().enumerate() {
                    @let video_exists = fs_support.is_file_exists(video.video, &data_dir.videos, EXT_VIDEO);
                    @let video_url = format!("/data/videos/{}{EXT_VIDEO}", video.video);
                    @let num_images = view.recipe_details.num_images();

                    label id={ "media-" (idx + 1 + num_images) } class={
                        @if num_images > 0 || idx > 0 { "hidden" }
                    } {
                        img src="" alt="" class="mb-2";
                        @if video_exists {
                            video controls class="mb-2" src={ "/data/videos/" (video.video) (EXT_VIDEO) } type="video/webm" {}
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
                                div class="divider uppercase" { (messages.or()) }
                                span class="hidden input-error" {}
                                div class="justify-center flex join" {
                                    input type="url" placeholder=(messages.recipe_page_media_enter_url_placeholder()) class="input input-sm join-item";
                                    button type="button" class="btn btn-sm join-item"
                                        hx-get="/fetch"
                                        hx-vals="js:{url: event.target.previousElementSibling.value}"
                                        hx-swap="none"
                                        _="on htmx:afterRequest
                                            if event.detail.successful then
                                                set a to first in event.target.parentElement.parentElement.children then
                                                call updateMediaFromFetch(a, event.detail.xhr.responseURL)
                                                set the value of the previous <input/> to ''
                                            end" { (messages.action_fetch()) }
                                }
                                div _="on load if not navigator.clipboard hide me" {
                                    div class="divider uppercase" { (messages.or()) }
                                    button type="button" class="btn btn-sm" onclick="pasteImage(event)" {
                                        (messages.recipe_page_paste_image())
                                    }
                                }
                            }
                            button type="button" class="btn btn-sm btn-error btn-outline" onclick="deleteMedia(event)" {
                                (messages.action_delete())
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_nutrition(view: &ViewRecipe, messages: &Messages) -> Markup {
    html! {
        table class="table table-zebra table-xs" {
            (nutrition_table_header(messages))
            tbody {
                @let nutrition = view.recipe_details.nutrition.per_100g.as_ref();
                @for (name, name_attr, placeholder, value) in nutrition_per_100g_data(nutrition, messages) {
                    tr data-nutrition-type="per-100g" {
                        td {
                            (name)
                        }
                        td {
                            label {
                                input type="text" name=(name_attr) autocomplete="off" placeholder=(placeholder) class="input input-xs max-w-24" value=(value);
                            }
                        }
                    }
                }

                @let nutrition = view.recipe_details.nutrition.per_serving.as_ref();
                @for (name, name_attr, placeholder, value) in nutrition_per_serving_data(nutrition, messages) {
                    tr data-nutrition-type="per-serving" .hidden {
                        td {
                            (name)
                        }
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

fn nutrition_per_100g_data<'a>(
    nutrition: Option<&Nutrition>,
    messages: &Messages,
) -> [(Message, &'a str, &'a str, String); 11] {
    [
        (
            messages.nutrition_calories(),
            "calories-per-100g",
            "368kcal",
            format_nutrition(
                nutrition.and_then(|n| n.calories_kcal.map(Into::into)),
                " kcal",
            ),
        ),
        (
            messages.nutrition_total_carbs(),
            "total-carbohydrates-per-100g",
            "35g",
            format_nutrition(nutrition.and_then(|n| n.total_carbohydrates), "g"),
        ),
        (
            messages.nutrition_sugars(),
            "sugars-per-100g",
            "3g",
            format_nutrition(nutrition.and_then(|n| n.sugars_g), "g"),
        ),
        (
            messages.nutrition_protein(),
            "protein-per-100g",
            "21g",
            format_nutrition(nutrition.and_then(|n| n.protein_g), "g"),
        ),
        (
            messages.nutrition_total_fat(),
            "total-fat-per-100g",
            "15g",
            format_nutrition(nutrition.and_then(|n| n.total_fat_g), "g"),
        ),
        (
            messages.nutrition_sat_fat(),
            "saturated-fat-per-100g",
            "1.8g",
            format_nutrition(nutrition.and_then(|n| n.saturated_fat_g), "g"),
        ),
        (
            messages.nutrition_unsat_fat(),
            "unsaturated-fat-per-100g",
            "1.8g",
            format_nutrition(nutrition.and_then(|n| n.unsaturated_fat_g), "g"),
        ),
        (
            messages.nutrition_trans_fat(),
            "trans-fat-per-100g",
            "1.8g",
            format_nutrition(nutrition.and_then(|n| n.trans_fat_g), "g"),
        ),
        (
            messages.nutrition_cholesterol(),
            "cholesterol-per-100g",
            "1.1mg",
            format_nutrition(nutrition.and_then(|n| n.cholesterol_mg), "mg"),
        ),
        (
            messages.nutrition_sodium(),
            "sodium-per-100g",
            "100mg",
            format_nutrition(nutrition.and_then(|n| n.sodium_mg), "mg"),
        ),
        (
            messages.nutrition_fibre(),
            "fibre-per-100g",
            "8g",
            format_nutrition(nutrition.and_then(|n| n.fiber_g), "g"),
        ),
    ]
}

fn nutrition_per_serving_data<'a>(
    nutrition: Option<&NutritionPerServingDetails>,
    messages: &Messages,
) -> [(Message, &'a str, &'a str, String); 12] {
    [
        (
            messages.nutrition_serving_size(),
            "serving-size",
            "1/4 cup (45g)",
            nutrition.map_or_else(|| "-".into(), |nutrition| nutrition.serving_size.clone()),
        ),
        (
            messages.nutrition_calories(),
            "calories-per-serving",
            "368kcal",
            format_nutrition(
                nutrition.and_then(|n| n.nutrition.calories_kcal.map(Into::into)),
                " kcal",
            ),
        ),
        (
            messages.nutrition_total_carbs(),
            "total-carbohydrates-per-serving",
            "35g",
            format_nutrition(nutrition.and_then(|n| n.nutrition.total_carbohydrates), "g"),
        ),
        (
            messages.nutrition_sugars(),
            "sugars-per-serving",
            "3g",
            format_nutrition(nutrition.and_then(|n| n.nutrition.sugars_g), "g"),
        ),
        (
            messages.nutrition_protein(),
            "protein-per-serving",
            "21g",
            format_nutrition(nutrition.and_then(|n| n.nutrition.protein_g), "g"),
        ),
        (
            messages.nutrition_total_fat(),
            "total-fat-per-serving",
            "15g",
            format_nutrition(nutrition.and_then(|n| n.nutrition.total_fat_g), "g"),
        ),
        (
            messages.nutrition_sat_fat(),
            "saturated-fat-per-serving",
            "1.8g",
            format_nutrition(nutrition.and_then(|n| n.nutrition.saturated_fat_g), "g"),
        ),
        (
            messages.nutrition_unsat_fat(),
            "unsaturated-fat-per-serving",
            "1.8g",
            format_nutrition(nutrition.and_then(|n| n.nutrition.unsaturated_fat_g), "g"),
        ),
        (
            messages.nutrition_trans_fat(),
            "trans-fat-per-serving",
            "1.8g",
            format_nutrition(nutrition.and_then(|n| n.nutrition.trans_fat_g), "g"),
        ),
        (
            messages.nutrition_cholesterol(),
            "cholesterol-per-serving",
            "1.1mg",
            format_nutrition(nutrition.and_then(|n| n.nutrition.cholesterol_mg), "mg"),
        ),
        (
            messages.nutrition_sodium(),
            "sodium-per-serving",
            "100mg",
            format_nutrition(nutrition.and_then(|n| n.nutrition.sodium_mg), "mg"),
        ),
        (
            messages.nutrition_fibre(),
            "fibre-per-serving",
            "8g",
            format_nutrition(nutrition.and_then(|n| n.nutrition.fiber_g), "g"),
        ),
    ]
}

fn render_source(view: &ViewRecipe, messages: &Messages) -> Markup {
    html! {
        fieldset .fieldset {
            label .label for="source" {
                (messages.recipe_page_source())
            }
            input #source type="text" placeholder=(messages.recipe_page_source()) name="source"
                class="input input-sm w-11/12"
                value=(view.recipe_details.recipe.source.as_str());
        }
        button type="button" class="tooltip tooltip-left absolute top-1 right-1" _="on click toggle .tooltip-open" data-tip=(messages.recipe_page_source_data_tip()) {
            (icons::information_circle(false))
        }
    }
}

fn render_times(view: &ViewRecipe, messages: &Messages) -> Markup {
    html! {
        div class="flex justify-self-center items-center gap-1 cursor-default" {
            span data-tip=(messages.recipe_page_prep_time()) class="tooltip tooltip-left" {
                (icons::cutting_board())
            }
            label {
                input type="text" name="time-prep"
                    value=(if view.formatted_times.prep_edit.is_empty() { "00:15:00" } else { &view.formatted_times.prep_edit })
                    class="input input-sm max-w-24 html-duration-picker";
            }
        }
        div class="flex justify-self-center items-center gap-1 cursor-default" {
            span data-tip=(messages.recipe_page_cook_time()) class="tooltip tooltip-left" {
                (icons::cooking_pot())
            }
            label {
                input type="text" name="time-cook"
                    value=(if view.formatted_times.cook_edit.is_empty() { "00:15:00" } else { &view.formatted_times.cook_edit })
                    class="input input-sm max-w-24 html-duration-picker";
            }
        }
    }
}

fn render_title(view: &ViewRecipe, messages: &Messages) -> Markup {
    let title = &view.recipe_details.recipe.name;

    html! {
        h2 class="card-title place-content-center rounded-t-2xl" {
            label .w-full {
                input required type="text" name="title" placeholder={ (messages.recipe_page_title_input_placeholder()) "*" }
                    autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200"
                    value=(title);
            }
        }
    }
}

fn render_tools(view: &ViewRecipe, messages: &Messages) -> Markup {
    html! {
        h2 class="font-semibold text-center pb-2" {
            span .underline {
                (messages.recipe_page_tools())
            }
        }
        ol #tools-list .pl-4 {
            @if !view.recipe_details.tools.is_empty() {
                @for tool in &view.recipe_details.tools {
                    (add_tool(Some(tool), messages))
                }
            } @else {
                (add_tool(None, messages))
            }
        }
    }
}

fn render_yield(view: &ViewRecipe, messages: &Messages) -> Markup {
    html! {
        fieldset .fieldset {
            label .label for="servings" {
                (messages.recipe_page_servings())
            }
            input #servings type="number" min="1" name="yield"
                value=(view.recipe_details.recipe.r#yield.to_string())
                class="input input-sm w-11/12";
        }
    }
}
