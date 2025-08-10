use crate::templates::icons::{
    icon_arrow_down_tray, icon_arrow_path, icon_building_library, icon_circle_stack, icon_cloud,
    icon_cube_transparent, icon_download_cloud, icon_information_circle, icon_rocket_launch,
    icon_server, icon_user_circle,
};
use math::cooking::units::MeasurementSystem;
use maud::{Markup, PreEscaped, html};
use models::data::Data;
use models::recipe::Category;
use models::settings::{Theme, UserSettingDetails};
use strum::IntoEnumIterator;

pub struct SettingsForView {
    pub is_autologin: bool,
    pub is_no_signups: bool,
    pub is_production: bool,

    pub email_admin: String,
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub azure_di_key: String,
    pub azure_di_endpoint: String,
}

/// Renders a new recipe category form.
pub fn new_recipe_category(category: &str) -> Markup {
    html! {
        div class="badge badge-outline p-3 pr-0" {
            form .inline-flex hx-delete="/recipes/categories" hx-target=(PreEscaped("closest <div/>")) hx-swap="delete" {
                input type="hidden" name="category" value=(category);
                span .select-none { (category) }
                button type="submit" .btn.btn-xs.btn-ghost { "X" }
            }
        }
        (empty_recipe_category())
    }
}

fn empty_recipe_category() -> Markup {
    html! {
        div class="badge badge-outline p-3 pr-0" {
            form .inline-flex hx-post="/recipes/categories" hx-target=(PreEscaped("closest <div/>")) hx-swap="outerHTML" {
                label class="input" {
                    input required type="text" placeholder="New category" class="input input-ghost input-xs w-[16ch] focus:outline-none" name="category" autocomplete="off";
                }
                button .btn.btn-xs.btn-ghost { (PreEscaped("&#10003;")) }
            }
        }
    }
}

/// Renders the settings dialog.
pub fn settings(
    data: Data,
    user_setting: UserSettingDetails,
    categories: Vec<Category>,
    config: &SettingsForView,
) -> Markup {
    html! {
        div class="flex flex-col menu-sm sm:flex-row sm:menu-md" {
            ul class="menu menu-horizontal flex-nowrap overflow-x-auto w-full sm:overflow-x-clip sm:w-48 sm:menu-vertical"
               _=(PreEscaped("on click remove .menu-active from .setting-tab then add .menu-active to closest <a/> to event.target")) {
                li {
                    a class="setting-tab menu-active" _=(PreEscaped("on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-recipes")) {
                        (icon_cube_transparent())
                        "Recipes"
                    }
                }
                @if data.is_admin {
                    li {
                        a class="setting-tab" _=(PreEscaped("on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-connections")) {
                            (icon_cloud())
                            "Connections"
                        }
                    }
                }
                li {
                    a class="setting-tab" _=(PreEscaped("on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-data")) {
                        (icon_circle_stack())
                        "Data"
                    }
                }
                @if data.is_admin {
                    li {
                        a class="setting-tab" _=(PreEscaped("on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-server")) {
                            (icon_server())
                            "Server"
                        }
                    }
                    li {
                        a class="setting-tab" _=(PreEscaped("on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-admin")) {
                            (icon_building_library())
                            "Admin"
                        }
                    }
                }
                li {
                    a class="setting-tab" _=(PreEscaped("on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-account")) {
                        (icon_user_circle())
                        "Account"
                    }
                }
                li {
                    a class="setting-tab" _=(PreEscaped("on click add .hidden to the children of #settings-blocks then remove .hidden from #settings-about")) {
                        (icon_information_circle())
                        "About"
                    }
                }
            }
            div #settings-blocks class="w-full md:h-[50vh] md:max-h-[50vh]" style="padding-right: 1rem" {
                (settings_recipes(categories, &user_setting))
                @if data.is_admin {
                    (settings_connections(&config))
                    (settings_server(&data, &config))
                    (settings_admin(&user_setting))
                }
                (settings_data(&data))
                (settings_account(&user_setting))
                (settings_about(data))
            }
        }
    }
}

fn settings_recipes(categories: Vec<Category>, settings: &UserSettingDetails) -> Markup {
    html! {
        div #settings-recipes class="p-3 md:max-h-96 overflow-y-auto" {
            div class="flex justify-between items-center text-sm" {
                details class="w-full" {
                    summary class="font-semibold cursor-default select-none" {
                        "Categories"
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
                        form class="inline-flex" hx-post="/recipes/categories" hx-target="closest <div/>" hx-swap="outerHTML" {
                            label class="form-control" {
                                input required type="text" placeholder="New category" class="input input-ghost input-xs w-[16ch] focus:outline-none" name="category" autocomplete="off";
                            }
                            button class="btn btn-xs btn-circle btn-ghost" {
                                (PreEscaped("&#10003;"))
                            }
                        }
                    }
                }
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                label for="settings-recipes-measurement-system" class="font-semibold" {
                    "Measurement system"
                }
                select #settings-recipes-measurement-system name="system" class="w-fit select select-bordered select-sm" hx-post="/settings/measurement-system" hx-swap="none" {
                    @for system in MeasurementSystem::iter() {
                        option value=(system)
                               selected[system == settings.measurement_system] {
                            (system)
                        }
                    }
                }
            }
            label class="flex justify-between items-center text-sm mt-2" for="settings-recipes-convert" {
                div {
                    span class="font-semibold" {
                        "Convert automatically"
                    }
                    br;
                    span class="text-xs" {
                        "Convert new recipes to your preferred measurement system."
                    }
                }
                input type="checkbox" name="convert" #settings-recipes-convert
                  checked[settings.is_convert_automatically]
                  class="checkbox"
                  hx-post="/settings/convert-automatically"
                  hx-trigger="click";
            }
            div class="divider m-0" {}
            label class="flex justify-between items-center text-sm mt-2" for="settings-recipes-calc-nutrition" {
                div {
                    span class="font-semibold" {
                        "Calculate nutrition facts"
                    }
                    br;
                    span class="text-xs block max-w-[45ch]" {
                        "Calculate the nutrition facts automatically when adding a recipe. The processing will be done in the background."
                    }
                }
                input #settings-recipes-calc-nutrition type="checkbox" name="calculate-nutrition"
                      checked[settings.is_calculate_nutrition]
                      class="checkbox"
                      hx-post="/settings/calculate-nutrition"
                      hx-trigger="click";
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                details class="w-full" {
                    summary class="font-semibold cursor-default select-none" {
                        "Placeholders"
                    }
                    div class="flex flex-wrap gap-2 p-2 flex-row" {
                        div class="max-w-60" {
                            p class="text-center mb-1 font-medium underline" {
                                "Recipes"
                            }
                            form hx-post="/placeholder" hx-encoding="multipart/form-data" hx-swap="none"
                                  _=(PreEscaped("on htmx:afterRequest call reloadImg('/data/images/Placeholders/placeholder.recipe.webp')")) {
                                img src="/data/images/Placeholders/placeholder.recipe.webp" alt="Recipe placeholder" class="w-60 h-60";
                                input type="hidden" name="name" value="recipe";
                                input type="file" name="images" class="file-input file-input-bordered file-input-sm max-w-60 mt-1";
                                button class="btn btn-neutral btn-sm btn-block my-1" {
                                    "Update"
                                }
                            }
                            button class="btn btn-error btn-sm btn-block"
                                   hx-post="/placeholder/restore"
                                   hx-vals=r#"js:{t: "recipe"}"#
                                   hx-swap="none"
                                   _=(PreEscaped("on htmx:afterRequest call reloadImg('/data/images/Placeholders/placeholder.recipe.webp')")) {
                                "Restore original"
                            }
                        }
                        div class="max-w-60" {
                            p class="text-center mb-1 font-medium underline" {
                                "Cookbook"
                            }
                            form hx-post="/placeholder" hx-encoding="multipart/form-data" hx-swap="none"
                                 _=(PreEscaped("on htmx:afterRequest call reloadImg('/data/images/Placeholders/placeholder.cookbook.webp')")) {
                                img src="/data/images/Placeholders/placeholder.cookbook.webp" alt="Cookbook placeholder" class="w-60 h-60";
                                input type="hidden" name="name" value="cookbook";
                                input type="file" name="images" class="file-input file-input-bordered file-input-sm max-w-60 mt-1";
                                button class="btn btn-neutral btn-sm btn-block my-1" {
                                    "Update"
                                }
                            }
                            button class="btn btn-error btn-sm btn-block"
                                   hx-post="/placeholder/restore"
                                   hx-vals=r#"js:{name: "cookbook"}"#
                                   hx-swap="none"
                                   _=(PreEscaped("on htmx:afterRequest call reloadImg('/data/images/Placeholders/placeholder.cookbook.webp')")) {
                                "Restore original"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn settings_connections(config: &SettingsForView) -> Markup {
    html! {
        div #settings-connections class="p-3 overflow-y-auto max-h-96 hidden" {
            div class="flex justify-between items-center text-sm" {
                details class="w-full" {
                    summary class="font-semibold cursor-default select-none" {
                        "SMTP Server"
                        br;
                        span class="text-xs font-normal" {
                            "This connection is used to send emails."
                        }
                    }
                    form class="grid w-full " hx-put="/settings/config" hx-swap="none" {
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "From" }
                            input name="email.from" type="email" placeholder="SMTP email" value=(config.email_admin) autocomplete="off" class="input input-sm";
                        }
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "Host" }
                            input name="email.host" type="text" placeholder="smtp.gmail.com" value=(config.smtp_host) autocomplete="off" class="input input-bordered input-sm";
                        }
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "Username" }
                            input name="email.username" type="email" placeholder="email@example.com" value=(config.smtp_username) autocomplete="off" class="input input-bordered input-sm";
                        }
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "Password" }
                            input name="email.password" type="password" placeholder="SMTP password or app password" value=(config.smtp_password) autocomplete="off" class="input input-bordered input-sm";
                        }
                        button class="btn btn-soft btn-sm mt-2" {
                            "Update"
                        }
                    }
                }
                button type="button" title="Test connection" class="btn btn-xs float-right self-baseline hover:text-secondary" hx-get="/integrations/test-connection?api=smtp" hx-swap="none" {
                    (icon_arrow_path())
                }
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                details class="w-full" {
                    summary class="font-semibold cursor-default select-none" {
                        "Azure AI Document Intelligence"
                        br;
                        span class="text-xs font-normal" {
                            "This connection is used to digitize recipe images."
                        }
                    }
                    form class="grid w-full" hx-put="/settings/config" hx-swap="none" {
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "Resource key" }
                            input name="integrations.ocr.key" type="text" placeholder="Resource key 1" value="{ data.Settings.Config.Integrations.AzureDI.Key }" autocomplete="off" class="input input-bordered input-sm";
                        }
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "Endpoint" }
                            input name="integrations.ocr.url" type="url" placeholder="Vision endpoint URL" value="{ data.Settings.Config.Integrations.AzureDI.Endpoint }" autocomplete="off" class="input input-bordered input-sm";
                        }
                        button class="btn btn-soft btn-sm mt-2" {
                            "Update"
                        }
                    }
                }
                button type="button" title="Test connection" class="btn btn-xs float-right self-baseline hover:text-secondary" hx-get="/integrations/test-connection?api=azure-di" hx-swap="none" {
                    (icon_arrow_path())
                }
            }
        }
    }
}

fn settings_server(data: &Data, config: &SettingsForView) -> Markup {
    html! {
        div #settings-server class="hidden p-3 md:max-h-96" {
            div class="flex justify-between items-center text-sm" {
                form class="grid w-full" hx-put="/settings/config" hx-swap="none" {
                    p class="font-semibold pb-2" {
                        "Configuration"
                    }
                    fieldset class="fieldset" {
                        label class="label text-base-content" {
                            input name="server.autologin" type="checkbox" checked[data.is_autologin] class="checkbox";
                            "Autologin"
                        }
                    }
                    fieldset class="fieldset" {
                        label class="label text-base-content" {
                            input name="server.noSignups" type="checkbox" checked[config.is_no_signups] class="checkbox";
                            "No signups"
                        }
                    }
                    fieldset class="fieldset" {
                        label class="label text-base-content" {
                            input name="server.production" type="checkbox" checked=(config.is_production) class="checkbox";
                            "Is production"
                        }
                    }
                    button class="btn btn-sm mt-2" {
                        "Update"
                    }
                }
            }
        }
    }
}

fn settings_admin(user_settings: &UserSettingDetails) -> Markup {
    html! {
        div #settings-admin class="hidden p-3 md:max-h-96"  {
            div class="flex justify-between items-center text-sm" {
                div {
                    p class="font-semibold" {
                        "Default theme"
                    }
                    p class="font-normal text-xs" {
                        "This sets the default theme for all users."
                    }
                }
                (themes_palette(true, &user_settings.default_theme, &user_settings.selected_theme))
            }
        }
    }
}

fn settings_data(data: &Data) -> Markup {
    html! {
       div #settings-data class="hidden p-3" {
            div class="flex justify-between items-center text-sm" {
                details class="w-full" {
                    summary class="font-semibold cursor-default select-none" {
                        "Import data"
                        br;
                        span class="text-xs font-normal" {
                            "Import from Mealie, Tandoor, Nextcloud, etc."
                        }
                    }
                    form class="flex flex-col text-sm" hx-post="/integrations/import" hx-swap="none" {
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "Solution" }
                            select name="integration" class="w-fit select select-sm" {
                                option value="mealie" selected { "Mealie" }
                                option value="nextcloud" { "Nextcloud" }
                                option value="tandoor" { "Tandoor" }
                            }
                        }
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "Base URL" }
                            input type="url" name="url" placeholder="https://instance.mydomain.com" class="input input-bordered input-sm" required;
                        }
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "Username" }
                            input type="text" name="username" placeholder="Enter your username" class="input input-bordered input-sm" required;
                        }
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "Password" }
                            input type="password" name="password" placeholder="Enter your password" class="input input-bordered input-sm" required;
                        }
                        button class="btn btn-soft btn-sm mt-2" {
                            (icon_download_cloud())
                            "Import"
                        }
                    }
                }
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                div {
                    p class="font-semibold" {
                        "Export data"
                    }
                    p class="text-xs" {
                        "Download your data in the selected file format."
                    }
                }
                form class="grid gap-1 grid-flow-col" hx-get="/settings/export/recipes" hx-include="select[name='type']" hx-swap="none" {
                    fieldset class="fieldset" {
                        select required #file-type name="type" class="select select-sm" {
                            optgroup label="Recipes" {
                                option value="json" selected { "JSON" }
                                option value="pdf" { "PDF" }
                            }
                        }
                    }
                    button class="btn btn-soft btn-sm mt-1" {
                        (icon_arrow_down_tray())
                    }
                }
            }
            /*@if data.Settings.Backups.len() > 0 {
                div class="divider m-0" {}
                div class="flex justify-between items-center text-sm" {
                    div {
                        p class="font-semibold" {
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
                            (icon_rocket_launch())
                        }
                    }
                }
            }*/
        }
    }
}

fn settings_account(user_settings: &UserSettingDetails) -> Markup {
    html! {
        div #settings-account class="hidden p-3 md:max-h-96" {
            div {
                div class="flex justify-between items-center text-sm" {
                    div {
                        p class="font-semibold" {
                            "Theme"
                        }
                        p class="font-normal text-xs" {
                            "Select your preferred theme."
                        }
                    }
                    (themes_palette(false, &user_settings.default_theme, &user_settings.selected_theme))
                }
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                details class="w-full" {
                    summary class="font-semibold cursor-default select-none" {
                        "Change password "
                    }
                    form class="flex flex-col text-sm" hx-post="/auth/change-password" hx-indicator="#fullscreen-loader" hx-swap="none" {
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "Current password" }
                            input type="password" placeholder="Enter current password" class="input input-sm" name="password-current" required;
                        }
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "New password" }
                            input type="password" placeholder="Enter new password" class="input input-sm" name="password-new" required;
                        }
                        fieldset class="fieldset" {
                            legend class="fieldset-legend" { "Confirm password" }
                            input type="password" placeholder="Retype new password" class="input input-sm" name="password-confirm" required;
                        }
                        button class="btn btn-soft btn-sm mt-2" {
                            "Update password"
                        }
                    }
                }
            }
            div class="divider m-0" {}
            div {
                div class="flex justify-between items-center text-sm" {
                    div {
                        p class="font-semibold" {
                            "Delete Account"
                        }
                        p class="font-normal text-xs" {
                            "This will delete all your data."
                        }
                    }
                    button type="submit" class="btn btn-soft btn-sm" hx-delete="/auth/user" hx-confirm="Are you sure you want to delete your account? This action is irreversible." {
                        "Delete"
                    }
                }
            }
        }
    }
}

fn themes_palette(is_set_default: bool, default_theme: &Theme, selected_theme: &Theme) -> Markup {
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
            div tabindex="0" role="button" class="btn btn-outline w-40" {
                svg width="20" height="20" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-5 w-5 stroke-current md:hidden" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" {}
                }
                span id=(theme_id) class="hidden font-normal md:inline" {
                    "Theme"
                }
                svg width="12px" height="12px" class="hidden h-2 w-2 fill-current opacity-60 sm:inline-block" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2048 2048" {
                    path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z" {}
                }
            }
            div tabindex="0" class="dropdown-content bg-base-200 text-base-content rounded-box top-px h-[28.6rem] max-h-[calc(100vh-10rem)] w-56 overflow-y-auto border border-white/5 shadow-2xl outline outline-1 outline-black/5 mt-16" {
                div class="grid grid-cols-1 gap-3 p-3" {
                    @if is_set_default {
                        @for theme in Theme::iter().skip(1) {
                            (render_theme(theme, endpoint, theme_id))
                        }
                    } @else {
                        @for theme in Theme::iter() {
                            (render_theme(theme, endpoint, theme_id))
                        }
                    }
                    a class="outline-base-content overflow-hidden rounded-lg text-center" href="/theme-generator/" {
                        p class="px-2 text-xs" {
                            "Credits to DaisyUI for this list"
                        }
                    }
                }
            }
        }
    }
}

fn render_theme(theme_name: Theme, endpoint: &str, theme_id: &str) -> Markup {
    if endpoint == "/settings/theme-default" {
        html! {
            button class="outline-base-content text-start outline-offset-4"
                    hx-post=(endpoint)
                    hx-vals=(format!(r#"{{"theme": "{theme_name}"}}"#))
                    hx-headers=(r#"{"Content-Type": "application/json"}"#)
                    hx-trigger="click"
                    hx-swap="none"
                    _=(PreEscaped(format!("on click put '{theme_name}' into #{theme_id}"))) {
                (render_theme_button_content(&theme_name))
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
                (render_theme_button_content(&theme_name))
            }
        }
    }
}

fn render_theme_button_content(theme_name: &Theme) -> Markup {
    html! {
        span class="bg-base-100 rounded-btn text-base-content block w-full cursor-pointer font-sans" data-theme=(theme_name) {
            span class="grid grid-cols-5 grid-rows-3" {
                span class="col-span-5 row-span-3 row-start-1 flex items-center gap-2 px-4 py-3" {
                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="invisible h-3 w-3 shrink-0" {
                        path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z" {}
                    }
                    span class="flex-grow text-sm" { (theme_name) }
                    span class="flex h-full shrink-0 flex-wrap gap-1" {
                        span class="bg-primary rounded-badge w-2" {}
                        span class="bg-secondary rounded-badge w-2" {}
                        span class="bg-accent rounded-badge w-2" {}
                        span class="bg-neutral rounded-badge w-2"{}
                    }
                }
            }
        }
    }
}

fn settings_about(data: Data) -> Markup {
    html! {
        div #settings-about class={
            "hidden p-3 md:p-0 md:pr-4"
            @if !data.about.is_check_update { " hidden" }
        } {
            div {
                div class="flex justify-between items-center text-sm" {
                    div {
                        p class="font-semibold" {
                            "Recipya Version"
                        }
                        p class="text-sm mt-2" {
                            "v" (data.about.version)
                            @if data.about.is_update_available {
                                " (update available)"
                            } @else {
                                " (latest)"
                            }
                        }
                        p class="text-xs" {
                            "Last checked: " (data.about.last_checked_update_at)
                            br;
                            "Last updated: " (data.about.last_updated_at)
                            br;
                            br;
                            "Read the " a class="link" href="https://recipya.musicavis.ca/about/changelog/v1.3.0" target="_blank" { "release notes" }
                        }
                    }
                    div class="flex flex-row self-start" {
                        @if data.about.is_update_available {
                            button class="btn btn-sm" hx-get="/update" hx-swap="none" hx-indicator="#fullscreen-loader" {
                                "Update"
                            }
                        } @else {
                            img #settings-about-update-check class="htmx-indicator mr-1" src="/public/img/bars.svg" alt="Checking...";
                            button class="btn btn-sm" hx-get="/update/check" hx-target="#settings-about" hx-swap="outerHTML" hx-indicator="#settings-about-update-check" {
                                "Check for updates"
                            }
                        }
                    }
                }
            }
            div class="divider m-0" {}
            div class="flex space-x-1" {
                a href="https://app.element.io/#/room/#recipya:matrix.org" target="_blank" {
                    img alt="Support" src="https://img.shields.io/badge/Element-Recipya-blue?logo=element&logoColor=white";
                }
                a href="https://github.com/reaper47/recipya" target="_blank" {
                    img alt="Github Repo" src="https://img.shields.io/github/stars/reaper47/recipya-rs?style=social&label=Star on Github";
                }
            }
        }
    }
}
