use serde::{Deserialize, Serialize, de::IntoDeserializer};
use uuid::Uuid;

/// The payload of a list.
#[derive(Default, Deserialize, Serialize)]
pub struct ListPayload {
    pub name: String,
}

/// The payload of a list's item.
#[derive(Default, Deserialize, Serialize)]
pub struct ListItemPayload {
    pub item: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub quantity: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub label: Option<String>,
    pub position: Option<i32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub notes: Option<String>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

/// The payload of a recipe's ingredients to add to a shopping list.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RecipeIngredientsPayload {
    pub list: Uuid,
    pub ingredients: Vec<String>,
    pub quantities: Vec<String>,
    pub notes: Vec<String>,
    #[serde(rename = "with-quantity", default)]
    pub with_quantities: Vec<String>,
}
