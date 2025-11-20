use diesel::{Identifiable, Insertable, Queryable, Selectable};

use repository::schema;
use schema_org::field::{
    ItemListItemListElementFieldEnum, PropertyValueValueFieldEnum, RecipeRecipeIngredientFieldEnum,
    RecipeRecipeInstructionsFieldEnum,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ComponentSections {
    Grouped(Vec<SectionItem>),
    Flat(Vec<Item>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SectionItem {
    pub title: String,
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub text: String,
    pub duration_seconds: Option<i16>,
}

impl ComponentSections {
    /// Creates a new `Sections::Items` from a list of text.
    pub fn new(items: Vec<String>) -> Self {
        Self::Flat(
            items
                .into_iter()
                .map(|item| Item {
                    text: item,
                    duration_seconds: None,
                })
                .collect(),
        )
    }

    /// Verifies whether there are items.
    pub fn is_empty(&self) -> bool {
        match self {
            ComponentSections::Grouped(section_items) => section_items.is_empty(),
            ComponentSections::Flat(items) => items.is_empty(),
        }
    }

    /// Assembles a vector of the items as text.
    pub fn items_as_text(&self) -> Vec<String> {
        self.iter().map(|item| item.text.clone()).collect()
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = &Item> + '_> {
        match self {
            ComponentSections::Grouped(sections) => {
                Box::new(sections.iter().flat_map(|s| s.items.iter()))
            }
            ComponentSections::Flat(items) => Box::new(items.iter()),
        }
    }

    /// Returns an iterator over the sections variant.
    pub fn sections_iter(&self) -> Option<impl Iterator<Item = &SectionItem>> {
        match self {
            ComponentSections::Grouped(section_items) => Some(section_items.iter()),
            ComponentSections::Flat(_) => None,
        }
    }
}

impl Default for ComponentSections {
    fn default() -> Self {
        Self::Grouped(vec![])
    }
}

impl From<Vec<SectionItem>> for ComponentSections {
    fn from(sections: Vec<SectionItem>) -> Self {
        match sections.as_slice() {
            [] => Self::Flat(Default::default()),
            [single] if single.title.is_empty() => {
                Self::Flat(sections.into_iter().next().unwrap().items)
            }
            _ => Self::Grouped(sections),
        }
    }
}

impl From<Vec<RecipeRecipeIngredientFieldEnum>> for ComponentSections {
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

                    let items = list
                        .item_list_element
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
                    let value = prop
                        .value
                        .first()
                        .map(|v| match v {
                            PropertyValueValueFieldEnum::BooleanEnumOrText(s) => s.clone(),
                            PropertyValueValueFieldEnum::Number(n) => n.to_string(),
                            PropertyValueValueFieldEnum::StructuredValue(v) => {
                                v.name.first().cloned().unwrap_or_default()
                            }
                        })
                        .unwrap_or_default();

                    let name = prop.name.first().cloned().unwrap_or_default();

                    let unit = {
                        let code = prop
                            .unit_code
                            .first()
                            .map(|s| s.to_string())
                            .unwrap_or_default()
                            .to_string();
                        let text = prop
                            .unit_text
                            .first()
                            .map(|s| s.to_string())
                            .unwrap_or_default()
                            .to_string();

                        if code.is_empty() { text } else { code }
                    };

                    current_items.push(Item {
                        text: format!(
                            "{}{}{}",
                            if !value.is_empty() {
                                format!("{} ", value)
                            } else {
                                "".to_string()
                            },
                            if !unit.is_empty() {
                                format!("{} ", unit)
                            } else {
                                "".to_string()
                            },
                            if !name.is_empty() { &name } else { "" }
                        ),
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
            Self::Flat(Vec::new())
        } else if sections.len() == 1 && sections[0].title.is_empty() {
            Self::Flat(sections.into_iter().next().unwrap().items)
        } else {
            Self::Grouped(sections)
        }
    }
}

impl From<Vec<RecipeRecipeInstructionsFieldEnum>> for ComponentSections {
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

                    let items = list
                        .item_list_element
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
                        items: work
                            .text
                            .into_iter()
                            .map(|text| Item {
                                text,
                                duration_seconds: None,
                            })
                            .collect(),
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
            Self::Flat(Vec::new())
        } else if sections.len() == 1 && sections[0].title.is_empty() {
            Self::Flat(sections.into_iter().next().unwrap().items)
        } else {
            Self::Grouped(sections)
        }
    }
}

impl IntoIterator for ComponentSections {
    type Item = Item;
    type IntoIter = Box<dyn Iterator<Item = Item>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ComponentSections::Grouped(section_items) => {
                Box::new(section_items.into_iter().flat_map(|s| s.items.into_iter()))
            }
            ComponentSections::Flat(items) => Box::new(items.into_iter()),
        }
    }
}

/// Represents a section in a recipe, typically used for organizing the recipe's
/// ingredients and instructions.
#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct Section {
    pub id: i64,
    pub name: String,
}

/// Represents the data required to insert a new section into the `sections` table.
#[derive(Insertable)]
#[diesel(table_name = schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct SectionForInsert {
    pub name: String,
}
