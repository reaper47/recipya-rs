use l10n::Messages;
use maud::{Markup, PreEscaped, html};
use time::macros::format_description;

use models::{
    data::{Data, PaginationData, PaginationHtmxData, ReportsData},
    reports::{ReportErrors, ViewReportLog, report_types::ReportTypePrimary},
    settings::UserSettingDetails,
};

use crate::templates::{layouts, pagination::pagination};

/// Renders the main reports page.
pub fn index(
    path: &str,
    data: &Data,
    messages: &Messages,
    user_setting: &UserSettingDetails,
) -> Markup {
    let reports_data = &data.reports.clone().unwrap_or_default();
    let content = render_index(reports_data, messages);

    html! {
        @if data.is_hx_request {
            title hx-swap-oob="true" {
                (messages.reports_tab_title())
            }
            (content)
        } @else {
            (layouts::main(&messages.reports(), path, data, &content, messages, user_setting))
        }
    }
}

fn render_index(data: &ReportsData, messages: &Messages) -> Markup {
    html! {
        div #report-index class="flex flex-col-reverse md:flex-row h-full" {
            aside class="relative max-h-full"
                hx-get="/reports/list"
                hx-trigger="refreshReports from:body"
                hx-target="#report-list-container"
                hx-include="#selected-report-id" {
                input #selected-report-id type="hidden" name="selected" value=(data.selected.as_ref().map(|r| r.id).unwrap_or_default());
                div #report-list-container {
                    div class="md:hidden divider m-0" {}
                    (render_reports_list(data, messages))
                }
            }
            div class="hidden order-1 divider my-0 md:block md:order-2 md:divider-horizontal md:mx-0" {}
            div class="order-0 flex-1 overflow-y-auto min-h-0 md:order-3 max-h-[94vh]" {
                div #report-view-pane {
                    @if data.reports.is_empty() {
                        p .p-4 {
                            (messages.reports_no_entries())
                        }
                    } @else {
                        @let report = data.selected.as_ref().unwrap_or_else(|| data.reports.first().unwrap());
                        (render_report(&report.report_type.primary, &report.report_logs, messages))
                    }
                }
            }
        }
        script {
            (PreEscaped(r#"
                document.body.addEventListener("htmx:wsAfterMessage", (event) => {
                    try {
                        const data = JSON.parse(event.detail.message);
                        if (data?.headers?.["HX-Trigger"]) {
                            htmx.trigger(document.body, data.headers["HX-Trigger"]);
                        }
                    } catch (_) {}
                });
            "#))
        }
    }
}

/// Renders the paginated list of reports.
///
/// # Panics
///
/// Panics if the time formatting is invalid.
pub fn render_reports_list(data: &ReportsData, messages: &Messages) -> Markup {
    html! {
        ul #report-menu class={
            "menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0"
            @if data.reports.len() < 10 { " h-full" }
        } {
            @for (idx, report) in data.reports.iter().enumerate() {
                li class=[data.selected.as_ref().map_or(idx == 0, |r| r.id == report.id).then_some("bg-base-300")]
                    hx-get={ "/reports/" (report.id) }
                    hx-target="#report-view-pane"
                    hx-push-url="false"
                    hx-trigger="mousedown"
                    hx-on:mousedown=(format!(
                        "document.querySelectorAll('#report-menu li').forEach((el) => el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-report-id').value = '{}';",                        report.id
                    )) {
                    div class="flex justify-between items-center gap-2 w-full" {
                        div .min-w-0 {
                            p class="font-bold text-sm truncate" {
                                (report.created_at.format(format_description!("[month repr:short] [day padding:space], [year] · [hour repr:12]:[minute] [period case:upper]")).unwrap())
                            }
                            p .text-sm {
                                (messages.reports_execution_time(report.format_duration()))
                            }
                        }
                        div class="flex flex-col items-end gap-1 shrink-0" {
                            div class="badge badge-xs badge-primary" {
                                (report.report_type.primary.name)
                            }
                            div class="flex gap-1 items-center" {
                                div class="tooltip tooltip-bottom" data-tip=(messages.reports_total()) {
                                    span class="badge badge-xs badge-info" {
                                        (report.items.total)
                                    }
                                }

                                div class="flex gap-1 p-1 border border-solid rounded-lg" {
                                    div class="tooltip tooltip-bottom" data-tip=(messages.toast_title_success()) {
                                        span class="badge badge-xs badge-success" {
                                            (report.items.success)
                                        }
                                    }
                                    div class="tooltip tooltip-bottom" data-tip=(messages.toast_title_warning()) {
                                        span class="badge badge-xs badge-warning" {
                                            (report.items.skipped)
                                        }
                                    }
                                    div class="tooltip tooltip-bottom" data-tip=(messages.toast_title_error()) {
                                        span class="badge badge-xs badge-error" {
                                            (report.items.failed)
                                        }
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
            ), messages))
        }
    }
}

/// Renders the logs of a report.
#[allow(clippy::too_many_lines)]
pub fn render_report(
    primary_report_type: &ReportTypePrimary,
    logs: &[ViewReportLog],
    messages: &Messages,
) -> Markup {
    html! {
        // Desktop table
        div class="hidden md:block overflow-x-auto" {
            table class="table table-sm" {
                thead {
                    tr {
                        th { "" }
                        th { (messages.reports_table_entity()) }
                        th { (messages.reports_table_level()) }
                        th { (messages.reports_table_error_code()) }
                        th { (messages.reports_table_error_reason()) }
                        th { (messages.reports_table_duration()) }
                        th {
                            @if logs.has_multiple_errors() {
                                button class="btn btn-xs" hx-post="/recipes/add/website" hx-swap="none" hx-vals=(format!("{{\"urls\": \"{}\"}}", logs.collect_errors())) {
                                    (messages.action_retry_all())
                                }
                            } @else {
                                (messages.action_actions())
                            }
                        }
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
                            td {
                                span class=(format!("badge badge-xs w-14 {}", match log.level.name.as_ref() {
                                    "success" => "badge-success",
                                    "warning" => "badge-warning",
                                    "error"   => "badge-error",
                                    _         => "badge-info",
                                })) { (log.level.name) }
                            }
                            td { (&log.error_code.as_deref().unwrap_or("-")) }
                            td { (&log.error_reason.as_deref().unwrap_or("-")) }
                            td { (log.format_duration()) }
                            td {
                                @match (log.level.name.as_ref(), log.recipe_id) {
                                    ("success" | "warning", Some(id)) => {
                                        button class="btn btn-xs" hx-get={ "/recipes/" (id) } hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" {
                                            (messages.action_view())
                                        }
                                    }
                                    ("error", _) if &primary_report_type.name == "website" => {
                                        button class="btn btn-xs" hx-post="/recipes/add/website" hx-swap="none" hx-vals=(format!("{{\"urls\": \"{}\"}}", log.entity_name)) {
                                            (messages.action_retry())
                                        }
                                    }
                                    _ => { "" }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Mobile card view
        div class="md:hidden flex flex-col gap-2 p-2 h-[50vh] overflow-y-auto" {
            @for (idx, log) in logs.iter().enumerate() {
                div class="bg-base-200 rounded-lg p-3 flex flex-col gap-2 text-sm" {
                    div class="flex justify-between items-center" {
                        span class="font-semibold text-base-content/60" { (idx + 1) }
                        span class=(format!("badge badge-xs {}", match log.level.name.as_ref() {
                            "success" => "badge-success",
                            "warning" => "badge-warning",
                            "error"   => "badge-error",
                            _         => "badge-info",
                        })) { (log.level.name) }
                    }
                    @if &primary_report_type.name == "website" {
                        a class="link text-xs break-all leading-relaxed" href=(log.entity_name) target="_blank" {
                            (log.entity_name)
                        }
                    } @else {
                        span class="text-xs break-all" { (log.entity_name) }
                    }
                    div class="flex justify-between items-center gap-2" {
                        div class="flex flex-col gap-1 text-xs text-base-content/60" {
                            @let reason = log.error_reason.as_deref().unwrap_or("-");
                            span hidden=[if reason == "-" { Some("") } else { None }] {
                                (messages.reports_table_reason(reason))
                            }
                            span {
                                (messages.reports_table_duration_arg(log.format_duration()))
                                @if log.error_code.is_some() {
                                    " · " (messages.reports_table_code_arg(log.error_code.as_deref().unwrap_or("-")))
                                }
                            }
                        }
                        div class="shrink-0" {
                            @match (log.level.name.as_ref(), log.recipe_id) {
                                ("success" | "warning", Some(id)) => {
                                    button class="btn btn-xs" hx-get={ "/recipes/" (id) } hx-target="#content" hx-trigger="mousedown" hx-push-url="true" hx-swap="innerHTML show:window:top transition:true" {
                                        (messages.action_view())
                                    }
                                }
                                ("error", _) if &primary_report_type.name == "website" => {
                                    button class="btn btn-xs" hx-post="/recipes/add/website" hx-swap="none" hx-vals=(format!("{{\"urls\": \"{}\"}}", log.entity_name)) {
                                        (messages.action_retry())
                                    }
                                }
                                _ => { "" }
                            }
                        }
                    }
                }
            }
        }
    }
}
