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

#[cfg(test)]
mod tests {
    use crate::reports::{
        report::ReportForCreate,
        report_log::ReportLogForCreate,
        report_types::{
            API, Import, PrimaryReportType, ReportTypeFull, SecondaryReportType, TertiaryReportType,
        },
    };

    use super::*;

    use test_db::TestDb;
    use test_utils::{create_app_state, insert_user};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    #[allow(clippy::too_many_lines)]
    async fn test_fetch_reports_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;
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
                user.id,
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
                user.id,
            ),
        ];
        reports[0].insert(&state.mm).await?;
        reports[1].insert(&state.mm).await?;

        let mut got = ViewReport::fetch_all(&state.mm, 1, user.id).await?;

        got.sort_by_key(|a| a.id);
        pretty_assertions::assert_eq!(
            got,
            vec![
                ViewReport {
                    id: 1,
                    report_type: ViewReportType {
                        primary: ReportTypePrimary {
                            id: 1,
                            name: "import".into(),
                        },
                        secondary: Some(
                            ReportTypeSecondary {
                                id: 1,
                                name: "api".into(),
                            },
                        ),
                        tertiary: Some(
                            ReportTypeTertiary {
                                id: 1,
                                name: "mealie".into(),
                            },
                        ),
                    },
                    report_logs: vec![
                        ViewReportLog {
                            id: 1,
                            seq_num: 1,
                            entity_name: "Raspberry Pi".into(),
                            recipe_id: None,
                            level: Level {
                                id: 2,
                                name: "success".into(),
                            },
                            error_code: None,
                            error_reason: None,
                            exec_time_ms: 167,
                        },
                        ViewReportLog {
                            id: 2,
                            seq_num: 2,
                            entity_name: "Orange Pi".into(),
                            recipe_id: None,
                            level: Level {
                                id: 2,
                                name: "success".into(),
                            },
                            error_code: None,
                            error_reason: None,
                            exec_time_ms: 544,
                        },
                        ViewReportLog {
                            id: 3,
                            seq_num: 3,
                            entity_name: "Pink Pi".into(),
                            recipe_id: None,
                            level: Level {
                                id: 2,
                                name: "success".into(),
                            },
                            error_code: None,
                            error_reason: None,
                            exec_time_ms: 78,
                        },
                    ],
                    items: Items {
                        total: 3,
                        success: 3,
                        skipped: 0,
                        failed: 0,
                    },
                    user_id: user.id,
                    total_exec_time_ms: 789,
                    created_at: got[0].created_at,
                },
                ViewReport {
                    id: 2,
                    report_type: ViewReportType {
                        primary: ReportTypePrimary {
                            id: 2,
                            name: "website".into(),
                        },
                        secondary: None,
                        tertiary: None,
                    },
                    report_logs: vec![
                        ViewReportLog {
                            id: 4,
                            seq_num: 1,
                            entity_name: "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/".into(),
                            recipe_id: None,
                            level: Level {
                                id: 2,
                                name: "success".into(),
                            },
                            error_code: None,
                            error_reason: None,
                            exec_time_ms: 243,
                        },
                        ViewReportLog {
                            id: 5,
                            seq_num: 2,
                            entity_name: "https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741".into(),
                            recipe_id: None,
                            level: Level {
                                id: 3,
                                name: "warning".into(),
                            },
                            error_code: None,
                            error_reason: Some(
                                "The recipe exists in your collection.".into(),
                            ),
                            exec_time_ms: 124,
                        },
                        ViewReportLog {
                            id: 6,
                            seq_num: 3,
                            entity_name: "https://www.allrecipes.com/southern-breakfast-potatoes-recipe-11907741".into(),
                            recipe_id: None,
                            level: Level {
                                id: 4,
                                name: "error".into(),
                            },
                            error_code: Some(
                                "NetworkFailure".into(),
                            ),
                            error_reason: Some(
                                "Failed to connect to the website.".into(),
                            ),
                            exec_time_ms: 124,
                        },
                    ],
                    items: Items {
                        total: 3,
                        success: 1,
                        skipped: 1,
                        failed: 1,
                    },
                    user_id: user.id,
                    total_exec_time_ms: 491,
                    created_at: got[1].created_at,
                },
            ]
        );
        Ok(())
    }
}
