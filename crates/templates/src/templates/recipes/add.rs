use maud::{Markup, PreEscaped, html};

use models::data::{Data, ViewRecipe};
use models::recipe::structs::recipe::{Category, Keyword};
use models::recipe::structs::section::SectionComponents;
use models::recipe::structs::types::Source;
use models::settings::UserSettingDetails;

use crate::recipes::common::{
    add_ingredient, add_instruction, add_section, add_tool, init_recipe_form_js,
    nutrition_table_header, recipe_keyword_empty, render_media_editor, render_rating,
};
use crate::templates::icons::{
    icon_cooking_pot, icon_cutting_board, icon_information_circle, icon_plus_circle,
};
use crate::templates::layouts;

/// Renders the add recipe manually page.
pub fn add_recipe_manual(
    data: &Data,
    user_setting: &UserSettingDetails,
    categories: Vec<Category>,
    keywords: Vec<Keyword>,
) -> Markup {
    let path = "/add/manual";
    let view = data.recipes.first();

    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" { "Add Recipe Manually | Recipya" }
            span #data-layout data-layout="no-aside" hx-swap-oob="true" {}
            (render_add_recipe_manual(view, categories, keywords))
        } @else {
            (&layouts::main("Add Recipe Manually", path, data, &render_add_recipe_manual(view, categories, keywords), user_setting, true))
        }
        (init_recipe_form_js())
    }
}

fn render_add_recipe_manual(
    view: Option<&ViewRecipe>,
    categories: Vec<Category>,
    keywords: Vec<Keyword>,
) -> Markup {
    html! {
        span #data-layout data-layout="no-aside" {}

        section .p-2 {
            div class="flex justify-center" {
                div class="card card-border bg-base-100 w-full border-gray-700 xl:w-[72rem]" {
                    form .card-body.contents style="padding: 0"
                         enctype="multipart/form-data" hx-encoding="multipart/form-data"
                         hx-post="/recipes/add/manual" hx-indicator="#fullscreen-loader" {
                        h2 class="card-title place-content-center rounded-t-2xl" {
                            label .w-full {
                                input required type="text" name="title" placeholder="Title of the recipe*"
                                    autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200"
                                    value=[view.map(|v| v.recipe_details.recipe.name.clone())];
                            }
                        }
                        div {
                            div class="grid md:grid-flow-col md:grid-cols-6" {
                                div #media-container class="grid grid-flow-col w-full text-center grid-cols-7 md:col-span-3 md:border-r dark:border-gray-700" {
                                    div class="buttons-container flex flex-col gap-1 p-1" {
                                        button #media-button-1 type="button" class="btn btn-sm btn-ghost btn-active" onclick="switchMedia(event)" {
                                            "Media 1"
                                        }
                                        button #add-media-button type="button" class="btn btn-sm btn-ghost" onclick="addMedia(event)" {
                                            (icon_plus_circle())
                                            "Add"
                                        }
                                    }
                                    div #media .col-span-6 {
                                        (render_media_editor(1, ""))
                                    }
                                }
                                div class="grid grid-cols-3 col-span-3 text-sm md:grid-flow-row md:grid-rows-4" style="grid-template-rows: auto" {
                                    div class="grid grid-flow-col border-gray-700 col-span-6 py-2 print:border-none" {
                                        div class="flex justify-center items-center" {
                                            (render_rating("rating", Some(3), None, false, None))
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
                                        div class="p-4 flex gap-2 flex-wrap" {
                                            (render_keywords(view, keywords))
                                        }
                                    }
                                    div class="grid grid-flow-col col-span-6 py-1 md:border-b md:grid-cols-2 md:row-span-1 dark:border-gray-700" {
                                        div class="contents md:col-span-2" {
                                            (render_times(view))
                                        }
                                    }
                                    div class="grid grid-flow-col col-span-6" {
                                        div class="col-span-6 min-h-40 border-r md:h-full md:col-span-1 dark:border-gray-700" {
                                            (render_description(view))
                                        }
                                        div class="col-span-6 md:col-span-1" {
                                            (render_nutrition_table())
                                        }
                                    }
                                }
                            }
                        }
                        div #ingredients-instructions-container class="md:border-y grid text-sm md:grid-flow-col md:col-span-6 dark:border-gray-700" {
                            div class="col-span-6 px-2 py-2 border-y md:col-span-2 md:border-r md:border-y-0 dark:border-gray-700" {
                                (render_tools(view))
                                div .divider {}
                                (render_ingredients(view))
                            }
                            div class="col-span-6 px-6 py-2 border-gray-700 md:rounded-bl-none md:col-span-4" {
                                (render_instructions(view))
                            }
                        }
                        div class="col-span-6 dark:border-gray-700" data-textarea-id="notes" _="on load call initNotes(me.dataset.textareaId)" {
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
fn render_categories(view: Option<&ViewRecipe>, categories: Vec<Category>) -> Markup {
    html! {
        fieldset .fieldset {
            label .label for="category" { "Category" }
            input #category type="text" list="categories" name="category"
                class="input input-sm w-11/12" placeholder="Breakfast"
                autocomplete="off"
                value=(view.map_or_else(String::new, |v| v.recipe_details.category.clone()));
            datalist id="categories" {
                @for c in categories {
                    option { (c.name) }
                }
            }
        }
    }
}

fn render_description(view: Option<&ViewRecipe>) -> Markup {
    html! {
        textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea textarea-ghost w-full h-full resize-none rounded-none focus:outline-none" {
            (
                view.map_or_else(String::new, |v| v.recipe_details.recipe.description.as_ref().map_or_else(String::new, Clone::clone))
            )
        }
    }
}

fn render_ingredients(view: Option<&ViewRecipe>) -> Markup {
    html! {
        (add_section("ingredient", Some("section-base-ingredient"), Some("hidden")))
        h2 class="font-semibold text-center pb-2" {
            span .underline { "Ingredients" }
            sup .text-red-600 { "*" }
        }
        ol #ingredients-list class="pl-4" {
            @if let Some(v) = view {
                @let ingredients = &v.recipe_details.ingredients;
                @if !ingredients.is_empty() {
                     @match ingredients {
                        SectionComponents::Grouped(section_items) => {
                            @for section in section_items.iter() {
                                @for ing in section.items.iter() {
                                    (add_ingredient(&ing.text))
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
            } @else {
                (add_ingredient(""))
            }
        }
    }
}

fn render_instructions(view: Option<&ViewRecipe>) -> Markup {
    html! {
        (add_section("instruction", Some("section-base-instruction"), Some("hidden")))
        h2 class="font-semibold text-center pb-2" {
            span .underline { "Instructions" }
            sup .text-red-600 { "*" }
        }
        ol #instructions-list class="grid [counter-reset:steps] list-none" {
            @if let Some(v) = view {
                @let instructions = &v.recipe_details.instructions;
                 @if !instructions.is_empty() {
                     @match instructions {
                        SectionComponents::Grouped(section_items) => {
                            @for section in section_items.iter() {
                                @for ins in section.items.iter() {
                                    (add_instruction(&ins.text))
                                }
                            }
                        },
                        SectionComponents::Flat(items) => {
                            @for ins in items.iter() {
                                (add_instruction(&ins.text))
                            }
                        },
                    }
                } @else {
                    (add_instruction(""))
                }
            } @else {
                (add_instruction(""))
            }
        }
    }
}

fn render_keywords(view: Option<&ViewRecipe>, keywords: Vec<Keyword>) -> Markup {
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
        (recipe_keyword_empty(keywords))
    }
}

fn render_nutrition_table() -> Markup {
    html! {
        table class="table table-zebra table-xs" {
            (nutrition_table_header())
            tbody {
                @for (name, name_attr, placeholder) in [
                    ("Calories", "calories-per-100g", "368kcal"),
                    ("Total carbs", "total-carbohydrates-per-100g", "35g"),
                    ("Sugars", "sugars-per-100g", "3g"),
                    ("Protein", "protein-per-100g", "21g"),
                    ("Total fat", "total-fat-per-100g", "15g"),
                    ("Saturated fat", "saturated-fat-per-100g", "1.8g"),
                    ("Unsaturated fat", "unsaturated-fat-per-100g", "1.8g"),
                    ("Trans fat", "trans-fat-per-100g", "1.8g"),
                    ("Cholesterol", "cholesterol-per-100g", "1.1mg"),
                    ("Sodium", "sodium-per-100g", "100mg"),
                    ("Fiber", "fiber-per-100g", "8g"),
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
                    ("Serving size", "serving-size", "1/4 cup (45g)"),
                    ("Calories", "calories-per-serving", "128kcal"),
                    ("Total carbs", "total-carbohydrates-per-serving", "12g"),
                    ("Sugars", "sugars-per-serving", "2g"),
                    ("Protein", "protein-per-serving", "5g"),
                    ("Total fat", "total-fat-per-serving", "15g"),
                    ("Saturated fat", "saturated-fat-per-serving", "3.8g"),
                    ("Unsaturated fat", "unsaturated-fat-per-serving", "3.2g"),
                    ("Trans fat", "trans-fat-per-serving", "0g"),
                    ("Cholesterol", "cholesterol-per-serving", "100mg"),
                    ("Sodium", "sodium-per-serving", "25mg"),
                    ("Fiber", "fiber-per-serving", "6g"),
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

fn render_source(view: Option<&ViewRecipe>) -> Markup {
    let source = view.map_or_else(Source::default, |v| v.recipe_details.recipe.source.clone());

    html! {
        fieldset .fieldset {
            label .label for="source" { "Source" }
            input #source type="text" placeholder="Source" name="source" class="input input-sm w-11/12" value=(source.as_str());
        }
        button type="button" class="tooltip tooltip-left absolute top-1 right-1"
            _="on click toggle .tooltip-open"
            data-tip="The source can be a website, name of a cookbook, a relative or friend, a magazine, etc." {
            (icon_information_circle(false))
        }
    }
}

fn render_times(view: Option<&ViewRecipe>) -> Markup {
    html! {
        div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time" {
            span data-tip="Prep time" class="tooltip tooltip-left" {
                (icon_cutting_board())
            }
            label {
                input type="text" name="time-prep"
                    value=(
                        view
                            .map(|v| v.formatted_times.prep_edit.as_str())
                            .filter(|s| !s.is_empty())
                            .unwrap_or("00:15:00")
                    )
                    class="input input-xs max-w-24 html-duration-picker";
            }
        }
        div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time" {
            span data-tip="Cook time" class="tooltip tooltip-left" {
                (icon_cooking_pot())
            }
            label {
                input type="text" name="time-cook"
                    value=(
                        view
                            .map(|v| v.formatted_times.cook_edit.as_str())
                            .filter(|s| !s.is_empty())
                            .unwrap_or("00:30:00")
                    )
                    class="input input-xs max-w-24 html-duration-picker";
            }
        }
    }
}

fn render_tools(view: Option<&ViewRecipe>) -> Markup {
    html! {
        h2 class="font-semibold text-center pb-2" {
            span .underline { "Tools" }
        }
        ol #tools-list class="pl-4" {
            @if let Some(v) = view {
                @if !v.recipe_details.tools.is_empty() {
                    @for tool in &v.recipe_details.tools {
                        (add_tool(Some(tool)))
                    }
                } @else {
                    (add_tool(None))
                }
            } @else {
                (add_tool(None))
            }
        }
    }
}

fn render_yield(view: Option<&ViewRecipe>) -> Markup {
    html! {
        fieldset .fieldset {
            label .label for="servings" { "Servings" }
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
