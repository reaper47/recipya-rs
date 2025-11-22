use std::{iter::FlatMap, slice::IterMut, vec};

use diesel::{Identifiable, Insertable, Queryable, Selectable};

use itertools::Either;
use repository::schema;
use schema_org::field::{
    ItemListItemListElementFieldEnum, PropertyValueValueFieldEnum, RecipeRecipeIngredientFieldEnum,
    RecipeRecipeInstructionsFieldEnum,
};

#[derive(Clone, Debug, PartialEq)]
pub enum SectionComponents {
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
    pub duration_seconds: Option<i32>,
}

type ItemIter<'a> = std::slice::Iter<'a, Item>;

pub enum SectionComponentsIter<'a> {
    Grouped(
        std::iter::FlatMap<
            std::slice::Iter<'a, SectionItem>,
            ItemIter<'a>,
            fn(&'a SectionItem) -> ItemIter<'a>,
        >,
    ),
    Flat(ItemIter<'a>),
}

impl<'a> Iterator for SectionComponentsIter<'a> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SectionComponentsIter::Grouped(iter) => iter.next(),
            SectionComponentsIter::Flat(iter) => iter.next(),
        }
    }
}

type ItemIntoIter = vec::IntoIter<Item>;

pub enum SectionComponentsIntoIter {
    Grouped(
        std::iter::FlatMap<
            vec::IntoIter<SectionItem>,
            ItemIntoIter,
            fn(SectionItem) -> ItemIntoIter,
        >,
    ),
    Flat(ItemIntoIter),
}

impl Iterator for SectionComponentsIntoIter {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SectionComponentsIntoIter::Grouped(iter) => iter.next(),
            SectionComponentsIntoIter::Flat(iter) => iter.next(),
        }
    }
}

type ItemIterMut<'a> = IterMut<'a, Item>;

type MapFn<'a> = fn(&'a mut SectionItem) -> ItemIterMut<'a>;

pub enum SectionComponentsIterMut<'a> {
    Grouped(FlatMap<IterMut<'a, SectionItem>, ItemIterMut<'a>, MapFn<'a>>),
    Flat(ItemIterMut<'a>),
}

impl<'a> Iterator for SectionComponentsIterMut<'a> {
    type Item = &'a mut Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SectionComponentsIterMut::Grouped(iter) => iter.next(),
            SectionComponentsIterMut::Flat(iter) => iter.next(),
        }
    }
}

impl SectionComponents {
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
            SectionComponents::Grouped(section_items) => section_items.is_empty(),
            SectionComponents::Flat(items) => items.is_empty(),
        }
    }

    /// Assembles a vector of the items as text.
    pub fn items_as_text(&self) -> Vec<String> {
        self.iter().map(|item| item.text.clone()).collect()
    }

    /// Iterates over the items
    pub fn iter(&self) -> SectionComponentsIter<'_> {
        fn get_items(s: &SectionItem) -> std::slice::Iter<'_, Item> {
            s.items.iter()
        }

        match self {
            SectionComponents::Grouped(sections) => {
                SectionComponentsIter::Grouped(sections.iter().flat_map(get_items))
            }
            SectionComponents::Flat(items) => SectionComponentsIter::Flat(items.iter()),
        }
    }

    /// Iterates over the items mutably
    pub fn iter_mut(&mut self) -> SectionComponentsIterMut<'_> {
        fn get_items_mut(s: &mut SectionItem) -> std::slice::IterMut<'_, Item> {
            s.items.iter_mut()
        }

        match self {
            SectionComponents::Grouped(sections) => {
                SectionComponentsIterMut::Grouped(sections.iter_mut().flat_map(get_items_mut))
            }
            SectionComponents::Flat(items) => SectionComponentsIterMut::Flat(items.iter_mut()),
        }
    }

    /// Returns the number of items in the section.
    pub fn len(&self) -> usize {
        match self {
            SectionComponents::Grouped(sections) => {
                sections.iter().map(|section| section.items.len()).sum()
            }
            SectionComponents::Flat(items) => items.len(),
        }
    }

    /// Returns an iterator over the sections variant.
    pub fn sections_iter(&self) -> Option<impl Iterator<Item = &SectionItem>> {
        match self {
            SectionComponents::Grouped(section_items) => Some(section_items.iter()),
            SectionComponents::Flat(_) => None,
        }
    }

    /// Returns the section titles.
    pub fn titles(&self) -> impl Iterator<Item = &str> {
        match self {
            SectionComponents::Grouped(section_items) => {
                Either::Left(section_items.iter().map(|section| section.title.as_str()))
            }
            SectionComponents::Flat(_) => Either::Right(std::iter::empty()),
        }
    }
}

impl Default for SectionComponents {
    fn default() -> Self {
        Self::Grouped(vec![])
    }
}

impl From<Vec<SectionItem>> for SectionComponents {
    fn from(mut sections: Vec<SectionItem>) -> Self {
        match sections.as_slice() {
            [] => Self::Flat(Default::default()),
            [single] if single.title.is_empty() => Self::Flat(sections.pop().unwrap().items),
            _ => Self::Grouped(sections),
        }
    }
}

impl From<Vec<RecipeRecipeIngredientFieldEnum>> for SectionComponents {
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

impl From<Vec<RecipeRecipeInstructionsFieldEnum>> for SectionComponents {
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

impl IntoIterator for SectionComponents {
    type Item = Item;
    type IntoIter = SectionComponentsIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        fn extract_items(s: SectionItem) -> vec::IntoIter<Item> {
            s.items.into_iter()
        }

        match self {
            SectionComponents::Grouped(section_items) => SectionComponentsIntoIter::Grouped(
                section_items.into_iter().flat_map(extract_items),
            ),
            SectionComponents::Flat(items) => SectionComponentsIntoIter::Flat(items.into_iter()),
        }
    }
}

impl SectionItem {
    /// Creates a new section item with the given title and items.
    pub fn new(title: impl Into<String>, items: Vec<Item>) -> Self {
        Self {
            title: title.into(),
            items,
        }
    }
}

impl Item {
    /// Creates a new item with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            duration_seconds: None,
        }
    }
}

/// Represents a section in a recipe, typically used for organizing the recipe's
/// ingredients and instructions.
#[allow(dead_code)]
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
