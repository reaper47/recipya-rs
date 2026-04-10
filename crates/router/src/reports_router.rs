use axum::{Router, middleware::from_fn_with_state, routing::get};

use app::state::AppState;

use crate::{
    handlers::reports::{report_handler, reports_handler, reports_list_handler},
    middleware::mw_auth::mw_refresh_token,
};

/// Defines the routes for endpoints related to the reports module.
#[allow(clippy::literal_string_with_formatting_args)]
pub fn reports_router(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(reports_handler))
        .route("/{:report_id}", get(report_handler))
        .route("/list", get(reports_list_handler))
        .layer(from_fn_with_state(state.clone(), mw_refresh_token))
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;
    use reqwest::Method;

    use models::reports::{
        report::{Items, ReportForCreate},
        report_log::ReportLogForCreate,
        report_types::{
            API, Import, PrimaryReportType, ReportTypeFull, SecondaryReportType, TertiaryReportType,
        },
    };
    use models::user::User;
    use repository::ModelManager;
    use test_db::TestDb;
    use test_fixtures::assert_html;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in, create_app_state};
    use uuid::Uuid;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_reports {
        use super::*;

        const BASE_URI: &str = "/reports";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_no_reports_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r#"<ul id="report-menu" class="menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0 h-full"></ul>"#,
                    r#"<p class="p-4">No reports found.</p>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_reports_exists_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            insert_reports(&state.mm, user_id).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r##"<div id="report-index" class="flex flex-col-reverse md:flex-row h-full"><aside class="relative max-h-full" hx-get="/reports/list" hx-trigger="refreshReports from:body" hx-target="#report-list-container" hx-include="#selected-report-id"><input id="selected-report-id" type="hidden" name="selected" value="0"><div id="report-list-container"><ul id="report-menu" class="menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0 h-full"><li class="bg-base-300" hx-get="/reports/2" hx-target="#report-view-pane" hx-push-url="false" hx-trigger="mousedown" hx-on:mousedown="document.querySelectorAll('#report-menu li').forEach((el) =&gt; el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-report-id').value = '2';"><div class="flex justify-between items-center gap-2 w-full"><div class="min-w-0"><p class="font-bold text-sm truncate">"##,
                    r#"<table class="table table-sm"><thead><tr><th></th><th>Entity</th><th>Level</th><th>Error code</th><th>Error reason</th><th>Duration</th><th>Actions</th></tr></thead><tbody><tr><td>1</td><td class="max-w-xs truncate"><a class="link" href="https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/" target="_blank">https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/</a></td><td>success</td><td>-</td><td>-</td><td>243ms</td><td></td></tr><tr><td>2</td><td class="max-w-xs truncate"><a class="link" href="https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741" target="_blank">https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741</a></td><td>warning</td><td>-</td><td>The recipe exists in your collection.</td><td>124ms</td><td></td></tr><tr><td>3</td><td class="max-w-xs truncate"><a class="link" href="https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741" target="_blank">https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741</a></td><td>error</td><td>NetworkFailure</td><td>Failed to connect to the website.</td><td>124ms</td><td><button class="btn btn-xs" hx-post="/recipes/add/website" hx-swap="none" hx-vals="{&quot;urls&quot;: &quot;https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741&quot;}">Retry</button></td></tr></tbody></table>"#,
                    r#"<footer id="pagination-reports" class="footer footer-center bg-base-200 p-2 gap-2 md:pb-2 mt-auto shrink-0 absolute bottom-0" style="grid-auto-flow: row;" onload="updateAddCookbookUrl(1)"><div class="join gap-0"><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Previous page" aria-label="Previous page">‹</button><button class="join-item btn btn-active btn-xs md:btn-sm w-8 md:w-12" aria-current="page" aria-label="Page 1, current page">1</button><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Next page" aria-label="Next page">›</button></div><div class="text-center"><p class="text-xs md:text-sm">Showing <span class="font-semibold text-base-content">1</span>-<span class="font-semibold text-base-content">2</span> of <span id="search-count" class="font-medium">2</span> results</p></div></footer>"#,
                ],
            );
            Ok(())
        }
    }

    mod tests_report {
        use super::*;

        fn base_uri(report_id: i64) -> String {
            format!("/reports/{report_id}")
        }

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, &base_uri(1)).await
        }

        #[tokio::test]
        async fn test_no_reports_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_not_found();
            Ok(())
        }

        #[tokio::test]
        async fn test_reports_exists_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            insert_reports(&state.mm, user_id).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r##"<div id="report-index" class="flex flex-col-reverse md:flex-row h-full"><aside class="relative max-h-full" hx-get="/reports/list" hx-trigger="refreshReports from:body" hx-target="#report-list-container" hx-include="#selected-report-id"><input id="selected-report-id" type="hidden" name="selected" value="1"><div id="report-list-container"><ul id="report-menu" class="menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0 h-full"><li hx-get="/reports/2" hx-target="#report-view-pane" hx-push-url="false" hx-trigger="mousedown" hx-on:mousedown="document.querySelectorAll('#report-menu li').forEach((el) =&gt; el.classList.remove('bg-base-300')); this.classList.add('bg-base-300'); document.getElementById('selected-report-id').value = '2';"><div class="flex justify-between items-center gap-2 w-full"><div class="min-w-0"><p class="font-bold text-sm truncate">"##,
                    r#"<footer id="pagination-reports" class="footer footer-center bg-base-200 p-2 gap-2 md:pb-2 mt-auto shrink-0 absolute bottom-0" style="grid-auto-flow: row;" onload="updateAddCookbookUrl(1)"><div class="join gap-0"><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Previous page" aria-label="Previous page">‹</button><button class="join-item btn btn-active btn-xs md:btn-sm w-8 md:w-12" aria-current="page" aria-label="Page 1, current page">1</button><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Next page" aria-label="Next page">›</button></div><div class="text-center"><p class="text-xs md:text-sm">Showing <span class="font-semibold text-base-content">1</span>-<span class="font-semibold text-base-content">2</span> of <span id="search-count" class="font-medium">2</span> results</p></div></footer>"#,
                    r#"<table class="table table-sm"><thead><tr><th></th><th>Entity</th><th>Level</th><th>Error code</th><th>Error reason</th><th>Duration</th><th>Actions</th></tr></thead><tbody><tr><td>1</td><td class="max-w-xs truncate">Raspberry Pi</td><td>success</td><td>-</td><td>-</td><td>167ms</td><td></td></tr><tr><td>2</td><td class="max-w-xs truncate">Orange Pi</td><td>success</td><td>-</td><td>-</td><td>544ms</td><td></td></tr><tr><td>3</td><td class="max-w-xs truncate">Pink Pi</td><td>success</td><td>-</td><td>-</td><td>78ms</td><td></td></tr></tbody></table>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_reports_exists_htmx_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let mut server = build_server_logged_in(config.clone()).await?;
            server.add_header(axum_htmx::HX_REQUEST, HeaderValue::from_static("true"));
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            insert_reports(&state.mm, user_id).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r#"<div class="hidden md:block overflow-x-auto"><table class="table table-sm"><thead><tr><th></th><th>Entity</th><th>Level</th><th>Error code</th><th>Error reason</th><th>Duration</th><th>Actions</th></tr></thead><tbody><tr><td>1</td><td class="max-w-xs truncate">Raspberry Pi</td><td>success</td><td>-</td><td>-</td><td>167ms</td><td></td></tr><tr><td>2</td><td class="max-w-xs truncate">Orange Pi</td><td>success</td><td>-</td><td>-</td><td>544ms</td><td></td></tr><tr><td>3</td><td class="max-w-xs truncate">Pink Pi</td><td>success</td><td>-</td><td>-</td><td>78ms</td><td></td></tr></tbody></table></div>"#,
                ],
            );
            Ok(())
        }
    }

    mod tests_list {
        use super::*;

        const BASE_URI: &str = "/reports/list";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_no_reports_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                vec![
                    r#"<ul id="report-menu" class="menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0 h-full"></ul>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_has_reports_ok() -> Result<()> {
            let (_test_db, config) = TestDb::new(None).await?;
            let server = build_server_logged_in(config.clone()).await?;
            let state = create_app_state(config).await;
            let user_id = User::all(&state.mm).await?[0].id;
            insert_reports(&state.mm, user_id).await?;

            let res = server.get(&format!("{BASE_URI}?selected=1")).await;

            res.assert_status_ok();
            let body = res.text();
            assert_eq!(body.matches(r#"hx-get="/reports/"#).count(), 2);
            assert!(body.contains(r#"hx-get="/reports/1"#));
            assert!(body.contains(r#"class="bg-base-300"#));
            assert!(body.contains(r#"badge-primary">website<"#));
            assert!(body.contains(r#"badge-primary">import<"#));
            assert_html(
                &res,
                vec![
                    r#"<footer id="pagination-reports" class="footer footer-center bg-base-200 p-2 gap-2 md:pb-2 mt-auto shrink-0 absolute bottom-0" style="grid-auto-flow: row;" onload="updateAddCookbookUrl(1)"><div class="join gap-0"><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Previous page" aria-label="Previous page">‹</button><button class="join-item btn btn-active btn-xs md:btn-sm w-8 md:w-12" aria-current="page" aria-label="Page 1, current page">1</button><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Next page" aria-label="Next page">›</button></div><div class="text-center"><p class="text-xs md:text-sm">Showing <span class="font-semibold text-base-content">1</span>-<span class="font-semibold text-base-content">2</span> of <span id="search-count" class="font-medium">2</span> results</p></div></footer>"#,
                ],
            );
            Ok(())
        }
    }

    async fn insert_reports(mm: &ModelManager, user_id: Uuid) -> Result<()> {
        let reports = [
            ReportForCreate::new(
                ReportTypeFull {
                    primary: PrimaryReportType::<Import>::new(),
                    secondary: Some(SecondaryReportType::<Import, API>::new()),
                    tertiary: Some(TertiaryReportType::<Import, API>::mealie()),
                },
                vec![
                    ReportLogForCreate::success(1, "Raspberry Pi", None, 167),
                    ReportLogForCreate::success(2, "Orange Pi", None, 544),
                    ReportLogForCreate::success(3, "Pink Pi", None, 78),
                ],
                Items {
                    total: 3,
                    success: 3,
                    skipped: 0,
                    failed: 0,
                },
                789,
                user_id,
            ),
            ReportForCreate::new(
                ReportTypeFull::website(),
                vec![
                    ReportLogForCreate::success(
                        1,
                        "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/",
                        None,
                        243,
                    ),
                    ReportLogForCreate::warning(
                        2,
                        "https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741",
                        None,
                        "The recipe exists in your collection.",
                        124,
                    ),
                    ReportLogForCreate::error(
                        3,
                        "https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741",
                        None,
                        "NetworkFailure",
                        "Failed to connect to the website.",
                        124,
                    ),
                ],
                Items {
                    total: 3,
                    success: 1,
                    skipped: 1,
                    failed: 1,
                },
                491,
                user_id,
            ),
        ];
        for report in reports {
            report.insert(mm).await?;
        }
        Ok(())
    }
}
