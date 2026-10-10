use l10n::Messages;
use maud::{Markup, PreEscaped, html};

use integrations::api::all_apis;
use integrations::{FileFormat, all_apps};
use models::data::{Data, PaginationData};
use models::settings::UserSettingDetails;

use crate::templates::helpers::{Replace, inject_markup_in_message};
use crate::templates::layouts;
use crate::templates::pagination::pagination;

/// Renders the add recipe page.
pub fn add_page(
    path: &str,
    data: &Data,
    messages: &Messages,
    user_setting: &UserSettingDetails,
) -> Markup {
    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" {
                (messages.add_recipe_page_tab_title())
            }
            (render_add_page(messages))
        } @else {
            (layouts::main(
                &messages.add_recipe_page_title(),
                path,
                data,
                &render_add_page(messages),
                messages,
                user_setting,
            ))
        }
        (pagination(&PaginationData::hidden(), messages))
    }
}
fn render_add_page(messages: &Messages) -> Markup {
    html! {
        div class="grid w-full h-full grid-cols-1 gap-4 p-4 md:grid-cols-2 md:grid-rows-[auto_1fr] xl:m-auto xl:max-w-6xl md:grid-flow-col" {
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                (render_manual_recipe_card(messages))
            }
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                (render_fetch_websites_card(messages))
            }
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                (render_ocr_card(messages))
            }
            div class="card card-border bg-base-200 h-96 shadow-sm rounded-xl" {
                (render_import_apps_card(messages))
            }
        }
        (add_ocr_dialog(messages))
        (import_recipes_dialog(messages))
        (supported_websites_dialog(messages))
        (supported_apps_import_dialog(messages))
        (websites_dialog(messages))
    }
}

fn render_fetch_websites_card(messages: &Messages) -> Markup {
    // let supported = messages.fetch_website_card_search_placeholder();
    let supported = messages.supported();
    let button_markup = html! {
        button .underline hx-get="/recipes/supported-websites" hx-target="#search-results" _="on click open #supported-websites-dialog" {
            (supported)
        }
    };

    html! {
        figure {
            img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/import.webp" alt=(messages.fetch_website_card_image_alt());
        }
        div .card-body {
            h2 .card-title {
                (messages.fetch_website_card_title())
            }
            p {
                (inject_markup_in_message(&messages.fetch_website_card_description(supported.to_string()), &[(&supported, button_markup)], Replace::First))
            }
            div class="card-actions justify-end" {
                button class="btn btn-outline btn-sm btn-block" _="on click open #websites-dialog" {
                    (messages.action_fetch())
                }
            }
        }
    }
}

fn render_import_apps_card(messages: &Messages) -> Markup {
    let various_apps = messages.import_apps_card_various_apps();
    let various_markup = html! {
        button class="underline cursor-pointer" hx-get="/recipes/supported-applications" hx-target="#application-results" _="on click open #supported-apps-import-dialog" {
            (various_apps)
        }
    };

    let schema = messages.import_apps_card_recipe_schema();
    let schema_markup = html! {
        a href="https://schema.org/Recipe" target="_blank" class="link" {
            (schema)
        }
    };

    let bookmarklet = messages.bookmarklet_name();
    let bookmarklet_markup = html! {
        a class="link tooltip" data-tip=(messages.bookmarklet_name_data_tip())
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
            (bookmarklet)
        }
    };

    html! {
        figure {
            img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/schema.webp" alt=(messages.import_apps_card_image_alt());
        }
        div .card-body {
            h2 .card-title {
                (messages.import_apps_card_title())
            }
            p {
                (inject_markup_in_message(
                    &messages.import_apps_card_description(various_apps.to_string(), schema.to_string()),
                     &[
                         (&various_apps, various_markup),
                         (&schema, schema_markup),
                     ],
                     Replace::All,
                ))
            }
            p {
                (inject_markup_in_message(
                    &messages.bookmarklet_description(bookmarklet.to_string()),
                     &[
                         (&bookmarklet, bookmarklet_markup),
                     ],
                     Replace::All,
                ))
            }
            div .card-actions {
                button class="btn btn-outline btn-sm btn-block" _="on click open #import-recipes-dialog" {
                    (messages.action_import())
                }
            }
        }
    }
}

fn render_manual_recipe_card(messages: &Messages) -> Markup {
    html! {
        figure {
            img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/manual.webp" alt=(messages.manual_recipe_card_image_alt());
        }
        div .card-body {
            h2 .card-title {
                (messages.manual_recipe_card_title())
            }
            p {
                (messages.manual_recipe_card_description())
            }
            div class="card-actions justify-end" {
                button class="btn btn-outline btn-sm btn-block"
                    hx-get="/recipes/add/manual"
                    hx-target="#content"
                    hx-push-url="true"
                    hx-swap="innerHTML transition:true" {
                    (messages.manual_recipe_card_fill_in())
                }
            }
        }
    }
}

fn render_ocr_card(messages: &Messages) -> Markup {
    html! {
        figure {
            img class="object-cover w-full h-40 rounded-t-xl" src="/public/img/recipes/new/camera.webp" alt=(messages.scan_card_image_alt());
        }
        div .card-body {
            h2 .card-title {
                (messages.scan_card_title())
            }
            p {
                (messages.scan_card_description())
            }
            div .card-actions {
                button type="button" class="btn btn-outline btn-sm btn-block"
                // onclick="document.querySelector('#add-ocr-dialog').showModal()"
                _="on click call alert('Not implemented yet')" {
                    (messages.action_upload())
                }
            }
        }
    }
}

fn add_ocr_dialog(messages: &Messages) -> Markup {
    html! {
        dialog #add-ocr-dialog .modal {
            div .modal-box {
                form method="dialog" {
                    button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                }
                h3 class="font-bold text-lg" {
                    (messages.scan_card_dialog_title())
                }
                form .py-4 hx-post="/recipes/add/ocr" hx-encoding="multipart/form-data" hx-indicator="#fullscreen-loader" hx-swap="none" _="on submit call document.querySelector('#add-ocr-dialog').close()" {
                    div class="grid mb-4" {
                        label for="add-ocr-files-input" class="floating-label text-sm font-medium mb-1" {
                            (messages.scan_card_dialog_description())
                        }
                        input #add-ocr-files-input type="file" name="files" accept=".jpg, .jpeg, .png, .bmp, .tiff, .heif, .pdf" multiple
                            class="p-2 border border-gray-300 rounded-lg shadow focus:ring-2 focus:ring-purple-600 dark:bg-gray-900 dark:border-none";
                    }
                    button class="btn btn-block btn-primary btn-sm" {
                        (messages.action_submit())
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
fn import_recipes_dialog(messages: &Messages) -> Markup {
    html! {
        dialog #import-recipes-dialog .modal {
            div #import-recipes-dialog-container class="modal-box max-w-none w-96 max-h-[92vh] p-4 flex flex-col" {
                form method="dialog" {
                    button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                }
                h3 class="font-bold text-lg" {
                    (messages.import_recipes_dialog_title())
                }
                div class="tabs tabs-lift pt-4" {
                    // Tab #1: Software
                    label .tab {
                        input type="radio" name="import-recipe-tab" checked _="on click set #import-recipes-dialog-container.style.width to ''";
                        (messages.import_recipes_dialog_software())
                    }
                    div class="tab-content bg-base-100 border-base-300 p-3" {
                        form .space-y-4 enctype="multipart/form-data"
                                hx-post="/recipes/add/import/app"
                                hx-indicator="#fullscreen-loader"
                                hx-swap="none"
                                hx-on:htmx:before-request="if(!this.checkValidity()) return false; document.querySelector('#import-recipes-dialog').close()" {
                            div {
                                fieldset .fieldset {
                                    legend .fieldset-legend {
                                        (messages.import_recipes_dialog_app())
                                    }
                                    select #app-select name="app" .select.block {
                                        option disabled selected {
                                            (messages.import_recipes_dialog_app_placeholder())
                                        }
                                        @for app in all_apps() {
                                            option value=(app) { (app) }
                                        }
                                    }
                                }
                                fieldset .fieldset {
                                    legend .fieldset-legend {
                                        (messages.import_recipes_dialog_select_file())
                                    }
                                    input #import-dialog-file type="file" name="file" required class="file-input w-full" accept=(FileFormat::extensions().join(","));
                                }
                            }
                            button type="submit" class="btn btn-block btn-sm btn-primary w-full" {
                                (messages.action_submit())
                            }
                        }
                    }

                    // Tab #2: Import from an application using the app's API
                    label .tab {
                        input type="radio" name="import-recipe-tab" _="on click set #import-recipes-dialog-container.style.width to ''";
                        (messages.import_recipes_dialog_api())
                    }
                    div class="tab-content bg-base-100 border-base-300 p-3" {
                        form .space-y-4
                                hx-post="/recipes/add/import/api"
                                hx-indicator="#fullscreen-loader"
                                hx-swap="none"
                                hx-on:htmx:before-request="if(!this.checkValidity()) return false; document.querySelector('#import-recipes-dialog').close()" {
                            div {
                                fieldset .fieldset {
                                    legend .fieldset-legend {
                                        (messages.import_recipes_dialog_choose_api())
                                    }
                                    select #api-select name="api" .select.block required {
                                        option value="" disabled selected {
                                            (messages.import_recipes_dialog_choose_api_placeholder())
                                        }
                                        @for api in all_apis() {
                                            option value=(api.to_string()) {
                                                (format!("{api:?}"))
                                            }
                                        }
                                    }
                                }
                                fieldset .fieldset {
                                    legend .fieldset-legend {
                                        (messages.import_recipes_dialog_base_url())
                                    }
                                    input type="url" name="url" required .input placeholder="http://localhost:9925";
                                }
                                fieldset .fieldset {
                                    legend .fieldset-legend {
                                        (messages.user_input_label())
                                    }
                                    input type="text" name="username" required .input placeholder=(messages.user_input_placeholder()) autocomplete="username";
                                }
                                fieldset .fieldset {
                                    legend .fieldset-legend {
                                        (messages.password_input_label())
                                    }
                                    input type="password" name="password" required .input placeholder=(messages.password_input_placeholder()) autocomplete="current-password";
                                }
                            }
                            button type="submit" class="btn btn-block btn-sm btn-primary w-full" {
                                (messages.action_submit())
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
                                    (messages.raw_json_dialog_beautify())
                                }
                                button type="button" #clear class="btn btn-xs" {
                                    (messages.action_clear())
                                }
                                label class="label cursor-pointer gap-2 text-xs" {
                                    span class="text-base-content" {
                                        (messages.raw_json_dialog_wrap())
                                    }
                                    input type="checkbox" #wrap-toggle class="toggle toggle-xs" checked _="on change if me.checked then set #highlighted-content.style['white-space'] to 'pre-wrap' then set #json-input.style['white-space'] to 'pre-wrap' else set #highlighted-content.style['white-space'] to 'pre' then set #json-input.style['white-space'] to 'pre' end";
                                }
                            }

                            div class="min-h-[40vh] md:min-h-[60vh] max-h-[70vh] flex flex-col overflow-hidden" {
                                div class="grid grid-cols-1 md:grid-cols-2 gap-3 items-stretch flex-1 min-h-0" {
                                    div class="flex flex-col min-h-0" {
                                        label for="json-input" class="floating-label text-sm font-semibold mb-1" {
                                            (messages.raw_json_dialog_paste_json())
                                        }

                                        div class="rounded relative h-full border-2 border-solid border border-gray-300 overflow-hidden bg-neutral-900 focus-within:border-sky-600 min-h-48" {
                                            div #highlighted-content class="highlighted-content text-gray-300 bg-neutral-900 pointer-events-none z-1 overflow-auto" {}
                                            textarea #json-input name="json-input" required
                                                class="h-full w-full editor-textarea bg-transparent text-transparent caret-[#d4d4d4] z-2 overflow-auto [-webkit-text-fill-color:transparent]"
                                                autocomplete="off" autocapitalize="off" autocorrect="off" spellcheck="false" {}
                                        }
                                        div #info-bar class="px-3 py-2 bg-base-200 text-xs text-gray-600 border-l border-sky-600" {
                                            (messages.raw_json_dialog_type_json())
                                        }
                                    }

                                    div class="flex flex-col min-h-0" {
                                        div class="flex items-center gap-2" {
                                            p class="floating-label text-sm font-semibold" {
                                                (messages.raw_json_dialog_preview())
                                            }
                                            img #spinner class="htmx-indicator mr-1" src="/public/img/bars.svg" alt=(messages.raw_json_dialog_fetching());
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
                                                (messages.raw_json_dialog_schema())
                                            }
                                        }
                                        div class="flex-1 overflow-auto border border-base-300 rounded text-sm " {
                                            div #preview-output .min-h-0 {
                                                div .p-4 {
                                                    p {
                                                        (messages.raw_json_dialog_paste_json_long())
                                                    }
                                                    div .relative {
                                                        button #copy-btn type="button" aria-label=(messages.raw_json_dialog_copy_example())
                                                            class="btn btn-xs btn-ghost absolute right-2 top-2 z-10"
                                                            onclick="copyText('copy-btn', 'example-json')" {
                                                            (messages.action_copy())
                                                        }

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
                                                    textarea #json-schema readonly name="json-schema" placeholder=(messages.raw_json_dialog_fetch_schema())
                                                        class="h-full w-full editor-textarea bg-transparent text-transparent caret-[#d4d4d4] z-2 overflow-auto [-webkit-text-fill-color:transparent]"
                                                        autocomplete="off" autocapitalize="off" autocorrect="off" spellcheck="false" {
                                                            (messages.raw_json_dialog_fetch_wait())
                                                        }
                                                }
                                            }
                                        }
                                        div #errors class="mt-2 text-xs text-error" {}
                                    }
                                }

                                div class="flex gap-2 justify-end pt-2" {
                                    button type="submit" class="btn btn-primary btn-sm btn-wide" {
                                        (messages.action_submit())
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

fn supported_apps_import_dialog(messages: &Messages) -> Markup {
    html! {
        dialog #supported-apps-import-dialog .modal {
            div class="modal-box h-2/3" {
                form method="dialog" {
                    button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                }
                h3 .mb-1 {
                    label .floating-label {
                        input type="search" placeholder=(messages.import_apps_card_search_placeholder()) class=(PreEscaped("input input-sm w-11/12"))
                            _=(PreEscaped("on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"));
                    }
                }
                div .overflow-x-auto {
                    table class="table table-zebra table-sm" {
                        thead {
                            tr .text-center {
                                th .py-1 {
                                    (messages.column_number())
                                }
                                th .py-1 {
                                    (messages.import_apps_card_app())
                                }
                                th .py-1 {
                                    (messages.import_apps_card_file_formats())
                                }
                            }
                        }
                        tbody #application-results {}
                    }
                }
            }
        }
    }
}

fn supported_websites_dialog(messages: &Messages) -> Markup {
    html! {
        dialog #supported-websites-dialog .modal {
            div class="modal-box h-2/3" {
                form method="dialog" {
                    button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                }
                h3 .mb-1 {
                    label .floating-label {
                        input type="search" placeholder=(messages.fetch_website_card_search_placeholder()) class="input input-sm w-11/12"
                              _=(PreEscaped("on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()"));
                    }
                }
                div .overflow-x-auto {
                    table class="table table-zebra table-sm" {
                        thead {
                            tr .text-center {
                                th .py-1 {
                                    (messages.column_number())
                                }
                                th .py-1 {
                                    (messages.fetch_website_card_title())
                                }
                            }
                        }
                        tbody #search-results {}
                    }
                }
            }
        }
    }
}

fn websites_dialog(messages: &Messages) -> Markup {
    html! {
        dialog #websites-dialog .modal {
            div class="modal-box max-w-md" {
                form method="dialog" {
                    button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" { "✕" }
                }
                h3 class="font-bold text-lg mb-1" {
                    (messages.fetch_website_card_dialog_title())
                }
                p class="text-sm mb-4" {
                    (messages.fetch_website_card_dialog_description())
                }
                form class="flex flex-col gap-3" hx-post="/recipes/add/website" hx-swap="none"
                    _=(PreEscaped("on submit call #websites-dialog.close() then set me.querySelector('textarea').value to ''")) {
                    textarea class="textarea w-full text-sm" name="urls" rows="10" placeholder="https://example.com/recipe-1\nhttps://example.com/recipe-2\nhttps://example.com/recipe-3" {}
                    div class="flex justify-end gap-2" {
                        button type="button" class="btn btn-ghost btn-sm" _="on click set #websites-dialog's querySelector('textarea').value to '' then call #websites-dialog.close()" {
                            (messages.action_cancel())
                        }
                        button class="btn btn-primary btn-sm" {
                            (messages.fetch_website_card_fetch_recipes())
                        }
                    }
                }
            }
        }
    }
}
