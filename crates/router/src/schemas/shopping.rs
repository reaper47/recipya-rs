use serde::{Deserialize, Serialize};

/// The payload of a list's item.
#[derive(Default, Deserialize, Serialize)]
pub struct ListItemPayload {
    pub item: String,
    pub quantity: Option<String>,
    pub label: Option<String>,
}
