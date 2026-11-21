use jiff::{SignedDuration, civil::Time};

use super::Result;
use crate::recipe::structs::time::Times;

/// Stores recipe times formatted for display to the user.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
        let cook = SignedDuration::from_secs(i64::from(times.cook_seconds));
        let prep = SignedDuration::from_secs(i64::from(times.prep_seconds));
        let total = SignedDuration::from_secs(i64::from(times.total_seconds));

        Ok(Self {
            cook: format!("{cook:#}"),
            cook_datetime: cook.to_string(),
            cook_edit: secs_to_time_str(cook),
            prep: format!("{prep:#}"),
            prep_datetime: prep.to_string(),
            prep_edit: secs_to_time_str(prep),
            total: format!("{total:#}"),
            total_datetime: total.to_string(),
        })
    }
}

fn secs_to_time_str(dur: SignedDuration) -> String {
    let secs = dur.as_secs();

    let time = (|| {
        let hour = i8::try_from(secs / 3600).ok()?;
        let minute = i8::try_from((secs % 3600) / 60).ok()?;
        let second = i8::try_from(secs % 60).ok()?;
        Time::new(hour, minute, second, 0).ok()
    })();

    time.map_or_else(|| "00:15:00".into(), |t| t.strftime("%H:%M:%S").to_string())
}
