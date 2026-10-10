use l10n::Messages;
use maud::{Markup, PreEscaped, html};

use models::data::{Data, ViewRecipe};
use models::recipe::structs::recipe::{Category, Keyword};
use models::recipe::structs::section::SectionComponents;
use models::recipe::structs::types::Source;
use models::settings::UserSettingDetails;

use crate::recipes::common::{
    add_ingredient, add_ingredient_without_section, add_instruction,
    add_instruction_without_section, add_section, add_tool, init_recipe_form_js,
    nutrition_table_header, recipe_keyword_empty, render_media_editor, render_rating,
};
use crate::templates::icons;
use crate::templates::layouts;

/// Renders the add recipe manually page.
pub fn add_recipe_manual(
    data: &Data,
    user_setting: &UserSettingDetails,
    categories: Vec<Category>,
    keywords: Vec<Keyword>,
    messages: &Messages,
) -> Markup {
    let path = "/add/manual";
    let view = data.recipes.first();

    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" {
                (messages.recipe_page_tab_title_add())
            }
            (render_add_recipe_manual(view, categories, keywords, messages))
        } @else {
            @let content = render_add_recipe_manual(view, categories, keywords, messages);
            (&layouts::main(&messages.recipe_page_title_add(), path, data, &content, messages, user_setting))
        }
        (init_recipe_form_js())
    }
}

fn render_add_recipe_manual(
    view: Option<&ViewRecipe>,
    categories: Vec<Category>,
    keywords: Vec<Keyword>,
    messages: &Messages,
) -> Markup {
    html! {
        section .p-2 {
            div class="flex justify-center" {
                div class="card card-border bg-base-100 w-full border-gray-700 xl:w-[72rem]" {
                    form class="card-body contents" style="padding: 0"
                         enctype="multipart/form-data" hx-encoding="multipart/form-data"
                         hx-post="/recipes/add/manual" hx-indicator="#fullscreen-loader" {
                        h2 class="card-title place-content-center rounded-t-2xl" {
                            label .w-full {
                                input required type="text" name="title"
                                    placeholder={ (messages.recipe_page_title_input_placeholder()) "*" }
                                    autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200"
                                    value=[view.map(|v| v.recipe_details.recipe.name.as_str())];
                            }
                        }
                        div {
                            div class="grid md:grid-flow-col md:grid-cols-6" {
                                div #media-container class="grid grid-flow-col w-full text-center grid-cols-7 md:col-span-3 md:border-r dark:border-gray-700" {
                                    div class="buttons-container flex flex-col gap-1 p-1" {
                                        button #media-button-1 type="button" class="btn btn-sm btn-ghost btn-active" onclick="switchMedia(event)" {
                                            (messages.recipe_page_media()) " 1"
                                        }
                                        button #add-media-button type="button" class="btn btn-sm btn-ghost" onclick="addMedia(event)" {
                                            (icons::plus_circle())
                                            (messages.action_add())
                                        }
                                    }
                                    div #media .col-span-6 {
                                        (render_media_editor(1, "", messages))
                                    }
                                }
                                div class="grid grid-cols-3 col-span-3 text-sm md:grid-flow-row md:grid-rows-4" style="grid-template-rows: auto" {
                                    div class="grid grid-flow-col border-gray-700 col-span-6 py-2 print:border-none" {
                                        div class="flex justify-center items-center" {
                                            (render_rating("rating", Some(3), None, false, None, messages))
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
                                        div class="p-4 flex gap-2 flex-wrap" {
                                            (render_keywords(view, keywords, messages))
                                        }
                                    }
                                    div class="grid grid-flow-col col-span-6 py-1 md:border-b md:grid-cols-2 md:row-span-1 dark:border-gray-700" {
                                        div class="contents md:col-span-2" {
                                            (render_times(view, messages))
                                        }
                                    }
                                    div class="grid grid-flow-col col-span-6" {
                                        div class="col-span-6 min-h-40 border-r md:h-full md:col-span-1 dark:border-gray-700" {
                                            (render_description(view, messages))
                                        }
                                        div class="col-span-6 md:col-span-1" {
                                            (render_nutrition_table(messages))
                                        }
                                    }
                                }
                            }
                        }
                        div #ingredients-instructions-container class="md:border-y grid text-sm md:grid-flow-col md:col-span-6 dark:border-gray-700" {
                            div class="col-span-6 px-2 py-2 border-y md:col-span-2 md:border-r md:border-y-0 dark:border-gray-700" {
                                (render_tools(view, messages))
                                div .divider {}
                                (render_ingredients(view, messages))
                            }
                            div class="col-span-6 px-6 py-2 border-gray-700 md:rounded-bl-none md:col-span-4" {
                                (render_instructions(view, messages))
                            }
                        }
                        div class="col-span-6 dark:border-gray-700" data-textarea-id="notes" _="on load call initNotes(me.dataset.textareaId)" {
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
fn render_categories(
    view: Option<&ViewRecipe>,
    categories: Vec<Category>,
    messages: &Messages,
) -> Markup {
    html! {
        fieldset .fieldset {
            label .label for="category" {
                (messages.recipe_page_category())
            }
            input #category type="text" list="categories" name="category"
                class="input input-sm w-11/12" placeholder=(messages.search_help_breakfast())
                autocomplete="off"
                value=(view.map_or_else(|| "", |v| &v.recipe_details.category));
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

fn render_description(view: Option<&ViewRecipe>, messages: &Messages) -> Markup {
    html! {
        textarea name="description" placeholder=(messages.recipe_page_description_placeholder()) class="textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none" {
            (
                view.map_or_else(|| "", |v| v.recipe_details.recipe.description.as_ref().map_or_else(|| "", |v| v.as_ref()))
            )
        }
    }
}

pub(super) fn render_ingredients(view: Option<&ViewRecipe>, messages: &Messages) -> Markup {
    html! {
        (add_section("ingredient", Some("section-base-ingredient"), Some("hidden"), messages))
        h2 class="font-semibold text-center pb-2" {
            span .underline {
                (messages.recipe_page_ingredients())
            }
            sup .text-red-600 { "*" }
        }
        ol #ingredients-list .pl-4 {
            @if let Some(v) = view {
                @let ingredients = &v.recipe_details.ingredients;
                @if !ingredients.is_empty() {
                     @match ingredients {
                        SectionComponents::Grouped(section_items) => {
                            @for section in section_items.iter() {
                                li .list-none {
                                    div .divider {
                                        div class="grid grid-flow-col gap-2 w-full" {
                                            input type="text" name="section-ingredient"
                                                placeholder=(messages.recipe_page_ingredients())
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
            } @else {
                (add_ingredient("", messages))
            }
        }
    }
}

fn render_instructions(view: Option<&ViewRecipe>, messages: &Messages) -> Markup {
    html! {
        (add_section("instruction", Some("section-base-instruction"), Some("hidden"), messages))
        h2 class="font-semibold text-center pb-2" {
            span .underline {
                (messages.recipe_page_instructions())
            }
            sup .text-red-600 { "*" }
        }
        ol #instructions-list class="grid [counter-reset:steps]" {
            @if let Some(v) = view {
                @let instructions = &v.recipe_details.instructions;
                 @if !instructions.is_empty() {
                     @match instructions {
                        SectionComponents::Grouped(section_items) => {
                            @for section in section_items.iter() {
                                li .list-none {
                                    div .divider {
                                        div class="grid grid-flow-col gap-2 w-full" {
                                            input type="text" name="section-instruction"
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
                                @for ins in section.items.iter() {
                                    (add_instruction_without_section(&ins.text, Some(&section.title), messages))
                                }
                            }
                        },
                        SectionComponents::Flat(items) => {
                            @for ins in items.iter() {
                                (add_instruction(&ins.text, None, messages))
                            }
                        },
                    }
                } @else {
                    (add_instruction("", None, messages))
                }
            } @else {
                (add_instruction("", None, messages))
            }
        }
    }
}

fn render_keywords(
    view: Option<&ViewRecipe>,
    keywords: Vec<Keyword>,
    messages: &Messages,
) -> Markup {
    html! {
        @if let Some(v) = view {
            @for kw in v.recipe_details.keywords.iter() {
                div class="badge badge-sm badge-neutral p-3 pr-0" {
                    input type="hidden" name="keyword" value=(kw);
                    span class="select-none" { (kw) }
                    button type="button" class="btn btn-xs btn-ghost" _=(PreEscaped("on click remove closest <div/>")) { "X" }
                }
            }
        }
        (recipe_keyword_empty(keywords, messages))
    }
}

fn render_nutrition_table(messages: &Messages) -> Markup {
    html! {
        table class="table table-zebra table-xs" {
            (nutrition_table_header(messages))
            tbody {
                @for (name, name_attr, placeholder) in [
                    (messages.nutrition_calories(), "calories-per-100g", "368kcal"),
                    (messages.nutrition_total_carbs(), "total-carbohydrates-per-100g", "35g"),
                    (messages.nutrition_sugars(), "sugars-per-100g", "3g"),
                    (messages.nutrition_protein(), "protein-per-100g", "21g"),
                    (messages.nutrition_total_fat(), "total-fat-per-100g", "15g"),
                    (messages.nutrition_sat_fat(), "saturated-fat-per-100g", "1.8g"),
                    (messages.nutrition_unsat_fat(), "unsaturated-fat-per-100g", "1.8g"),
                    (messages.nutrition_trans_fat(), "trans-fat-per-100g", "1.8g"),
                    (messages.nutrition_cholesterol(), "cholesterol-per-100g", "1.1mg"),
                    (messages.nutrition_sodium(), "sodium-per-100g", "100mg"),
                    (messages.nutrition_fibre(), "fibre-per-100g", "8g"),
                ] {
                    tr data-nutrition-type="per-100g" {
                        td { (name) }
                        td {
                            label {
                                input type="text" name=(name_attr) autocomplete="off" placeholder=(placeholder) class="input input-xs max-w-24";
                            }
                        }
                    }
                }

                @for (name, name_attr, placeholder) in [
                    (messages.nutrition_serving_size(), "serving-size", "1/4 cup (45g)"),
                    (messages.nutrition_calories(), "calories-per-serving", "128kcal"),
                    (messages.nutrition_total_carbs(), "total-carbohydrates-per-serving", "12g"),
                    (messages.nutrition_sugars(), "sugars-per-serving", "2g"),
                    (messages.nutrition_protein(), "protein-per-serving", "5g"),
                    (messages.nutrition_total_fat(), "total-fat-per-serving", "15g"),
                    (messages.nutrition_sat_fat(), "saturated-fat-per-serving", "3.8g"),
                    (messages.nutrition_unsat_fat(), "unsaturated-fat-per-serving", "3.2g"),
                    (messages.nutrition_trans_fat(), "trans-fat-per-serving", "0g"),
                    (messages.nutrition_cholesterol(), "cholesterol-per-serving", "100mg"),
                    (messages.nutrition_sodium(), "sodium-per-serving", "25mg"),
                    (messages.nutrition_fibre(), "fibre-per-serving", "6g"),
                ] {
                    tr data-nutrition-type="per-serving" .hidden {
                        td { (name) }
                        td {
                            label {
                                input type="text" name=(name_attr) autocomplete="off" placeholder=(placeholder) class="input input-xs max-w-24";
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_source(view: Option<&ViewRecipe>, messages: &Messages) -> Markup {
    let source = view.map_or_else(Source::default, |v| v.recipe_details.recipe.source.clone());

    html! {
        fieldset .fieldset {
            label .label for="source" {
                (messages.recipe_page_source())
            }
            input #source type="text" placeholder=(messages.recipe_page_source()) name="source" class="input input-sm w-11/12" value=(source.as_str());
        }
        button type="button" class="tooltip tooltip-left absolute top-1 right-1" _="on click toggle .tooltip-open" data-tip=(messages.recipe_page_source_data_tip()) {
            (icons::information_circle(false))
        }
    }
}

fn render_times(view: Option<&ViewRecipe>, messages: &Messages) -> Markup {
    html! {
        div class="flex justify-self-center items-center gap-1 cursor-default" title=(messages.recipe_page_prep_time()) {
            span data-tip=(messages.recipe_page_prep_time()) class="tooltip tooltip-left" {
                (icons::cutting_board())
            }
            label {
                input type="text" name="time-prep"
                    value=(
                        view
                            .map(|v| v.formatted_times.prep_edit.as_str())
                            .filter(|s| !s.is_empty())
                            .unwrap_or("00:15:00")
                    )
                    class="input input-sm max-w-24 html-duration-picker";
            }
        }
        div class="flex justify-self-center items-center gap-1 cursor-default" title=(messages.recipe_page_cook_time()) {
            span data-tip=(messages.recipe_page_cook_time()) class="tooltip tooltip-left" {
                (icons::cooking_pot())
            }
            label {
                input type="text" name="time-cook"
                    value=(
                        view
                            .map(|v| v.formatted_times.cook_edit.as_str())
                            .filter(|s| !s.is_empty())
                            .unwrap_or("00:30:00")
                    )
                    class="input input-sm max-w-24 html-duration-picker";
            }
        }
    }
}

fn render_tools(view: Option<&ViewRecipe>, messages: &Messages) -> Markup {
    html! {
        h2 class="font-semibold text-center pb-2" {
            span .underline {
                (messages.recipe_page_tools())
            }
        }
        ol #tools-list .pl-4 {
            @if let Some(v) = view {
                @if !v.recipe_details.tools.is_empty() {
                    @for tool in &v.recipe_details.tools {
                        (add_tool(Some(tool), messages))
                    }
                } @else {
                    (add_tool(None, messages))
                }
            } @else {
                (add_tool(None, messages))
            }
        }
    }
}

fn render_yield(view: Option<&ViewRecipe>, messages: &Messages) -> Markup {
    html! {
        fieldset .fieldset {
            label .label for="servings" {
                (messages.recipe_page_servings())
            }
            input #servings type="number" min="1" name="yield"
                value=(
                    view.map_or_else(|| "1".into(), |v| {
                        if v.recipe_details.recipe.r#yield == 0 {
                            "1".into()
                        } else {
                            v.recipe_details.recipe.r#yield.to_string()
                        }
                    })
                )
                class="input input-sm w-11/12";
        }
    }
}
