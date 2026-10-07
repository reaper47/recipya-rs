use fluent_static::Message;
use maud::{Markup, PreEscaped, html};
use strum::IntoEnumIterator;
use time::macros::format_description;
use time_tz::TimeZone;

use config::{DemoState, States};
use l10n::Messages;
use math::cooking::units::system::MeasurementSystem;
use models::{
    Recipe, data::Data, language::Language, recipe::structs::recipe::Category,
    settings::UserSettingDetails, theme::Theme, user::User,
};
use nutrition::NutritionDataSource;

use crate::templates::common::cancel_submit_form_actions;
use crate::templates::helpers::inject_markup_in_message;
use crate::templates::icons;

/// A Hyperscript snippet for filtering table rows.
pub(super) const SEARCH_INPUT_JS: &str = "on input show <tbody>tr/> in next <table/> when its textContent.toLowerCase() contains my value.toLowerCase()";

/// Stores all the settings required for rendering the settings page.
pub struct SettingsForView {
    pub states: States,
    pub email: EmailSettingsForView,
    pub azure_di_key: String,
    pub azure_di_endpoint: String,
}

/// Components for the email configuration settings.
pub struct EmailSettingsForView {
    pub email_admin: String,
    pub host: String,
    pub username: String,
    pub is_connected: bool,
}

/// Renders a new recipe category form.
pub fn new_recipe_category(category: &str, messages: &Messages) -> Markup {
    html! {
        div class="badge badge-outline p-3 pr-0" {
            form .inline-flex hx-delete="/recipes/categories" hx-target=(PreEscaped("closest <div/>")) hx-swap="delete" {
                input type="hidden" name="category" value=(category);
                span .select-none { (category) }
                button type="submit" .btn.btn-xs.btn-ghost { "X" }
            }
        }
        (empty_recipe_category(messages))
    }
}

fn empty_recipe_category(messages: &Messages) -> Markup {
    html! {
        div class="badge badge-outline p-3 pr-0" {
            form .inline-flex hx-post="/recipes/categories" hx-target=(PreEscaped("closest <div/>")) hx-swap="outerHTML" {
                label .input {
                    input required type="text" placeholder=(messages.settings_recipes_new_category()) class="input input-ghost input-xs w-[16ch] focus:outline-none" name="category" autocomplete="off";
                }
                button .btn.btn-xs.btn-ghost {
                    (PreEscaped("&#10003;"))
                }
            }
        }
    }
}

/// Renders the settings dialog.
pub fn settings(
    data: &Data,
    users: Option<Vec<User>>,
    user_settings: &UserSettingDetails,
    categories: &[Category],
    languages: &[Language],
    config: &SettingsForView,
    messages: &Messages,
) -> Markup {
    let onclick = |settings_block_id: &str| -> Markup {
        PreEscaped(format!(
            "on mousedown add .hidden to the children of #settings-blocks then remove .hidden from #settings-{settings_block_id}"
        ))
    };

    html! {
        div class="flex flex-col menu-sm sm:flex-row sm:menu-md" {
            ul class="menu menu-horizontal flex-nowrap overflow-x-auto w-full sm:overflow-x-clip sm:w-48 sm:menu-vertical"
               _=(PreEscaped("on click remove .menu-active from .setting-tab then add .menu-active to closest <a/> to event.target")) {

                li {
                    a class="setting-tab menu-active" _=(onclick("recipes")) {
                        (icons::cube_transparent())
                        (messages.recipes())
                    }
                }
                li {
                    a class="setting-tab" _=(onclick("general")) {
                        (icons::cpu())
                        (messages.settings_tabs_general())
                    }
                }
                @if data.is_admin {
                    li {
                        a class="setting-tab" _=(onclick("connections")) {
                            (icons::cloud())
                            (messages.settings_tabs_connections())
                        }
                    }
                }
                li {
                    a class="setting-tab" _=(onclick("data")) {
                        (icons::circle_stack())
                        (messages.settings_tabs_data())
                    }
                }
                @if data.is_admin {
                    li {
                        a class="setting-tab" _=(onclick("server")) {
                            (icons::server())
                            (messages.settings_tabs_server())
                        }
                    }
                    li {
                        a class="setting-tab" _=(onclick("admin")) {
                            (icons::building_library())
                            (messages.settings_tabs_admin())
                        }
                    }
                }
                li {
                    a class="setting-tab" _=(onclick("account")) {
                        (icons::user_circle())
                        (messages.settings_tabs_account())
                    }
                }
                li {
                    a class="setting-tab" _=(onclick("about")) {
                        (icons::information_circle(false))
                        (messages.settings_tabs_about())
                    }
                }
            }
            div #settings-blocks class="w-full md:h-[50vh] md:max-h-[50vh]" style="padding-right: 1rem" {
                (settings_recipes(categories, user_settings, messages))
                (settings_general(user_settings, languages, messages))
                @if data.is_admin {
                    (settings_connections(config, messages))
                    (settings_server(data, config, messages))
                    (settings_admin(&users.unwrap_or_default(), user_settings, config.states.demo, messages))
                }
                (settings_data(data, messages))
                (settings_account(messages))
                (settings_about(data, messages))
            }
        }
        (export_data_dialog())
        (paper_sizes_dialog())
        (supported_nutrition_sources_dialog(user_settings, messages))
    }
}

#[allow(clippy::too_many_lines)]
fn settings_recipes(
    categories: &[Category],
    settings: &UserSettingDetails,
    messages: &Messages,
) -> Markup {
    html! {
        div #settings-recipes class="p-3 md:h-[50vh] overflow-y-auto" {
            div class="flex justify-between items-center text-sm" {
                details .w-full {
                    summary class="font-semibold cursor-default select-none" {
                        (messages.settings_recipes_categories())
                    }
                    div class="flex flex-wrap gap-2 p-2" {
                        @for category in categories.iter().map(|c| c.name.as_str()) {
                            div class="category-badge badge badge-outline p-3 pr-0" {
                                form class="inline-flex" hx-delete="/recipes/categories" hx-target="closest <div/>" hx-swap="delete" {
                                    input type="hidden" name="category" value=(category);
                                    span class="select-none" { (category) }
                                    button type="submit" class="btn btn-xs btn-circle btn-ghost" {
                                        "X"
                                    }
                                }
                            }
                        }
                    }
                    div class="badge badge-outline p-3 pr-0 ml-2" {
                        form .inline-flex hx-post="/recipes/categories" hx-target="closest <div/>" hx-swap="outerHTML" {
                            label .form-control {
                                input required type="text" placeholder=(messages.settings_recipes_new_category()) class="input input-ghost input-xs w-[16ch] focus:outline-none" name="category" autocomplete="off";
                            }
                            button class="btn btn-xs btn-circle btn-ghost" {
                                (PreEscaped("&#10003;"))
                            }
                        }
                    }
                }
            }
            label class="flex justify-between items-center text-sm mt-3" for="settings-recipes-bold-ingredients" {
                div {
                    span .font-semibold {
                        (messages.settings_recipes_bold_ingredients())
                    }
                    br;
                    span .text-xs {
                        (messages.settings_recipes_bold_ingredients_description())
                    }
                }
                input type="checkbox" name="bold-ingredients" #settings-recipes-bold-ingredients .checkbox
                  checked[settings.is_bold_ingredients]
                  hx-post="/settings/bold-ingredients"
                  hx-trigger="click";
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                label for="settings-recipes-measurement-system" .font-semibold {
                    (messages.settings_recipes_measurement_system())
                }
                select #settings-recipes-measurement-system name="system" class="block w-fit select select-bordered select-sm" hx-post="/settings/measurement-system" hx-swap="none" {
                    @for system in MeasurementSystem::iter() {
                        option value=(system) selected[system == settings.measurement_system] {
                            (system)
                        }
                    }
                }
            }
            label class="flex justify-between items-center text-sm mt-2" for="settings-recipes-convert" {
                div {
                    span .font-semibold {
                        (messages.settings_recipes_convert_automatically())
                    }
                    br;
                    span class="text-xs" {
                        (messages.settings_recipes_convert_automatically_description())
                    }
                }
                input type="checkbox" name="convert" #settings-recipes-convert .checkbox
                  checked[settings.is_convert_automatically]
                  hx-post="/settings/convert-automatically"
                  hx-trigger="click";
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                div {
                    p .font-semibold {
                        (messages.settings_recipes_nutrition_data_source())
                    }
                    p .text-xs {
                        (messages.settings_recipes_nutrition_data_source_description())
                    }
                    button class="btn btn-xs mt-2" _="on click open #supported-nutrition-sources-dialog" {
                        (messages.settings_recipes_view_sources())
                    }
                }
                select #settings-recipes-nutrition-source name="nutrition-source" class="block w-fit select select-bordered select-sm" hx-post="/settings/nutrition/source" hx-swap="none" {
                    optgroup label=(messages.country_usa()) {
                        option value=(NutritionDataSource::USDAFoodDataCentral) selected[NutritionDataSource::USDAFoodDataCentral == settings.nutrition_source] {
                            (NutritionDataSource::USDAFoodDataCentral)
                        }
                    }
                }
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                details .w-full {
                    summary class="font-semibold cursor-default select-none" {
                        (messages.placeholders_title())
                    }
                    div class="flex flex-wrap gap-2 p-2 flex-row" {
                        div .max-w-60 {
                            p class="text-center mb-1 font-medium underline" {
                                (messages.recipes())
                            }
                            form hx-post="/placeholder" hx-encoding="multipart/form-data" hx-swap="none"
                                  _=(PreEscaped("on htmx:afterRequest call reloadImg('/data/images/Placeholders/placeholder.recipe.webp')")) {
                                img src="/data/images/Placeholders/placeholder.recipe.webp" alt="Recipe placeholder" class="w-60 h-60";
                                input type="hidden" name="name" value="recipe";
                                input type="file" name="images" class="file-input file-input-bordered file-input-sm max-w-60 mt-1";
                                button class="btn btn-neutral btn-sm btn-block my-1" {
                                    (messages.action_update())
                                }
                            }
                            button class="btn btn-error btn-sm btn-block"
                                   hx-post="/placeholder/restore"
                                   hx-vals=r#"js:{t: "recipe"}"#
                                   hx-swap="none"
                                   _=(PreEscaped("on htmx:afterRequest call reloadImg('/data/images/Placeholders/placeholder.recipe.webp')")) {
                                (messages.action_restore_original())
                            }
                        }
                        div class="max-w-60" {
                            p class="text-center mb-1 font-medium underline" {
                                (messages.cookbooks())
                            }
                            form hx-post="/placeholder" hx-encoding="multipart/form-data" hx-swap="none"
                                 _=(PreEscaped("on htmx:afterRequest call reloadImg('/data/images/Placeholders/placeholder.cookbook.webp')")) {
                                img src="/data/images/Placeholders/placeholder.cookbook.webp" alt="Cookbook placeholder" class="w-60 h-60";
                                input type="hidden" name="name" value="cookbook";
                                input type="file" name="images" class="file-input file-input-bordered file-input-sm max-w-60 mt-1";
                                button class="btn btn-neutral btn-sm btn-block my-1" {
                                    (messages.action_update())
                                }
                            }
                            button class="btn btn-error btn-sm btn-block"
                                   hx-post="/placeholder/restore"
                                   hx-vals=r#"js:{name: "cookbook"}"#
                                   hx-swap="none"
                                   _=(PreEscaped("on htmx:afterRequest call reloadImg('/data/images/Placeholders/placeholder.cookbook.webp')")) {
                                (messages.action_restore_original())
                            }
                        }
                    }
                }
            }
        }
    }
}

fn supported_nutrition_sources_dialog(
    settings: &UserSettingDetails,
    messages: &Messages,
) -> Markup {
    html! {
        dialog #supported-nutrition-sources-dialog class="justify-self-center self-center" {
            div class="card bg-base-100 shadow-sm min-w-[50vw]" {
                div .card-body {
                    h3 .mb-1 {
                        label class="input input-sm" {
                            svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {
                                g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor" {
                                    circle cx="11" cy="11" r="8" {}
                                    path d="m21 21-4.3-4.3" {}
                                 }
                            }
                            input type="search" placeholder=(messages.nutrition_sources_dialog_search_placeholder()) _=(PreEscaped(SEARCH_INPUT_JS));
                        }
                    }
                    div class="overflow-auto h-96" {
                        table class="table table-zebra table-sm" {
                            thead {
                                tr .text-center {
                                    th .py-1 { (messages.column_number()) }
                                    th .py-1 { (messages.column_name()) }
                                    th .py-1 { (messages.column_description()) }
                                    th .py-1 { (messages.column_country()) }
                                    th .py-1 { (messages.nutrition_sources_dialog_last_updated()) }
                                    th .py-1 { (messages.column_website()) }
                                }
                            }
                            tbody #search-results {
                                @for (idx, source) in settings.nutrition_sources.iter().enumerate() {
                                    tr {
                                        td .py-1 { (idx + 1) }
                                        td .py-1 { (source.name) }
                                        td .py-1 { (source.description) }
                                        td .py-1 { (source.country) }
                                        td .py-1 {
                                            (source.updated_on.map_or_else(
                                                || messages.unknown().to_string(),
                                                |date| {
                                                    date.format(format_description!("[year]-[month]-[day]"))
                                                        .unwrap()
                                                },
                                            ))
                                        }
                                        td .py-1 {
                                            a .link href=(source.url) target="_blank" {
                                                (messages.action_visit())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div class="card-actions justify-end" {
                        button class="btn btn-sm" onclick="this.closest('dialog').close()" {
                            (messages.action_close())
                        }
                    }
                }
            }
        }
    }
}

fn settings_general(
    user_settings: &UserSettingDetails,
    languages: &[Language],
    messages: &Messages,
) -> Markup {
    let mut all_tz = time_tz::timezones::iter()
        .filter_map(|tz| {
            let name = tz.name();
            (!name.to_lowercase().starts_with("etc/")).then_some(name)
        })
        .collect::<Vec<_>>();
    all_tz.sort_unstable();

    html! {
        div #settings-general class="p-3 max-h-96 hidden" {
            div class="flex justify-between items-center text-sm" {
                div {
                    p .font-semibold {
                        (messages.settings_general_theme())
                    }
                    p class="font-normal text-xs" {
                        (messages.settings_general_theme_description())
                    }
                }
                (themes_palette(false, &user_settings.default_theme, &user_settings.selected_theme, messages))
            }
            div class="flex justify-between items-center text-sm mt-2" {
                div {
                    p .font-semibold {
                        (messages.settings_general_language())
                    }
                    p class="font-normal text-xs" {
                        (messages.settings_general_language_description())
                    }
                }
                select name="locale" class="block w-fit select select-bordered select-sm" hx-post="/settings/language" hx-swap="none" {
                    @for lang in languages {
                        option value=(lang.id) selected[lang.locale == user_settings.selected_locale] {
                            (lang.name)
                        }
                    }
                }
            }
            div class="flex justify-between items-center text-sm mt-2" {
                div {
                    p .font-semibold {
                        (messages.settings_general_paper_size())
                    }
                    p class="text-xs" {
                        (messages.settings_general_paper_size_description())
                    }
                    button class="btn btn-xs mt-2"
                        hx-get="/paper-sizes"
                        hx-trigger="mousedown"
                        hx-target="#paper-sizes-dialog"
                        _="on click open #paper-sizes-dialog" {
                        (messages.settings_general_view_sizes())
                    }
                }
                select #settings-general-paper-size name="paper-size" class="block w-fit select select-bordered select-sm" hx-post="/settings/paper-size" hx-swap="none" {
                    @for (category, papers) in &user_settings.paper_sizes {
                        optgroup label=(category) {
                            @for paper in papers {
                                option value=(paper.id) selected[paper.id == user_settings.paper_size_id] {
                                    (paper.name)
                                }
                            }
                        }
                    }
                }
            }
            div class="flex justify-between items-center text-sm mt-2" {
                @let selected_tz = user_settings.tz_name();
                div {
                    p .font-semibold {
                        (messages.settings_general_timezone())
                    }
                    p class="text-xs" {
                        (messages.settings_general_timezone_description())
                    }
                }
                select #settings-general-tz name="tz" class="block w-fit select select-bordered select-sm" hx-post="/settings/tz" hx-swap="none" {
                        @for tz in all_tz {
                            option value=(tz) selected[tz == selected_tz] { (tz) }
                        }
                }
            }
        }
    }
}

fn paper_sizes_dialog() -> Markup {
    html! {
        dialog #paper-sizes-dialog class="justify-self-center self-center" {}
    }
}

fn settings_connections(config: &SettingsForView, messages: &Messages) -> Markup {
    html! {
        div #settings-connections class="p-3 overflow-y-auto max-h-96 hidden" {
            div class="flex justify-between items-center text-sm" {
                details .w-full {
                    summary class="font-semibold cursor-default select-none" {
                        (messages.email_configuration_title())
                        br;
                        span class="text-xs font-normal" {
                            (messages.email_configuration_description())
                        }
                    }
                    div .pt-2 {
                        div class="overflow-x-auto" {
                            table class="table table-xs" {
                                thead {
                                    tr {
                                        th {}
                                        th { (messages.column_setting()) }
                                        th { (messages.column_environment()) }
                                        th { (messages.column_value()) }
                                    }
                                }
                                tbody {
                                    @for (setting, env, value) in [
                                        (messages.email_configuration_host(), "SMTP_HOST", &config.email.host),
                                        (messages.email_configuration_from(), "SMTP_FROM_EMAIL", &config.email.email_admin),
                                        (messages.email_configuration_username(), "SMTP_USERNAME", &config.email.username),
                                        (messages.email_configuration_password(), "SMTP_PASSWORD", &"Not displayed".to_string()),
                                    ] {
                                        tr {
                                            th { }
                                            td { (setting) }
                                            td { (env) }
                                            td { (value) }
                                        }
                                    }
                                }
                            }
                        }
                        p class="pt-2 text-xs text-center" {
                            (messages.settings_general_no_edit_runtime())
                        }
                    }
                }
                @if config.email.is_connected {
                    div class="float-right self-baseline" title=(messages.settings_connections_established()) {
                        (icons::check_circle())
                    }
                } @else {
                    div class="float-right self-baseline" title=(messages.settings_connections_no_connection()) {
                        (icons::x_circle())
                    }
                }
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                details .w-full {
                    summary class="font-semibold cursor-default select-none" {
                        (messages.settings_connections_azure_ai_document_intelligence())
                        br;
                        span class="text-xs font-normal" {
                            (messages.settings_connections_azure_ai_document_intelligence_description())
                        }
                    }
                    form class="grid w-full" hx-put="/settings/config" hx-swap="none" {
                        fieldset .fieldset {
                            legend .fieldset-legend {
                                (messages.settings_connections_resource_key())
                            }
                            input name="integrations.ocr.key" type="text" placeholder=(messages.settings_connections_resource_key_placeholder()) value="{ data.Settings.Config.Integrations.AzureDI.Key }" autocomplete="off" class="input input-bordered input-sm";
                        }
                        fieldset .fieldset {
                            legend .fieldset-legend {
                                (messages.settings_connections_endpoint())
                            }
                            input name="integrations.ocr.url" type="url" placeholder=(messages.settings_connections_vision_placeholder()) value="{ data.Settings.Config.Integrations.AzureDI.Endpoint }" autocomplete="off" class="input input-bordered input-sm";
                        }
                        button class="btn btn-soft btn-sm mt-2" {
                            (messages.action_update())
                        }
                    }
                }
                button type="button" title=(messages.settings_connections_test()) class="btn btn-xs float-right self-baseline hover:text-secondary" hx-get="/integrations/test-connection?api=azure-di" hx-swap="none" {
                    (icons::arrow_path())
                }
            }
        }
    }
}

fn settings_server(data: &Data, config: &SettingsForView, messages: &Messages) -> Markup {
    html! {
        div #settings-server class="hidden p-3 md:max-h-96" {
            div class="flex justify-between items-center text-sm" {
                div .pt-2 {
                    p class="font-semibold pb-2" {
                        (messages.server_configuration_table_title())
                    }
                    div class="overflow-x-auto" {
                        table class="table table-xs" {
                            thead {
                                tr {
                                    th {}
                                    th { (messages.column_setting()) }
                                    th { (messages.column_description()) }
                                    th { (messages.column_environment()) }
                                    th { (messages.column_value()) }
                                }
                            }
                            tbody {
                                @let options: [(Message, Message, &str, bool); 3] = [
                                    (
                                        messages.server_configuration_table_autologin(),
                                        messages.server_configuration_table_autologin_description(),
                                        "RECIPYA_IS_AUTOLOGIN",
                                        data.states.autologin.into(),
                                    ),
                                    (
                                        messages.server_configuration_table_allow_signups(),
                                        messages.server_configuration_table_allow_signups_description(),
                                        "RECIPYA_IS_ALLOW_SIGNUPS",
                                        data.states.signups.into(),
                                    ),
                                    (
                                        messages.server_configuration_table_is_demo(),
                                        messages.server_configuration_table_is_demo_description(),
                                        "RECIPYA_IS_DEMO",
                                        config.states.demo.into(),
                                    ),
                                ];

                                @for (setting, description, env, value) in options {
                                    tr {
                                        th { }
                                        td { (setting) }
                                        td { (description) }
                                        td { (env) }
                                        td { (value) }
                                    }
                                }
                            }
                        }
                    }
                    p class="pt-2 text-xs text-center" {
                        (messages.settings_general_no_edit_runtime())
                    }
                }
            }
        }
    }
}

fn settings_admin(
    users: &[User],
    user_settings: &UserSettingDetails,
    demo: DemoState,
    messages: &Messages,
) -> Markup {
    html! {
        div #settings-admin class="hidden p-3 md:max-h-96"  {
            div class="flex justify-between items-center text-sm" {
                div {
                    p .font-semibold {
                        (messages.settings_admin_default_theme())
                    }
                    p class="font-normal text-xs" {
                        (messages.settings_admin_default_theme_description())
                    }
                }
                (themes_palette(true, &user_settings.default_theme, &user_settings.selected_theme, messages))
            }
            div class="divider m-0" {}
            @if demo == DemoState::Off {
                div class="flex justify-between items-center text-sm pb-4" {
                    details .w-full {
                        summary class="font-semibold cursor-default select-none" {
                            (messages.users_label())
                        }
                        div class="overflow-x-auto overflow-y-auto max-h-96" {
                            (render_users_table(users, false, messages))
                        }
                    }
                }
            }
        }
    }
}

pub fn render_users_table(users: &[User], is_swap_oob: bool, messages: &Messages) -> Markup {
    if is_swap_oob {
        html! {
            table #users-table class="table table-zebra table-sm" hx-swap-oob="true" {
                tbody {
                    @for (idx, user) in users.iter().enumerate() {
                        (user_row(idx + 1, user, messages))
                    }
                    (new_user_row(users.len() + 1, messages))
                }
            }
        }
    } else {
        html! {
            table #users-table class="table table-zebra table-sm" {
                tbody {
                    @for (idx, user) in users.iter().enumerate() {
                        (user_row(idx + 1, user, messages))
                    }
                    (new_user_row(users.len() + 1, messages))
                }
            }
        }
    }
}

pub fn new_user_with_new_row(curr_idx: usize, user: &User, messages: &Messages) -> Markup {
    html! {
        (user_row(curr_idx, user, messages))
        (new_user_row(curr_idx + 1, messages))
    }
}

pub fn user_row(idx: usize, user: &User, messages: &Messages) -> Markup {
    let user_id = user.id;

    html! {
        tr id={ "user-row-" (idx) } {
            th { (idx) }
            td { (user.email) }
            td { "" }
            td class="grid grid-flow-col gap-2" {
            button type="button" class="btn btn-ghost btn-square btn-xs" hx-get={ "/admin/user/" (user_id) } hx-target={ "#user-row-" (idx) } hx-swap="outerHTML" hx-vals=(format!(r#"{{"row-index": "{idx}"}}"#)) {
                    (icons::pencil(true))
                }
                button type="submit" class="btn btn-ghost btn-square btn-xs" hx-delete={ "/admin/user/" (user_id) } hx-confirm=(messages.users_delete_question()) {
                    (icons::trash())
                }
            }
        }
    }
}

pub fn edit_user_row(curr_idx: usize, user: &User, messages: &Messages) -> Markup {
    let user_id = user.id;
    let hx_target = format!("#user-row-{curr_idx}");

    html! {
        tr id={ "user-row-" (curr_idx) } {
            th { (curr_idx + 1) }
            td { (user.email) }
            td {
                input type="hidden" name="row-index" value=(curr_idx);
                input #password type="password" required
                       placeholder=(messages.new_password_input_label())
                       class="input input-sm mb-1" name="new-password" autocomplete="off"
                       hx-post="/admin/user"
                       hx-target="#new-row-user"
                       hx-swap="outerHTML"
                       hx-include="#email,#password,#confirm-password"
                       hx-trigger="keydown[key=='Enter']"
                       _="on htmx:afterRequest call document.activeElement.blur()";
                input #confirm-password type="password" required
                       placeholder=(messages.confirm_password_input_placeholder())
                       class="input input-sm" name="new-password-confirm" autocomplete="off"
                       hx-post="/admin/user"
                       hx-target="#new-row-user"
                       hx-swap="outerHTML"
                       hx-include="#email,#password,#confirm-password"
                       hx-trigger="keydown[key=='Enter']"
                       _="on htmx:afterRequest call document.activeElement.blur()";
            }
            td class="grid grid-flow-col gap-2" {
                button type="button"
                       class="btn btn-ghost btn-square btn-xs hover:text-green-600"
                       hx-patch=(format!("/admin/user/{user_id}"))
                       hx-target=(hx_target)
                       hx-swap="outerHTML"
                       hx-include="closest tr" {
                    (icons::check_circle())
                }

                button type="button"
                       class="btn btn-ghost btn-square btn-xs hover:text-red-600"
                       hx-get={ "/admin/user/" (user_id) "/row" }
                       hx-target=(hx_target)
                       hx-swap="outerHTML"
                       hx-vals=(format!(r#"{{"row-index": "{curr_idx}"}}"#)) {
                    (icons::x_circle())
                }
            }
        }
    }
}

fn new_user_row(num_users: usize, messages: &Messages) -> Markup {
    html! {
        tr id="new-row-user" {
            th { (num_users) }
            td {
                input #email type="email" required
                       placeholder=(messages.email_input_label())
                       class="input input-sm" name="email" autocomplete="off"
                       hx-post="/admin/user"
                       hx-target="#new-row-user"
                       hx-swap="outerHTML"
                       hx-include="#email,#password,#confirm-password"
                       hx-trigger="keydown[key=='Enter']"
                       _="on htmx:afterRequest call document.activeElement.blur()";
            }
            td {
                input #password type="password" required
                       placeholder=(messages.password_input_label())
                       class="input input-sm mb-1" name="password" autocomplete="off"
                       hx-post="/admin/user"
                       hx-target="#new-row-user"
                       hx-swap="outerHTML"
                       hx-include="#email,#password,#confirm-password"
                       hx-trigger="keydown[key=='Enter']"
                       _="on htmx:afterRequest call document.activeElement.blur()";
                input #confirm-password type="password" required
                       placeholder=(messages.confirm_password_input_placeholder())
                       class="input input-sm" name="password-confirm" autocomplete="off"
                       hx-post="/admin/user"
                       hx-target="#new-row-user"
                       hx-swap="outerHTML"
                       hx-include="#email,#password,#confirm-password"
                       hx-trigger="keydown[key=='Enter']"
                       _="on htmx:afterRequest call document.activeElement.blur()";
            }
            td {
                button class="btn btn-ghost btn-square btn-xs"
                  hx-post="/admin/user"
                  hx-target="#new-row-user"
                  hx-swap="outerHTML"
                  hx-include="#email,#password,#confirm-password" {
                    (icons::plus_circle())
                }
            }
        }
    }
}

fn settings_data(_data: &Data, messages: &Messages) -> Markup {
    html! {
       div #settings-data class="hidden p-3" {
            div class="flex justify-between items-center text-sm" {
                div {
                    p .font-semibold {
                        (messages.export_data_form())
                    }
                    p class="text-xs" {
                        (messages.export_data_form_description())
                    }
                }
                button class="btn btn-soft btn-sm"
                    hx-get="/settings/export-data"
                    hx-on::after-request="if (event.detail.successful) document.querySelector('#export-data-dialog').showModal()"
                    hx-target="#export-data-dialog"
                    hx-swap="innerHTML" {
                    (icons::chevron_right())
                }
            }
            /*@if data.Settings.Backups.len() > 0 {
                div class="divider m-0" {}
                div class="flex justify-between items-center text-sm" {
                    div {
                        p .font-semibold {
                            "Restore from backup"
                        }
                        p class="text-xs block max-w-[45ch]" {
                            "This restores your user data from an automatic backup."
                        }
                    }
                    form class="grid gap-1 grid-flow-col w-fit"
                        hx-post="/settings/backups/restore"
                        hx-include="select[name='date']"
                        hx-swap="none"
                        hx-indicator="#fullscreen-loader"
                        hx-confirm="Continue with this backup? Today's data will be backed up if not already done." {
                        label {
                            select required #file-type name="date" class="select select-bordered select-sm" {
                                @for b in data.Settings.Backups {
                                    option value=(b.Value) selected {
                                        (b.Display)
                                    }
                                }
                            }
                        }
                        button class="btn btn-sm btn-outline" {
                            (icons::rocket_launch())
                        }
                    }
                }
            }*/
        }
    }
}

fn export_data_dialog() -> Markup {
    html! {
        dialog #export-data-dialog class="justify-self-center self-center" {}
    }
}

/// Renders the content of the export data dialog.
pub fn render_export_data_dialog_recipes(
    current_url: &str,
    recipes: Vec<Recipe>,
    messages: &Messages,
) -> Markup {
    html! {
        form class="card bg-base-100 shadow-sm min-w-[50vw]"
            hx-post="/settings/export-data"
            hx-indicator="#export-data-spinner"
            hx-on:download-ready="document.querySelector('#export-data-dialog').close(); window.location.href = event.detail.url;" {
            div .card-body {
                h3 class="mb-1 grid grid-flow-col" {
                    label class="input input-sm" {
                        svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" {
                            g stroke-linejoin="round" stroke-linecap="round" stroke-width="2.5" fill="none" stroke="currentColor" {
                                circle cx="11" cy="11" r="8" {}
                                path d="m21 21-4.3-4.3" {}
                             }
                        }
                        input type="search" placeholder=(messages.search_recipe_input_placeholder()) _=(PreEscaped(SEARCH_INPUT_JS));
                    }
                    select required name="type" class="[display:ruby] md:block select select-sm w-fit place-self-end" {
                        option value="json" selected { "JSON" }
                        option value="markdown" { "Markdown" }
                        option value="pdf" { "PDF" }
                        option value="text" { "Text" }
                    }
                }
                div class="overflow-auto h-[50vh]" {
                    table class="table table-zebra table-sm" {
                        thead {
                            tr .text-center {
                                th class="py-1 text-left" {
                                    label {
                                        input type="checkbox" class="checkbox"
                                            _="on change set <input.checkbox-recipe-id/>'s checked to my checked then call checkDataSubmit('checkbox-recipe-id', 'export-data-submit-button')";
                                    }
                                }
                                th .py-1 { (messages.column_name()) }
                                th .py-1 { (messages.export_data_table_favourite()) }
                                th .py-1 { (messages.export_data_table_rating()) }
                                th .py-1 { (messages.export_data_table_page()) }
                                th .py-1 { (messages.column_source()) }
                            }
                        }
                        tbody #search-results {
                            @for recipe in recipes {
                                tr {
                                    td .py-1 {
                                        label {
                                            input type="checkbox" name="recipe-ids" class="checkbox-recipe-id checkbox" value=(recipe.id)
                                                _="on change call checkDataSubmit('checkbox-recipe-id', 'export-data-submit-button')";
                                        }
                                    }
                                    td .py-1 { (recipe.name) }
                                    td class="py-1 text-center select-none" {
                                        @if recipe.is_favourite {
                                            span aria-label=(messages.export_data_table_favourite()) { "♥" }
                                        }
                                    }
                                    td .py-1.text-center {
                                        @if let Some(rating) = recipe.rating {
                                            (rating) "/5"
                                        }
                                    }
                                    td .py-1.text-center {
                                        a class="link" href={ (current_url) "/recipes/" (recipe.id) } target="_blank" {
                                            (messages.action_view())
                                        }

                                    }
                                    td .py-1.text-center {
                                        a class="link" href=(recipe.source) target="_blank" {
                                            (messages.action_visit())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                (cancel_submit_form_actions(&icons::arrow_down_tray(), "export-data-submit-button", true, messages))
            }
        }
    }
}

fn settings_account(messages: &Messages) -> Markup {
    html! {
        div #settings-account class="hidden p-3 md:max-h-96" {
            div class="flex justify-between items-center text-sm" {
                details .w-full {
                    summary class="font-semibold cursor-default select-none" {
                        (messages.change_password_form_title())
                    }
                    form class="flex flex-col text-sm" hx-post="/auth/change-password" hx-indicator="#fullscreen-loader" hx-swap="none" {
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" {
                                (messages.current_password_input_label())
                            }
                            input type="password" placeholder=(messages.current_password_input_placeholder())  class="input input-sm" name="password-current" required;
                        }
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" {
                                (messages.new_password_input_label())
                            }
                            input type="password" placeholder=(messages.new_password_input_placeholder()) class="input input-sm" name="password-new" required;
                        }
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" {
                                (messages.confirm_password_input_label())
                            }
                            input type="password" placeholder=(messages.confirm_password_input_placeholder()) class="input input-sm" name="password-confirm" required;
                        }
                        button class="btn btn-soft btn-sm mt-2" {
                            (messages.action_update())
                        }
                    }
                }
            }
            div class="divider m-0" {}
            div {
                div class="flex justify-between items-center text-sm" {
                    div {
                        p .font-semibold {
                            (messages.delete_account_form_title())
                        }
                        p class="font-normal text-xs" {
                            (messages.delete_account_form_description())
                        }
                    }
                    button type="submit" class="btn btn-soft btn-sm" hx-delete="/auth/user" hx-confirm=(messages.delete_account_form_confirm()) {
                        (messages.action_delete())
                    }
                }
            }
        }
    }
}

fn themes_palette(
    is_set_default: bool,
    default_theme: &Theme,
    selected_theme: &Theme,
    messages: &Messages,
) -> Markup {
    let theme_id = if is_set_default {
        "theme-name-default"
    } else {
        "theme-name"
    };

    let palette_id = if is_set_default {
        "themes-palette-default"
    } else {
        "themes-palette"
    };

    let selected_theme = if selected_theme == &Theme::Default {
        default_theme
    } else {
        selected_theme
    };

    let init = if is_set_default {
        format!(
            "on load put '{default_theme}' into #{theme_id} then call themeChange(document.querySelector('#{palette_id}'))"
        )
    } else {
        format!(
            "on load put '{selected_theme}' into #{theme_id} then call themeChange(document.querySelector('#{palette_id}'))"
        )
    };

    let endpoint = if is_set_default {
        "/settings/theme-default"
    } else {
        "/settings/theme-selected"
    };

    html! {
        div id=(palette_id) class="dropdown dropdown-end hidden z-30 [@supports(color:oklch(0%_0_0))]:block" _=(PreEscaped(init)) {
            div tabindex="0" role="button" class="btn btn-sm btn-outline w-40 justify-between" {
                svg width="20" height="20" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-5 w-5 stroke-current md:hidden" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" {}
                }
                span id=(theme_id) class="hidden font-normal md:inline" {
                    (messages.settings_general_theme())
                }
                svg width="12px" height="12px" class="hidden h-2 w-2 fill-current opacity-60 sm:inline-block" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2048 2048" {
                    path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z" {}
                }
            }
            div tabindex="0" class="dropdown-content bg-base-200 text-base-content rounded-box top-px h-[28.6rem] max-h-[calc(100vh-10rem)] w-56 overflow-y-auto shadow-2xl mt-12" {
                div class="grid grid-cols-1 gap-3 p-3" {
                    @if is_set_default {
                        @for theme in Theme::iter().skip(1) {
                            (render_theme(&theme, endpoint, theme_id))
                        }
                    } @else {
                        @for theme in Theme::iter() {
                            (render_theme(&theme, endpoint, theme_id))
                        }
                    }
                    a class="outline-base-content overflow-hidden rounded-lg text-center" href="/theme-generator/" {
                        p class="px-2 text-xs" {
                            (messages.settings_general_themes_credits())
                        }
                    }
                }
            }
        }
    }
}

fn render_theme(theme_name: &Theme, endpoint: &str, theme_id: &str) -> Markup {
    if endpoint == "/settings/theme-default" {
        html! {
            button class="outline-base-content text-start outline-offset-4"
                    hx-post=(endpoint)
                    hx-vals=(format!(r#"{{"theme": "{theme_name}"}}"#))
                    hx-headers=(r#"{"Content-Type": "application/json"}"#)
                    hx-trigger="click"
                    hx-swap="none"
                    hx-on::after-request="this.closest('.dropdown').querySelector(':focus')?.blur()"
                    _=(PreEscaped(format!("on click put '{theme_name}' into #{theme_id}"))) {
                (render_theme_button_content(theme_name))
            }
        }
    } else {
        html! {
            button class="outline-base-content text-start outline-offset-4"
                    data-act-class="[&_svg]:visible"
                    data-set-theme=(theme_name)
                    hx-post=(endpoint)
                    hx-vals=(format!(r#"{{"theme": "{theme_name}"}}"#))
                    hx-headers=(r#"{"Content-Type": "application/json"}"#)
                    hx-trigger="click"
                    hx-swap="none"
                    _=(PreEscaped(format!("on click put '{theme_name}' into #{theme_id}"))) {
                (render_theme_button_content(theme_name))
            }
        }
    }
}

fn render_theme_button_content(theme_name: &Theme) -> Markup {
    html! {
        span class="bg-base-100 rounded-btn text-base-content block w-full cursor-pointer font-sans" {
            span class="grid grid-cols-5 grid-rows-3" {
                span class="col-span-5 row-span-3 row-start-1 flex items-center gap-2 px-4 py-3" {
                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="invisible h-3 w-3 shrink-0" {
                        path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z" {}
                    }
                    span class="flex-grow text-sm font-medium" { (theme_name) }
                    span class="flex h-full shrink-0 flex-wrap gap-1" data-theme=(theme_name) {
                        span class="bg-primary rounded-badge w-2" {}
                        span class="bg-secondary rounded-badge w-2" {}
                        span class="bg-accent rounded-badge w-2" {}
                        span class="bg-neutral rounded-badge w-2" {}
                    }
                }
            }
        }
    }
}

fn settings_about(data: &Data, messages: &Messages) -> Markup {
    html! {
        div #settings-about class={
            "hidden p-3 md:p-0 md:pr-4"
            @if !data.about.is_check_update { " hidden" }
        } {
            div {
                div class="flex justify-between items-center text-sm" {
                    div {
                        p .font-semibold {
                            (messages.version())
                        }
                        p class="text-sm mt-2" {
                            "v" (env!("CARGO_PKG_VERSION"))
                            @if data.about.is_update_available {
                                " " (messages.update_div_available())
                            } @else {
                                " " (messages.update_div_latest())
                            }
                        }
                        p .text-xs {
                            (messages.update_div_last_checked_at(data.about.last_checked_update_at.to_string()))
                            br;
                            (messages.update_div_last_updated_at(data.about.last_updated_at.to_string()))
                            br;
                            br;
                            @let release_notes = messages.update_div_release_notes();
                            (inject_markup_in_message(
                                &messages.update_div_read_release_notes(release_notes.to_string()),
                                &[(&release_notes, html! {
                                    a class="link" href="https://recipya.musicavis.ca/about/changelog/v1.3.0" target="_blank" {
                                        (release_notes)
                                    }
                                })]))
                        }
                    }
                    div class="flex flex-row self-start" {
                        @if data.about.is_update_available {
                            button class="btn btn-sm" hx-get="/update" hx-swap="none" hx-indicator="#fullscreen-loader" {
                                (messages.action_update())
                            }
                        } @else {
                            img #settings-about-update-check class="htmx-indicator mr-1" src="/public/img/bars.svg" alt=(messages.update_div_checking());
                            button class="btn btn-sm" hx-get="/update/check" hx-target="#settings-about" hx-swap="outerHTML" hx-indicator="#settings-about-update-check" {
                                (messages.update_div_check_for_updates())
                            }
                        }
                    }
                }
            }
            div class="divider m-0" {}
            div class="flex space-x-1" {
                a href="https://app.element.io/#/room/#recipya:matrix.org" target="_blank" {
                    img alt=(messages.action_support()) src="https://img.shields.io/badge/Element-Recipya-blue?logo=element&logoColor=white";
                }
                a href="https://github.com/reaper47/recipya" target="_blank" {
                    img alt="Github repo" src="https://img.shields.io/github/stars/reaper47/recipya-rs?style=social&label=Star on Github";
                }
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                details .w-full {
                    summary class="font-semibold cursor-default select-none" {
                        (messages.keyboard_shortcuts_title())
                    }
                    div class="grid gap-4 p-2" {
                        (&render_shortcuts_table(&messages.keyboard_shortcuts_title(), vec![
                            (vec!["Ctrl", "Alt", "S"], &messages.keyboard_shortcuts_open_settings_dialog()),
                            (vec!["Ctrl", "Alt", "N"], &messages.keyboard_shortcuts_create_recipe_manually()),
                            (vec!["Ctrl", "Alt", "I"], &messages.keyboard_shortcuts_open_import_recipes_dialog()),
                            (vec!["Ctrl", "Alt", "W"], &messages.keyboard_shortcuts_open_fetch_recipes_dialog()),
                            (vec!["Ctrl", "Alt", "R"], &messages.keyboard_shortcuts_open_reports_page())
                        ]))
                        (render_shortcuts_table(&messages.keyboard_shortcuts_manual_recipe_form(), vec![
                            (vec!["Ctrl", "S"], &messages.keyboard_shortcuts_save_recipe()),
                        ]))
                        (render_shortcuts_table(&messages.keyboard_shortcuts_edit_recipe_form(), vec![
                            (vec!["Ctrl", "S"], &messages.keyboard_shortcuts_save_recipe()),
                        ]))
                        (render_shortcuts_table(&messages.keyboard_shortcuts_view_recipe(), vec![
                            (vec!["Ctrl", "Alt", "D"], &messages.keyboard_shortcuts_duplicate_recipe()),
                            (vec!["Ctrl", "E"], &messages.keyboard_shortcuts_edit_recipe()),
                            (vec!["Ctrl", "Shift", "F"], &messages.keyboard_shortcuts_toggle_favourite_recipe()),
                            (vec!["Ctrl", "P"], &messages.keyboard_shortcuts_print_recipe()),
                            (vec!["Ctrl", "X"], &messages.keyboard_shortcuts_share_recipe()),
                            (vec!["Ctrl", "Del"], &messages.keyboard_shortcuts_delete_recipe()),
                        ]))
                        p .text-center {
                            @let ctrl_markup = html! { kbd class="kbd" { "Ctrl" } };
                            @let cmd_markup = html! { kbd class="kbd" { "Cmd" } };
                            (inject_markup_in_message(&messages.keyboard_shortcuts_macos_replacement("Ctrl", "Cmd"), &[
                                ("Ctrl", ctrl_markup),
                                ("Cmd", cmd_markup),
                            ]))
                        }
                    }
                }
            }
        }
    }
}

fn render_shortcuts_table(title: &Message, shortcuts: Vec<(Vec<&str>, &Message)>) -> Markup {
    html! {
        div {
            p .font-medium {
                (title)
            }
            div class="overflow-x-auto" {
                table class="table table-zebra table-sm" {
                    tbody {
                        @for (keys, description) in shortcuts {
                            tr {
                                td class="w-48" {
                                    @for (idx, key) in keys.iter().enumerate() {
                                        kbd class="kbd" { (key) }
                                        @if idx != keys.len() - 1 {
                                            " + "
                                        }
                                    }
                                }
                                td {
                                    (description)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
