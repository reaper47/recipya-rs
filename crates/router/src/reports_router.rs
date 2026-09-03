use axum::{Router, middleware::from_fn_with_state, routing::get};

use app::state::AppState;

use crate::{
    handlers::reports::{report_handler, reports_handler, reports_list_handler},
    middleware::mw_auth::mw_refresh_token,
};

/// Defines the routes for endpoints related to the reports module.
#[allow(clippy::literal_string_with_formatting_args)]
pub fn reports_routes(state: &AppState) -> Router<AppState> {
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
    use uuid::Uuid;

    use models::reports::{
        report::{Items, ReportForCreate},
        report_log::ReportLogForCreate,
        report_types::{
            API, Import, PrimaryReportType, ReportTypeFull, SecondaryReportType, TertiaryReportType,
        },
    };
    use models::user::User;
    use repository::ModelManager;
    use test_db::default_config;
    use test_fixtures::assert_html;
    use test_utils::{assert_must_be_logged_in, build_server_logged_in};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_reports {
        use super::*;

        const BASE_URI: &str = "/reports";

        #[tokio::test]
        async fn test_must_be_logged_in_ok() -> Result<()> {
            assert_must_be_logged_in(Method::GET, BASE_URI).await
        }

        #[tokio::test]
        async fn test_reports_no_reports_ok() -> Result<()> {
            let (server, _) = build_server_logged_in(default_config()).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<ul id="report-menu" class="menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0 h-full"></ul>"#,
                    r#"<p class="p-4">No reports found.</p>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_reports_reports_exists_ok() -> Result<()> {
            let (server, state) = build_server_logged_in(default_config()).await?;
            let user_id = User::all(&state.mm).await?[0].id;
            let ids = insert_reports(&state.mm, user_id).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            res.assert_text_contains(format!(
                r#"<li class="bg-base-300" hx-get="/reports/{}""#,
                ids[0]
            ));
            res.assert_text_contains(format!(
                "document.getElementById('selected-report-id').value = '{}';",
                ids[0]
            ));
            res.assert_text_contains(format!(r#"<li hx-get="/reports/{}""#, ids[1]));
            assert_html(
                &res,
                &[
                    r#"<table class="table table-sm"><thead><tr><th></th><th>Entity</th><th>Level</th><th>Error code</th><th>Error reason</th><th>Duration</th><th>Actions</th></tr></thead><tbody><tr><td>1</td><td class="max-w-xs truncate">Raspberry Pi</td><td><span class="badge badge-xs w-14 badge-success">success</span></td><td>-</td><td>-</td><td>167ms</td><td></td></tr><tr><td>2</td><td class="max-w-xs truncate">Orange Pi</td><td><span class="badge badge-xs w-14 badge-success">success</span></td><td>-</td><td>-</td><td>544ms</td><td></td></tr><tr><td>3</td><td class="max-w-xs truncate">Pink Pi</td><td><span class="badge badge-xs w-14 badge-success">success</span></td><td>-</td><td>-</td><td>78ms</td><td></td></tr></tbody></table>"#,
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
            let (server, _) = build_server_logged_in(default_config()).await?;

            let res = server.get(&base_uri(1)).await;

            res.assert_status_not_found();
            Ok(())
        }

        #[tokio::test]
        async fn test_reports_exists_ok() -> Result<()> {
            let (server, state) = build_server_logged_in(default_config()).await?;
            let user_id = User::all(&state.mm).await?[0].id;
            let ids = insert_reports(&state.mm, user_id).await?;

            let res = server.get(&base_uri(ids[0])).await;

            res.assert_status_ok();
            res.assert_text_contains(format!(
                r#"<input id="selected-report-id" type="hidden" name="selected" value="{}">"#,
                ids[0]
            ));
            res.assert_text_contains(format!(r#"<li hx-get="/reports/{}""#, ids[1]));
            assert_html(
                &res,
                &[
                    r#"<footer id="pagination-reports" class="footer footer-center bg-base-200 p-2 gap-2 md:pb-2 mt-auto shrink-0 absolute bottom-0" style="grid-auto-flow: row;" onload="updateAddCookbookUrl(1)"><div class="join gap-0"><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Previous page" aria-label="Previous page">‹</button><button class="join-item btn btn-active btn-xs md:btn-sm w-8 md:w-12" aria-current="page" aria-label="Page 1, current page">1</button><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Next page" aria-label="Next page">›</button></div><div class="text-center"><p class="text-xs md:text-sm">Showing <span class="font-semibold text-base-content">1</span>-<span class="font-semibold text-base-content">2</span> of <span id="search-count" class="font-medium">2</span> results</p></div></footer>"#,
                    r#"<table class="table table-sm"><thead><tr><th></th><th>Entity</th><th>Level</th><th>Error code</th><th>Error reason</th><th>Duration</th><th>Actions</th></tr></thead><tbody><tr><td>1</td><td class="max-w-xs truncate">Raspberry Pi</td><td><span class="badge badge-xs w-14 badge-success">success</span></td><td>-</td><td>-</td><td>167ms</td><td></td></tr><tr><td>2</td><td class="max-w-xs truncate">Orange Pi</td><td><span class="badge badge-xs w-14 badge-success">success</span></td><td>-</td><td>-</td><td>544ms</td><td></td></tr><tr><td>3</td><td class="max-w-xs truncate">Pink Pi</td><td><span class="badge badge-xs w-14 badge-success">success</span></td><td>-</td><td>-</td><td>78ms</td><td></td></tr></tbody></table>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_reports_exists_htmx_ok() -> Result<()> {
            let (mut server, state) = build_server_logged_in(default_config()).await?;
            server.add_header(axum_htmx::HX_REQUEST, HeaderValue::from_static("true"));
            let user_id = User::all(&state.mm).await?[0].id;
            let ids = insert_reports(&state.mm, user_id).await?;

            let res = server.get(&base_uri(ids.first().copied().unwrap())).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<div class="hidden md:block overflow-x-auto"><table class="table table-sm"><thead><tr><th></th><th>Entity</th><th>Level</th><th>Error code</th><th>Error reason</th><th>Duration</th><th>Actions</th></tr></thead><tbody><tr><td>1</td><td class="max-w-xs truncate">Raspberry Pi</td><td><span class="badge badge-xs w-14 badge-success">success</span></td><td>-</td><td>-</td><td>167ms</td><td></td></tr><tr><td>2</td><td class="max-w-xs truncate">Orange Pi</td><td><span class="badge badge-xs w-14 badge-success">success</span></td><td>-</td><td>-</td><td>544ms</td><td></td></tr><tr><td>3</td><td class="max-w-xs truncate">Pink Pi</td><td><span class="badge badge-xs w-14 badge-success">success</span></td><td>-</td><td>-</td><td>78ms</td><td></td></tr></tbody></table></div>"#,
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
            let (server, _) = build_server_logged_in(default_config()).await?;

            let res = server.get(BASE_URI).await;

            res.assert_status_ok();
            assert_html(
                &res,
                &[
                    r#"<ul id="report-menu" class="menu block bg-base-100 w-full overflow-y-auto max-h-[40vh] md:max-h-[89vh] pb-16 md:pb-0 h-full"></ul>"#,
                ],
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_has_reports_ok() -> Result<()> {
            let (server, state) = build_server_logged_in(default_config()).await?;
            let user_id = User::all(&state.mm).await?[0].id;
            let ids = insert_reports(&state.mm, user_id).await?;

            let res = server.get(&format!("{BASE_URI}?selected={}", ids[0])).await;

            res.assert_status_ok();
            res.assert_text_contains(format!(r#"hx-get="/reports/{}"#, ids[0]));
            res.assert_text_contains(r#"class="bg-base-300"#);
            res.assert_text_contains(r#"badge-primary">website<"#);
            res.assert_text_contains(r#"badge-primary">import<"#);
            assert_html(
                &res,
                &[
                    r#"<footer id="pagination-reports" class="footer footer-center bg-base-200 p-2 gap-2 md:pb-2 mt-auto shrink-0 absolute bottom-0" style="grid-auto-flow: row;" onload="updateAddCookbookUrl(1)"><div class="join gap-0"><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Previous page" aria-label="Previous page">‹</button><button class="join-item btn btn-active btn-xs md:btn-sm w-8 md:w-12" aria-current="page" aria-label="Page 1, current page">1</button><button class="join-item btn btn-disabled btn-xs md:btn-sm w-8 md:w-12" title="Next page" aria-label="Next page">›</button></div><div class="text-center"><p class="text-xs md:text-sm">Showing <span class="font-semibold text-base-content">1</span>-<span class="font-semibold text-base-content">2</span> of <span id="search-count" class="font-medium">2</span> results</p></div></footer>"#,
                ],
            );
            let body = res.text();
            assert_eq!(body.matches(r#"hx-get="/reports/"#).count(), 2);
            Ok(())
        }
    }

    async fn insert_reports(mm: &ModelManager, user_id: Uuid) -> Result<Vec<i64>> {
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

        let mut ids = Vec::with_capacity(reports.len());
        for report in reports {
            let id = report.insert(mm).await?;
            ids.push(id);
        }

        Ok(ids)
    }
}
