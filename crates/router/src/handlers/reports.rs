use axum::{
    extract::{OriginalUri, Path, Query, State},
    http::HeaderMap,
    response::IntoResponse,
};
use config::States;
use serde::Deserialize;

use app::state::AppState;
use models::{
    data::{Data, ReportsData},
    reports::ViewReport,
};
use tracing::error;

use crate::{
    Result,
    handlers::{get_settings, helpers::is_hx_request},
    middleware::mw_auth::RequireAuth,
};

#[derive(Deserialize)]
pub struct ReportsParams {
    pub page: Option<i64>,
    pub view: Option<String>,
    pub selected: Option<i64>,
}

/// Handles the reports page.
pub async fn reports_handler(
    header_map: HeaderMap,
    OriginalUri(uri): OriginalUri,
    Query(params): Query<ReportsParams>,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let settings = get_settings(&state, user.id).await?;
    let page = params.page.unwrap_or(1);
    let reports = ViewReport::fetch_all(&state.mm, page, user.id).await?;

    Ok(templates::reports::index(
        uri.path(),
        &Data {
            is_admin: user.is_admin,
            is_authenticated: true,
            states: States {
                autologin: state.config.read().await.states.autologin,
                ..Default::default()
            },
            is_hx_request: is_hx_request(&header_map),
            reports: Some(ReportsData {
                reports: reports.clone(),
                selected: params
                    .view
                    .map(|v| {
                        if &v == "latest" {
                            reports.first().cloned()
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default(),
                page,
            }),
            ..Default::default()
        },
        &settings,
    )
    .into_response())
}

/// Handles fetching and rendering the report.
pub async fn report_handler(
    header_map: HeaderMap,
    OriginalUri(uri): OriginalUri,
    RequireAuth(user): RequireAuth,
    Path(report_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let report = ViewReport::fetch(&state.mm, report_id, user.id)
        .await
        .inspect_err(|err| tracing::error!(?err, report_id, "Failed to fetch report"))?;

    let is_hx_request = is_hx_request(&header_map);

    if is_hx_request {
        Ok(
            templates::reports::render_report(&report.report_type.primary, &report.report_logs)
                .into_response(),
        )
    } else {
        let settings = get_settings(&state, user.id).await?;
        let reports = ViewReport::fetch_all(&state.mm, 1, user.id).await?;

        Ok(templates::reports::index(
            uri.path(),
            &Data {
                is_admin: user.is_admin,
                is_authenticated: true,
                states: States {
                    autologin: state.config.read().await.states.autologin,
                    ..Default::default()
                },
                is_hx_request,
                reports: Some(ReportsData {
                    reports: reports.clone(),
                    selected: Some(report),
                    page: 1,
                }),
                ..Default::default()
            },
            &settings,
        )
        .into_response())
    }
}

/// Refreshes the list of reports.
pub async fn reports_list_handler(
    Query(params): Query<ReportsParams>,
    RequireAuth(user): RequireAuth,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let page = params.page.unwrap_or(1);
    let reports = ViewReport::fetch_all(&state.mm, page, user.id).await?;
    // dbg!(&page, &reports);
    let report_id = params.selected.unwrap_or(1);
    let report = ViewReport::fetch(&state.mm, report_id, user.id)
        .await
        .inspect_err(|err| error!(?err, report_id, "Failed to fetch report"))
        .ok();

    Ok(templates::reports::render_reports_list(&ReportsData {
        reports,
        selected: report,
        page,
    }))
}
