use maud::{Markup, html};
use models::{data::Data, reports::ViewReport, settings::UserSettingDetails};

use crate::templates::layouts;

/// Renders the main reports page.
pub fn index(path: &str, data: &Data, user_setting: &UserSettingDetails) -> Markup {
    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" { "Reports | Recipya" }
            span #data-layout data-layout="no-aside" hx-swap-oob="true" {}
            (render_index(data))
        } @else {
            (layouts::main("Reports", path, data, &render_index(data), user_setting, true))
        }
    }
}

fn render_index(data: &Data) -> Markup {
    html! {
        div class="flex flex-col md:flex-row h-full" {
            div class="order-2 w-full md:w-40 overflow-y-auto max-h-[33vh] md:order-1 md:flex-shrink-0 md:h-full" {
                (render_reports_list(&data.reports))
            }
            div class="order-1 divider my-0 md:order-2 md:divider-horizontal md:mx-0" {}
            div class="order-0 flex-1 overflow-y-auto md:order-3" {
                div id="report-content"{
                    @if data.reports.is_empty() {
                        p class="p-4" { "No reports found." }
                    } @else {
                        (render_report(&data.reports[0]))
                    }
                }
            }
        }
    }
}

fn render_reports_list(reports: &Vec<ViewReport>) -> Markup {
    html! {
        ul class="menu bg-base-100 w-full" {
            @for (idx, report) in reports.iter().enumerate() {
                li {
                    a {
                        (format!("Report {idx}"))
                    }
                }
            }
        }
    }
}

fn render_report(report: &ViewReport) -> Markup {
    html! {}
}
