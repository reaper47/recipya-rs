use diesel::{Identifiable, Insertable, Queryable, Selectable};

use repository::schema;
use schema_org::field::{ItemListItemListElementFieldEnum, PropertyValueValueFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum};

pub enum Sections {
    Sections(Vec<SectionItem>),
    Items(Vec<Item>),
}

impl Sections {
    /// Creates a new `Sections::Items` from a list of text.
    pub fn new(items: Vec<String>) -> Self {
        Self::Items(items.into_iter().map(|item| Item {
            text: item,
            duration_seconds: None,
        }).collect())
    }


    /// Assembles a vector of the items as text.
    pub fn items_as_text(&self) -> Vec<String> {
        match self {
            Sections::Sections(section) => section.iter().flat_map(|section| section.items.iter().map(|item| item.text.clone())).collect::<Vec<_>>(),
            Sections::Items(items) => items.iter().map(|item| item.text.clone()).collect::<Vec<_>>()
        }
    }
}

impl Default for Sections {
    fn default() -> Self {
        Self::Sections(vec![])
    }
}

pub struct SectionItem {
    pub title: String,
    pub items: Vec<Item>,
}

pub struct Item {
    pub text: String,
    pub duration_seconds: Option<i16>,
}

impl From<Vec<RecipeRecipeIngredientFieldEnum>> for Sections {
    fn from(items: Vec<RecipeRecipeIngredientFieldEnum>) -> Self {
        let mut sections = Vec::new();
        let mut current_items = Vec::new();

        for item in items {
            match item {
                RecipeRecipeIngredientFieldEnum::ItemList(list) => {
                    if !current_items.is_empty() {
                        sections.push(SectionItem {
                            title: String::new(),
                            items: std::mem::take(&mut current_items),
                        });
                    }

                    let items = list.item_list_element
                        .into_iter()
                        .filter_map(|elem| match elem {
                            ItemListItemListElementFieldEnum::Text(text) => Some(Item {
                                text,
                                duration_seconds: None,
                            }),
                            _ => None,
                        })
                        .collect();

                    sections.push(SectionItem {
                        title: list.name.first().cloned().unwrap_or_default(),
                        items,
                    });
                }
                RecipeRecipeIngredientFieldEnum::PropertyValue(prop) => {
                    let value = prop.value.first().map(|v| match v {
                        PropertyValueValueFieldEnum::BooleanEnumOrText(s) => s.clone(),
                        PropertyValueValueFieldEnum::Number(n) => n.to_string(),
                        PropertyValueValueFieldEnum::StructuredValue(v) => v.name.first().cloned().unwrap_or_default(),
                    }).unwrap_or_default();

                    let name = prop.name.first().cloned().unwrap_or_default();

                    let unit = {
                        let code = prop.unit_code.first().map(|s| s.to_string()).unwrap_or_default().to_string();
                        let text = prop.unit_text.first().map(|s| s.to_string()).unwrap_or_default().to_string();

                        if code.is_empty() {
                            text
                        } else {
                            code
                        }
                    };

                    current_items.push(Item {
                        text: format!("{}{}{}", if !value.is_empty() {
                            &format!("{} ", value)
                        } else {
                            ""
                        }, if !unit.is_empty() {
                            &format!("{} ", unit)
                        } else {
                            ""
                        }, if !name.is_empty() {
                            &name
                        } else {
                            ""
                        }),
                        duration_seconds: None,
                    });
                }
                RecipeRecipeIngredientFieldEnum::Text(text) => {
                    current_items.push(Item {
                        text,
                        duration_seconds: None,
                    });
                }
            }
        }

        if !current_items.is_empty() {
            sections.push(SectionItem {
                title: String::new(),
                items: current_items,
            });
        }

        if sections.is_empty() {
            Self::Items(Vec::new())
        } else if sections.len() == 1 && sections[0].title.is_empty() {
            Self::Items(sections.into_iter().next().unwrap().items)
        } else {
            Self::Sections(sections)
        }
    }
}

impl From<Vec<RecipeRecipeInstructionsFieldEnum>> for Sections {
    fn from(items: Vec<RecipeRecipeInstructionsFieldEnum>) -> Self {
        let mut sections = Vec::new();
        let mut current_items = Vec::new();

        for item in items {
            match item {
                RecipeRecipeInstructionsFieldEnum::ItemList(list) => {
                    if !current_items.is_empty() {
                        sections.push(SectionItem {
                            title: String::new(),
                            items: std::mem::take(&mut current_items),
                        });
                    }

                    let items = list.item_list_element
                        .into_iter()
                        .filter_map(|elem| match elem {
                            ItemListItemListElementFieldEnum::Text(text) => Some(Item {
                                text,
                                duration_seconds: None,
                            }),
                            _ => None,
                        })
                        .collect();

                    sections.push(SectionItem {
                        title: list.name.first().cloned().unwrap_or_default(),
                        items,
                    });
                }
                RecipeRecipeInstructionsFieldEnum::CreativeWork(work) => {
                    if !current_items.is_empty() {
                        sections.push(SectionItem {
                            title: String::new(),
                            items: std::mem::take(&mut current_items),
                        });
                    }

                    sections.push(SectionItem {
                        title: work.name.first().cloned().unwrap_or_default(),
                        items: work.text.into_iter().map(|text| Item { text, duration_seconds: None }).collect()
                    });
                }
                RecipeRecipeInstructionsFieldEnum::Text(text) => {
                    current_items.push(Item {
                        text,
                        duration_seconds: None,
                    });
                }
            }
        }

        if !current_items.is_empty() {
            sections.push(SectionItem {
                title: String::new(),
                items: current_items,
            });
        }

        if sections.is_empty() {
            Self::Items(Vec::new())
        } else if sections.len() == 1 && sections[0].title.is_empty() {
            Self::Items(sections.into_iter().next().unwrap().items)
        } else {
            Self::Sections(sections)
        }
    }
}

/// Represents a section in a recipe, typically used for organizing the recipe's
/// ingredients and instructions.
#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(super) struct Section {
    pub id: i64,
    pub name: String,
}

/// Represents the data required to insert a new section into the `sections` table.
#[derive(Insertable)]
#[diesel(table_name = schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(in crate::recipe) struct SectionForInsert {
    pub name: String,
}
