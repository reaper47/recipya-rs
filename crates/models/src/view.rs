use std::{fmt, str::FromStr};

use serde::Deserialize;

/// Represents the view mode of an item.
#[derive(Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ViewMode {
    #[default]
    Edit,
    Print,
    View,
}

impl fmt::Display for ViewMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Edit => write!(f, "edit"),
            Self::Print => write!(f, "print"),
            Self::View => write!(f, "view"),
        }
    }
}

impl FromStr for ViewMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "edit" => Ok(Self::Edit),
            "print" => Ok(Self::Print),
            "view" => Ok(Self::View),
            _ => Err(()),
        }
    }
}
