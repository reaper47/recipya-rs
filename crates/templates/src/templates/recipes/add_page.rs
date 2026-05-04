use maud::{Markup, PreEscaped, html};

use integrations::api::all_apis;
use integrations::{FileFormat, all_apps};
use models::data::{Data, PaginationData};
use models::settings::UserSettingDetails;

use crate::templates::layouts;
use crate::templates::pagination::pagination;

/// Renders the add recipe page.
pub fn add_page(path: &str, data: &Data, user_setting: &UserSettingDetails) -> Markup {
    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" { "Add Recipe | Recipya" }
            span #data-layout data-layout="no-aside" hx-swap-oob="true" {}
            (render_add_page())
        } @else {
            (layouts::main(
                "Add Recipe",
                path,
                data,
                &render_add_page(),
                user_setting,
                true
            ))
        }
        (pagination(&PaginationData::hidden()))
    }
}
fn render_add_page() -> Markup {
    html! {
        div class="grid w-full h-full grid-cols-1 gap-4 p-4 md:grid-cols-2 md:grid-rows-[auto_1fr] xl:m-auto xl:max-w-6xl md:grid-flow-col" {
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                (render_manual_recipe_card())
            }
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                (render_fetch_websites_card())
            }
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                (render_ocr_card())
            }
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                (render_import_apps_card())
            }
        }
        (add_ocr_dialog())
        (import_recipes_dialog())
        (supported_websites_dialog())
        (supported_apps_import_dialog())
        (websites_dialog())
    }
}

fn render_fetch_websites_card() -> Markup {
    html! {
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
}

fn render_import_apps_card() -> Markup {
    html! {
        figure {
            img class="object-cover w-full h-40 rounded-t-xl"
                src="/public/img/recipes/new/schema.webp"
                alt="A bunch of shipping containers on a cargo boat.";
        }
        div class="card-body" {
            h2 class="card-title" { "Import" }
            p {
                "Import recipes via API from Mealie, Tandoor, and Nextcloud, as well as from "
                button class="underline cursor-pointer"
                    hx-get="/recipes/supported-applications"
                    hx-target="#application-results"
                    onclick="document.querySelector('#supported-apps-import-dialog').showModal()" {
                    "various apps,"
                }
                " and raw JSON files adhering to the "
                a href="https://schema.org/Recipe" target="_blank" class="link" { "recipe schema" }
                " standard."
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
}

fn render_manual_recipe_card() -> Markup {
    html! {
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
                    hx-push-url="true"
                    hx-swap="innerHTML transition:true" {
                    "Fill In"
                }
            }
        }
    }
}

fn render_ocr_card() -> Markup {
    html! {
        figure {
            img class="object-cover w-full h-40 rounded-t-xl"
                src="/public/img/recipes/new/camera.webp"
                alt="A cellphone used as a camera.";
        }
        div class="card-body" {
            h2 class="card-title" { "Scan" }
            p { "Upload the image files or PDF of the recipe you want to add or take a picture using your device's camera." }
            div class="card-actions" {
                button type="button" class="btn btn-outline btn-sm btn-block"
                // onclick="document.querySelector('#add-ocr-dialog').showModal()"
                _="on click call alert('Not implemented yet')" {
                    "Upload"
                }
            }
        }
    }
}

fn add_ocr_dialog() -> Markup {
    html! {
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
    }
}

#[allow(clippy::too_many_lines)]
fn import_recipes_dialog() -> Markup {
    html! {
        dialog #import-recipes-dialog .modal {
            div #import-recipes-dialog-container class="modal-box max-w-none w-96 max-h-[92vh] p-4 flex flex-col" {
                form method="dialog" {
                    button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                }
                h3 class="font-bold text-lg" { "Import Recipes" }
                div class="tabs tabs-lift pt-4" {
                    // Tab #1: Software
                    label class="tab" {
                        input type="radio" name="import-recipe-tab" checked _="on click set #import-recipes-dialog-container.style.width to ''";
                        "Software"
                    }
                    div class="tab-content bg-base-100 border-base-300 p-3" {
                        form class="space-y-4" enctype="multipart/form-data"
                                hx-post="/recipes/add/import/app"
                                hx-indicator="#fullscreen-loader"
                                hx-swap="none"
                                hx-on:htmx:before-request="if(!this.checkValidity()) return false; document.querySelector('#import-recipes-dialog').close()" {
                            div {
                                fieldset .fieldset {
                                    legend class="fieldset-legend" { "Choose an application" }
                                    select #app-select name="app" .select.block {
                                        option disabled selected { "Pick an application" }
                                        @for app in all_apps() {
                                            option value=(app.to_string()) { (format!("{app:?}")) }
                                        }
                                    }
                                }
                                fieldset .fieldset {
                                    legend class="fieldset-legend" { "Select a file" }
                                    input #import-dialog-file type="file" name="file" required class="file-input w-full" accept=(FileFormat::extensions().join(","));
                                }
                            }
                            button type="submit" class="btn btn-block btn-sm btn-primary w-full" {
                                "Submit"
                            }
                        }
                    }

                    // Tab #2: Import from an application using the app's API
                    label .tab {
                        input type="radio" name="import-recipe-tab" _="on click set #import-recipes-dialog-container.style.width to ''";
                        "API"
                    }
                    div class="tab-content bg-base-100 border-base-300 p-3" {
                        form class="space-y-4"
                                hx-post="/recipes/add/import/api"
                                hx-indicator="#fullscreen-loader"
                                hx-swap="none"
                                hx-on:htmx:before-request="if(!this.checkValidity()) return false; document.querySelector('#import-recipes-dialog').close()" {
                            div {
                                fieldset .fieldset {
                                    legend class="fieldset-legend" { "Choose an API" }
                                    select #api-select name="api" .select.block required {
                                        option value="" disabled selected { "Pick an API" }
                                        @for api in all_apis() {
                                            option value=(api.to_string()) { (format!("{api:?}")) }
                                        }
                                    }
                                }
                                fieldset .fieldset {
                                    legend class="fieldset-legend" { "Base URL" }
                                    input type="url" name="url" required class="input" placeholder="http://localhost:9925";
                                }
                                fieldset .fieldset {
                                    legend class="fieldset-legend" { "Username or email" }
                                    input type="text" name="username" required class="input" placeholder="Enter your username" autocomplete="username";
                                }
                                fieldset .fieldset {
                                    legend class="fieldset-legend" { "Password" }
                                    input type="password" name="password" required class="input" placeholder="Enter your password" autocomplete="current-password";
                                }
                            }
                            button type="submit" class="btn btn-block btn-sm btn-primary w-full" {
                                "Submit"
                            }
                        }
                    }

                    // Tab #3: Raw
                    label .tab {
                        input type="radio" name="import-recipe-tab" _="on click set #import-recipes-dialog-container.style.width to 'min(96vw, 1100px)' then call initJSONHighlighter('import-recipes-json')";
                        "JSON"
                    }
                    div #import-recipes-json class="tab-content bg-base-100 border-base-300 p-3" {
                        form class="flex flex-col gap-3" hx-post="/recipes/add/import/raw-json"
                             hx-indicator="#fullscreen-loader" hx-swap="none"
                             hx-on:htmx:before-request="if(!this.checkValidity()) return false"
                             hx-on:htmx:after-request="if (event.detail.xhr && event.detail.xhr.status < 400) document.querySelector('#import-recipes-dialog').close()" {

                            div class="flex flex-wrap items-center gap-2" {
                                button type="button" #beautify class="btn btn-xs" {
                                    "Beautify"
                                }
                                button type="button" #clear class="btn btn-xs" {
                                    "Clear"
                                }
                                label class="label cursor-pointer gap-2 text-xs" {
                                    span class="text-base-content" {
                                        "Wrap"
                                    }
                                    input type="checkbox" #wrap-toggle class="toggle toggle-xs" checked _="on change if me.checked then set #highlighted-content.style['white-space'] to 'pre-wrap' then set #json-input.style['white-space'] to 'pre-wrap' else set #highlighted-content.style['white-space'] to 'pre' then set #json-input.style['white-space'] to 'pre' end";
                                }
                            }

                            div class="min-h-[40vh] md:min-h-[60vh] max-h-[70vh] flex flex-col overflow-hidden" {
                                div class="grid grid-cols-1 md:grid-cols-2 gap-3 items-stretch flex-1 min-h-0" {
                                    div class="flex flex-col min-h-0" {
                                        label for="json-input" class="floating-label text-sm font-semibold mb-1" {
                                            "Paste JSON"
                                        }

                                        div class="rounded relative h-full border-2 border-solid border border-gray-300 overflow-hidden bg-neutral-900 focus-within:border-sky-600 min-h-48" {
                                            div #highlighted-content class="highlighted-content text-gray-300 bg-neutral-900 pointer-events-none z-1 overflow-auto" {}
                                            textarea #json-input name="json-input" required
                                                class="h-full w-full editor-textarea bg-transparent text-transparent caret-[#d4d4d4] z-2 overflow-auto [-webkit-text-fill-color:transparent]"
                                                autocomplete="off" autocapitalize="off" autocorrect="off" spellcheck="false" {}
                                        }
                                        div #info-bar class="px-3 py-2 bg-base-200 text-xs text-gray-600 border-l border-sky-600" {
                                            "Ready - Start typing or paste JSON to see syntax highlighting"
                                        }
                                    }

                                    div class="flex flex-col min-h-0" {
                                        div class="flex items-center gap-2" {
                                            p class="floating-label text-sm font-semibold" {
                                                "Preview"
                                            }
                                            img #spinner class="htmx-indicator mr-1" src="/public/img/bars.svg" alt="Fetching...";
                                            button type="button" class="btn btn-xs ml-auto"
                                                hx-get="/recipes/schema"
                                                hx-target="#json-schema"
                                                hx-trigger="click once"
                                                hx-indicator="#spinner"
                                                hx-on:htmx:after-request="event.stopPropagation(); if(event.detail.successful) {
                                                    const json = event.detail.xhr.responseText;
                                                    document.querySelector('#json-schema').value = event.detail.xhr.responseText;
                                                    document.querySelector('#highlighted-content2').textContent = json;
                                                    initJSONHighlighter('import-recipes-json');
                                                }"
                                                _="on click toggle .hidden on #preview-output then toggle .hidden on #schema-output then toggle .btn-active" {
                                                "Schema"
                                            }
                                        }
                                        div class="flex-1 overflow-auto border border-base-300 rounded text-sm " {
                                            div #preview-output class="min-h-0" {
                                                div class="p-4" {
                                                    p {
                                                        "Paste JSON on the left to render a preview here. For example, try:"
                                                    }
                                                    div class="relative" {
                                                        button #copy-btn type="button" aria-label="Copy example JSON"
                                                            class="btn btn-xs btn-ghost absolute right-2 top-2 z-10"
                                                            onclick="copyText('copy-btn', 'example-json')" { "Copy" }

                                                        textarea #example-json class="textarea w-full h-full" rows="30" {
                                                        r#"{
  "@context": "https://schema.org",
  "@type": "Recipe",
  "author": "John Smith",
  "cookTime": "PT1H",
  "datePublished": "2009-05-08",
  "description": "This classic banana bread recipe comes from my mom -- the walnuts add a nice texture and flavor to the banana bread.",
  "image": "bananabread.jpg",
  "recipeIngredient": [
    "3 or 4 ripe bananas, smashed",
    "1 egg",
    "3/4 cup of sugar"
  ],
  "interactionStatistic": {
    "@type": "InteractionCounter",
    "interactionType": "https://schema.org/Comment",
    "userInteractionCount": "140"
  },
  "name": "Mom's World Famous Banana Bread",
  "nutrition": {
    "@type": "NutritionInformation",
    "calories": "240 calories",
    "fatContent": "9 grams fat"
  },
  "prepTime": "PT15M",
  "recipeInstructions": "Preheat the oven to 350 degrees. Mix in the ingredients in a bowl. Add the flour last. Pour the mixture into a loaf pan and bake for one hour.",
  "recipeYield": "1 loaf",
  "suitableForDiet": "https://schema.org/LowFatDiet"
}"#
                                                        }
                                                    }
                                                }
                                            }
                                            div #schema-output class="hidden min-h-0" {
                                                div class="rounded relative min-h-0 border-2 border-solid border border-gray-300 overflow-hidden bg-neutral-900 focus-within:border-sky-600 min-h-48" {
                                                    div #highlighted-content2 class="highlighted-content text-gray-300 bg-neutral-900 pointer-events-none z-1 overflow-auto" {}
                                                    textarea #json-schema readonly name="json-schema" placeholder="Fetching schema..."
                                                        class="h-full w-full editor-textarea bg-transparent text-transparent caret-[#d4d4d4] z-2 overflow-auto [-webkit-text-fill-color:transparent]"
                                                        autocomplete="off" autocapitalize="off" autocorrect="off" spellcheck="false" {
                                                            "Please wait while schema is being fetched..."
                                                        }
                                                }
                                            }
                                        }
                                        div #errors class="mt-2 text-xs text-error" {}
                                    }
                                }

                                div class="flex gap-2 justify-end pt-2" {
                                    button type="submit" class="btn btn-primary btn-sm btn-wide" {
                                        "Submit"
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

fn supported_apps_import_dialog() -> Markup {
    html! {
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
                                th class="py-1" { "File Formats" }
                            }
                        }
                        tbody #application-results {}
                    }
                }
            }
        }
    }
}

fn supported_websites_dialog() -> Markup {
    html! {
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
    }
}

fn websites_dialog() -> Markup {
    html! {
        dialog #websites-dialog class="modal" {
            div class="modal-box max-w-md" {
                form method="dialog" {
                    button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                }
                h3 class="font-bold text-lg mb-1" { "Fetch recipes from websites" }
                p class="text-sm mb-4" { "Enter one or more URLs, each on a new line." }
                form class="flex flex-col gap-3"
                    hx-post="/recipes/add/website"
                    hx-swap="none"
                    _=(PreEscaped("on submit call #websites-dialog.close() then set me.querySelector('textarea').value to ''")) {
                    textarea class="textarea w-full text-sm" name="urls" rows="10" placeholder="https://example.com/recipe-1\nhttps://example.com/recipe-2\nhttps://example.com/recipe-3" {}
                    div class="flex justify-end gap-2" {
                        button type="button" class="btn btn-ghost btn-sm"
                            _="on click set #websites-dialog's querySelector('textarea').value to '' then call #websites-dialog.close()" {                            "Cancel"
                        }
                        button class="btn btn-primary btn-sm" { "Fetch recipes" }
                    }
                }
            }
        }
    }
}
