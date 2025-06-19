use diesel::internal::derives::multiconnection::chrono;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use repository::{ModelManager, schema};

use super::Result;
use crate::user::User;

/// An enumeration of all report types.
pub enum ReportTypes {
    Import,
}

impl ReportTypes {
    fn to_id(&self) -> i16 {
        match self {
            ReportTypes::Import => 1,
        }
    }
}

/// Represents a collection of reports.
pub struct ReportForCreate {
    report_type_id: i16,
    user_id: i64,

    pub report_logs: Vec<ReportLogForCreate>,
    pub exec_time_ms: i64,
}

impl ReportForCreate {
    /// Creates a new report with all the data required to insert into the database later on.
    pub fn new(report_type: ReportTypes, user_id: i64) -> Self {
        Self {
            report_type_id: report_type.to_id(),
            user_id,
            report_logs: vec![],
            exec_time_ms: 0,
        }
    }

    /// Inserts report into the database.
    pub async fn insert(&self, mm: &ModelManager) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        let report_id = diesel::insert_into(schema::reports::table)
            .values(&ReportForInsert::from(self))
            .returning(schema::reports::columns::id)
            .get_result::<i64>(&mut conn)
            .await?;

        let logs = self
            .report_logs
            .iter()
            .map(|log| ReportLogForInsert {
                report_id,
                title: log.title.clone(),
                is_success: log.is_success,
                is_warning: log.is_warning,
                is_error: log.is_error,
                error_reason: log.error_reason.clone(),
            })
            .collect::<Vec<_>>();

        diesel::insert_into(schema::reports_logs::table)
            .values(&logs)
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}

impl From<&ReportForCreate> for ReportForInsert {
    fn from(value: &ReportForCreate) -> Self {
        Self {
            report_type_id: value.report_type_id,
            user_id: value.user_id,
            exec_time_ms: value.exec_time_ms,
        }
    }
}

/// Represents a type of report.
#[derive(Identifiable, Queryable)]
#[diesel(table_name = schema::report_types)]
struct ReportType {
    id: i64,
    name: String,
}

/// Represents a collection of reports for the database.
#[derive(Default, Associations, Identifiable, Queryable, Selectable)]
#[diesel(table_name = schema::reports)]
#[diesel(belongs_to(ReportType))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Report {
    pub id: i64,
    pub report_type_id: i16,
    pub user_id: i64,
    pub exec_time_ms: i64,
    pub created_at: chrono::NaiveDateTime,
}

/// Represents a Report to be inserted into the database.
#[derive(Insertable)]
#[diesel(table_name = schema::reports)]
struct ReportForInsert {
    report_type_id: i16,
    user_id: i64,
    exec_time_ms: i64,
}

/// Represents the details of a report.
#[derive(Debug, PartialEq, Associations, Identifiable, Queryable, Selectable)]
#[diesel(table_name = schema::reports_logs)]
#[diesel(belongs_to(Report))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportLog {
    pub id: i64,
    pub report_id: i64,
    pub title: String,
    pub is_success: bool,
    pub is_warning: bool,
    pub is_error: bool,
    pub error_reason: String,
}

/// Represents the details of a report to create in the database.
#[derive(Default)]
pub struct ReportLogForCreate {
    title: String,
    is_success: bool,
    is_warning: bool,
    is_error: bool,
    error_reason: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::reports_logs)]
struct ReportLogForInsert {
    report_id: i64,
    title: String,
    is_success: bool,
    is_warning: bool,
    is_error: bool,
    error_reason: String,
}

impl ReportLogForCreate {
    /// Creates a new success report log.
    pub fn new_success(title: String) -> Self {
        Self {
            title,
            is_success: true,
            ..Default::default()
        }
    }

    /// Creates a new warning report log.
    pub fn new_warning(title: String, error_reason: String) -> Self {
        Self {
            title,
            is_warning: true,
            error_reason,
            ..Default::default()
        }
    }

    /// Creates a new error report log.
    pub fn new_error(title: String, error_reason: String) -> Self {
        Self {
            title,
            is_error: true,
            error_reason,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use testing::utils::{TestDb, create_app_state, insert_user};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_insert_report() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;
        let mut report = ReportForCreate::new(ReportTypes::Import, user.id);
        let warning_report = ReportLogForCreate::new_warning("Warning".into(), String::new());
        let error_report =
            ReportLogForCreate::new_error("Error".into(), "An error occurred".into());
        let success_report = ReportLogForCreate::new_success("Success".into());
        report.report_logs.push(warning_report);
        report.report_logs.push(error_report);
        report.report_logs.push(success_report);

        report.insert(&state.mm).await?;

        let mut conn = state.mm.pool.get().await?;
        let got_report = schema::reports::table
            .select(Report::as_select())
            .first(&mut conn)
            .await?;
        assert_report(
            &got_report,
            Report {
                id: 1,
                report_type_id: ReportTypes::Import.to_id(),
                user_id: user.id,
                ..Default::default()
            },
        );
        let got_logs = schema::reports_logs::table
            .select(ReportLog::as_select())
            .load(&mut conn)
            .await?;
        pretty_assertions::assert_eq!(
            got_logs,
            vec![
                ReportLog {
                    id: 1,
                    report_id: got_report.id,
                    title: "Warning".into(),
                    is_success: false,
                    is_warning: true,
                    is_error: false,
                    error_reason: "".into(),
                },
                ReportLog {
                    id: 2,
                    report_id: got_report.id,
                    title: "Error".into(),
                    is_success: false,
                    is_warning: false,
                    is_error: true,
                    error_reason: "An error occurred".into(),
                },
                ReportLog {
                    id: 3,
                    report_id: got_report.id,
                    title: "Success".into(),
                    is_success: true,
                    is_warning: false,
                    is_error: false,
                    error_reason: "".into(),
                }
            ]
        );
        Ok(())
    }

    fn assert_report(got_report: &Report, want: Report) {
        pretty_assertions::assert_eq!(got_report.id, want.id);
        pretty_assertions::assert_eq!(got_report.report_type_id, want.report_type_id);
        pretty_assertions::assert_eq!(got_report.user_id, want.user_id);
    }
}
