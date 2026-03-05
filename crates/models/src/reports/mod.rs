pub mod report;
pub mod report_log;
pub mod report_types;

mod view;

// TODO: Remove viewreporttype once ui designed
pub use view::{ViewReport, ViewReportLog, ViewReportType};
