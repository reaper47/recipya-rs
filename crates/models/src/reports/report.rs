use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use time::OffsetDateTime;
use uuid::Uuid;

use repository::{ModelManager, schema};

use crate::{
    Result,
    reports::{
        report_log::{ReportLog, ReportLogForCreate, ReportLogForInsert},
        report_types::{
            PrimaryReportType, ReportTypeFull, ReportTypeId, ReportTypePrimary,
            ReportTypeSecondary, ReportTypeTertiary,
        },
    },
    user::User,
};

/// Represents a report.
#[derive(Associations, Identifiable, Queryable, Selectable)]
#[diesel(table_name = schema::reports)]
#[diesel(belongs_to(ReportTypePrimary))]
#[diesel(belongs_to(ReportTypeSecondary))]
#[diesel(belongs_to(ReportTypeTertiary))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Report {
    pub id: i64,
    pub report_type_primary_id: i16,
    pub report_type_secondary_id: Option<i16>,
    pub report_type_tertiary_id: Option<i16>,
    pub items_total: i32,
    pub items_success: i32,
    pub items_skipped: i32,
    pub items_failed: i32,
    pub user_id: Uuid,
    pub total_exec_time_ms: i64,
    pub created_at: OffsetDateTime,
}

impl Report {
    /// Fetches all reports for a user.
    pub async fn fetch_all(mm: &ModelManager, user_id: Uuid) -> Result<Vec<Self>> {
        let mut conn = mm.pool.get().await?;

        let reports = schema::reports::table
            .filter(schema::reports::user_id.eq(user_id))
            .load::<Self>(&mut conn)
            .await?;

        Ok(reports)
    }

    /// Fetches the report logs for a report.
    pub async fn fetch_logs(&self, mm: &ModelManager) -> Result<Vec<ReportLog>> {
        let mut conn = mm.pool.get().await?;

        let logs = schema::reports_logs::table
            .filter(schema::reports_logs::report_id.eq(self.id))
            .load::<ReportLog>(&mut conn)
            .await?;

        Ok(logs)
    }
}

/// Represents a `Report` to be inserted into the database.
#[derive(Insertable)]
#[diesel(table_name = schema::reports)]
struct ReportForInsert {
    report_type_primary_id: i16,
    report_type_secondary_id: Option<i16>,
    report_type_tertiary_id: Option<i16>,
    items_total: i32,
    items_success: i32,
    items_skipped: i32,
    items_failed: i32,
    user_id: Uuid,
    total_exec_time_ms: i64,
}

/// Represents a `Report` to create.
pub struct ReportForCreate {
    pub report_type_primary_id: i16,
    pub report_type_secondary_id: Option<i16>,
    pub report_type_tertiary_id: Option<i16>,
    pub items: Items,
    pub user_id: Uuid,
    pub total_exec_time_ms: i64,
    pub report_logs: Vec<ReportLogForCreate>,
}

/// Holds information about the items processed in a report.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Items {
    pub total: i32,
    pub success: i32,
    pub skipped: i32,
    pub failed: i32,
}

impl From<&[ReportLogForCreate]> for Items {
    fn from(logs: &[ReportLogForCreate]) -> Self {
        logs.iter().fold(Self::default(), |mut acc, log| {
            acc.total += 1;
            match log.level_id {
                2 => acc.success += 1,
                3 => acc.skipped += 1,
                4 => acc.failed += 1,
                _ => {}
            }
            acc
        })
    }
}

impl ReportForCreate {
    /// Creates a new report with all the data required to insert into the database later on.
    pub fn new<P, S>(
        report_type: ReportTypeFull<P, S>,
        report_logs: Vec<ReportLogForCreate>,
        items: Items,
        total_exec_time_ms: i64,
        user_id: Uuid,
    ) -> Self
    where
        PrimaryReportType<P>: ReportTypeId,
    {
        Self {
            report_type_primary_id: report_type.primary.to_id(),
            report_type_secondary_id: report_type.secondary.map(|t| t.to_id()),
            report_type_tertiary_id: report_type.tertiary.map(|t| t.to_id()),
            user_id,
            items,
            total_exec_time_ms,
            report_logs,
        }
    }

    /// Inserts a report into the database.
    pub async fn insert(&self, mm: &ModelManager) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        let report_id = diesel::insert_into(schema::reports::table)
            .values(&ReportForInsert::from(self))
            .returning(schema::reports::columns::id)
            .get_result::<i64>(&mut conn)
            .await?;

        diesel::insert_into(schema::reports_logs::table)
            .values(
                &self
                    .report_logs
                    .iter()
                    .map(|log| ReportLogForInsert {
                        report_id,
                        seq_num: log.seq_num,
                        entity_name: log.entity_name.clone(),
                        recipe_id: log.recipe_id,
                        level_id: log.level_id,
                        error_code: log.error_code.clone(),
                        error_reason: log.error_reason.clone(),
                        exec_time_ms: log.exec_time_ms,
                    })
                    .collect::<Vec<_>>(),
            )
            .execute(&mut conn)
            .await?;

        Ok(())
    }
}

impl From<&ReportForCreate> for ReportForInsert {
    fn from(value: &ReportForCreate) -> Self {
        Self {
            user_id: value.user_id,
            report_type_primary_id: value.report_type_primary_id,
            report_type_secondary_id: value.report_type_secondary_id,
            report_type_tertiary_id: value.report_type_tertiary_id,
            items_total: value.items.total,
            items_success: value.items.success,
            items_skipped: value.items.skipped,
            items_failed: value.items.failed,
            total_exec_time_ms: value.total_exec_time_ms,
        }
    }
}

/// Represents a report with its associated report types.
#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ReportWithTypes {
    #[diesel(embed)]
    pub report: Report,
    #[diesel(embed)]
    pub report_type_primary: ReportTypePrimary,
    #[diesel(embed)]
    pub report_type_secondary: Option<ReportTypeSecondary>,
    #[diesel(embed)]
    pub report_type_tertiary: Option<ReportTypeTertiary>,
}
