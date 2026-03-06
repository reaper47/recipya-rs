use diesel::prelude::*;
use repository::schema;

use crate::reports::report::Report;

/// Represents the details of a report.
#[derive(Debug, Eq, PartialEq, Associations, Identifiable, Queryable, Selectable)]
#[diesel(table_name = schema::reports_logs)]
#[diesel(belongs_to(Report))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportLog {
    pub id: i64,
    pub seq_num: i32,
    pub report_id: i64,
    pub entity_name: String,
    pub recipe_id: Option<i64>,
    pub level_id: i16,
    pub error_code: Option<String>,
    pub error_reason: Option<String>,
    pub exec_time_ms: i64,
}

/// Represents the details of a report to create in the database.
#[derive(Debug, Default)]
pub struct ReportLogForCreate {
    pub seq_num: i32,
    pub entity_name: String,
    pub recipe_id: Option<i64>,
    pub level_id: i16,
    pub error_code: Option<String>,
    pub error_reason: Option<String>,
    pub exec_time_ms: i64,
}

#[derive(Insertable)]
#[diesel(table_name = schema::reports_logs)]
pub struct ReportLogForInsert {
    pub seq_num: i32,
    pub report_id: i64,
    pub entity_name: String,
    pub recipe_id: Option<i64>,
    pub level_id: i16,
    pub error_code: Option<String>,
    pub error_reason: Option<String>,
    pub exec_time_ms: i64,
}

impl ReportLogForCreate {
    /// Creates a new success report log.
    pub fn success(
        seq_num: i32,
        entity_name: &str,
        recipe_id: Option<i64>,
        exec_time_ms: i64,
    ) -> Self {
        Self {
            entity_name: entity_name.into(),
            recipe_id,
            level_id: 2,
            seq_num,
            exec_time_ms,
            ..Default::default()
        }
    }

    /// Creates a new warning report log.
    pub fn warning(
        seq_num: i32,
        entity_name: &str,
        recipe_id: Option<i64>,
        error_reason: &str,
        exec_time_ms: i64,
    ) -> Self {
        Self {
            entity_name: entity_name.into(),
            recipe_id,
            level_id: 3,
            error_reason: Some(error_reason.into()),
            seq_num,
            error_code: None,
            exec_time_ms,
        }
    }

    /// Creates a new error report log.
    pub fn error(
        seq_num: i32,
        entity_name: &str,
        recipe_id: Option<i64>,
        error_code: &str,
        error_reason: &str,
        exec_time_ms: i64,
    ) -> Self {
        Self {
            entity_name: entity_name.into(),
            recipe_id,
            level_id: 4,
            error_code: Some(error_code.into()),
            error_reason: Some(error_reason.into()),
            seq_num,
            exec_time_ms,
        }
    }
}

/// Represents a log level.
#[derive(Clone, Debug, Eq, PartialEq, Identifiable, Queryable, Selectable)]
#[diesel(table_name = schema::levels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Level {
    pub id: i16,
    pub name: String,
}

/// Represents a report log with its associated log level.
#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportLogWithLevel {
    #[diesel(embed)]
    pub report_log: ReportLog,
    #[diesel(embed)]
    pub level: Level,
}
