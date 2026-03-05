use maud::{Markup, html};

use models::{
    data::{Data, PaginationData, PaginationHtmxData, ReportsData},
    reports::{ViewReportLog, report_types::ReportTypePrimary},
    settings::UserSettingDetails,
};

use crate::templates::{layouts, pagination::pagination};

/// Renders the main reports page.
pub fn index(path: &str, data: &Data, user_setting: &UserSettingDetails) -> Markup {
    let reports_data = &data.reports.clone().unwrap_or_default();
    let content = render_index(reports_data);

    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" { "Reports | Recipya" }
            span #data-layout data-layout="no-aside" hx-swap-oob="true" {}
            (content)
        } @else {
            (layouts::main("Reports", path, data, &content, user_setting, true))
        }
    }
}

fn render_index(data: &ReportsData) -> Markup {
    html! {
        div class="flex flex-col md:flex-row h-full" {
            aside class="relative max-h-full" {
                (render_reports_list(data))
            }
            div class="order-1 divider my-0 md:order-2 md:divider-horizontal md:mx-0" {}
            div class="order-0 flex-1 overflow-y-auto md:order-3" {
                div id="report-view-pane" {
                    @if data.reports.is_empty() {
                        p class="p-4" { "No reports found." }
                    } @else {
                        @let report = data.selected.as_ref().unwrap_or_else(|| data.reports.first().unwrap());
                        (render_report(&report.report_type.primary, &report.report_logs))
                    }
                }
            }
        }
    }
}

fn render_reports_list(data: &ReportsData) -> Markup {
    html! {
        ul #report-menu class={
            "menu block bg-base-100 w-full overflow-y-auto max-h-[89vh]"
            @if data.reports.len() < 10 { " h-full" }
        } {
            @for (idx, report) in data.reports.iter().enumerate() {
                li class=[if idx == 0 { Some("bg-base-300") } else { None }]
                    hx-get=(format!("/reports/{}", report.id))
                    hx-target="#report-view-pane"
                    hx-push-url="true"
                    hx-trigger="mousedown"
                    hx-on:mousedown="document.querySelectorAll('#report-menu li').forEach((el) => el.classList.remove('bg-base-300')); this.classList.add('bg-base-300')" {
                    div class="grid" {
                        div class="grid place-self-start" {
                            p class="font-bold" {
                                (report.created_at.format("%b %e, %Y · %I:%M %p %Z"))
                            }
                            p class="text-sm" {
                                "Execution time: " (report.total_exec_time_ms) "ms"
                            }
                        }
                        div class="grid gap-2" {
                            div class="badge badge-xs badge-primary" {
                                (report.report_type.primary.name)
                            }
                            div class="grid grid-flow-col gap-1 place-items-center" {
                                div class="tooltip tooltip-bottom" data-tip="total" {
                                    span class="badge badge-xs badge-info" { (report.items.total) }
                                }

                                div class="grid grid-flow-col gap-1 p-1 border-1 border-solid rounded-lg" {
                                    div class="tooltip tooltip-bottom" data-tip="success" {
                                        span class="badge badge-xs badge-success" { (report.items.success) }
                                    }
                                    div class="tooltip tooltip-bottom" data-tip="warning" {
                                        span class="badge badge-xs badge-warning" { (report.items.skipped) }
                                    }
                                    div class="tooltip tooltip-bottom" data-tip="error" {
                                        span class="badge badge-xs badge-error" { (report.items.failed) }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        @if !data.reports.is_empty() {
            (pagination(&PaginationData::new(
                "pagination-reports",
                "/reports",
                String::new(),
                data.page.max(1).cast_unsigned(),
                data.reports.len() as u64,
                50,
                PaginationHtmxData {
                    is_swap: false,
                    target: "#content".into(),
                },
                Some("absolute bottom-0"),
            )))
        }
    }
}

/// Renders the logs of a report.
pub fn render_report(primary_report_type: &ReportTypePrimary, logs: &[ViewReportLog]) -> Markup {
    html! {
        div class="overflow-x-auto" {
            table class="table table-sm" {
                thead {
                    tr {
                        th { "" }
                        th { "Entity" }
                        th { "Level" }
                        th { "Error code" }
                        th { "Error reason" }
                        th { "Duration" }
                        th { "Actions" }
                    }
                }
                tbody {
                    @for log in logs {
                        tr {
                            td { (log.seq_num) }
                            td class="max-w-xs truncate" {
                                @if &primary_report_type.name == "website" {
                                    a href=(log.entity_name) .link target="_blank" { (log.entity_name) }
                                } @else {
                                    (log.entity_name)
                                }
                            }
                            td { (log.level.name) }
                            td { (log.error_code.clone().unwrap_or_else(|| "-".into())) }
                            td { (log.error_reason.clone().unwrap_or_else(|| "-".into())) }
                            td { (log.format_duration()) }
                            td {
                                @if &primary_report_type.name == "website" {
                                    @match log.level.name.as_ref() {
                                        "success" if log.recipe_id.is_some() => {
                                            button class="btn btn-xs" hx-get=(format!("/recipes/{}", log.recipe_id.unwrap_or_default())) hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" {
                                                "View"
                                            }
                                        }
                                        "warning" if log.recipe_id.is_some() => {
                                            button class="btn btn-xs" hx-get=(format!("/recipes/{}", log.recipe_id.unwrap_or_default())) hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" {
                                                "View"
                                            }
                                        }
                                        "error" => {
                                            button class="btn btn-xs" hx-post="/recipes/add/website" hx-swap="none" hx-vals=(format!("{{\"urls\": \"{}\"}}", log.entity_name)) {
                                                "Retry"
                                            }
                                        }
                                        _ => {
                                            ""
                                        }
                                    }
                                } @else {
                                    @match log.level.name.as_ref() {
                                        "success" if log.recipe_id.is_some() => {
                                            button class="btn btn-xs" hx-get=(format!("/recipes/{}", log.recipe_id.unwrap_or_default())) hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" {
                                                "View"
                                            }
                                        }
                                        "warning" if log.recipe_id.is_some() => {
                                            button class="btn btn-xs" hx-get=(format!("/recipes/{}", log.recipe_id.unwrap_or_default())) hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" {
                                                "View"
                                            }
                                        }
                                        _ => {
                                            ""
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
}
