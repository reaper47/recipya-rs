use std::sync::Arc;

use maud::{Markup, PreEscaped, html};
use url::Url;

use crate::core::config::DataDir;
use crate::core::model::RecipeDetails;
use crate::core::model::recipe::{Category, Keyword, ToolRecipe};
use crate::core::support::fs::FsSupport;
use crate::server::templates::data::{Data, ViewRecipe};
use crate::server::templates::helpers::cut_string;
use crate::server::templates::icons::{
    icon_bars_3, icon_bulb_on, icon_clock, icon_cooking_pot, icon_cutting_board,
    icon_document_duplicate, icon_ellipsis_vertical, icon_information_circle, icon_pencil,
    icon_plus_circle, icon_printer, icon_share, icon_trash,
};
use crate::server::templates::layouts;
use crate::server::templates::layouts::{render_nav, render_recipe_button};
use crate::server::templates::pagination::pagination;
use crate::server::templates::search::{search_help, searchbar};
use crate::server::{Error, Result};

/// Renders the add recipe manually page.
pub fn add_recipe_manual(data: Data, categories: Vec<Category>, keywords: Vec<Keyword>) -> Markup {
    let path = "/add/manual";
    let view = data.recipes.first();

    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" { "Add Recipe Manually | Recipya" }
            (render_recipe_button(true, false))
            (render_nav(path, true))
            (render_add_recipe_manual(view, categories, keywords))
        } @else {
            (layouts::main("Add Recipe Manually", path, &data, render_add_recipe_manual(view, categories, keywords)))
        }
    }
}

fn render_add_recipe_manual(
    view: Option<&ViewRecipe>,
    categories: Vec<Category>,
    keywords: Vec<Keyword>,
) -> Markup {
    html! {
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
                                    div #media class="col-span-6" {
                                        label #media-1 {
                                            img src="" alt="" class="object-cover mb-2 w-full max-h-[39rem]";
                                            span class="grid gap-1 max-w-sm" style="margin: auto auto 0.25rem;" {
                                                div class="mr-1" {
                                                    input type="file" accept="image/*,video/*" name="media"
                                                        class="file-input file-input-sm file-input-bordered w-full max-w-sm"
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
                                                    div .flex.join {
                                                        div .w-full {
                                                            input type="url" placeholder="Enter the URL of an image" class="input input-sm join-item";
                                                        }
                                                        button type="button" class="btn btn-sm join-item"
                                                            hx-get="/fetch"
                                                            hx-vals="js:{url: event.target.previousElementSibling.value}"
                                                            hx-swap="none"
                                                            _="on htmx:afterRequest
                                                                if event.detail.successful then
                                                                    set a to first in event.target.parentElement.parentElement.children then
                                                                    call updateMediaFromFetch(a, event.detail.xhr.responseURL)
                                                                end" { "Fetch" }
                                                    }
                                                    div _="on load if not navigator.clipboard hide me" {
                                                        div .divider { "OR" }
                                                        button type="button" class="btn btn-sm" onclick="pasteImage(event)" {
                                                            "Paste copied image"
                                                        }
                                                    }
                                                }
                                                button type="button" class="hidden btn btn-sm btn-error btn-outline" onclick="deleteMedia(event)" {
                                                    "Delete"
                                                }
                                            }
                                        }
                                    }
                                }
                                div class="grid grid-cols-3 col-span-3 text-sm md:grid-flow-row md:grid-rows-4" style="grid-template-rows: auto" {
                                    div class="grid col-span-6 pb-2 md:grid-cols-3 md:pb-0 md:border-gray-700 md:border-t" {
                                        div class="grid grid-flow-col grid-cols-3 gap-2 border-b border-t border-gray-700 px-2 md:col-span-2 md:border-b-0 md:border-r md:border-t-0 md:px-0" {
                                            div class="col-span-2 border-r border-gray-700 pb-2 px-2" {
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
                                            div class="col-span-1 pb-2" {
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
                                        div class="relative px-2 pb-2 md:pr-0" {
                                            fieldset .fieldset {
                                                label .label for="source" { "Source" }
                                                input #source type="text" placeholder="Source" name="source"
                                                    class="input input-sm w-11/12"
                                                    value=(
                                                        if let Some(v) = view {
                                                            if let Some(src) = &v.recipe_details.recipe.source {
                                                                src.to_string()
                                                            } else {
                                                                String::new()
                                                            }
                                                        } else {
                                                            String::new()
                                                        }
                                                    );
                                            }
                                            button type="button" class="tooltip tooltip-left absolute top-1 right-1"
                                                _="on click toggle .tooltip-open"
                                                data-tip="The source can be a website, name of a cookbook, from a relative or friend, a magazine, etc." {
                                                (icon_information_circle())
                                            }
                                        }
                                    }
                                    div class="border-gray-700 border-y col-span-6 md:grid-cols-3" {
                                        div class="p-4 flex gap-2 flex-wrap" {
                                            @for kw in keywords.iter() {
                                                div class="badge badge-sm badge-neutral p-3 pr-0" {
                                                    input type="hidden" name="keyword" value=(kw.name);
                                                    span class="select-none" { (kw.name) }
                                                    button type="button" class="btn btn-xs btn-ghost" _=(PreEscaped("on click remove closest <div/>")) { "X" }
                                                }
                                            }
                                            (recipe_keyword_empty(keywords))
                                        }
                                    }
                                    div class="grid grid-flow-col col-span-6 py-1 md:grid-cols-2 md:row-span-1" {
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
                                    div class="grid grid-flow-col col-span-6 border-gray-700 border-y overflow-x-auto md:row-span-2" {
                                        table class="table table-zebra table-xs" {
                                            thead {
                                                tr {
                                                    th { "Nutrition (per 100g)" }
                                                    th { "Amount" }
                                                }
                                            }
                                            tbody {
                                                tr {
                                                    td { "Calories" }
                                                    td {
                                                        label {
                                                            input type="text" name="calories" autocomplete="off" placeholder="368kcal" class="input input-xs max-w-24";
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Total carbs" }
                                                    td {
                                                        label {
                                                            input type="text" name="total-carbohydrates" autocomplete="off" placeholder="35g" class="input input-xs max-w-24";
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Sugars" }
                                                    td {
                                                        label {
                                                            input type="text" name="sugars" autocomplete="off" placeholder="3g" class="input input-xs max-w-24";
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Protein" }
                                                    td {
                                                        label {
                                                            input type="text" name="protein" autocomplete="off" placeholder="21g" class="input input-xs max-w-24";
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Total fat" }
                                                    td {
                                                        label {
                                                            input type="text" name="total-fat" autocomplete="off" placeholder="15g" class="input input-xs max-w-24";
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Saturated fat" }
                                                    td {
                                                        label {
                                                            input type="text" name="saturated-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24";
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Unsaturated fat" }
                                                    td {
                                                        label {
                                                            input type="text" name="unsaturated-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24";
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Trans fat" }
                                                    td {
                                                        label {
                                                            input type="text" name="trans-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24";
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Cholesterol" }
                                                    td {
                                                        label {
                                                            input type="text" name="cholesterol" autocomplete="off" placeholder="1.1mg" class="input input-xs max-w-24";
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Sodium" }
                                                    td {
                                                        label {
                                                            input type="text" name="sodium" autocomplete="off" placeholder="100mg" class="input input-xs max-w-24";
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Fiber" }
                                                    td {
                                                        label {
                                                            input type="text" name="fiber" autocomplete="off" placeholder="8g" class="input input-xs max-w-24";
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div class="col-span-3 min-h-40 md:h-full md:row-span-1" {
                                        label {
                                            textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea w-full h-full resize-none" {
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
                            }
                        }
                        div #ingredients-instructions-container class="md:border-t grid text-sm md:grid-flow-col md:col-span-6 dark:border-gray-700" {
                            div class="col-span-6 px-2 py-2 border-y md:col-span-2 md:border-r md:border-y-0 dark:border-gray-700" {
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
                                div .divider {}
                                h2 class="font-semibold text-center pb-2" {
                                    span .underline { "Ingredients" }
                                    sup .text-red-600 { "*" }
                                }
                                ol #ingredients-list class="pl-4 list-decimal" {
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
                            div class="col-span-6 px-6 py-2 border-gray-700 md:rounded-bl-none md:col-span-4" {
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

fn recipe_keyword_empty(keywords: Vec<Keyword>) -> Markup {
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

fn add_tool(tool: Option<&ToolRecipe>) -> Markup {
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

fn add_ingredient(name: &str) -> Markup {
    html! {
        li .pb-2 {
            div class="grid grid-flow-col items-center" {
                label {
                    input required type="text" name="ingredient" value=(name)
                        placeholder="1 cup of chopped onions"
                        class="input input-bordered input-sm w-full"
                        _="on keydown if event.key is 'Enter' halt the event then call addItem(event)";
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
                    div class="inline-block h-4 cursor-move handle" {
                        (icon_bars_3())
                    }
                }
            }
        }
    }
}

fn add_instruction(name: &str) -> Markup {
    html! {
        li class="pt-2 md:pl-0" {
            div .flex {
                label class="w-11/12" {
                    textarea required name="instruction" rows="4" class="textarea textarea-bordered w-full"
                        placeholder="Mix all ingredients together"
                        _="on keydown if event.key is 'Enter' halt the event then call addItem(event)" {
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

/// Renders the add recipe page.
pub fn add_page(path: &str, data: Data) -> Markup {
    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" { "Add Recipe | Recipya" }
            (render_recipe_button(true, false))
            (render_nav(path, true))
            (render_add_page())
        } @else {
            (layouts::main("Add Recipe", path, &data, render_add_page()))
        }
    }
}

fn render_add_page() -> Markup {
    html! {
        div class="grid w-full h-full grid-cols-1 gap-4 p-4 md:grid-cols-2 md:grid-rows-[auto_1fr] xl:m-auto xl:max-w-6xl md:grid-flow-col" {
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                figure {
                    img class="object-cover w-full h-40 rounded-t-xl"
                        src="/public/img/recipes/new/manual.webp"
                        alt="Writing on a piece of paper with a traditional pen.";
                }
                div class="card-body" {
                    h2 class="card-title" { "Manual" }
                    p { "Add a new recipe by filling out its content manually." }
                    div class="card-actions justify-end" {
                        button
                            class="btn btn-outline btn-sm btn-block"
                            hx-get="/recipes/add/manual"
                            hx-target="#content"
                            hx-push-url="true" {
                            "Fill In"
                        }
                    }
                }
            }
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                figure {
                    img class="object-cover w-full h-40 rounded-t-xl"
                        src="/public/img/recipes/new/import.webp"
                        alt="Earth connected from end-to-end by telecommunications.";
                }
                div class="card-body" {
                    h2 class="card-title" { "Website" }
                    p {
                        "Fetch a recipe or recipes from "
                        button
                            class="underline"
                            hx-get="/recipes/supported-websites"
                            hx-target="#search-results"
                            onclick="document.querySelector('#supported-websites-dialog').showModal()" {
                            "supported"
                        }
                        " websites. If the website is unsupported, the software will try to extract "
                        "the recipe, but there is no guarantee of success."
                    }
                    div class="card-actions justify-end" {
                        button class="btn btn-outline btn-sm btn-block" onclick="document.querySelector('#websites-dialog').showModal()" {
                            "Fetch"
                        }
                    }
                }
            }
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                figure {
                    img class="object-cover w-full h-40 rounded-t-xl"
                        src="/public/img/recipes/new/camera.webp"
                        alt="A cellphone used as a camera.";
                }
                div class="card-body" {
                    h2 class="card-title" { "Scan" }
                    p { "Upload the image files or PDF of the recipe you want to add or take a picture using your device's camera." }
                    div class="card-actions" {
                        button class="btn btn-outline btn-sm btn-block" type="button" onclick="document.querySelector('#add-ocr-dialog').showModal()" {
                            "Upload"
                        }
                    }
                }
            }
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                figure {
                    img class="object-cover w-full h-40 rounded-t-xl"
                        src="/public/img/recipes/new/schema.webp"
                        alt="A bunch of shipping containers on a cargo boat.";
                }
                div class="card-body" {
                    h2 class="card-title" { "Import" }
                    p {
                        "Import exported recipes from "
                        button class="underline cursor-pointer"
                            hx-get="/recipes/supported-applications"
                            hx-target="#application-results"
                            onclick="document.querySelector('#supported-apps-import-dialog').showModal()" {
                            "various apps,"
                        }
                        " plain text files or files that adhere to the "
                        a href="https://schema.org/Recipe" target="_blank" class="link" { "recipe schema " }
                        "standard. You may import all your Mealie or Tandoor recipes from the "
                        b { "Data" }
                        " tab in the settings."
                    }
                    p {
                        "You may also download recipe schema files directly using the "
                        a class="link tooltip"
                          data-tip="Simply drag this link to your bookmarks bar, and click the bookmark while on a recipe website. If a recipe schema downloads successfully, you can import it here."
                          href="javascript:(function(){
                                let recipeCount = 0;
                                const jsonSchemaScripts = ([...document.querySelectorAll('script[type=%22application/ld+json%22]')]);
                                for (const jsonSchemaScript of jsonSchemaScripts) {
                                    let data = JSON.parse(jsonSchemaScript.innerHTML);
                                    if (Array.isArray(data) && data.length > 0) {
                                        data = data[0];
                                    }
                                    if (data['@type'] !== 'Recipe') {
                                        continue;
                                    }
                                    const blob = new Blob([JSON.stringify(data)]);
                                    const url = URL.createObjectURL(blob, { type: 'application/json' });
                                    const a = document.createElement('a');
                                    a.href = url;
                                    a.download = data.name ? data.name + '.json' : 'recipe.json';
                                    a.click();
                                    URL.revokeObjectURL(url);
                                    recipeCount++;
                                } if (recipeCount === 0) {
                                    alert('Unable to find a Recipe schema on this site!');
                                }
                            })()" {
                            "Recipya Bookmarklet"
                        }
                        "."
                    }
                    div class="card-actions" {
                        button class="btn btn-outline btn-sm btn-block" onclick="document.querySelector('#import-recipes-dialog').showModal()" {
                            "Import"
                        }
                    }
                }
            }
            dialog #websites-dialog class="modal" {
                div class="modal-box" {
                    form method="dialog" {
                        button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                    }
                    h3 class="font-bold text-lg" { "Fetch recipes from websites" }
                    form class="py-4" hx-post="/recipes/add/website" hx-swap="none" _=(PreEscaped("on submit call #websites-dialog.close() then set me.querySelector('textarea').value to ''")) {
                        div class="grid mb-4" {
                            // TODO: Validate whether we need floating-label.
                            label class="floating-label" {
                                span { "Enter one or more URLs, each on a new line." }
                                // TODO: Validate whether we can inline the placeholder line breaks.
                                textarea class="textarea whitespace-pre-line" name="urls" rows="5" placeholder="URL 1
    URL 2
    URL 3
    URL 4
    etc..." {}
                            }
                        }
                        button class="btn btn-block btn-primary btn-sm" { "Submit" }
                    }
                }
            }
            dialog #supported-websites-dialog class="modal" {
                div class="modal-box h-2/3" {
                    form method="dialog" {
                        button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                    }
                    h3 class="mb-1" {
                        label class="floating-label" {
                            input type="search" placeholder="Search a website" class="input input-sm w-11/12"
                                  _=(PreEscaped("on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"));
                        }
                    }
                    div class="overflow-x-auto" {
                        table class="table table-zebra table-sm" {
                            thead {
                                tr class="text-center" {
                                    th class="py-1" { "Number" }
                                    th class="py-1" { "Website" }
                                }
                            }
                            tbody #search-results {}
                        }
                    }
                }
            }
            dialog #supported-apps-import-dialog .modal {
                div class="modal-box h-2/3" {
                    form method="dialog" {
                        button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                    }
                    h3 class="mb-1" {
                        label class="floating-label" {
                            input type="search" placeholder="Search an application" class=(PreEscaped("input input-sm w-11/12"))
                                _=(PreEscaped("on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"));
                        }
                    }
                    div class="overflow-x-auto" {
                        table class="table table-zebra table-sm" {
                            thead {
                                tr class="text-center" {
                                    th class="py-1" { "Number" }
                                    th class="py-1" { "Application" }
                                }
                            }
                            tbody #application-results {}
                        }
                    }
                }
            }
            dialog #add-ocr-dialog .modal {
                div class="modal-box" {
                    form method="dialog" {
                        button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                    }
                    h3 class="font-bold text-lg" { "Scan Recipe" }
                    form .py-4 hx-post="/recipes/add/ocr" hx-encoding="multipart/form-data" hx-indicator="#fullscreen-loader" hx-swap="none" _="on submit call document.querySelector('#add-ocr-dialog').close()" {
                        div class="grid mb-4" {
                            label for="add-ocr-files-input" class="floating-label text-sm font-medium mb-1" {
                                "Select your recipe's images ordered by page or a recipe document in the PDF format."
                            }
                            input #add-ocr-files-input type="file" name="files" accept=".jpg, .jpeg, .png, .bmp, .tiff, .heif, .pdf" multiple
                                class="p-2 border border-gray-300 rounded-lg shadow focus:ring-2 focus:ring-purple-600 dark:bg-gray-900 dark:border-none";
                        }
                        button class="btn btn-block btn-primary btn-sm" {
                            "Submit"
                        }
                    }
                }
            }
            dialog #import-recipes-dialog .modal {
                div .modal-box {
                    form method="dialog" {
                        button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                    }
                    h3 class="font-bold text-lg" { "Import Recipes" }
                    form .py-4 hx-post="/recipes/add/import" enctype="multipart/form-data" hx-indicator="#fullscreen-loader" hx-swap="none" {
                        div class="grid mb-4" {
                            label for="import-dialog-file" class="floating-label text-sm font-semibold mb-1" {
                                "Choose files in the .json, .txt, .zip or other application format."
                            }
                            input #import-dialog-file type="file" name="files" accept=".cml,.crumb,.json,.mxp,.paprikarecipes,.txt,.zip" multiple
                                  class="p-2 border border-gray-300 rounded-lg shadow focus:ring-2 focus:ring-purple-600 dark:bg-gray-900 dark:border-none";
                        }
                        button type="submit" class="btn btn-block btn-primary btn-sm" onclick="document.querySelector('#import-recipes-dialog').close()" {
                            "Submit"
                        }
                    }
                }
            }
        }
    }
}

/// Renders the edit recipe page.
pub fn edit_recipe(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    mut data: Data,
    data_dir: &DataDir,
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
            (render_recipe_button(true, false))
            (render_nav(&path, true))
            (render_edit_recipe(fs_support, view, &data_dir, categories, keywords))
        } @else {
            (layouts::main(&page_title, &path, &data, render_edit_recipe(fs_support, view, data_dir, categories, keywords)))
        }
    })
}

fn render_edit_recipe(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    view: ViewRecipe,
    data_dir: &DataDir,
    categories: Vec<Category>,
    keywords: Vec<Keyword>,
) -> Markup {
    let recipe_id = view.recipe_details.recipe.id;
    let title = &view.recipe_details.recipe.name;

    html! {
        section .p-2 {
            div class="flex justify-center" {
                div class="card card-border bg-base-100 w-full border-gray-700 xl:w-[72rem]" {
                    form .card-body style="padding: 0" enctype="multipart/form-data" hx-put=(&format!("/recipes/{recipe_id}/edit")) hx-indicator="#fullscreen-loader" {
                        h2 class="card-title place-content-center rounded-t-2xl" {
                            label .w-full {
                                input required type="text" name="title" placeholder="Title of the recipe*"
                                    autocomplete="off" class="input w-full text-center rounded-t-lg rounded-b-none bg-base-200"
                                    value=(title);
                            }
                        }
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
                                                button #media-button-1 type="button" class="btn btn-sm btn-ghost btn-active" onclick="switchMedia(event)" {
                                                    (format!("Media {}", i + 1))
                                                }
                                            }
                                        }
                                        button #add-media-button type="button" class="btn btn-sm btn-ghost" onclick="addMedia(event)" {
                                            (icon_plus_circle())
                                            "Add"
                                        }
                                    }
                                    div #media class="col-span-6" {
                                        @if view.recipe_details.videos.is_empty() && view.recipe_details.recipe.image.is_none() {
                                            label #media-1 {
                                                img src="" alt="" class="object-cover mb-2 w-full max-h-[39rem]";
                                                span class="grid gap-1 max-w-sm" style="margin: auto auto 0.25rem;" {
                                                    div class="mr-1" {
                                                        input type="file" accept="image/*,video/*" name="media"
                                                            class="file-input file-input-sm file-input-bordered w-full max-w-sm"
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
                                                        div .flex.join {
                                                            div .w-full {
                                                                input type="url" placeholder="Enter the URL of an image" class="input input-sm join-item";
                                                            }
                                                            button type="button" class="btn btn-sm join-item"
                                                                hx-get="/fetch"
                                                                hx-vals="js:{url: event.target.previousElementSibling.value}"
                                                                hx-swap="none"
                                                                _="on htmx:afterRequest
                                                                    if event.detail.successful then
                                                                        set a to first in event.target.parentElement.parentElement.children then
                                                                        call updateMediaFromFetch(a, event.detail.xhr.responseURL)
                                                                    end" { "Fetch" }
                                                        }
                                                        div _="on load if not navigator.clipboard hide me" {
                                                            div .divider { "OR" }
                                                            button type="button" class="btn btn-sm" onclick="pasteImage(event)" {
                                                                "Paste copied image"
                                                            }
                                                        }
                                                    }
                                                    button type="button" class="hidden btn btn-sm btn-error btn-outline" onclick="deleteMedia(event)" {
                                                        "Delete"
                                                    }
                                                }
                                            }
                                        } @else {
                                            @for (idx, &image) in view.recipe_details.all_images().iter().enumerate() {
                                                @let image_exists = fs_support.is_file_exists(image, &data_dir.images);
                                                @let image_url = format!("/data/images/{image}.webp");

                                                label id=(format!("media-{}", idx+1)) class={
                                                    @if idx > 0 { "hidden" }
                                                } {
                                                    img src=(if image_exists {
                                                            &image_url
                                                        } else {
                                                            ""
                                                        })
                                                        alt=(format!("Image #{} of the recipe", idx+1)) class="object-cover mb-2 w-full max-h-[39rem]";
                                                    span class="grid gap-1 max-w-sm" style="margin: auto auto 0.25rem;" {
                                                        div class="mr-1" {
                                                            input type="file" accept="image/*,video/*" name="media"
                                                                class="file-input file-input-sm file-input-bordered w-full max-w-sm"
                                                                value=(if image_exists {
                                                                    &image_url
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
                                                            div .flex.join {
                                                                div .w-full {
                                                                    input type="url" placeholder="Enter the URL of an image" class="input input-sm join-item";
                                                                }
                                                                button type="button" class="btn btn-sm join-item"
                                                                    hx-get="/fetch"
                                                                    hx-vals="js:{url: event.target.previousElementSibling.value}"
                                                                    hx-swap="none"
                                                                    _="on htmx:afterRequest
                                                                        if event.detail.successful then
                                                                            set a to first in event.target.parentElement.parentElement.children then
                                                                            call updateMediaFromFetch(a, event.detail.xhr.responseURL)
                                                                        end" { "Fetch" }
                                                            }
                                                            div _="on load if not navigator.clipboard hide me" {
                                                                div .divider { "OR" }
                                                                button type="button" class="btn btn-sm" onclick="pasteImage(event)" {
                                                                    "Paste copied image"
                                                                }
                                                            }
                                                        }
                                                        button type="button" class={
                                                                "btn btn-sm btn-error btn-outline"
                                                                @if image_exists { " hidden" }
                                                            }
                                                            onclick="deleteMedia(event)" { "Delete" }
                                                    }
                                                }
                                            }
                                            @for (idx, video) in view.recipe_details.videos.iter().enumerate() {
                                                @let video_exists = fs_support.is_file_exists(video.video, &data_dir.videos);
                                                @let video_url = format!("/data/videos/{}.webp", video.video);
                                                @let num_images = view.recipe_details.num_images();

                                                label id=(format!("media-{}", idx+1+num_images)) class={
                                                    @if num_images > 0 || idx > 0 { "hidden" }
                                                } {
                                                    img src="" alt="" class="mb-2";
                                                    @if video_exists {
                                                        video controls class="mb-2" src=(format!("/data/videos/{}.webm", video.video)) type="video/webm" {}
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
                                                            div .flex.join {
                                                                div .w-full {
                                                                    input type="url" placeholder="Enter the URL of an image" class="input input-sm join-item";
                                                                }
                                                                button type="button" class="btn btn-sm join-item"
                                                                    hx-get="/fetch"
                                                                    hx-vals="js:{url: event.target.previousElementSibling.value}"
                                                                    hx-swap="none"
                                                                    _="on htmx:afterRequest
                                                                        if event.detail.successful then
                                                                            set a to first in event.target.parentElement.parentElement.children then
                                                                            call updateMediaFromFetch(a, event.detail.xhr.responseURL)
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
                                div class="grid grid-cols-3 col-span-3 text-sm md:grid-flow-row md:grid-rows-4" style="grid-template-rows: auto" {
                                    div class="grid col-span-6 pb-2 md:grid-cols-3 md:pb-0 md:border-gray-700 md:border-t" {
                                        div class="grid grid-flow-col grid-cols-3 gap-2 border-b border-t border-gray-700 px-2 md:col-span-2 md:border-b-0 md:border-r md:border-t-0 md:px-0" {
                                            div class="col-span-2 border-r border-gray-700 pb-2 px-2" {
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
                                            div class="col-span-1 pb-2" {
                                                fieldset .fieldset {
                                                    label .label for="servings" { "Servings" }
                                                    input #servings type="number" min="1" name="yield"
                                                        value=(view.recipe_details.recipe.yield_.to_string())
                                                        class="input input-sm w-11/12";
                                                }
                                            }
                                        }
                                        div class="relative px-2 pb-2 md:pr-0" {
                                            fieldset .fieldset {
                                                label .label for="source" { "Source" }
                                                input #source type="text" placeholder="Source" name="source"
                                                    class="input input-sm w-11/12"
                                                    value=(&view.recipe_details.recipe.source.as_deref().unwrap_or(""));
                                            }
                                            button type="button" class="tooltip tooltip-left absolute top-1 right-1"
                                                _="on click toggle .tooltip-open"
                                                data-tip="The source can be a website, name of a cookbook, from a relative or friend, a magazine, etc." {
                                                (icon_information_circle())
                                            }
                                        }
                                    }
                                    div class="border-gray-700 border-y col-span-6 md:grid-cols-3" {
                                        div class="p-4 flex gap-2 flex-wrap" {
                                            @for kw in keywords.iter() {
                                                div class="badge badge-sm badge-neutral p-3 pr-0" {
                                                    input type="hidden" name="keyword" value=(kw.name);
                                                    span class="select-none" { (kw.name) }
                                                    button type="button" class="btn btn-xs btn-ghost" _=(PreEscaped("on click remove closest <div/>")) { "X" }
                                                }
                                            }
                                            (recipe_keyword_empty(keywords))
                                        }
                                    }
                                    div class="grid grid-flow-col col-span-6 py-1 md:grid-cols-2 md:row-span-1" {
                                        div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time" {
                                            span data-tip="Prep time" class="tooltip tooltip-left" {
                                                (icon_cutting_board())
                                            }
                                            label {
                                                input type="text" name="time-prep"
                                                    value=(view.formatted_times.prep_edit.is_empty().then(|| "00:15:00".to_string()).unwrap_or_else(|| view.formatted_times.prep_edit))
                                                    class="input input-xs max-w-24 html-duration-picker";
                                            }
                                        }
                                        div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time" {
                                            span data-tip="Cook time" class="tooltip tooltip-left" {
                                                (icon_cooking_pot())
                                            }
                                            label {
                                                input type="text" name="time-cook"
                                                    value=(view.formatted_times.cook_edit.is_empty().then(|| "00:15:00".to_string()).unwrap_or_else(|| view.formatted_times.cook_edit))
                                                    class="input input-xs max-w-24 html-duration-picker";
                                            }
                                        }
                                    }
                                    div class="grid grid-flow-col col-span-6 border-gray-700 border-y overflow-x-auto md:row-span-2" {
                                        @let nutrition = view.recipe_details.nutrition.as_ref();
                                        table class="table table-zebra table-xs" {
                                            thead {
                                                tr {
                                                    th { "Nutrition (per 100g)" }
                                                    th { "Amount" }
                                                }
                                            }
                                            tbody {
                                                tr {
                                                    td { "Calories" }
                                                    td {
                                                        label {
                                                            input type="text" name="calories" autocomplete="off" placeholder="368kcal" class="input input-xs max-w-24"
                                                                 value=(nutrition
                                                                            .and_then(|nutrition| nutrition.calories_kcal.map(|v| format!("{v} kcal")))
                                                                            .as_deref()
                                                                            .unwrap_or("-"));
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Total carbs" }
                                                    td {
                                                        label {
                                                            input type="text" name="total-carbohydrates" autocomplete="off" placeholder="35g" class="input input-xs max-w-24"
                                                                value=(nutrition
                                                                            .and_then(|nutrition| nutrition.total_carbohydrates.map(|v| format!("{v}g")))
                                                                            .as_deref()
                                                                            .unwrap_or("-"));
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Sugars" }
                                                    td {
                                                        label {
                                                            input type="text" name="sugars" autocomplete="off" placeholder="3g" class="input input-xs max-w-24"
                                                                value=(nutrition
                                                                            .and_then(|nutrition| nutrition.sugars_g.map(|v| format!("{v}g")))
                                                                            .as_deref()
                                                                            .unwrap_or("-"));
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Protein" }
                                                    td {
                                                        label {
                                                            input type="text" name="protein" autocomplete="off" placeholder="21g" class="input input-xs max-w-24"
                                                                value=(nutrition
                                                                            .and_then(|nutrition| nutrition.protein_g.map(|v| format!("{v}g")))
                                                                            .as_deref()
                                                                            .unwrap_or("-"));
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Total fat" }
                                                    td {
                                                        label {
                                                            input type="text" name="total-fat" autocomplete="off" placeholder="15g" class="input input-xs max-w-24"
                                                                value=(nutrition
                                                                            .and_then(|nutrition| nutrition.total_fat_g.map(|v| format!("{v}g")))
                                                                            .as_deref()
                                                                            .unwrap_or("-"));
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Saturated fat" }
                                                    td {
                                                        label {
                                                            input type="text" name="saturated-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"
                                                                value=(nutrition
                                                                            .and_then(|nutrition| nutrition.saturated_fat_g.map(|v| format!("{v}g")))
                                                                            .as_deref()
                                                                            .unwrap_or("-"));
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Unsaturated fat" }
                                                    td {
                                                        label {
                                                            input type="text" name="unsaturated-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"
                                                                value=(nutrition
                                                                            .and_then(|nutrition| nutrition.unsaturated_fat_g.map(|v| format!("{v}g")))
                                                                            .as_deref()
                                                                            .unwrap_or("-"));
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Trans fat" }
                                                    td {
                                                        label {
                                                            input type="text" name="trans-fat" autocomplete="off" placeholder="1.8g" class="input input-xs max-w-24"
                                                                value=(nutrition
                                                                            .and_then(|nutrition| nutrition.trans_fat_g.map(|v| format!("{v}g")))
                                                                            .as_deref()
                                                                            .unwrap_or("-"));
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Cholesterol" }
                                                    td {
                                                        label {
                                                            input type="text" name="cholesterol" autocomplete="off" placeholder="1.1mg" class="input input-xs max-w-24"
                                                                value=(nutrition
                                                                            .and_then(|nutrition| nutrition.cholesterol_mg.map(|v| format!("{v}mg")))
                                                                            .as_deref()
                                                                            .unwrap_or("-"));
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Sodium" }
                                                    td {
                                                        label {
                                                            input type="text" name="sodium" autocomplete="off" placeholder="100mg" class="input input-xs max-w-24"
                                                                value=(nutrition
                                                                            .and_then(|nutrition| nutrition.sodium_mg.map(|v| format!("{v}mg")))
                                                                            .as_deref()
                                                                            .unwrap_or("-"));
                                                        }
                                                    }
                                                }
                                                tr {
                                                    td { "Fiber" }
                                                    td {
                                                        label {
                                                            input type="text" name="fiber" autocomplete="off" placeholder="8g" class="input input-xs max-w-24"
                                                                value=(nutrition
                                                                            .and_then(|nutrition| nutrition.fiber_g.map(|v| format!("{v}g")))
                                                                            .as_deref()
                                                                            .unwrap_or("-"));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div class="col-span-3 min-h-40 md:h-full md:row-span-1" {
                                        label {
                                            textarea name="description" placeholder="This Thai curry chicken will make you drool." class="textarea w-full h-full resize-none" {
                                                (view.recipe_details.recipe.description.as_ref().map_or(String::new(), ToString::to_string))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div #ingredients-instructions-container class="md:border-t grid text-sm md:grid-flow-col md:col-span-6 dark:border-gray-700" {
                            div class="col-span-6 px-2 py-2 border-y md:col-span-2 md:border-r md:border-y-0 dark:border-gray-700" {
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
                                div .divider {}
                                h2 class="font-semibold text-center pb-2" {
                                    span .underline { "Ingredients" }
                                    sup .text-red-600 { "*" }
                                }
                                ol #ingredients-list class="pl-4 list-decimal" {
                                     @if !view.recipe_details.ingredients.is_empty() {
                                        @for (_section, ingredients) in &view.recipe_details.ingredients {
                                            @for ing in ingredients.iter() {
                                                (add_ingredient(&ing))
                                            }
                                        }
                                    } @else {
                                        (add_ingredient(""))
                                    }
                                }
                            }
                            div class="col-span-6 px-6 py-2 border-gray-700 md:rounded-bl-none md:col-span-4" {
                                h2 class="font-semibold text-center pb-2" {
                                    span .underline { "Instructions" }
                                    sup .text-red-600 { "*" }
                                }
                                ol #instructions-list class="grid list-decimal" {
                                     @if !view.recipe_details.instructions.is_empty() {
                                        @for (_section, instructions) in &view.recipe_details.instructions {
                                            @for ins in instructions.iter() {
                                                (add_instruction(&ins))
                                            }
                                        }
                                    } @else {
                                        (add_instruction(""))
                                    }
                                }
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

/// Renders the index page of recipes.
pub fn index(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    path: &str,
    data: Data,
    data_dir: DataDir,
) -> Markup {
    if data.is_hx_request {
        html! {
            title hx-swap-oob="true" { "Recipes | Recipya" }
            (render_recipe_button(true, true))
            (render_nav(path, true))
            (render_index(fs_support, &data, &data_dir))
        }
    } else {
        layouts::main(
            "Recipes",
            path,
            &data,
            render_index(fs_support, &data, &data_dir),
        )
    }
}

fn render_index(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    data: &Data,
    data_dir: &DataDir,
) -> Markup {
    if data.recipes.is_empty() {
        html! {
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
            div class="flex flex-col" {
                    section class="grid justify-center px-4 pt-4" {
                        search {
                            form
                                class="w-72 flex md:w-96"
                                hx-get="/recipes/search"
                                hx-vals=(
                                    if let Some(p) = &data.pagination {
                                        PreEscaped(format!("{{\"page\": {}}}", p.search.current_page))
                                    } else {
                                        PreEscaped(String::new())
                                    }
                                )
                                hx-target="#list-recipes"
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
                div #list-recipes class="min-h-[79vh]" {
                    (list_recipes(fs_support, &data, &data_dir))
                }
                @if let Some(p) = &data.pagination {
                    (pagination(p))
                }
        }
    }
}

/// Renders a list of recipes.
pub fn list_recipes(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    data: &Data,
    data_dir: &DataDir,
) -> Markup {
    html! {
        @if data.is_hx_request {
            input #search-recipes .w-full type="search" hx-swap-oob="true" name="q"
                 placeholder="Search for recipes..."
                 value=(
                    if let Some(search) = &data.searchbar {
                        &search.term
                    } else {
                        ""
                    }
                 )
                 _=(PreEscaped("on keyup
                       if event.target.value !== '' then
                           remove .md:block from #search_shortcut
                       else
                           add .md:block to #search_shortcut then
                           if (event.key is not 'Delete' and not event.key.startsWith('Arrow')) then
                               send submit to closest <form/> then
                           end
                       end"));
        }
        article class="grid gap-4 p-4 text-sm place-items-center grid-cols-1 sm:grid-cols-2 md:m-auto md:max-w-7xl md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 md:text-base" {
            @for view in data.recipes.iter() {
                section class="card-side sm:card card-compact card-border bg-base-100 shadow-lg indicator w-full" {
                    span class="hidden sm:block" {
                        (category_badge(&view.recipe_details.category, false))
                    }
                    figure class="relative cursor-pointer" hx-get=(format!("/recipes/{}", view.recipe_details.recipe.id)) hx-target="#content" hx-push-url="true" hx-trigger="mousedown" hx-swap="innerHTML show:window:top transition:true" {
                        img class="h-28 w-24 object-cover rounded-t-lg sm:h-40 sm:min-w-full sm:w-full"
                            src=(match view.recipe_details.all_images().first() {
                                Some(&first_image) => {
                                    if !view.recipe_details.all_images().is_empty() && fs_support.is_file_exists(first_image, &data_dir.images) {
                                        format!("/data/images/thumbnails/{}.webp", first_image)
                                    } else {
                                        "/data/images/Placeholders/placeholder.recipe.webp".into()
                                    }
                                },
                                None => {
                                    "/data/images/Placeholders/placeholder.recipe.webp".into()
                                }
                            })
                            alt=(format!("Image for the {} recipe", view.recipe_details.recipe.name));

                        div class="hidden absolute inset-0 bg-black opacity-0 hover:opacity-80 transition-opacity duration-300 items-center justify-center text-white select-none rounded-t-lg sm:flex" {
                            p class="p-2 text-sm" {
                                @match &view.recipe_details.recipe.description {
                                    Some(description) => (cut_string(description, 127)),
                                    None => "No recipe description."
                                }

                            }
                        }
                    }
                    div class="card-body justify-between" {
                        h2 class={
                            "sm:font-semibold sm:w-[25ch] sm:break-words sm:min-h-14"
                            @if view.recipe_details.keywords.is_empty() { " sm:min-h-28" }
                        } {
                            (view.recipe_details.recipe.name)
                        }
                        div class={
                            "sm:max-h-14 sm:overflow-y-auto sm:content-end"
                            @if !view.recipe_details.keywords.is_empty() { " sm:min-h-14" }
                        } {
                            div class="flex flex-col flex-wrap overflow-x-auto max-h-12 pb-2 sm:pb-0 sm:max-h-none sm:flex-auto sm:flex-row" {
                                span class="sm:hidden" {
                                    (category_badge(&view.recipe_details.category, true))
                                }
                                @for kw in view.recipe_details.keywords.iter() {
                                    span class="badge badge-neutral badge-sm select-none p-2 m-1 cursor-pointer"
                                        hx-get="/recipes/search" hx-target="#list-recipes"
                                        hx-push-url="true" hx-swap="innerHTML show:window:top transition:true"
                                        hx-vals=(PreEscaped(format!("{{\"q\": \"tag\":{}}}", kw)))
                                        _=(format!("on click put \"tag:{}\" into #search-recipes.value", kw)) {
                                        (kw)
                                    }
                                }
                            }
                        }
                        div class="card-actions flex-col-reverse h-fit" {
                            button class="btn btn-block btn-xs btn-outline sm:btn-sm" hx-get=(format!("/recipes/{}", view.recipe_details.recipe.id))
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
                hx-vals=(PreEscaped(format!("{{\"q\": \"cat:{category}\"}}")))
                _=(format!("on click put \"cat:{category}\" into #search_recipes.value")) {
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
                        hx-vals=(PreEscaped(format!("{{\"q\": \"cat:{sub_cat}\"}}")))
                        _=(format!("on click put 'cat:{sub_cat}' into #search_recipes.value")) {
                        (sub_cat)
                    }
                }
            }
        }
    }
}

/// Renders the details of a recipe.
pub fn view_recipe(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    path: &str,
    data_dir: DataDir,
    data: Data,
) -> Result<Markup> {
    let view = data
        .recipes
        .first()
        .ok_or("Must have at least one recipe.")
        .map_err(|_| Error::NoRecipe)?;

    Ok(html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" {
                 (view.recipe_details.recipe.name) " | Recipya"
            }
            (view_recipe_helper(fs_support, data_dir, &data)?)
        } @else {
            (layouts::main(
                &view.recipe_details.recipe.name,
                path,
                &data,
                view_recipe_helper(fs_support, data_dir, &data)?
            ))
        }
    })
}

fn view_recipe_helper(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    data_dir: DataDir,
    data: &Data,
) -> Result<Markup> {
    let view = data
        .recipes
        .first()
        .ok_or("Must have at least one recipe.")
        .map_err(|_| Error::NoRecipe)?;

    let recipe_id = view.recipe_details.recipe.id;
    let recipe_details = &view.recipe_details;
    let recipe = &recipe_details.recipe;

    Ok(html! {
        @if matches!(&data.share, Some(share) if share.is_shared) {
             dialog #share-dialog .modal {
                div class="modal-box w-4/5 sm:w-96" {
                    div #share-dialog-result {}
                    div class="modal-action block mt-4" {
                        form method="dialog" {
                            button class="btn btn-block btn-outline btn-sm" {
                                "Close"
                            }
                        }
                    }
                }
            }
        }

        section class="p-2" {
            div class="flex justify-center" {
                div class="card card-border bg-base-100 shadow-none w-full border-gray-700 xl:w-[72rem] print:rounded-none" {
                    div class="card-body" style="padding: 0" {
                        (view_recipe_header(recipe_id, &data, recipe_details))
                        div class="grid md:grid-flow-col md:grid-cols-6" {
                            (view_recipe_media(fs_support, &view.recipe_details, &data_dir))
                            div class="grid grid-cols-3 col-span-3 md:grid-flow-row md:grid-rows-4 print:grid-rows-2" style="grid-template-rows: auto" {
                                div class="grid grid-flow-col col-span-6 md:row-span-1 md:border-y md:border-gray-700 print:row-span-1 print:grid-cols-2 print:border-b-black print:border" {
                                    div class="col-span-2 grid place-items-center md:col-span-1 print:col-span-1 print:float-left print:ml-2 print:border-r print:border-black" {
                                        div class="badge badge-primary badge-outline" {
                                            (view.recipe_details.category)
                                        }
                                    }
                                    div class="grid col-span-2 border-gray-700 place-items-center text-sm border-x p-2 md:p-2 md:col-span-1 print:hidden" {
                                        @if data.is_authenticated {
                                            form autocomplete="off" _="on submit halt the event" class="print:hidden" {
                                                fieldset class="fieldset" {
                                                    legend { "Servings" }
                                                    label class="label" for="yield" { "Servings" }
                                                    input #yield
                                                        type="number"
                                                        min="1"
                                                        name="yield"
                                                        value=(if recipe.yield_ == 0 {
                                                            "1".into()
                                                        } else {
                                                            recipe.yield_.to_string()
                                                        })
                                                        class="input"
                                                        hx-get=(format!("/recipes/{recipe_id}/scale"))
                                                        hx-trigger="input"
                                                        hx-target="#ingredients-instructions-container";
                                                }
                                            }
                                        } @else {
                                            p class="text-sm text-center" {
                                                (format!("{} servings", recipe.yield_))
                                            }
                                        }
                                    }
                                    p class="hidden p-0 pt-2 print:grid print:text-center print:place-content-center" {
                                        (recipe.yield_.to_string()) " servings"
                                    }
                                    div class="flex items-center justify-center col-span-2 text-sm md:col-span-1 print:hidden" {
                                        @if let Some(source) = &recipe.source {
                                           @if Url::parse(source).is_ok() {
                                                a class="btn btn-sm btn-outline no-underline print:hidden" href=(source) target="_blank" { "Source" }
                                                p class="hidden print:block print:whitespace-nowrap print:overflow-hidden print:text-ellipsis print:max-w-xs" { (source) }
                                           } @else {
                                               p class="text-center" { "Source: " (source) }
                                           }
                                        } @else {
                                            p class="text-center" { "Source: Unknown" }
                                        }
                                    }
                                }
                                @if !recipe_details.keywords.is_empty() {
                                    div class="border-gray-700 border-y col-span-6 md:border-t-0 md:grid-cols-3 print:border-none" {
                                        div class="p-4" {
                                            @for kw in recipe_details.keywords.iter() {
                                                div class="badge badge-sm badge-neutral m-1 flex-auto" { (kw) }
                                            }
                                        }
                                    }
                                }
                                div class={
                                        "grid grid-flow-col border-gray-700 col-span-6 py-1 md:border-y md:grid-cols-3 md:row-span-1 print:border-none"
                                        @if recipe_details.nutrition.is_none() { " print:hidden" }
                                    } {
                                    div class="flex justify-self-center items-center gap-1 cursor-default" title="Prep time" {
                                        (icon_cutting_board())
                                        time datetime=(view.formatted_times.prep_datetime) { (view.formatted_times.prep) }
                                    }
                                    div class="flex justify-self-center items-center gap-1 cursor-default" title="Cooking time" {
                                        (icon_cooking_pot())
                                        time datetime=(view.formatted_times.cook_datetime) { (view.formatted_times.cook) }
                                    }
                                    div class="flex justify-self-center items-center gap-1 cursor-default" title="Total time" {
                                        (icon_clock())
                                        time datetime=(view.formatted_times.total_datetime) { (view.formatted_times.total) }
                                    }
                                }
                                (view_recipe_nutrition(&recipe_details))
                                @if let Some(description) = &recipe.description {
                                    div class="col-span-3 min-h-40 md:h-full md:row-span-1 print:hidden" {
                                        label {
                                            textarea class="textarea w-full h-full resize-none" readonly {
                                                (description)
                                            }
                                        }
                                    }
                                } @else {
                                    div class="col-span-3 min-h-40 md:h-full md:row-span-1 hidden print:hidden" {}
                                }
                            }
                        }
                        @if let Some(description) = &recipe.description {
                            div class="hidden print:block print:mx-2" {
                                h1 class="text-sm print:mb-1" {
                                    b { "Description" }
                                }
                                (description)
                            }
                        }
                        div class="border-gray-700 md:border-t" {
                            (ingredients_instructions(&recipe_details))
                            div class="hidden print:grid col-span-6 ml-2 my-1" {
                                @if !recipe_details.tools.is_empty() {
                                    h1 class="text-sm print:mb-1" {
                                        b { "Tools" }
                                    }
                                    ol class="col-span-6 w-full mb-4"
                                        style=(if recipe_details.tools.len() > 10 {
                                            "column-count: 2"
                                        } else {
                                            "column-count: 1"
                                        }) {
                                        @for t in recipe_details.tools.iter() {
                                            li class="text-sm" {
                                                label {
                                                    input type="checkbox";
                                                }
                                                span class="pl-2" {
                                                    (t.quantity.to_string()) (t.name)
                                                }
                                            }
                                        }
                                    }
                                }
                                h1 class="text-sm print:mb-1" {
                                    b { "Ingredients" }
                                }
                                ol class="col-span-6 w-full print:mb-2"
                                    style=(if recipe_details.ingredients.len() > 10 {
                                        "column-count: 2"
                                    } else {
                                        "column-count: 1"
                                    }) {
                                    @for (_section, ingredients) in recipe_details.ingredients.iter() {
                                        @for ing in ingredients.iter() {
                                            li class="text-sm" {
                                                label {
                                                    input type="checkbox";
                                                }
                                                span class="pl-2" { (ing) }
                                            }
                                        }
                                    }
                                }
                            }
                            div class="hidden col-span-5 overflow-visible print:inline" {
                                h1 class="text-sm print:ml-2 print:mb-1" {
                                    b { "Instructions" }
                                }
                                ol class="col-span-6 list-decimal w-full ml-6" {
                                    @for (_section, instructions) in recipe_details.instructions.iter() {
                                        @for ins in instructions.iter() {
                                            li class="print:mr-4" {
                                                span class="text-sm whitespace-pre-line" {
                                                    (ins)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div class="hidden print:block print:mx-2 print:mb-2 print:text-sm" {
                            h1 class="print:mb-1" {
                                b { "Source" }
                            }
                            @if let Some(source) = &recipe.source {
                                 @if Url::parse(source).is_ok() {
                                    p class="print:overflow-hidden" {
                                        (source)
                                    }
                                 } @else {
                                       p { "Source: Unknown" }
                                }
                            } @else {
                                p { "Source: Unknown" }
                            }
                        }
                    }
                }
            }
        }

        script defer src="/public/js/wakelock.min.js" {}
    })
}

fn view_recipe_header(recipe_id: i64, data: &Data, recipe_details: &RecipeDetails) -> Markup {
    html! {
        h2 class="card-title bg-base-200 px-2 pt-2 place-content-center rounded-t-2xl print:border-b print:border-black" style="justify-content: space-between" {
            span class="grid grid-flow-col place-items-center pb-2 print:hidden" {
                button title="Toggle screen lock"
                    _="on load if not navigator.wakeLock hide me end
                           on click
                           if wakeLock wakeLock.release() then
                                add @d='M 12.276 18.55 v -0.748 a 4.79 4.79 0 0 1 1.463 -3.458 a 5.763 5.763 0 0 0 1.804 -4.21 a 5.821 5.821 0 0 0 -6.475 -5.778 c -2.779 0.307 -4.99 2.65 -5.146 5.448 a 5.82 5.82 0 0 0 1.757 4.503 a 4.906 4.906 0 0 1 1.5 3.495 v 0.747 a 1.44 1.44 0 0 0 1.44 1.439 h 2.218 a 1.44 1.44 0 0 0 1.44 -1.439 z m -1.058 0 c 0 0.209 -0.17 0.38 -0.38 0.38 h -2.22 c -0.21 0 -0.38 -0.171 -0.38 -0.38 v -0.748 c 0 -1.58 -0.664 -3.13 -1.822 -4.254 A 4.762 4.762 0 0 1 4.98 9.863 c 0.127 -2.289 1.935 -4.204 4.205 -4.455 a 4.762 4.762 0 0 1 5.3 4.727 a 4.714 4.714 0 0 1 -1.474 3.443 a 5.853 5.853 0 0 0 -1.791 4.225 v 0.746 z M 11.45 20.51 H 8.006 a 0.397 0.397 0 1 0 0 0.795 h 3.444 a 0.397 0.397 0 1 0 0 -0.794 z M 11.847 22.162 a 0.397 0.397 0 0 0 -0.397 -0.397 H 8.006 a 0.397 0.397 0 1 0 0 0.794 h 3.444 c 0.22 0 0.397 -0.178 0.397 -0.397 z z z z z z z z M 10.986 23.416 H 8.867 a 0.397 0.397 0 1 0 0 0.794 h 1.722 c 0.22 0 0.397 -0.178 0.397 -0.397 z' to #icon-bulb
                           else
                                call initWakeLock() then
                                add @d='M12.276 18.55v-.748a4.79 4.79 0 0 1 1.463-3.458 5.763 5.763 0 0 0 1.804-4.21 5.821 5.821 0 0 0-6.475-5.778c-2.779.307-4.99 2.65-5.146 5.448a5.82 5.82 0 0 0 1.757 4.503 4.906 4.906 0 0 1 1.5 3.495v.747a1.44 1.44 0 0 0 1.44 1.439h2.218a1.44 1.44 0 0 0 1.44-1.439zm-1.058 0c0 .209-.17.38-.38.38h-2.22c-.21 0-.38-.171-.38-.38v-.748c0-1.58-.664-3.13-1.822-4.254A4.762 4.762 0 0 1 4.98 9.863c.127-2.289 1.935-4.204 4.205-4.455a4.762 4.762 0 0 1 5.3 4.727 4.714 4.714 0 0 1-1.474 3.443 5.853 5.853 0 0 0-1.791 4.225v.746zM11.45 20.51H8.006a.397.397 0 1 0 0 .795h3.444a.397.397 0 1 0 0-.794zM11.847 22.162a.397.397 0 0 0-.397-.397H8.006a.397.397 0 1 0 0 .794h3.444c.22 0 .397-.178.397-.397zM.397 10.125h2.287a.397.397 0 1 0 0-.794H.397a.397.397 0 1 0 0 .794zM19.456 9.728a.397.397 0 0 0-.397-.397h-2.287a.397.397 0 1 0 0 .794h2.287c.22 0 .397-.178.397-.397zM9.331.397v2.287a.397.397 0 1 0 .794 0V.397a.397.397 0 1 0-.794 0zM16.045 2.85 14.43 4.465a.397.397 0 1 0 .561.561l1.617-1.617a.397.397 0 1 0-.562-.56zM5.027 14.429a.397.397 0 0 0-.56 0l-1.618 1.616a.397.397 0 1 0 .562.562l1.617-1.617a.397.397 0 0 0 0-.561zM4.466 5.027a.396.396 0 0 0 .562 0 .397.397 0 0 0 0-.56L3.41 2.848a.397.397 0 1 0-.561.561zM16.045 16.607a.396.396 0 0 0 .562 0 .397.397 0 0 0 0-.562L14.99 14.43a.397.397 0 1 0-.561.56zM10.986 23.416a.397.397 0 0 0-.397-.397H8.867a.397.397 0 1 0 0 .794h1.722c.22 0 .397-.178.397-.397z' to #icon-bulb
                           end" {
                    (icon_bulb_on())
                }

                @if data.is_authenticated && matches!(&data.share, Some(share) if share.is_from_host) {
                    button class="ml-2 hidden sm:block"
                        title="Edit recipe"
                        hx-get=(format!("/recipes/{recipe_id}/edit"))
                        hx-push-url="true"
                        hx-target="#content"
                        hx-swap="innerHTML transition:true" {
                        (icon_pencil(false))
                    }
                }
            }
            span class="text-center pb-2 print:w-full" itemprop="name" {
                    (recipe_details.recipe.name)
            }
            span class="md:hidden" {
                button title="Open recipe options menu" popovertarget="recipe_menu" popovertargetaction="toggle" {
                    (icon_ellipsis_vertical())
                }
                div #recipe-menu
                    popover
                    style="inset: unset; top: 3.5rem; right: 0.5rem;"
                    class="rounded-box z-10 shadow bg-base-100"
                    _="on click if me.matches(':popover-open') then me.hidePopover()" {
                    ul tabindex="0" class="menu w-full" {
                        li {
                            a title="Edit recipe"
                                hx-get=(format!("/recipes/{recipe_id}/edit"))
                                hx-push-url="true"
                                hx-target="#content"
                                hx-swap="innerHTML transition:true" {
                                (icon_pencil(false))
                                "Edit"
                            }
                        }
                        @if !matches!(&data.share, Some(share) if share.is_shared) {
                            li {
                                a title="Share recipe"
                                    hx-post=(format!("/recipes/{recipe_id}/share"))
                                    hx-target="#share-dialog-result"
                                    _="on htmx:afterRequest from me
                                            if event.detail.successful
                                                if navigator.canShare
                                                    set name to document.querySelector('[itemprop=name]').textContent then
                                                    set data to {title: name, text: name, url: document.querySelector('#share-dialog-result input').value} then
                                                    call navigator.share(data)
                                                else
                                                    call share_dialog.showModal()
                                            end" {
                                    (icon_share())
                                    "Share"
                                }
                            }
                            li {
                                a title="Duplicate recipe"
                                    hx-push-url="/recipes/add/manual"
                                    hx-get=(format!("/recipes/{recipe_id}/duplicate"))
                                    hx-target="#content" {
                                    (icon_document_duplicate())
                                    "Duplicate"
                                }
                            }
                        }
                        li title="Print recipe" _="on click print()" {
                            a {
                                (icon_printer())
                                "Print"
                            }
                        }
                        @if matches!(&data.share, Some(share) if share.is_from_host) {
                            li {
                                a title="Delete recipe"
                                    hx-delete=(format!("/recipes/{recipe_id}"))
                                    hx-swap="none"
                                    hx-confirm="Are you sure you wish to delete this recipe?"
                                    hx-indicator="#fullscreen-loader" {
                                    (icon_trash())
                                    "Delete"
                                }
                            }
                        }
                    }
                }
            }
            span class="grid grid-flow-col place-items-center pb-2 print:hidden" {
                @if matches!(&data.share, Some(share) if share.is_shared) {
                    @if !matches!(&data.share, Some(share) if share.is_from_host) {
                        button class="mr-2"
                            title="Add recipe to collection"
                            hx-get=(format!("/recipes/{recipe_id}/share"))
                            hx-push-url="true" {
                            (icon_plus_circle())
                        }
                    }
                } @else {
                    button title="Share recipe" class="mr-2 hidden sm:block"
                        hx-post=(format!("/recipes/{recipe_id}/share"))
                        hx-target="#share-dialog-result"
                        _="on htmx:afterRequest from me
                            if event.detail.successful
                                if navigator.canShare
                                    set name to document.querySelector('[itemprop=name]').textContent then
                                    set data to {title: name, text: name, url: document.querySelector('#share-dialog-result input').value} then
                                    call navigator.share(data)
                                else
                                    call share_dialog.showModal()
                            end" {
                        (icon_share())
                    }
                }
                button class="mr-2 hidden sm:block" title="Duplicate recipe" hx-push-url="/recipes/add/manual" hx-get=(format!("/recipes/{recipe_id}/duplicate")) hx-target="#content" {
                    (icon_document_duplicate())
                }
                button class="mr-2 hidden sm:block" title="Print recipe" _="on click print()" {
                    (icon_printer())
                }
                @if !matches!(&data.share, Some(share) if share.is_shared) {
                    button title="Delete recipe"
                        class="mr-2 hidden sm:block"
                        hx-delete=(format!("/recipes/{recipe_id}"))
                        hx-swap="none"
                        hx-confirm="Are you sure you wish to delete this recipe?"
                        hx-indicator="#fullscreen-loader" {
                        (icon_trash())
                    }
                }
            }
        }
    }
}

fn view_recipe_media(
    fs_support: Arc<dyn FsSupport + Sync + Send>,
    recipe_details: &RecipeDetails,
    data_dir: &DataDir,
) -> Markup {
    html! {
        div class="w-[95vw] md:w-full text-center border-b border-gray-700 md:col-span-3 md:border-r md:border-b-0 flex items-center justify-center print:hidden" {
            @match recipe_details.num_media()  {
                0 => {
                    img style="object-fit: cover"
                        alt="Image of the recipe"
                        class="w-full max-h-80 md:max-h-[34rem]"
                        src="/data/images/Placeholders/placeholder.recipe.webp" {}
                },
                1 => {
                    @if recipe_details.num_images() == 1 {
                        @if let Some(image) = recipe_details.recipe.image {
                            img #output style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src=(format!("/data/images/{image}.webp"));
                        } @else {
                           img #output style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]" src="/data/images/Placeholders/placeholder.recipe.webp";
                        }
                    } @else if let Some(video) = recipe_details.videos.first() {
                        @if let Some(url) = &video.embed_url {
                            iframe src=(url) title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;" {}
                        } @else if let Some(url) = &video.content_url {
                            video controls preload="metadata" src=(url) {}
                        } @else if fs_support.is_file_exists(video.video, &data_dir.videos) {
                            video controls preload="metadata" src=(format!("/data/videos/{}.webm",video.video)) type="video/webm" {}
                        } @else {
                            p {
                                "Video is currently being processed."
                                br;
                                "Please refresh the page later."
                            }
                        }
                    }
                },
                _ => {
                    div class="carousel w-full" {
                        @for (idx, &img) in recipe_details.all_images().iter().enumerate() {
                            div id=(format!("media-{idx}")) class="carousel-item relative w-full" {
                                @if fs_support.is_file_exists(img, &data_dir.images) {
                                     img style="object-fit: cover"
                                    alt="Image of the recipe"
                                    class="w-full max-h-80 md:max-h-[34rem]"
                                    src=(format!("/data/images/{img}.webp"));
                                } @else {
                                      img style="object-fit: cover" alt="Image of the recipe" class="w-full max-h-80 md:max-h-[34rem]"
                                          src="/data/images/Placeholders/placeholder.recipe.webp";
                                }
                                div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2" {
                                    a class="btn btn-circle"
                                        href=(if idx == 0 {
                                            format!("#media-{}", recipe_details.num_media() - 1)
                                        } else {
                                            format!("#media-{}", idx-1)
                                        }) {
                                        "❮"
                                    }
                                    a class="btn btn-circle"
                                      href=(if idx == recipe_details.num_media() - 1 {
                                            "#media-0".into()
                                      } else {
                                            format!("#media-{}", idx+1)
                                        }) {
                                        "❯"
                                    }
                                }
                            }
                        }
                        @for (idx, v) in recipe_details.videos.iter().enumerate() {
                            div id=(format!("media-{}", idx+recipe_details.num_images())) class="carousel-item relative w-full" {
                                @if let Some(url) = &v.embed_url {
                                    iframe src=(url) title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen="" style="height: 100%;width: 100%;" {}
                                } @else if let Some(url) = &v.content_url {
                                    video controls preload="metadata" src=(url) {}
                                } @else if fs_support.is_file_exists(v.video, &data_dir.videos) {
                                    video controls preload="metadata" src=(format!("/data/videos/{}.webm", v.video)) type="video/webm" {}
                                } @else {
                                    p class="grid place-self-center" {
                                        (format!("Video #{} is currently being processed.", idx+1))
                                        br;
                                        "Please refresh the page later."
                                    }
                                }
                                div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2" {
                                    a class="btn btn-circle" href=(format!("#media-{}", idx.checked_add(recipe_details.num_images()).and_then(|val| val.checked_sub(1)).unwrap_or(1))) { "❮" }
                                    a class="btn btn-circle"
                                      href=(if idx == recipe_details.num_videos() - 1 {
                                            "#media-0".into()
                                        } else {
                                            format!("#media-{}", idx+recipe_details.num_images() +1)
                                        }) {
                                        "❯"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn view_recipe_nutrition(recipe_details: &RecipeDetails) -> Markup {
    html! {
        div class={
                "grid grid-flow-col border-gray-700 border-y col-span-6 md:border-t-0 md:row-span-2 print:border-none"
                @if recipe_details.nutrition.is_none() { " print:hidden" }
            } {
            table class="table table-zebra table-xs print:hidden" {
                thead {
                    tr {
                        th {
                            "Nutrition (per"
                            @if let Some(nutrition) = &recipe_details.nutrition {
                                @if nutrition.serving_size.as_deref() != Some("100g") {
                                    " serving)"
                                } @else {
                                    " 100g)"
                                }
                            } @else {
                                " 100g)"
                            }
                        }
                        th {
                            "Amount"
                        }
                    }
                }
                tbody {
                    @if let Some(nutrition) = &recipe_details.nutrition {
                        tr {
                            td { "Calories:" }
                            td { (nutrition.calories_kcal.map(|v| format!("{v} kcal")).unwrap_or("-".into())) }
                        }
                        tr {
                            td { "Total carbs:" }
                            td { (nutrition.total_carbohydrates.map(|v| format!("{v} g")).unwrap_or("-".into())) }
                        }
                        tr {
                            td { "Sugars:" }
                            td { (nutrition.sugars_g.map(|v| format!("{v} g")).unwrap_or("-".into())) }
                        }
                        tr {
                            td { "Protein:" }
                            td { (nutrition.protein_g.map(|v| format!("{v} g")).unwrap_or("-".into())) }
                        }
                        tr {
                            td { "Total fat:" }
                            td { (nutrition.total_fat_g.map(|v| format!("{v} g")).unwrap_or("-".into())) }
                        }
                        tr {
                            td { "Saturated fat:" }
                            td { (nutrition.saturated_fat_g.map(|v| format!("{v} g")).unwrap_or("-".into())) }
                        }
                        tr {
                            td { "Unsaturated fat:" }
                            td { (nutrition.unsaturated_fat_g.map(|v| format!("{v} g")).unwrap_or("-".into())) }
                        }
                        tr {
                            td { "Trans fat:" }
                            td { (nutrition.trans_fat_g.map(|v| format!("{v} g")).unwrap_or("-".into())) }
                        }
                        tr {
                            td { "Cholesterol:" }
                            td { (nutrition.cholesterol_mg.map(|v| format!("{v} mg")).unwrap_or("-".into())) }
                        }
                        tr {
                            td { "Sodium:" }
                            td { (nutrition.sodium_mg.map(|v| format!("{v} mg")).unwrap_or("-".into())) }
                        }
                        tr {
                            td { "Fiber:" }
                            td { (nutrition.fiber_g.map(|v| format!("{v} g")).unwrap_or("-".into())) }
                        }
                    } @else {
                        tr {
                            td { "Calories:" }
                            td { "-" }
                        }
                        tr {
                            td { "Total carbs:" }
                            td { "-" }
                        }
                        tr {
                            td { "Sugars:" }
                            td { "-" }
                        }
                        tr {
                            td { "Protein:" }
                            td { "-" }
                        }
                        tr {
                            td { "Total fat:" }
                            td { "-" }
                        }
                        tr {
                            td { "Saturated fat:" }
                            td { "-" }
                        }
                        tr {
                            td { "Unsaturated fat:" }
                            td { "-" }
                        }
                        tr {
                            td { "Trans fat:" }
                            td { "-" }
                        }
                        tr {
                            td { "Cholesterol:" }
                            td { "-" }
                        }
                        tr {
                            td { "Sodium:" }
                            td { "-" }
                        }
                        tr {
                            td { "Fiber:" }
                            td { "-" }
                        }
                    }
                }
            }
            @if let Some(nutrition) = &recipe_details.nutrition {
                div class="hidden pt-2 print:block print:mx-2 print:my-1" {
                    h1 class="text-sm print:mb-1" {
                        b { "Nutrition Facts" }
                    }
                    p class="text-xs" {
                        (nutrition.to_line())
                    }
                }
            }
        }
    }
}

fn ingredients_instructions(recipe: &RecipeDetails) -> Markup {
    html! {
        div #ingredients-instructions-container class="grid text-sm md:grid-flow-col md:col-span-6" {
            div class="col-span-6 border-gray-700 px-4 py-2 border-y md:col-span-2 md:border-r md:border-y-0 print:hidden" {
                @if !recipe.tools.is_empty() {
                    h2 class="font-semibold text-center underline pb-1" { "Tools" }
                    ul class="md:pb-2" {
                        @for tool in recipe.tools.iter() {
                            li class="hover:bg-gray-100 dark:hover:bg-gray-700" {
                                label class="label justify-start" {
                                    input type="checkbox" class="checkbox";
                                    span class="pl-2" { (tool.quantity.to_string()) (tool.name) }
                                }
                            }
                        }
                    }
                }
                h2 class="font-semibold text-center underline pb-1" { "Ingredients" }
                ul {
                    @for (_section, ingredients) in recipe.ingredients.iter() {
                        @for ingredient in ingredients.iter() {
                             li class="hover:bg-gray-100 dark:hover:bg-gray-700" {
                                label class="label justify-start" {
                                    input type="checkbox" class="checkbox";
                                    span class="pl-2" { (ingredient) }
                                }
                            }
                        }
                    }
                }
            }
            div class="col-span-6 px-8 py-2 border-gray-700 md:rounded-bl-none md:col-span-4 print:hidden" {
                h2 class="font-semibold text-center underline pb-1" { "Instructions" }
                ol class="grid list-decimal" {
                    @for (_section, instruction) in recipe.instructions.iter() {
                        @for instruction in instruction.iter() {
                            li class="min-w-full py-2 select-none hover:bg-gray-100 dark:hover:bg-gray-700" _="on mousedown toggle .line-through" {
                                span class="whitespace-pre-line" { (instruction) }
                            }
                        }
                    }
                }
            }
        }
    }
}
