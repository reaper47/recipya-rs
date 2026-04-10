use std::{iter::FlatMap, slice::IterMut, vec};

use diesel::prelude::*;

use indexmap::IndexMap;
use itertools::Either;

use repository::schema;
use schema_org::{
    AtType, ItemList, at_context,
    field::{
        ItemListItemListElementFieldEnum, ItemListItemListOrderFieldEnum,
        PropertyValueValueFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SectionComponents {
    Grouped(Vec<SectionItem>),
    Flat(Vec<Item>),
}

impl SectionComponents {
    pub fn from_map(map: IndexMap<String, Vec<String>>) -> Self {
        if map.len() == 1
            && let Some(k) = map.keys().next()
            && k.is_empty()
        {
            map.into_iter()
                .next()
                .map(|(_, values)| Self::new(values))
                .unwrap_or_default()
        } else {
            Self::Grouped(
                map.into_iter()
                    .map(|(title, values)| {
                        SectionItem::new(title, values.into_iter().map(Item::new).collect())
                    })
                    .collect(),
            )
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SectionItem {
    pub title: String,
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
            Self::Grouped(iter) => iter.next(),
            Self::Flat(iter) => iter.next(),
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
            Self::Grouped(iter) => iter.next(),
            Self::Flat(iter) => iter.next(),
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
            Self::Grouped(iter) => iter.next(),
            Self::Flat(iter) => iter.next(),
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
    pub const fn is_empty(&self) -> bool {
        match self {
            Self::Grouped(section_items) => section_items.is_empty(),
            Self::Flat(items) => items.is_empty(),
        }
    }

    /// Assembles a vector of the items as text.
    pub fn items_as_text(&self) -> Vec<&str> {
        self.iter().map(|item| item.text.as_str()).collect()
    }

    /// Iterates over the items
    pub fn iter(&self) -> SectionComponentsIter<'_> {
        fn get_items(s: &SectionItem) -> std::slice::Iter<'_, Item> {
            s.items.iter()
        }
        match self {
            Self::Grouped(sections) => {
                SectionComponentsIter::Grouped(sections.iter().flat_map(get_items))
            }
            Self::Flat(items) => SectionComponentsIter::Flat(items.iter()),
        }
    }
    /// Iterates over the items mutably
    pub fn iter_mut(&mut self) -> SectionComponentsIterMut<'_> {
        fn get_items_mut(s: &mut SectionItem) -> std::slice::IterMut<'_, Item> {
            s.items.iter_mut()
        }

        match self {
            Self::Grouped(sections) => {
                SectionComponentsIterMut::Grouped(sections.iter_mut().flat_map(get_items_mut))
            }
            Self::Flat(items) => SectionComponentsIterMut::Flat(items.iter_mut()),
        }
    }

    /// Returns the number of items in the section.
    pub fn len(&self) -> usize {
        match self {
            Self::Grouped(sections) => sections.iter().map(|section| section.items.len()).sum(),
            Self::Flat(items) => items.len(),
        }
    }

    /// Returns an iterator over the sections variant.
    pub fn sections_iter(&self) -> Option<impl Iterator<Item = &SectionItem>> {
        match self {
            Self::Grouped(section_items) => Some(section_items.iter()),
            Self::Flat(_) => None,
        }
    }

    /// Returns the section titles.
    pub fn titles(&self) -> impl Iterator<Item = &str> {
        match self {
            Self::Grouped(section_items) => {
                Either::Left(section_items.iter().map(|section| section.title.as_str()))
            }
            Self::Flat(_) => Either::Right(std::iter::empty()),
        }
    }
}

impl<'a> IntoIterator for &'a SectionComponents {
    type Item = &'a Item;
    type IntoIter = SectionComponentsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut SectionComponents {
    type Item = &'a mut Item;
    type IntoIter = SectionComponentsIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl Default for SectionComponents {
    fn default() -> Self {
        Self::Grouped(vec![])
    }
}

impl TryFrom<Vec<SectionItem>> for SectionComponents {
    type Error = &'static str;

    fn try_from(mut sections: Vec<SectionItem>) -> Result<Self, Self::Error> {
        match sections.as_slice() {
            [] => Ok(Self::Flat(Vec::default())),
            [single] if single.title.is_empty() => sections
                .pop()
                .map(|section| Self::Flat(section.items))
                .ok_or("Failed to extract section"),
            _ => Ok(Self::Grouped(sections)),
        }
    }
}

impl TryFrom<Vec<RecipeRecipeIngredientFieldEnum>> for SectionComponents {
    type Error = &'static str;

    #[allow(clippy::too_many_lines)]
    fn try_from(items: Vec<RecipeRecipeIngredientFieldEnum>) -> Result<Self, Self::Error> {
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
                            PropertyValueValueFieldEnum::QuantitativeValue(q) => q
                                .value
                                .first()
                                .map(|v| match v {
                                    schema_org::field::FieldEnum64::BooleanEnumOrText(s) => {
                                        s.clone()
                                    }
                                    schema_org::field::FieldEnum64::Number(n) => n.to_string(),
                                    schema_org::field::FieldEnum64::StructuredValue(v) => {
                                        v.name.first().cloned().unwrap_or_default()
                                    }
                                    schema_org::field::FieldEnum64::QuantitativeValue(_) => {
                                        String::new()
                                    }
                                })
                                .unwrap_or_default(),
                        })
                        .unwrap_or_default();

                    let name = prop.name.first().cloned().unwrap_or_default();

                    let unit = {
                        let code = prop.unit_code.first().cloned().unwrap_or_default().clone();

                        let text = if let Some(s) = prop.value.first().map(|v| match v {
                            PropertyValueValueFieldEnum::QuantitativeValue(q) => {
                                q.unit_text.first().cloned().unwrap_or_default()
                            }
                            _ => String::new(),
                        }) {
                            s
                        } else {
                            prop.unit_text.first().cloned().unwrap_or_default().clone()
                        };

                        if code.is_empty() { text } else { code }
                    };

                    current_items.push(Item {
                        text: format!(
                            "{}{}{}",
                            if value.is_empty() {
                                String::new()
                            } else {
                                format!("{value} ")
                            },
                            if unit.is_empty() {
                                String::new()
                            } else {
                                format!("{unit} ")
                            },
                            if name.is_empty() { "" } else { &name }
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
            Ok(Self::Flat(Vec::new()))
        } else if sections.len() == 1 && sections[0].title.is_empty() {
            sections
                .into_iter()
                .next()
                .map(|section| Self::Flat(section.items))
                .ok_or("Failed to extract section")
        } else {
            Ok(Self::Grouped(sections))
        }
    }
}

impl From<SectionComponents> for Vec<RecipeRecipeInstructionsFieldEnum> {
    fn from(components: SectionComponents) -> Self {
        match components {
            SectionComponents::Flat(items) => items
                .into_iter()
                .map(|item| RecipeRecipeInstructionsFieldEnum::Text(item.text))
                .collect(),
            SectionComponents::Grouped(sections) => sections
                .into_iter()
                .map(|section| {
                    RecipeRecipeInstructionsFieldEnum::ItemList(Box::new(section.into()))
                })
                .collect(),
        }
    }
}

impl From<SectionComponents> for Vec<RecipeRecipeIngredientFieldEnum> {
    fn from(components: SectionComponents) -> Self {
        match components {
            SectionComponents::Flat(items) => items
                .into_iter()
                .map(|item| RecipeRecipeIngredientFieldEnum::Text(item.text))
                .collect(),
            SectionComponents::Grouped(sections) => sections
                .into_iter()
                .map(|section| RecipeRecipeIngredientFieldEnum::ItemList(section.into()))
                .collect(),
        }
    }
}

impl From<SectionItem> for ItemList {
    fn from(item: SectionItem) -> Self {
        let num_items = i32::try_from(item.items.len()).unwrap_or_default();

        Self {
            r#type: AtType::ItemList.to_string(),
            context: at_context(),
            item_list_element: item
                .items
                .into_iter()
                .map(|item| ItemListItemListElementFieldEnum::Text(item.text))
                .collect(),
            item_list_order: vec![ItemListItemListOrderFieldEnum::Text(
                "Ascending".to_string(),
            )],
            name: vec![item.title],
            number_of_items: vec![num_items],
            ..Default::default()
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
                RecipeRecipeInstructionsFieldEnum::HowToStep(how_to_step) => {
                    if let Some(text) = how_to_step.text.first().cloned() {
                        current_items.push(Item {
                            text,
                            duration_seconds: None,
                        });
                    }
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
            sections
                .first()
                .map(|section| Self::Flat(section.items.clone()))
                .unwrap_or_default()
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
            Self::Grouped(section_items) => SectionComponentsIntoIter::Grouped(
                section_items.into_iter().flat_map(extract_items),
            ),
            Self::Flat(items) => SectionComponentsIntoIter::Flat(items.into_iter()),
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

    #[must_use]
    pub const fn with_duration(mut self, duration_seconds: i32) -> Self {
        self.duration_seconds = Some(duration_seconds);
        self
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
