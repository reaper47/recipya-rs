use serde::{Deserialize, Serialize};

/// The payload of a list.
#[derive(Default, Deserialize, Serialize)]
pub struct ListPayload {
    pub name: String,
}

/// The payload of a list's item.
#[derive(Default, Deserialize, Serialize)]
pub struct ListItemPayload {
    pub item: String,
    pub quantity: Option<String>,
    pub label: Option<String>,
    pub position: Option<i32>,
}
