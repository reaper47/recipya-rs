use serde::{Deserialize, Serialize, de::IntoDeserializer};

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
    pub list: String,
    pub ingredients: Vec<String>,
    pub quantities: Vec<String>,
    pub notes: Vec<String>,
    #[serde(rename = "with-quantity", default)]
    pub with_quantities: Vec<String>,
}

impl RecipeIngredientsPayload {
    /// Removes noise from the fields.
    pub fn clean(&mut self) {
        let trim = |v: &mut Vec<String>| {
            for s in v.iter_mut() {
                *s = s.trim().to_string();
            }
            v.retain(|s| !s.is_empty());
        };

        trim(&mut self.ingredients);
        trim(&mut self.quantities);
        trim(&mut self.notes);
        trim(&mut self.with_quantities);
    }

    /// Checks whether the payload has values.
    pub const fn is_empty(&self) -> bool {
        self.ingredients.is_empty()
            || self.quantities.is_empty()
            || self.with_quantities.is_empty()
            || self.notes.is_empty()
    }
}
