use math::cooking::units::MeasurementSystem;
use maud::{Markup, PreEscaped, html};
use models::data::Data;
use models::recipe::Category;
use models::settings::UserSettingDetails;
use strum::IntoEnumIterator;

use crate::templates::icons::{
    icon_arrow_down_tray, icon_arrow_path, icon_circle_stack, icon_cloud, icon_cube_transparent,
    icon_download_cloud, icon_information_circle, icon_rocket_launch, icon_server,
    icon_user_circle,
};

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
            ul class="menu menu-horizontal pt-0 flex-nowrap overflow-x-auto sm:w-56 sm:overflow-x-clip sm:flex-wrap sm:menu sm:menu-vertical"
               _=(PreEscaped("on click remove .active from .setting-tab then add .active to closest <a/> to event.target")) {
                li {
                    a class="setting-tab active" _=(PreEscaped("on click add .hidden to the children of #settings_blocks then remove .hidden from #settings-recipes")) {
                        (icon_cube_transparent())
                        "Recipes"
                    }
                }
                @if data.is_admin {
                    li {
                        a class="setting-tab" _=(PreEscaped("on click add .hidden to the children of #settings_blocks then remove .hidden from #settings-connections")) {
                            (icon_cloud())
                            "Connections"
                        }
                    }
                }
                li {
                    a class="setting-tab" _=(PreEscaped("on click add .hidden to the children of #settings_blocks then remove .hidden from #settings-data")) {
                        (icon_circle_stack())
                        "Data"
                    }
                }
                @if data.is_admin {
                    li {
                        a class="setting-tab" _=(PreEscaped("on click add .hidden to the children of #settings_blocks then remove .hidden from #settings-server")) {
                            (icon_server())
                            "Server"
                        }
                    }
                }
                li {
                    a class="setting-tab" _=(PreEscaped("on click add .hidden to the children of #settings_blocks then remove .hidden from #settings-account")) {
                        (icon_user_circle())
                        "Account"
                    }
                }
                li {
                    a class="setting-tab" _=(PreEscaped("on click add .hidden to the children of #settings_blocks then remove .hidden from #settings-about")) {
                        (icon_information_circle())
                        "About"
                    }
                }
            }
            div #settings-blocks class="w-full md:h-[26rem] md:max-h-[26rem]" style="padding-right: 1rem" {
                (settings_recipes(categories, user_setting))
                @if data.is_admin {
                    (settings_connections(&config))
                    (settings_server(&data, &config))
                }
                (settings_data(&data))
                (settings_account())
                (settings_about(data))
            }
        }
    }
}

fn settings_recipes(categories: Vec<Category>, settings: UserSettingDetails) -> Markup {
    html! {
        div #settings-recipes class="p-3 md:p-0 md:pr-4 md:max-h-96 overflow-y-auto" {
            div class="flex justify-between items-center text-sm" {
                details class="w-full" {
                    summary class="font-semibold cursor-default" {
                        "Categories"
                    }
                    div class="flex flex-wrap gap-2 p-2" {
                        @for category in categories.iter().map(|c| c.name.as_str()) {
                            div class="badge badge-outline p-3 pr-0" {
                                form class="inline-flex" hx-delete="/recipes/categories" hx-target="closest <div/>" hx-swap="delete" {
                                    input type="hidden" name="category" value=(category);
                                    span class="select-none" { (category) }
                                    button type="submit" class="btn btn-xs btn-ghost" { "X" }
                                }
                            }
                        }
                    }
                    div class="badge badge-outline p-3 pr-0" {
                        form class="inline-flex" hx-post="/recipes/categories" hx-target="closest <div/>" hx-swap="outerHTML" {
                            label class="form-control" {
                                input required type="text" placeholder="New category" class="input input-ghost input-xs w-[16ch] focus:outline-none" name="category" autocomplete="off";
                            }
                            button class="btn btn-xs btn-ghost" { (PreEscaped("&#10003;")) }
                        }
                    }
                }
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                label for="settings-recipes-measurement-system" class="font-semibold" {
                    "Measurement system"
                }
                select id="settings-recipes-measurement-system" name="system" class="w-fit select select-bordered select-sm" hx-post="/settings/measurement-system" hx-swap="none" {
                    @for system in MeasurementSystem::iter() {
                        option value=(system)
                               selected[system == settings.measurement_system] {
                            (system)
                        }
                    }
                }
            }
            div class="flex justify-between items-center text-sm mt-2" {
                label for="settings-recipes-convert" {
                    span class="font-semibold" {
                        "Convert automatically"
                    }
                    br;
                    span class="text-sm" {
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
            div class="flex justify-between items-center text-sm mt-2" {
                label for="settings-recipes-calc-nutrition" {
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
                    summary class="font-semibold cursor-default" {
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

fn settings_connections(config: &SettingsForView) -> Markup {
    html! {
        div #settings-connections class="p-3 overflow-y-auto max-h-96 hidden md:p-0 md:pr-4" {
            div class="flex justify-between items-center text-sm" {
                details class="w-full" {
                    summary class="font-semibold cursor-default" {
                        "SMTP Server"
                        br;
                        span class="text-xs font-normal" {
                            "This connection is used to send emails."
                        }
                    }
                    form class="grid w-full" hx-put="/settings/config" hx-swap="none" {
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "From"
                                }
                            }
                            input name="email.from" type="text" placeholder="SMTP email" value=(config.email_admin) autocomplete="off" class="input input-bordered input-sm w-full";
                        }
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "SMTP Host"
                                }
                            }
                            input name="email.host" type="text" placeholder="smtp.gmail.com" value=(config.smtp_host) autocomplete="off" class="input input-bordered input-sm w-full";
                        }
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "SMTP Username"
                                }
                            }
                            input name="email.username" type="text" placeholder="email@example.com" value=(config.smtp_username) autocomplete="off" class="input input-bordered input-sm w-full";
                        }
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "SMTP Password"
                                }
                            }
                            input name="email.password" type="password" placeholder="SMTP password or app password" value=(config.smtp_password) autocomplete="off" class="input input-bordered input-sm w-full";
                        }
                        button class="btn btn-sm mt-2" {
                            "Update"
                        }
                    }
                }
                button type="button" title="Test connection" class="btn btn-xs float-right self-baseline" hx-get="/integrations/test-connection?api=smtp" hx-swap="none" {
                    (icon_arrow_path())
                }
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                details class="w-full" {
                    summary class="font-semibold cursor-default" {
                        "Azure AI Document Intelligence"
                        br;
                        span class="text-xs font-normal" {
                            "This connection is used to digitize recipe images."
                        }
                    }
                    form class="grid w-full" hx-put="/settings/config" hx-swap="none" {
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "Resource key"
                                }
                            }
                            input name="integrations.ocr.key" type="text" placeholder="Resource key 1" value="{ data.Settings.Config.Integrations.AzureDI.Key }" autocomplete="off" class="input input-bordered input-sm w-full";
                        }
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "Endpoint"
                                }
                            }
                            input name="integrations.ocr.url" type="url" placeholder="Vision endpoint URL" value="{ data.Settings.Config.Integrations.AzureDI.Endpoint }" autocomplete="off" class="input input-bordered input-sm w-full";
                        }
                        button class="btn btn-sm mt-2" {
                            "Update"
                        }
                    }
                }
                button type="button" title="Test connection" class="btn btn-xs float-right self-baseline" hx-get="/integrations/test-connection?api=azure-di" hx-swap="none" {
                    (icon_arrow_path())
                }
            }
        }
    }
}

fn settings_server(data: &Data, config: &SettingsForView) -> Markup {
    html! {
        div #settings-server class="hidden p-3 md:p-0 md:pr-4 md:max-h-96" {
            div class="flex justify-between items-center text-sm" {
                form class="grid w-full" hx-put="/settings/config" hx-swap="none" {
                    p class="font-semibold" {
                        "Configuration"
                    }
                    div class="form-control" {
                        label class="label cursor-pointer" {
                            span class="label-text" {
                                "Autologin"
                            }
                            input name="server.autologin" type="checkbox" checked[data.is_autologin] class="checkbox";
                        }
                    }
                    div class="form-control" {
                        label class="label cursor-pointer" {
                            span class="label-text" {
                                "No signups"
                            }
                            input name="server.noSignups" type="checkbox" checked[config.is_no_signups] class="checkbox";
                        }
                    }
                    div class="form-control" {
                        label class="label cursor-pointer" {
                            span class="label-text" {
                                "Is production"
                            }
                            input name="server.production" type="checkbox" checked=(config.is_production) class="checkbox";
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

fn settings_data(data: &Data) -> Markup {
    html! {
       div #settings-data class="hidden p-3 md:p-0 md:pr-4" {
            div class="flex justify-between items-center text-sm" {
                details class="w-full" {
                    summary class="font-semibold cursor-default" {
                        "Import data"
                        br;
                        span class="text-xs font-normal" {
                            "Import from Mealie, Tandoor, Nextcloud, etc."
                        }
                    }
                    form class="flex flex-col text-sm" hx-post="/integrations/import" hx-swap="none" {
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "Solution"
                                }
                            }
                            select name="integration" class="w-fit select select-bordered select-sm" {
                                option value="mealie" selected {
                                    "Mealie"
                                }
                                option value="nextcloud" {
                                    "Nextcloud"
                                }
                                option value="tandoor" {
                                    "Tandoor"
                                }
                            }
                        }
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "Base URL"
                                }
                            }
                            input type="url" name="url" placeholder="https://instance.mydomain.com" class="input input-bordered input-sm w-full" required;
                        }
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "Username"
                                }
                            }
                            input type="text" name="username" placeholder="Enter your username" class="input input-bordered input-sm w-full" required;
                        }
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "Password"
                                }
                            }
                            input type="password" name="password" placeholder="Enter your password" class="input input-bordered input-sm w-full" required;
                        }
                        button class="btn btn-sm mt-2" {
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
                form class="grid gap-1 grid-flow-col w-fit" hx-get="/settings/export/recipes" hx-include="select[name='type']" hx-swap="none" {
                    label class="form-control w-full max-w-xs" {
                        select required id="file-type" name="type" class="w-fit select select-bordered select-sm" {
                            optgroup label="Recipes" {
                                option value="json" selected {
                                    "JSON"
                                }
                                option value="pdf" {
                                    "PDF"
                                }
                            }
                        }
                    }
                    button class="btn btn-outline btn-sm" {
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
                            select required id="file-type" name="date" class="select select-bordered select-sm" {
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

fn settings_account() -> Markup {
    html! {
        div #settings-account class="hidden p-3 md:p-0 md:pr-4 md:max-h-96" {
            div {
                div class="flex justify-between items-center text-sm" {
                    div {
                        p class="font-semibold" {
                            "Theme"
                        }
                        p class="font-normal text-sm" {
                            "Select your preferred theme."
                        }
                    }
                    (themes_palette())
                }
            }
            div class="divider m-0" {}
            div class="flex justify-between items-center text-sm" {
                details class="w-full" {
                    summary class="font-semibold cursor-default" {
                        "Change password "
                    }
                    form class="flex flex-col text-sm" hx-post="/auth/change-password" hx-indicator="#fullscreen-loader" hx-swap="none" {
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "Current password"
                                }
                            }
                            input type="password" placeholder="Enter current password"
                                  class="input input-bordered input-sm w-full"
                                  name="password-current" required;
                        }
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "New password"
                                }
                            }
                            input type="password" placeholder="Enter new password"
                                class="input input-bordered input-sm w-full"
                                name="password-new" required;
                        }
                        label class="form-control w-full" {
                            span class="label" {
                                span class="label-text text-sm" {
                                    "Confirm password"
                                }
                            }
                            input type="password" placeholder="Retype new password"
                                class="input input-bordered input-sm w-full"
                                name="password-confirm" required;
                        }
                        button class="btn btn-sm mt-2" {
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
                        p class="font-normal text-sm" {
                            "This will delete all your data."
                        }
                    }
                    button type="submit" class="btn btn-sm" hx-delete="/auth/user"
                           hx-confirm="Are you sure you want to delete your account? This action is irreversible." {
                        "Delete"
                    }
                }
            }
        }
    }
}

fn themes_palette() -> Markup {
    html! {
        div #themes-palette class="dropdown dropdown-end hidden z-30 [@supports(color:oklch(0%_0_0))]:block"
            _=(PreEscaped("on load call themeChange(document.querySelector('#theme_palette'))")) {
            div tabindex="0" role="button" class="btn btn-ghost" {
                svg width="20" height="20" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="h-5 w-5 stroke-current md:hidden" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" {}
                }
                span id="theme_name" class="hidden font-normal md:inline" _="on load set theme to localStorage.getItem('theme') then  if not theme put 'system' into me else put theme into me" {
                    "Theme"
                }
                svg width="12px" height="12px" class="hidden h-2 w-2 fill-current opacity-60 sm:inline-block" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2048 2048" {
                    path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z" {}
                }
            }
            div tabindex="0" class="dropdown-content bg-base-200 text-base-content rounded-box top-px h-[28.6rem] max-h-[calc(100vh-10rem)] w-56 overflow-y-auto border border-white/5 shadow-2xl outline outline-1 outline-black/5 mt-16" {
                div class="grid grid-cols-1 gap-3 p-3" {
                    button class="outline-base-content text-start outline-offset-4 [&_svg]:visible" data-act-class="[&_svg]:visible" data-set-theme="" _="on click put 'system' into #theme_name" {
                        span class="bg-base-100 rounded-btn text-base-content block w-full cursor-pointer font-sans" data-theme="" {
                            span class="grid grid-cols-5 grid-rows-3" {
                                span class="col-span-5 row-span-3 row-start-1 flex items-center gap-2 px-4 py-3" {
                                    svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="invisible h-3 w-3 shrink-0" {
                                        path d="M20.285 2l-11.285 11.567-5.286-5.011-3.714 3.716 9 8.728 15-15.285z" {}
                                    }
                                    span class="flex-grow text-sm" { "system" }
                                    span class="flex h-full shrink-0 flex-wrap gap-1" {
                                        span class="bg-primary rounded-badge w-2" {}
                                        span class="bg-secondary rounded-badge w-2" {}
                                        span class="bg-accent rounded-badge w-2" {}
                                        span class="bg-neutral rounded-badge w-2" {}
                                    }
                                }
                            }
                        }
                    }
                    @for theme in [
                        "light", "dark", "cupcake", "bumblebee", "emerald", "corporate", "synthwave", "retro", "cyberpunk", "valentine", "halloween", "garden", "forest", "aqua", "lofi", "pastel", "fantasy",
                        "wireframe", "black", "luxury", "dracula", "cmyk", "autumn", "business", "acid", "lemonade", "night", "coffee", "winter", "dim", "nord", "sunset"
                    ] {
                        (render_theme(theme))
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

fn render_theme(theme_name: &str) -> Markup {
    html! {
        button class="outline-base-content text-start outline-offset-4" data-act-class="[&_svg]:visible" data-set-theme=(theme_name) _=(PreEscaped(format!("on click put '{theme_name}' into #theme_name"))) {
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
}

fn settings_about(data: Data) -> Markup {
    html! {
        div #settings-about class={
            "p-3 md:p-0 md:pr-4"
            @if !data.about.is_check_update { "hidden" }
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
                                "update available"
                            } @else {
                                "latest"
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
                            img id="settings_about_update_check" class="htmx-indicator mr-1" src="/static/img/bars.svg" alt="Checking...";
                            button class="btn btn-sm" hx-get="/update/check" hx-target="#settings_about" hx-swap="outerHTML" hx-indicator="#settings_about_update_check" {
                                "Check for updates"
                            }
                        }
                    }
                }
            }
            div class="divider m-0" {}
            div class="flex space-x-1" {
                a href="https://app.element.io/#/room/#recipya:matrix.org" {
                    img alt="Support" src="https://img.shields.io/badge/Element-Recipya-blue?logo=element&logoColor=white";
                }
                a href="https://github.com/reaper47/recipya" target="_blank" {
                    img alt="Github Repo" src="https://img.shields.io/github/stars/reaper47/recipya?style=social&label=Star on Github";
                }
            }
        }
    }
}
