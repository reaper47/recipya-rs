use maud::{Markup, PreEscaped, html};

use models::data::{Data, ViewRecipe};
use models::recipe::{Category, Keyword};
use models::settings::UserSettingDetails;

use crate::recipes::common::{
    add_ingredient, add_instruction, add_tool, rating, recipe_keyword_empty,
};
use crate::templates::icons::{
    icon_arrow_uturn_left, icon_arrow_uturn_right, icon_arrows_right_left, icon_arrows_up_down,
    icon_check, icon_cooking_pot, icon_crop, icon_cutting_board, icon_information_circle,
    icon_magnifying_glass_minus, icon_magnifying_glass_plus, icon_move_thin, icon_pencil,
    icon_plus_circle, icon_trash, icon_x_mark,
};
use crate::templates::layouts;

/// Renders the add recipe manually page.
pub fn add_recipe_manual(
    data: Data,
    user_setting: UserSettingDetails,
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
            (layouts::main("Add Recipe Manually", path, &data, render_add_recipe_manual(view, categories, keywords), user_setting, true))
        }
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
                    form .card-body style="padding: 0" enctype="multipart/form-data" hx-post="/recipes/add/manual" hx-indicator="#fullscreen-loader" {
                        h2 class="card-title place-content-center rounded-t-2xl" {
                            label .w-full {
                                input required type="text" name="title" placeholder="Title of the recipe*"
                                    autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200"
                                    value=[view.map(|v| v.recipe_details.recipe.name.to_string())];
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
                                    (render_media())
                                }
                                div class="grid grid-cols-3 col-span-3 text-sm md:grid-flow-row md:grid-rows-4" style="grid-template-rows: auto" {
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
                                    div class="grid grid-flow-col col-span-6 py-1 md:grid-cols-2 md:row-span-1" {
                                        div class="contents md:col-span-2" {
                                            (render_times(view))
                                        }
                                        div class="md:col-span-1" {
                                            (rating("rating", Some(3), "", false))
                                        }
                                    }
                                    div class="grid grid-flow-col col-span-6 border-gray-700 border-y overflow-x-auto md:row-span-2" {
                                        (render_nutrition_table())
                                    }
                                    div class="col-span-3 min-h-40 md:h-full md:row-span-1" {
                                        (render_description(view))
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
                        div class="card-actions justify-end" {
                            button class="btn btn-primary btn-block btn-sm" { "Submit" }
                        }
                    }
                }
            }
        }
        (PreEscaped("<script defer>window.onload = () => loadRecipesManualScripts()</script>"))
    }
}
fn render_categories(view: Option<&ViewRecipe>, categories: Vec<Category>) -> Markup {
    html! {
        fieldset .fieldset {
            label .label for="category" { "Category" }
            input #category type="text" list="categories" name="category"
                class="input input-sm w-11/12" placeholder="Breakfast"
                autocomplete="off"
                value=(
                    if let Some(v) = view {
                        v.recipe_details.category.to_string()
                    } else {
                        String::new()
                    }
                );
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
        label {
            textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea w-full h-full resize-none rounded-none" {
                (
                    if let Some(v) = view {
                        if let Some(description) = &v.recipe_details.recipe.description {
                            description.to_string()
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    }
                )
            }
        }
    }
}

fn render_ingredients(view: Option<&ViewRecipe>) -> Markup {
    html! {
        h2 class="font-semibold text-center pb-2" {
            span .underline { "Ingredients" }
            sup .text-red-600 { "*" }
        }
        ol #ingredients-list class="pl-4" {
            @if let Some(v) = view {
                 @if !v.recipe_details.ingredients.is_empty() {
                    @for (_section, ingredients) in &v.recipe_details.ingredients {
                        @for ing in ingredients.iter() {
                            (add_ingredient(&ing))
                        }
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
        h2 class="font-semibold text-center pb-2" {
            span .underline { "Instructions" }
            sup .text-red-600 { "*" }
        }
        ol #instructions-list class="grid list-decimal" {
            @if let Some(v) = view {
                 @if !v.recipe_details.instructions.is_empty() {
                    @for (_section, instructions) in &v.recipe_details.instructions {
                        @for ins in instructions.iter() {
                            (add_instruction(&ins))
                        }
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

fn render_media() -> Markup {
    html! {
        div #media .col-span-6 {
            label #media-1 .block {
                div class="cropper-wrap mb-2 w-full max-h-[39rem] relative overflow-hidden" {
                    img src="" alt="" class="block w-full h-full object-contain";
                }

                span class="grid gap-1" {
                    div class="mr-1 image-selector" {
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
                                 add .hidden to the first <.image-selector/> in root
                                 add .hidden to the first <.edit-toolbox/> in root
                                 add @disabled"));

                        div class="divider" { "OR" }
                        span class="hidden input-error" {}
                        div class="flex join" {
                           div class="w-full" {
                                input type="url" placeholder="Enter the URL of an image" class="input input-sm join-item";
                           }
                           button type="button" class="btn btn-sm join-item" hx-get="/fetch" hx-vals="js:{url: event.target.previousElementSibling.value}" hx-swap="none" _="on htmx:afterRequest
                              if event.detail.successful then
                              set a to first in event.target.parentElement.parentElement.children then
                              call updateMediaFromFetch(a, event.detail.xhr.responseURL)
                              end" { "Fetch" }
                        }
                        div _="on load if not navigator.clipboard hide me" {
                           div class="divider" { "OR" }
                           button type="button" class="btn btn-sm" onclick="pasteImage(event)" { "Paste copied image" }
                        }
                    }
                    div class="image-actions hidden" {
                        div class="main-toolbox flex gap-2 justify-between" {
                           button type="button" class="btn btn-sm" onclick="editImage(event)" {
                              (icon_pencil(false))
                              "Edit"
                           }
                           button type="button" class="btn btn-sm btn-error" onclick="deleteMedia(event)" {
                              (icon_trash())
                              "Delete"
                           }
                        }
                        div class="edit-toolbox flex items-center w-full" {
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
                              button type="button" title="Cancel" class="btn join-item btn-square btn-sm" onclick="cancelCropper()" {
                                 (icon_x_mark())
                              }
                              button type="button" title="Apply" class="btn join-item btn-square btn-sm" onclick="applyCropper()" {
                                 (icon_check())
                              }
                           }
                        }
                    }
                }
            }
        }
    }
}

fn render_nutrition_table() -> Markup {
    html! {
        table class="table table-zebra table-xs" {
            thead {
                tr {
                    th { "Nutrition (per 100g)" }
                    th { "Amount" }
                }
            }
            tbody {
                @let rows = [
                    ("Calories", "calories", "368kcal"),
                    ("Total carbs", "total-carbohydrates", "35g"),
                    ("Sugars", "sugars", "3g"),
                    ("Protein", "protein", "21g"),
                    ("Total fat", "total-fat", "15g"),
                    ("Saturated fat", "saturated-fat", "1.8g"),
                    ("Unsaturated fat", "unsaturated-fat", "1.8g"),
                    ("Trans fat", "trans-fat", "1.8g"),
                    ("Cholesterol", "cholesterol", "1.1mg"),
                    ("Sodium", "sodium", "100mg"),
                    ("Fiber", "fiber", "8g"),
                ];
                @for (name, name_attr, placeholder) in rows {
                    tr {
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
    let source = if let Some(v) = view {
        let src = &v.recipe_details.recipe.source;
        if src.is_empty() {
            src.clone()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    html! {
        fieldset .fieldset {
            label .label for="source" { "Source" }
            input #source type="text" placeholder="Source" name="source" class="input input-sm w-11/12" value=(source);
        }
        button type="button" class="tooltip tooltip-left absolute top-1 right-1"
            _="on click toggle .tooltip-open"
            data-tip="The source can be a website, name of a cookbook, from a relative or friend, a magazine, etc." {
            (icon_information_circle())
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
                    view.map(|v| {
                        if v.recipe_details.recipe.yield_ == 0 {
                            "1".into()
                        } else {
                             v.recipe_details.recipe.yield_.to_string()
                        }
                    })
                    .unwrap_or("1".into())
                )
                class="input input-sm w-11/12";
        }
    }
}
