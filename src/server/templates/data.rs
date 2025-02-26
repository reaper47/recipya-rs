use std::path::Path;

use diesel::internal::derives::multiconnection::chrono::NaiveTime;
use uuid::Uuid;

use crate::core::model::recipe::Times;
use crate::core::model::RecipeDetails;

use crate::server::Result;

/// Data holds data to pass on to the templates.
pub struct Data {
    pub is_admin: bool,
    pub is_authenticated: bool,
    pub is_autologin: bool,
    pub is_hx_request: bool,

    pub about: AboutData,
    pub share: ShareData,
    pub view: ViewRecipe,
}

/// NewAboutData creates a new instance of AboutData.
pub struct AboutData {
    pub(crate) is_update_available: bool,
}

/// ShareData holds information on the entity being shared.
#[derive(Default)]
pub struct ShareData {
    pub is_from_host: bool,
    pub is_shared: bool,
}

/// ViewRecipeData holds template data related to viewing a recipe.
pub struct ViewRecipe {
    pub recipe_details: RecipeDetails,
    pub formatted_times: FormattedTimes,
}

/// Stores recipe times formatted for display to the user.
#[derive(Debug, PartialEq)]
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
    /// Creates a new FormattedTimes from the recipe's Times.
    pub fn from_times(times: &Times) -> Result<Self> {
        let cook = humantime::parse_duration(&format!("{}s", times.cook_seconds))?;
        let prep = humantime::parse_duration(&format!("{}s", times.prep_seconds))?;
        let total = humantime::parse_duration(&format!("{}s", times.total_seconds))?;

        let prep_edit = NaiveTime::from_num_seconds_from_midnight_opt(prep.as_secs() as u32, 0)
            .map_or_else(
                || String::from("00:15:00"),
                |t| t.format("%H:%M:%S").to_string(),
            );

        let cook_edit = NaiveTime::from_num_seconds_from_midnight_opt(cook.as_secs() as u32, 0)
            .map_or_else(
                || String::from("00:15:00"),
                |t| t.format("%H:%M:%S").to_string(),
            );

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
        iso_duration.push_str(&format!("{}D", days));
    }
    if hours > 0 || minutes > 0 || seconds > 0 {
        iso_duration.push('T'); // Time component starts
        if hours > 0 {
            iso_duration.push_str(&format!("{}H", hours));
        }
        if minutes > 0 {
            iso_duration.push_str(&format!("{}M", minutes));
        }
        if seconds > 0 {
            iso_duration.push_str(&format!("{}S", seconds));
        }
    }

    if iso_duration == "P" {
        iso_duration.push_str("T0S");
    }

    iso_duration
}

// TODO: Put this somewhere else because it might be used
/// Checks whether the media file exists in the file system.
pub(super) fn is_file_exists(media_file: Uuid, dir: &Path) -> bool {
    Path::new(dir)
        .join(media_file.to_string())
        .exists()
}

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_formatted_times {
        use super::*;
        use crate::core::model::recipe::Times;
        use crate::server::templates::data::FormattedTimes;

        #[test]
        fn test_with_hours_and_minutes_ok() -> Result<()> {
            let times = Times {
                cook_seconds: 10800,
                prep_seconds: 900,
                total_seconds: 10800 + 900,
                id: 0,
                recipe_id: 0,
            };

            let got = FormattedTimes::from_times(&times)?;

            pretty_assertions::assert_eq!(
                got,
                FormattedTimes {
                    cook: String::from("3h"),
                    cook_datetime: String::from("PT3H"),
                    cook_edit: String::from("03:00:00"),
                    prep: String::from("15m"),
                    prep_datetime: String::from("PT15M"),
                    prep_edit: String::from("00:15:00"),
                    total: String::from("3h 15m"),
                    total_datetime: String::from("PT3H15M"),
                }
            );
            Ok(())
        }

        #[test]
        fn test_with_minutes_ok() -> Result<()> {
            let times = Times {
                cook_seconds: 960,
                prep_seconds: 900,
                total_seconds: 960 + 900,
                id: 0,
                recipe_id: 0,
            };

            let got = FormattedTimes::from_times(&times)?;

            pretty_assertions::assert_eq!(
                got,
                FormattedTimes {
                    cook: String::from("16m"),
                    cook_datetime: String::from("PT16M"),
                    cook_edit: String::from("00:16:00"),
                    prep: String::from("15m"),
                    prep_datetime: String::from("PT15M"),
                    prep_edit: String::from("00:15:00"),
                    total: String::from("31m"),
                    total_datetime: String::from("PT31M"),
                }
            );
            Ok(())
        }
    }
}
