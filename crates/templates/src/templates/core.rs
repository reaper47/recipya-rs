use maud::{Markup, html};

const BUILD_HASH: &str = env!("BUILD_HASH");

/// Renders the <head> section of the HTML.
pub(super) fn head(title: &str) -> Markup {
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
            link rel="canonical" href="https://www.recipya.ca/";

            link rel="icon" href="/data/images/Icon/favicon.ico" sizes="48x48";
            link rel="icon" type="image/png" sizes="32x32" href="/data/images/Icon/favicon-32x32.png";
            link rel="icon" type="image/png" sizes="16x16" href="/data/images/Icon/favicon-16x16.png";
            link rel="apple-touch-icon" sizes="180x180" href="/data/images/Icon/apple-touch-icon.png";
            link rel="mask-icon" href="/data/images/Icon/safari-pinned-tab.svg" color="#5bbad5";

            link rel="manifest" href="/public/site.webmanifest";

            link rel="stylesheet" href=(format!("/public/css/tailwind.css?v={BUILD_HASH}"));
            link rel="stylesheet" href=(format!("/public/css/app.css?v={BUILD_HASH}"));
            link rel="stylesheet" href=(format!("/public/css/vendor/easymde.min.css?v={BUILD_HASH}"));

            script src=(format!("/public/js/vendor/htmx-2.0.9.min.js?v={BUILD_HASH}")) {}
            script src="https://cdn.jsdelivr.net/npm/hyperscript.org@0.9.91/dist/_hyperscript.min.js" integrity="sha384-OT9bNmUa5rM34SmxFpRftn2F6GbgM/4xnTmn0z106OE5uvsigkdtUMOpdPKOigyO" crossorigin="anonymous" {}
            script src=(format!("/public/js/vendor/ws-2.0.4.min.js?v={BUILD_HASH}")) {}
            script src=(format!("/public/js/vendor/theme-change-2.0.2.min.js?v={BUILD_HASH}")) {}
            script src=(format!("/public/js/vendor/html-duration-picker.min.js?v={BUILD_HASH}")) {}
            script src=(format!("/public/js/vendor/cropper-2.1.1.min.js?v={BUILD_HASH}")) {}
            script src=(format!("/public/js/vendor/easymde.min.js?v={BUILD_HASH}")) {}
            script type="module" src=(format!("/public/js/vendor/cally.min.js?v={BUILD_HASH}")) {}
            script defer src=(format!("/public/js/core.min.js?v={BUILD_HASH}")) {}
            script defer src=(format!("/public/js/gestures.min.js?v={BUILD_HASH}")) {}
            script defer src=(format!("/public/js/json-highlighter.min.js?v={BUILD_HASH}")) {}
            script defer src=(format!("/public/js/toast.min.js?v={BUILD_HASH}")) {}
            script defer src=(format!("/public/js/media.min.js?v={BUILD_HASH}")) {}
            script defer src=(format!("/public/js/timer.min.js?v={BUILD_HASH}")) {}
            script defer src=(format!("/public/js/wakelock.min.js?v={BUILD_HASH}")) {}
        }
    }
}

/// Renders a notification toast.
pub(super) fn toast() -> Markup {
    html! {
        div #toast-container .toast.toast-top.toast-end.hidden.z-20.cursor-default {
            div .hidden.alert-error.alert-info.alert-success.alert-warning {}
            div #toast-alert .alert.shadow-lg.hidden role="alert" {
                svg #toast-icon .stroke-current.shrink-0.w-6.h-6 xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" {}
                div .text-left {
                    h3 #toast-title .font-bold {}
                    div #toast-message .text-xs {}
                }
                button #toast-button .btn.btn-sm {
                    "See"
                }
            }
        }
    }
}

/// Renders a notification toast through websockets.
pub(super) fn toast_ws(title: &str, content: &str, is_toast_visible: bool) -> Markup {
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
