use std::fmt::Write;

use chrono::NaiveTime;
use tracing::error;

use crate::recipe::structs::time::Times;

use super::Result;

/// Stores recipe times formatted for display to the user.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FormattedTimes {
    pub cook: String,
    pub cook_datetime: String,
    pub cook_edit: String,
    pub prep: String,
    pub prep_datetime: String,
    pub prep_edit: String,
    pub total: String,
    pub total_datetime: String,
}

impl FormattedTimes {
    /// Creates a new `FormattedTimes` from the recipe's Times.
    pub fn from_times(times: &Times) -> Result<Self> {
        let cook = humantime::parse_duration(&format!("{}s", times.cook_seconds))?;
        let prep = humantime::parse_duration(&format!("{}s", times.prep_seconds))?;
        let total = humantime::parse_duration(&format!("{}s", times.total_seconds))?;

        let prep_edit = NaiveTime::from_num_seconds_from_midnight_opt(
            u32::try_from(prep.as_secs())
                .inspect_err(|err| error!("Failed to convert prep time '{prep:?}' to u32: {err}"))
                .unwrap_or_default(),
            0,
        )
        .map_or_else(|| "00:15:00".into(), |t| t.format("%H:%M:%S").to_string());

        let cook_edit = NaiveTime::from_num_seconds_from_midnight_opt(
            u32::try_from(cook.as_secs())
                .inspect_err(|err| error!("Failed to convert cook time '{cook:?}' to u32: {err}"))
                .unwrap_or_default(),
            0,
        )
        .map_or_else(|| "00:15:00".into(), |t| t.format("%H:%M:%S").to_string());

        Ok(Self {
            cook: humantime::format_duration(cook).to_string(),
            cook_datetime: duration_to_iso8601(cook.into()),
            cook_edit,
            prep: humantime::format_duration(prep).to_string(),
            prep_datetime: duration_to_iso8601(prep.into()),
            prep_edit,
            total: humantime::format_duration(total).to_string(),
            total_datetime: duration_to_iso8601(total.into()),
        })
    }
}

fn duration_to_iso8601(duration: humantime::Duration) -> String {
    let total_secs = duration.as_secs();
    let days = total_secs / 86_400;
    let hours = (total_secs % 86_400) / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    let mut iso_duration = "P".to_string();

    if days > 0 {
        write!(iso_duration, "{days}D").unwrap();
    }

    if hours > 0 || minutes > 0 || seconds > 0 {
        iso_duration.push('T');
        if hours > 0 {
            write!(iso_duration, "{hours}H").unwrap();
        }
        if minutes > 0 {
            write!(iso_duration, "{minutes}M").unwrap();
        }
        if seconds > 0 {
            write!(iso_duration, "{seconds}S").unwrap();
        }
    }

    if iso_duration == "P" {
        iso_duration.push_str("T0S");
    }

    iso_duration
}
