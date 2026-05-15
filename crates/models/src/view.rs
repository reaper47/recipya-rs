use serde::Deserialize;

/// Represents the view mode of an item.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewMode {
    #[default]
    Edit,
    Print,
    View,
}
