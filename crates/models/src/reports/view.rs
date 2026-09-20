use std::collections::HashMap;

use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use itertools::Itertools;
use repository::extensions::pagination::Paginate;
use time::OffsetDateTime;
use time_tz::OffsetDateTimeExt;
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::reports::report::{Items, ReportWithTypes};
use crate::reports::report_log::{Level, ReportLog, ReportLogWithLevel};
use crate::reports::report_types::{ReportTypePrimary, ReportTypeSecondary, ReportTypeTertiary};
use crate::settings::UserSettingDetails;
use crate::{Error, Result};

pub const DEFAULT_REPORTS_PER_PAGE: i64 = 50;

/// Represents a report to be presented to the user.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewReport {
    pub id: i64,
    pub report_type: ViewReportType,
    pub report_logs: Vec<ViewReportLog>,
    pub items: Items,
    pub user_id: Uuid,
    pub total_exec_time_ms: i64,
    pub created_at: OffsetDateTime,
}

impl Default for ViewReport {
    fn default() -> Self {
        Self {
            id: 0,
            report_type: ViewReportType::default(),
            report_logs: Vec::new(),
            items: Items::default(),
            user_id: Uuid::nil(),
            total_exec_time_ms: 0,
            created_at: OffsetDateTime::UNIX_EPOCH,
        }
    }
}

/// Represents a log entry for a report to be presented to the user.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewReportLog {
    pub id: i64,
    pub seq_num: i32,
    pub entity_name: String,
    pub recipe_id: Option<i64>,
    pub level: Level,
    pub error_code: Option<String>,
    pub error_reason: Option<String>,
    pub exec_time_ms: i64,
}

/// Represents a report type to be presented to the user.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ViewReportType {
    pub primary: ReportTypePrimary,
    pub secondary: Option<ReportTypeSecondary>,
    pub tertiary: Option<ReportTypeTertiary>,
}

impl ViewReport {
    /// Fetches all reports for a user.
    pub async fn fetch_all(mm: &ModelManager, page: i64, user_id: Uuid) -> Result<Vec<Self>> {
        let tz = UserSettingDetails::get(mm, user_id).await?.timezone;

        let mut conn = mm.pool.get().await?;

        let reports = schema::reports::table
            .filter(schema::reports::user_id.eq(user_id))
            .inner_join(schema::report_types_primary::table)
            .left_join(schema::report_types_secondary::table)
            .left_join(schema::report_types_tertiary::table)
            .select(ReportWithTypes::as_select())
            .order(schema::reports::created_at.desc())
            .paginate(page.max(1), DEFAULT_REPORTS_PER_PAGE)
            .load::<ReportWithTypes>(&mut conn)
            .await?;

        let inner_reports = reports.iter().map(|r| &r.report).collect::<Vec<_>>();

        let logs = ReportLog::belonging_to(&inner_reports)
            .inner_join(schema::levels::table)
            .select(ReportLogWithLevel::as_select())
            .order(schema::reports_logs::seq_num.asc())
            .load::<ReportLogWithLevel>(&mut conn)
            .await?;

        drop(conn);

        let mut logs_map: HashMap<i64, Vec<ReportLogWithLevel>> = HashMap::new();
        for log in logs {
            logs_map
                .entry(log.report_log.report_id)
                .or_default()
                .push(log);
        }

        let grouped = reports
            .into_iter()
            .map(|r| {
                let logs = logs_map.remove(&r.report.id).unwrap_or_default();
                Self {
                    id: r.report.id,
                    report_type: ViewReportType {
                        primary: r.report_type_primary,
                        secondary: r.report_type_secondary,
                        tertiary: r.report_type_tertiary,
                    },
                    report_logs: logs
                        .into_iter()
                        .map(|l| ViewReportLog {
                            id: l.report_log.id,
                            seq_num: l.report_log.seq_num,
                            entity_name: l.report_log.entity_name,
                            recipe_id: l.report_log.recipe_id,
                            level: l.level,
                            error_code: l.report_log.error_code,
                            error_reason: l.report_log.error_reason,
                            exec_time_ms: l.report_log.exec_time_ms,
                        })
                        .collect(),
                    items: Items {
                        total: r.report.items_total,
                        success: r.report.items_success,
                        skipped: r.report.items_skipped,
                        failed: r.report.items_failed,
                    },
                    user_id,
                    total_exec_time_ms: r.report.total_exec_time_ms,
                    created_at: r.report.created_at.to_timezone(tz),
                }
            })
            .collect::<Vec<_>>();

        Ok(grouped)
    }

    /// Fetch a report by its ID.
    pub async fn fetch(mm: &ModelManager, report_id: i64, user_id: Uuid) -> Result<Self> {
        let mut conn = mm.pool.get().await?;

        let r = schema::reports::table
            .filter(schema::reports::user_id.eq(user_id))
            .filter(schema::reports::id.eq(report_id))
            .inner_join(schema::report_types_primary::table)
            .left_join(schema::report_types_secondary::table)
            .left_join(schema::report_types_tertiary::table)
            .select(ReportWithTypes::as_select())
            .first::<ReportWithTypes>(&mut conn)
            .await
            .map_err(|_| Error::EntityNotFound {
                entity: "report",
                id: report_id.to_string(),
            })?;

        let logs = ReportLog::belonging_to(&r.report)
            .inner_join(schema::levels::table)
            .select(ReportLogWithLevel::as_select())
            .order(schema::reports_logs::seq_num.asc())
            .load::<ReportLogWithLevel>(&mut conn)
            .await?;

        drop(conn);

        Ok(Self {
            id: r.report.id,
            report_type: ViewReportType {
                primary: r.report_type_primary,
                secondary: r.report_type_secondary,
                tertiary: r.report_type_tertiary,
            },
            report_logs: logs
                .into_iter()
                .map(|l| ViewReportLog {
                    id: l.report_log.id,
                    seq_num: l.report_log.seq_num,
                    entity_name: l.report_log.entity_name,
                    recipe_id: l.report_log.recipe_id,
                    level: l.level,
                    error_code: l.report_log.error_code,
                    error_reason: l.report_log.error_reason,
                    exec_time_ms: l.report_log.exec_time_ms,
                })
                .collect(),
            items: Items {
                total: r.report.items_total,
                success: r.report.items_success,
                skipped: r.report.items_skipped,
                failed: r.report.items_failed,
            },
            user_id,
            total_exec_time_ms: r.report.total_exec_time_ms,
            created_at: r.report.created_at,
        })
    }

    /// Formats the execution time in milliseconds as a human-readable duration string.
    #[allow(clippy::cast_precision_loss)]
    pub fn format_duration(&self) -> String {
        format_duration_ms(self.total_exec_time_ms)
    }
}

impl ViewReportLog {
    /// Formats the execution time in milliseconds as a human-readable duration string.
    #[allow(clippy::cast_precision_loss)]
    pub fn format_duration(&self) -> String {
        format_duration_ms(self.exec_time_ms)
    }
}

/// Trait for types that can have multiple errors.
pub trait ReportErrors {
    /// Returns `true` if more than one log has an error level.
    fn has_multiple_errors(&self) -> bool;
    /// Collects all error logs into a newline-delimited string.
    fn collect_errors(&self) -> String;
}

impl ReportErrors for &[ViewReportLog] {
    fn has_multiple_errors(&self) -> bool {
        self.iter().filter(|l| l.level.id == 4).count() > 1
    }

    fn collect_errors(&self) -> String {
        self.iter()
            .filter(|l| l.level.id == 4)
            .map(|l| l.entity_name.as_str())
            .join("\\n")
    }
}

#[allow(clippy::cast_precision_loss)]
fn format_duration_ms(ms: i64) -> String {
    match ms {
        0 => "0ms".to_string(),
        ms if ms < 1_000 => format!("{ms}ms"),
        ms if ms < 60_000 => format!("{:.1}s", ms as f64 / 1_000.0),
        ms if ms < 3_600_000 => format!("{:.1}m", ms as f64 / 60_000.0),
        _ => format!("{:.1}h", ms as f64 / 3_600_000.0),
    }
}
