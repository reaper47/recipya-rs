use maud::{html, Markup};

pub fn head(title: &str) -> Markup {
    html! {
        head {
            title hx-swap-oob="true" {
                @if title.is_empty() {
                    "Recipya"
                } @else {
                    (title) " | Recipya"
                }
            }
            meta charset="UTF-8";
            meta http-equiv="X-UA-Compatible" content="IE=edge";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            meta name="description" content="A clean, simple and powerful recipe manager your whole family will enjoy.";
            meta name="keywords" content="Cooking, Lifestyle, Recipes, Groceries, Fast";
            meta name="msapplication-TileColor" content="#da532c";
            meta name="theme-color" content="#ffffff";
            link rel="canonical" href="https://recipes.musicavis.com/";
            link rel="stylesheet" href="/static/css/tailwind.css";
            link rel="stylesheet" href="/static/css/app.css";
            link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png";
            link rel="icon" type="image/png" sizes="32x32" href="/static/favicon-32x32.png";
            link rel="icon" type="image/png" sizes="16x16" href="/static/favicon-16x16.png";
            link rel="manifest" href="/static/site.webmanifest";
            link rel="mask-icon" href="/static/safari-pinned-tab.svg" color="#5bbad5";
            script src="https://unpkg.com/htmx.org@2.0.2"{}
            script src="https://unpkg.com/hyperscript.org@0.9.11" {}
            script src="https://unpkg.com/htmx-ext-ws@2.0.1/ws.js" {}
            script defer src="/static/js/core.min.js" {}
            script defer src="/static/js/toast.min.js" {}
        }
    }
}

pub fn toast() -> Markup {
    html! {
        div #toast_container .toast.toast-top.toast-end.hidden.z-20.cursor-default {
            div .hidden.alert-error.alert-info.alert-success.alert-warning {}
            div #toast_alert .alert.shadow-lg.hidden role="alert" {
                svg #toast_icon .stroke-current.shrink-0.w-6.h-6 xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" {}
                div .text-left {
                    h3 #toast_title .font-bold {}
                    div #toast_message .text-xs {}
                }
                button #toast_button .btn.btn-sm {
                    "See"
                }
            }
        }
        script defer src="/static/js/toast.min.js" {}
    }
}

pub fn toast_ws(title: &str, content: &str, is_toast_visible: bool) -> Markup {
    html! {
        (toast())
        div #ws-notification-container class={
            @if is_toast_visible {
                "z-20 fixed bottom-0 right-0 p-6 cursor-default"
            } @else {
                "z-20 fixed bottom-0 right-0 p-6 cursor-default hidden"
            }
        } {
            div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md" {
                p class="font-medium text-center pb-1" {
                    (title)
                }
                (maud::PreEscaped(content))
            }
        }
    }
}
