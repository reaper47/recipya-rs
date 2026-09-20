use std::{collections::HashMap, io::Write, mem::take, path::PathBuf};

use diesel::{dsl::exists, prelude::*, sql_types::Text};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use indexmap::IndexMap;
use krilla::{
    geom::{PathBuilder, Rect},
    metadata::{DateTime, Metadata, PageLayout},
    page::PageSettings,
    text::{Font, TextDirection},
};
use tempfile::{NamedTempFile, env::temp_dir};
use time::{PrimitiveDateTime, macros::format_description};
use tracing::error;
use uuid::Uuid;

use pdf::{
    components::{add_header, add_page_title},
    fonts::{ROBOTO_LIGHT_FONT_BYTES, ROBOTO_REGULAR_FONT_BYTES, ROBOTO_SEMIBOLD_FONT_BYTES},
    math::{measure_text_height, measure_text_width_pt},
};
use repository::{ModelManager, schema};

use crate::{
    Error, Result,
    export::{ExportOptions, ExportType},
    user::User,
};

const FONT_SIZE_BODY_PT: f32 = 11.0;
const FONT_SIZE_TITLE_PT: f32 = 14.0;
const ROW_STEP: f32 = 7.5 * 2.834_645_7;

/// Database representation of a shopping list.
#[derive(Clone, Debug, Eq, PartialEq, Queryable, Associations, Identifiable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::shopping_lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShoppingList {
    pub id: Uuid,
    pub name: String,
    pub num_items: i64,
    pub user_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Associations, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = schema::shopping_lists)]
struct ShoppingListForInsert {
    name: String,
    user_id: Uuid,
}

/// Represents a shopping list label.
#[derive(Debug, Eq, PartialEq, Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::shopping_list_labels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct ShoppingListLabel {
    id: i64,
    name: String,
}

#[derive(AsChangeset, Insertable, Queryable, Selectable)]
#[diesel(table_name = schema::shopping_list_labels)]
struct ShoppingListLabelForInsert<'a> {
    name: &'a str,
}

/// Represents a shopping list item.
#[derive(Debug, Eq, PartialEq, Queryable, Associations, Identifiable, Selectable)]
#[diesel(belongs_to(ShoppingList), belongs_to(ShoppingListLabel))]
#[diesel(table_name = schema::shopping_list_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShoppingListItem {
    pub id: i64,
    pub shopping_list_id: Uuid,
    pub ingredient: String,
    pub quantity: Option<String>,
    pub notes: Option<String>,
    pub shopping_list_label_id: i64,
    pub position: i32,
    pub is_checked: bool,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

/// Represents a shopping list item for creation.
pub struct ShoppingListItemForCreate {
    pub ingredient: String,
    pub quantity: Option<String>,
    pub label: Option<String>,
    pub recipe_id: Option<i64>,
    pub notes: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::shopping_list_items)]
struct ShoppingListItemForInsert<'a> {
    shopping_list_id: Uuid,
    ingredient: &'a str,
    quantity: Option<&'a str>,
    notes: Option<&'a str>,
    shopping_list_label_id: Option<i64>,
}

/// Represents a shopping list item for update.
#[derive(Debug, Default)]
pub struct ShoppingListItemForUpdate {
    pub ingredient: Option<String>,
    pub quantity: Option<String>,
    pub label: Option<String>,
    pub notes: Option<String>,
    pub position: Option<i32>,
    pub is_checked: Option<bool>,
}

impl ShoppingListItemForUpdate {
    /// Creates a new item for update only for the checked state.
    pub fn new_checked(state: bool) -> Self {
        Self {
            is_checked: Some(state),
            ..Default::default()
        }
    }
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = schema::shopping_list_items)]
struct ShoppingListItemForUpdateInternal<'a> {
    ingredient: Option<&'a str>,
    quantity: Option<&'a str>,
    notes: Option<&'a str>,
    shopping_list_label_id: Option<i64>,
    position: Option<i32>,
    is_checked: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::shopping_list_recipes)]
struct ShoppingListRecipeForInsert {
    shopping_list_item_id: i64,
    recipe_id: i64,
}

#[derive(Associations, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(ShoppingListLabel, foreign_key = label_id))]
#[diesel(table_name = schema::users_shopping_list_labels)]
struct UserShoppingListLabelForInsert {
    user_id: Uuid,
    label_id: i64,
}

/// Represents a shopping list with its details.
#[derive(Debug, PartialEq, Eq)]
pub struct ShoppingListDetails {
    pub id: Uuid,
    pub name: String,
    pub items: Vec<ShoppingListItemDetails>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

enum DrawCommand {
    Header,
    Title { text: String, y: f32 },
    SectionLabel { text: String, y: f32 },
    Checkbox { y: f32 },
    ItemText { text: String, y: f32 },
    NotesText { text: String, y: f32 },
}

impl ShoppingListDetails {
    /// Returns a map of items grouped by their label.
    pub fn items_per_label(&self) -> IndexMap<&str, Vec<&ShoppingListItemDetails>> {
        let mut map: IndexMap<&str, Vec<&ShoppingListItemDetails>> = IndexMap::new();

        for item in &self.items {
            map.entry(&item.label)
                .and_modify(|v| v.push(item))
                .or_insert(vec![item]);
        }

        map
    }

    /// Exports the shopping list to a file using the given writer.
    pub fn export(&self, format: &ExportType, options: Option<ExportOptions>) -> Result<PathBuf> {
        if self.items.is_empty() {
            return Err(Error::EmptyInput);
        }

        let file = NamedTempFile::new()?;

        let mut writer = std::io::BufWriter::new(file);
        match format {
            ExportType::Markdown => self.write_markdown(&mut writer)?,
            ExportType::Text => self.write_text(&mut writer)?,
            ExportType::Pdf => self.write_pdf(&mut writer, options)?,
            ExportType::Json => unimplemented!(),
        }
        {}

        writer.flush()?;

        let named = temp_dir().join(format!("{}.{}", self.name, format.extension()));

        let temp_path = writer
            .into_inner()
            .map_err(|err| Error::File(err.to_string()))?
            .into_temp_path();

        temp_path
            .persist(&named)
            .map_err(|err| Error::File(err.to_string()))?;

        Ok(named)
    }

    /// Writes the shopping list in text format to the given writer.
    pub fn write_text(&self, writer: &mut impl Write) -> Result<()> {
        if self.items.is_empty() {
            return Err(Error::EmptyInput);
        }

        writeln!(writer, "{}", self.name)?;
        writeln!(writer, "{}", "-".repeat(self.name.chars().count()))?;
        for (label, items) in &self.items_per_label() {
            writeln!(writer)?;
            if label != &"No label" {
                writeln!(writer, "[{label}]")?;
            }

            for item in items {
                write!(writer, "- {}", item.ingredient)?;
                if let Some(q) = &item.quantity {
                    write!(writer, " ({q})")?;
                }

                if let Some(recipe) = &item.recipe {
                    write!(writer, " | {}", recipe.name)?;
                }

                if let Some(notes) = &item.notes {
                    writeln!(writer)?;
                    write!(writer, "\t*{notes}")?;
                }

                writeln!(writer)?;
            }
        }

        Ok(())
    }

    /// Writes the shopping list in Markdown format to the given writer.
    pub fn write_markdown(&self, writer: &mut impl Write) -> Result<()> {
        if self.items.is_empty() {
            return Err(Error::EmptyInput);
        }

        writeln!(writer, "## {}", self.name)?;
        for (label, items) in &self.items_per_label() {
            writeln!(writer)?;
            if label != &"No label" {
                writeln!(writer, "### {label}")?;
                writeln!(writer)?;
            }

            for item in items {
                write!(writer, "- [ ] {}", item.ingredient)?;
                if let Some(q) = &item.quantity {
                    write!(writer, " ({q})")?;
                }

                if let Some(recipe) = &item.recipe {
                    write!(writer, " | **{}**", recipe.name)?;
                }

                if let Some(notes) = &item.notes {
                    writeln!(writer)?;
                    write!(writer, "\t* {notes}")?;
                }

                writeln!(writer)?;
            }
        }

        Ok(())
    }

    /// Writes the shopping list as a PDF file.
    ///
    /// # Panics
    ///
    /// Panics if the fonts are not available.
    ///
    #[allow(clippy::too_many_lines)]
    pub fn write_pdf(&self, writer: &mut impl Write, options: Option<ExportOptions>) -> Result<()> {
        if self.items.is_empty() {
            return Err(Error::EmptyInput);
        }

        let (width, height) = options.map_or((612.0, 792.0), |o| o.paper_size);
        let font_light = Font::new(ROBOTO_LIGHT_FONT_BYTES.into(), 0).unwrap();
        let font_regular = Font::new(ROBOTO_REGULAR_FONT_BYTES.into(), 0).unwrap();
        let font_semibold = Font::new(ROBOTO_SEMIBOLD_FONT_BYTES.into(), 0).unwrap();

        let y_title = measure_text_height(FONT_SIZE_TITLE_PT, 1.25, 1) + 5.0;
        let y_body_1_25 = measure_text_height(FONT_SIZE_BODY_PT, 1.25, 1);
        let y_body_1_00 = measure_text_height(FONT_SIZE_BODY_PT, 1.0, 1);
        let y_small_1_00 = measure_text_height(FONT_SIZE_BODY_PT - 2.0, 1.0, 1);
        let y_small_0_50 = measure_text_height(FONT_SIZE_BODY_PT - 2.0, 0.50, 1);

        // Prepare the pages.
        let mut pages: Vec<Vec<DrawCommand>> = Vec::new();
        let mut current_page: Vec<DrawCommand> = Vec::new();
        let mut curr_y = 72.0;

        let start_new_page = |current_page: &mut Vec<DrawCommand>,
                              pages: &mut Vec<Vec<DrawCommand>>,
                              curr_y: &mut f32| {
            let finished = take(current_page);
            pages.push(finished);
            current_page.push(DrawCommand::Header);
            *curr_y = 72.0;
        };

        current_page.push(DrawCommand::Header);
        current_page.push(DrawCommand::Title {
            text: self.name.clone(),
            y: curr_y,
        });
        curr_y += y_title;

        for (idx, (&section, items)) in self.items_per_label().iter().enumerate() {
            if ROW_STEP.mul_add(3.0, curr_y) >= (height - 72.0) {
                start_new_page(&mut current_page, &mut pages, &mut curr_y);
            }

            if section != "No label" {
                if idx > 0 {
                    curr_y += 20.0;
                }

                current_page.push(DrawCommand::SectionLabel {
                    text: section.into(),
                    y: curr_y,
                });

                curr_y += y_body_1_25;
            }

            for item in items {
                let num_rows = if item.notes.is_some() && item.recipe.is_some() {
                    3.0
                } else if item.notes.is_some() || item.recipe.is_some() {
                    2.0
                } else {
                    1.0
                };

                if ROW_STEP.mul_add(num_rows, curr_y) >= (height - 72.0) {
                    start_new_page(&mut current_page, &mut pages, &mut curr_y);
                } else {
                    curr_y += y_body_1_00;
                }

                current_page.push(DrawCommand::Checkbox { y: curr_y });
                current_page.push(DrawCommand::ItemText {
                    text: item.quantity.as_ref().map_or_else(
                        || item.ingredient.clone(),
                        |q| format!("{} ({q})", item.ingredient),
                    ),
                    y: curr_y,
                });
                curr_y += y_body_1_00;

                if let Some(notes) = &item.notes {
                    current_page.push(DrawCommand::NotesText {
                        text: notes.clone(),
                        y: curr_y + 2.5,
                    });
                    curr_y += y_small_1_00;
                }

                if let Some(recipe) = &item.recipe {
                    if item.notes.is_some() {
                        curr_y += y_small_0_50;
                    }

                    current_page.push(DrawCommand::NotesText {
                        text: format!("For recipe: {}", recipe.name),
                        y: curr_y + 2.5,
                    });
                    curr_y += y_small_1_00;
                }
            }
        }

        pages.push(current_page);
        let num_pages = pages.len();

        // Render the pages
        let mut doc = krilla::Document::new();

        doc.set_metadata(
            Metadata::new()
                .title(format!("Shopping List - {}", self.name))
                .creation_date({
                    let now = time::UtcDateTime::now();
                    DateTime::new(u16::try_from(now.year()).unwrap_or_default())
                        .day(now.day())
                        .month(now.month() as u8)
                        .hour(now.hour())
                        .minute(now.minute())
                        .second(now.second())
                })
                .creator("Recipya".into())
                .description(format!("PDF export of the '{}' shopping list", self.name))
                .page_layout(PageLayout::SinglePage)
                .language("en".into()),
        );

        for (idx, commands) in pages.into_iter().enumerate() {
            let page_num = idx + 1;
            let mut page =
                doc.start_page_with(PageSettings::from_wh(width, height).unwrap_or_default());
            let mut surface = page.surface();

            for cmd in commands {
                match cmd {
                    DrawCommand::Header => add_header(
                        Some("Recipya shopping list"),
                        Some(
                            &self
                                .updated_at
                                .format(&format_description!("[year]-[month]-[day]"))
                                .unwrap(),
                        ),
                        &mut surface,
                        (
                            &font_regular,
                            ROBOTO_REGULAR_FONT_BYTES,
                            FONT_SIZE_TITLE_PT / 2.0,
                        ),
                        72.0,
                        width,
                    ),
                    DrawCommand::Title { text, y } => add_page_title(
                        &text,
                        (&font_regular, ROBOTO_REGULAR_FONT_BYTES, FONT_SIZE_TITLE_PT),
                        &mut surface,
                        y,
                        width,
                    ),
                    DrawCommand::SectionLabel { text, y } => surface.draw_text(
                        krilla::geom::Point::from_xy(72.0, y),
                        font_semibold.clone(),
                        FONT_SIZE_BODY_PT,
                        &text,
                        false,
                        TextDirection::Auto,
                    ),
                    DrawCommand::Checkbox { y } => {
                        surface.set_stroke(Some(krilla::paint::Stroke {
                            paint: krilla::color::rgb::Color::new(0, 0, 0).into(),
                            width: FONT_SIZE_BODY_PT * 0.06,
                            ..Default::default()
                        }));
                        let mut path = PathBuilder::new();
                        path.push_rect(
                            Rect::from_ltrb(72.0, y - 8.0, 72.0 + 7.5, y - 0.5).unwrap(),
                        );
                        surface.draw_path(&path.finish().unwrap());
                        surface.set_stroke(None);
                    }
                    DrawCommand::ItemText { text, y } => surface.draw_text(
                        krilla::geom::Point::from_xy(72.0 + 12.5, y),
                        font_regular.clone(),
                        FONT_SIZE_BODY_PT,
                        &text,
                        false,
                        TextDirection::Auto,
                    ),
                    DrawCommand::NotesText { text, y } => surface.draw_text(
                        krilla::geom::Point::from_xy(72.0 + 12.5, y),
                        font_light.clone(),
                        FONT_SIZE_BODY_PT - 2.0,
                        &format!("    * {text}"),
                        false,
                        TextDirection::Auto,
                    ),
                }
            }

            let footer_text = format!("Page {page_num} of {num_pages}");
            let footer_width = measure_text_width_pt(
                &footer_text,
                &font_regular,
                ROBOTO_REGULAR_FONT_BYTES,
                0,
                FONT_SIZE_BODY_PT - 3.0,
            );
            surface.draw_text(
                krilla::geom::Point::from_xy((width - footer_width) / 2.0, height - 20.0),
                font_regular.clone(),
                FONT_SIZE_BODY_PT - 3.0,
                &footer_text,
                false,
                TextDirection::Auto,
            );

            surface.finish();
            page.finish();
        }

        let bytes = doc
            .finish()
            .inspect_err(|err| error!(?err, "Failed to create shopping list PDF"))
            .unwrap_or_default();

        writer.write_all(&bytes)?;
        Ok(())
    }
}

/// Represents a shopping list item with its details.
#[derive(Debug, PartialEq, Eq)]
pub struct ShoppingListItemDetails {
    pub id: i64,
    pub ingredient: String,
    pub quantity: Option<String>,
    pub notes: Option<String>,
    pub label_id: i64,
    pub label: String,
    pub position: i32,
    pub recipe: Option<ShoppingListRecipeDetails>,
    pub is_checked: bool,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

/// Represents a shared recipe
#[derive(Debug, Eq, PartialEq, Queryable, Identifiable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(ShoppingList))]
#[diesel(table_name = schema::shares_shopping_lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShareShoppingList {
    pub id: i64,
    pub link: Uuid,
    pub user_id: Uuid,
    pub list_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub expires_at: PrimitiveDateTime,
    pub last_accessed: PrimitiveDateTime,
    pub click_count: i32,
}

/// A struct for inserting a new shared recipe into the database.
#[derive(Insertable)]
#[diesel(table_name = schema::shares_shopping_lists)]
pub(crate) struct ShareShoppingListForInsert {
    pub user_id: Uuid,
    pub list_id: Uuid,
    pub expires_at: Option<PrimitiveDateTime>,
}

/// Represents a recipe with its details for a shopping list item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShoppingListRecipeDetails {
    pub id: i64,
    pub name: String,
}

#[derive(QueryableByName)]
struct IdRow {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    id: i64,
}

#[derive(QueryableByName)]
struct QueryableLabel {
    #[diesel(sql_type = diesel::sql_types::Int8)]
    id: i64,
}

impl ShoppingList {
    /// Creates a new shopping list with the given title for the given user.
    pub async fn create(mm: &ModelManager, title: &str, user_id: Uuid) -> Result<Uuid> {
        Ok(diesel::insert_into(schema::shopping_lists::table)
            .values(&ShoppingListForInsert {
                name: title.into(),
                user_id,
            })
            .returning(schema::shopping_lists::id)
            .get_result::<Uuid>(&mut mm.pool.get().await?)
            .await?)
    }

    /// Gets a shopping list by its ID and user ID.
    pub async fn get(mm: &ModelManager, list_id: Uuid, user_id: Uuid) -> Result<Self> {
        Ok(schema::shopping_lists::table
            .filter(schema::shopping_lists::id.eq(list_id))
            .filter(schema::shopping_lists::user_id.eq(user_id))
            .first::<Self>(&mut mm.pool.get().await?)
            .await?)
    }

    /// Gets all of the user's shopping lists.
    pub async fn get_all(mm: &ModelManager, user_id: Uuid) -> Result<Vec<Self>> {
        Ok(schema::shopping_lists::table
            .filter(schema::shopping_lists::user_id.eq(user_id))
            .order(schema::shopping_lists::created_at.desc())
            .load::<Self>(&mut mm.pool.get().await?)
            .await?)
    }

    /// Adds an item to a shopping list.
    pub async fn add_item(
        mm: &ModelManager,
        list_id: Uuid,
        item_c: ShoppingListItemForCreate,
        user_id: Uuid,
    ) -> Result<ShoppingListItemDetails> {
        let mut conn = mm.pool.get().await?;

        Self::verify_ownership(&mut conn, list_id, user_id).await?;

        let label_id = if let Some(label) = item_c
            .label
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            Some(Self::upsert_label_for_user(&mut conn, label, user_id).await?)
        } else {
            None
        };

        let item: ShoppingListItem = diesel::insert_into(schema::shopping_list_items::table)
            .values(&ShoppingListItemForInsert {
                shopping_list_id: list_id,
                ingredient: item_c.ingredient.as_str(),
                quantity: item_c.quantity.as_deref(),
                notes: item_c.notes.as_deref(),
                shopping_list_label_id: label_id,
            })
            .returning(ShoppingListItem::as_select())
            .get_result(&mut conn)
            .await?;

        if let Some(recipe_id) = item_c.recipe_id {
            diesel::insert_into(schema::shopping_list_recipes::table)
                .values(&ShoppingListRecipeForInsert {
                    shopping_list_item_id: item.id,
                    recipe_id,
                })
                .execute(&mut conn)
                .await?;
        }

        drop(conn);

        Ok(ShoppingListItemDetails {
            id: item.id,
            ingredient: item.ingredient,
            quantity: item.quantity,
            notes: item.notes,
            label_id: label_id.unwrap_or(1),
            label: item_c.label.unwrap_or_else(|| "No label".to_string()),
            position: item.position,
            recipe: None,
            is_checked: false,
            created_at: item.created_at,
            updated_at: item.updated_at,
        })
    }

    /// Batch insert items into a shopping list.
    pub async fn add_items_for_recipe(
        &self,
        mm: &ModelManager,
        items_c: &[ShoppingListItemForCreate],
        recipe_id: i64,
        user_id: Uuid,
    ) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        Self::verify_ownership(&mut conn, self.id, user_id).await?;

        let values = items_c
            .iter()
            .map(|item| ShoppingListItemForInsert {
                shopping_list_id: self.id,
                ingredient: item.ingredient.as_str(),
                quantity: item.quantity.as_deref(),
                notes: item.notes.as_deref(),
                shopping_list_label_id: None,
            })
            .collect::<Vec<_>>();

        let values = diesel::insert_into(schema::shopping_list_items::table)
            .values(&values)
            .on_conflict_do_nothing()
            .get_results::<ShoppingListItem>(&mut conn)
            .await?
            .into_iter()
            .map(|item| ShoppingListRecipeForInsert {
                shopping_list_item_id: item.id,
                recipe_id,
            })
            .collect::<Vec<_>>();

        diesel::insert_into(schema::shopping_list_recipes::table)
            .values(&values)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Deletes a shopping list.
    pub async fn delete(mm: &ModelManager, list_id: Uuid, user_id: Uuid) -> Result<()> {
        diesel::delete(schema::shopping_lists::table)
            .filter(schema::shopping_lists::id.eq(list_id))
            .filter(schema::shopping_lists::user_id.eq(user_id))
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Deletes an item from a shopping list.
    pub async fn delete_item(
        mm: &ModelManager,
        list_id: Uuid,
        item_id: i64,
        user_id: Uuid,
    ) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        Self::verify_ownership(&mut conn, list_id, user_id).await?;

        diesel::delete(schema::shopping_list_items::table)
            .filter(schema::shopping_list_items::id.eq(item_id))
            .filter(schema::shopping_list_items::shopping_list_id.eq(list_id))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Gets the number of items in a shopping list.
    pub async fn items_count(mm: &ModelManager, list_id: Uuid) -> Result<i64> {
        let count = schema::shopping_list_items::table
            .filter(schema::shopping_list_items::shopping_list_id.eq(list_id))
            .count()
            .get_result(&mut mm.pool.get().await?)
            .await?;

        Ok(count)
    }

    /// Gets the details of an item.
    pub async fn get_item(
        mm: &ModelManager,
        list_id: Uuid,
        item_id: i64,
        user_id: Uuid,
    ) -> Result<ShoppingListItemDetails> {
        let mut conn = mm.pool.get().await?;

        Self::verify_ownership(&mut conn, list_id, user_id).await?;

        let (item, label) = schema::shopping_list_items::table
            .inner_join(
                schema::shopping_list_labels::table
                    .on(schema::shopping_list_items::shopping_list_label_id
                        .eq(schema::shopping_list_labels::id)),
            )
            .filter(schema::shopping_list_items::shopping_list_id.eq(list_id))
            .filter(schema::shopping_list_items::id.eq(item_id))
            .select((
                ShoppingListItem::as_select(),
                schema::shopping_list_labels::name,
            ))
            .first::<(ShoppingListItem, String)>(&mut conn)
            .await
            .map_err(|_| Error::EntityNotFound {
                entity: "shopping_list_item",
                id: item_id.to_string(),
            })?;

        drop(conn);

        Ok(ShoppingListItemDetails {
            id: item.id,
            ingredient: item.ingredient,
            quantity: item.quantity,
            notes: item.notes,
            label_id: item.shopping_list_label_id,
            label,
            position: item.position,
            recipe: None,
            is_checked: item.is_checked,
            created_at: item.created_at,
            updated_at: item.updated_at,
        })
    }

    /// Returns the label of a shopping list item.
    pub async fn label(mm: &ModelManager, label_id: i64) -> Result<String> {
        let label = schema::shopping_list_labels::table
            .filter(schema::shopping_list_labels::id.eq(label_id))
            .select(schema::shopping_list_labels::name)
            .first::<String>(&mut mm.pool.get().await?)
            .await
            .map_err(|_| Error::EntityNotFound {
                entity: "shopping_list_label",
                id: label_id.to_string(),
            })?;

        Ok(label)
    }

    /// Returns all of the user's labels.
    pub async fn labels(mm: &ModelManager, user_id: Uuid) -> Result<Vec<String>> {
        let labels = schema::users_shopping_list_labels::table
            .inner_join(schema::shopping_list_labels::table.on(
                schema::shopping_list_labels::id.eq(schema::users_shopping_list_labels::label_id),
            ))
            .filter(schema::users_shopping_list_labels::user_id.eq(user_id))
            .select(schema::shopping_list_labels::name)
            .distinct()
            .order(schema::shopping_list_labels::name.asc())
            .load::<String>(&mut mm.pool.get().await?)
            .await?;

        Ok(labels)
    }

    /// Gets or inserts a label by name, returning the label ID.
    pub async fn get_or_insert_label<T: AsRef<str>>(
        mm: &ModelManager,
        name: T,
        user_id: Uuid,
    ) -> Result<i64> {
        let mut conn = mm.pool.get().await?;

        let mut name = name.as_ref();
        if name.is_empty() {
            name = "No label";
        }

        let label_id = diesel::sql_query(
            "INSERT INTO shopping_list_labels (name) VALUES ($1)
                ON CONFLICT (lower(name)) DO UPDATE SET name = EXCLUDED.name
                RETURNING id",
        )
        .bind::<diesel::sql_types::Text, _>(name)
        .get_result::<QueryableLabel>(&mut conn)
        .await?
        .id;

        diesel::insert_into(schema::users_shopping_list_labels::table)
            .values((
                schema::users_shopping_list_labels::user_id.eq(user_id),
                schema::users_shopping_list_labels::label_id.eq(label_id),
            ))
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .await?;

        Ok(label_id)
    }

    /// Creates a new label by name, returning the label ID.
    pub async fn new_label<T: AsRef<str>>(
        mm: &ModelManager,
        name: T,
        list_id: Uuid,
        user_id: Uuid,
    ) -> Result<i64> {
        if name.as_ref().is_empty() {
            return Err(Error::EmptyInput);
        }

        let name = name.as_ref();

        mm.pool
            .get()
            .await?
            .transaction::<i64, Error, _>(async |conn| {
                let maybe_id = diesel::insert_into(schema::shopping_list_labels::table)
                    .values(schema::shopping_list_labels::name.eq(name))
                    .on_conflict(diesel::dsl::sql::<Text>("(lower(name))"))
                    .do_nothing()
                    .returning(schema::shopping_list_labels::id)
                    .get_result::<i64>(conn)
                    .await
                    .optional()?;

                let label_id = match maybe_id {
                    Some(id) => id,
                    None => {
                        schema::shopping_list_labels::table
                            .filter(
                                diesel::dsl::sql::<diesel::sql_types::Bool>("lower(name) = lower(")
                                    .bind::<diesel::sql_types::Text, _>(name)
                                    .sql(")"),
                            )
                            .select(schema::shopping_list_labels::id)
                            .get_result::<i64>(conn)
                            .await?
                    }
                };

                let is_label_in_list: bool = diesel::select(exists(
                    schema::shopping_list_items::table
                        .filter(schema::shopping_list_items::shopping_list_id.eq(list_id))
                        .filter(schema::shopping_list_items::shopping_list_label_id.eq(label_id)),
                ))
                .get_result(conn)
                .await?;

                if is_label_in_list {
                    return Err(Error::DuplicateEntity);
                }

                diesel::insert_into(schema::users_shopping_list_labels::table)
                    .values((
                        schema::users_shopping_list_labels::user_id.eq(user_id),
                        schema::users_shopping_list_labels::label_id.eq(label_id),
                    ))
                    .on_conflict_do_nothing()
                    .execute(conn)
                    .await?;

                Ok(label_id)
            })
            .await
    }

    /// Toggles the checked state of an item in a shopping list.
    pub async fn toggle_item_check(mm: &ModelManager, item_id: i64) -> Result<()> {
        diesel::update(schema::shopping_list_items::table)
            .filter(schema::shopping_list_items::id.eq(item_id))
            .set(
                schema::shopping_list_items::is_checked
                    .eq(diesel::dsl::not(schema::shopping_list_items::is_checked)),
            )
            .execute(&mut mm.pool.get().await?)
            .await?;

        Ok(())
    }

    /// Updates the title of a shopping list.
    pub async fn update_title<T: AsRef<str>>(
        mm: &ModelManager,
        list_id: Uuid,
        title: T,
        user_id: Uuid,
    ) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        Self::verify_ownership(&mut conn, list_id, user_id).await?;

        let is_title_exists = diesel::select(diesel::dsl::exists(
            schema::shopping_lists::table
                .filter(schema::shopping_lists::user_id.eq(user_id))
                .filter(schema::shopping_lists::name.eq(title.as_ref()))
                .filter(schema::shopping_lists::id.ne(list_id)),
        ))
        .get_result::<bool>(&mut conn)
        .await?;

        if is_title_exists {
            return Err(Error::NameExists);
        }

        diesel::update(schema::shopping_lists::table)
            .filter(schema::shopping_lists::id.eq(list_id))
            .set(schema::shopping_lists::name.eq(title.as_ref()))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Updates the labels of an item in a shopping list.
    pub async fn update_item_labels(
        mm: &ModelManager,
        list_id: Uuid,
        old_label_id: i64,
        new_label_id: i64,
        user_id: Uuid,
    ) -> Result<()> {
        let mut conn = mm.pool.get().await?;

        Self::verify_ownership(&mut conn, list_id, user_id).await?;

        let _ = diesel::update(schema::shopping_list_items::table)
            .filter(schema::shopping_list_items::shopping_list_label_id.eq(old_label_id))
            .set(schema::shopping_list_items::shopping_list_label_id.eq(new_label_id))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Updates an item in a shopping list.
    pub async fn update_item(
        mm: &ModelManager,
        list_id: Uuid,
        item_id: i64,
        item_u: ShoppingListItemForUpdate,
        user_id: Uuid,
    ) -> Result<ShoppingListItem> {
        let mut conn = mm.pool.get().await?;

        Self::verify_ownership(&mut conn, list_id, user_id).await?;

        let label_id = if let Some(label) = item_u
            .label
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            Some(Self::upsert_label_for_user(&mut conn, label, user_id).await?)
        } else {
            None
        };

        let item = diesel::update(schema::shopping_list_items::table)
            .filter(schema::shopping_list_items::id.eq(item_id))
            .set(&ShoppingListItemForUpdateInternal {
                ingredient: item_u.ingredient.as_deref(),
                quantity: item_u.quantity.filter(|s| !s.is_empty()).as_deref(),
                notes: item_u.notes.as_deref(),
                shopping_list_label_id: label_id,
                position: item_u.position,
                is_checked: item_u.is_checked,
            })
            .get_result::<ShoppingListItem>(&mut conn)
            .await?;

        Ok(item)
    }

    /// Updates the positions of items in a shopping list.
    pub async fn update_item_positions(
        mm: &ModelManager,
        list_id: Uuid,
        values: HashMap<i64, i32>,
        user_id: Uuid,
    ) -> Result<()> {
        let mut conn = mm.pool.get().await?;
        Self::verify_ownership(&mut conn, list_id, user_id).await?;

        let mut sorted = values.into_iter().collect::<Vec<_>>();
        sorted.sort_by_key(|(id, _)| *id);

        conn.transaction::<_, Error, _>(async |conn| {
            for (item_id, position) in sorted {
                diesel::update(schema::shopping_list_items::table)
                    .filter(schema::shopping_list_items::id.eq(item_id))
                    .set(schema::shopping_list_items::position.eq(position))
                    .execute(conn)
                    .await?;
            }
            Ok(())
        })
        .await?;

        Ok(())
    }

    async fn verify_ownership(
        conn: &mut AsyncPgConnection,
        list_id: Uuid,
        user_id: Uuid,
    ) -> Result<()> {
        let exists: bool = diesel::select(diesel::dsl::exists(
            schema::shopping_lists::table
                .filter(schema::shopping_lists::id.eq(list_id))
                .filter(schema::shopping_lists::user_id.eq(user_id)),
        ))
        .get_result(conn)
        .await?;

        if !exists {
            return Err(Error::EntityNotFound {
                id: format!("(list_id: {list_id}, user_id: {user_id})"),
                entity: "shopping_list",
            });
        }

        Ok(())
    }

    async fn upsert_label_for_user(
        conn: &mut diesel_async::AsyncPgConnection,
        label_name: &str,
        user_id: Uuid,
    ) -> Result<i64> {
        let label_id: i64 = diesel::sql_query(
            "INSERT INTO shopping_list_labels (name)
             VALUES ($1)
             ON CONFLICT (lower(name)) DO UPDATE SET name = EXCLUDED.name
             RETURNING id",
        )
        .bind::<diesel::sql_types::Text, _>(label_name)
        .get_result::<IdRow>(conn)
        .await?
        .id;

        diesel::insert_into(schema::users_shopping_list_labels::table)
            .values(&UserShoppingListLabelForInsert { user_id, label_id })
            .on_conflict_do_nothing()
            .execute(conn)
            .await?;

        Ok(label_id)
    }
}

impl ShoppingListDetails {
    /// Gets the details of a shopping list.
    pub async fn get(mm: &ModelManager, list_id: Uuid, user_id: Uuid) -> Result<Self> {
        let mut conn = mm.pool.get().await?;

        let list = schema::shopping_lists::table
            .filter(schema::shopping_lists::id.eq(list_id))
            .filter(schema::shopping_lists::user_id.eq(user_id))
            .select(ShoppingList::as_select())
            .first(&mut conn)
            .await?;

        let items = schema::shopping_list_items::table
            .filter(schema::shopping_list_items::shopping_list_id.eq(list_id))
            .inner_join(schema::shopping_list_labels::table)
            .left_join(schema::shopping_list_recipes::table.left_join(schema::recipes::table))
            .order((
                schema::shopping_list_items::shopping_list_label_id.asc(),
                schema::shopping_list_items::position.asc(),
            ))
            .select((
                ShoppingListItem::as_select(),
                schema::shopping_list_labels::name,
                schema::recipes::id.nullable(),
                schema::recipes::name.nullable(),
            ))
            .load::<(ShoppingListItem, String, Option<i64>, Option<String>)>(&mut conn)
            .await?
            .into_iter()
            .map(
                |(item, label_name, recipe_id, recipe_name)| ShoppingListItemDetails {
                    id: item.id,
                    ingredient: item.ingredient,
                    quantity: item.quantity,
                    notes: item.notes,
                    position: item.position,
                    label_id: item.shopping_list_label_id,
                    label: label_name,
                    is_checked: item.is_checked,
                    recipe: recipe_id
                        .zip(recipe_name)
                        .map(|(id, name)| ShoppingListRecipeDetails { id, name }),
                    created_at: item.created_at,
                    updated_at: item.updated_at,
                },
            )
            .collect::<Vec<_>>();

        Ok(Self {
            id: list.id,
            name: list.name,
            items,
            created_at: list.created_at,
            updated_at: list.updated_at,
        })
    }
}

impl ShoppingListItemForCreate {
    /// Creates a new `ShoppingListItemForCreate` with the given quantity, ingredient, label, and recipe ID.
    pub fn new(
        quantity: Option<String>,
        ingredient: impl Into<String>,
        label: Option<String>,
        notes: Option<String>,
        recipe_id: Option<i64>,
    ) -> Self {
        Self {
            ingredient: ingredient.into(),
            quantity,
            label,
            notes,
            recipe_id,
        }
    }
}

impl ShareShoppingList {
    /// Creates a new `ShareShoppingList` in the database.
    pub async fn new(
        mm: &ModelManager,
        list_id: Uuid,
        user_id: Uuid,
        expires_at: Option<PrimitiveDateTime>,
    ) -> Result<Self> {
        diesel::insert_into(schema::shares_shopping_lists::table)
            .values(&ShareShoppingListForInsert {
                user_id,
                list_id,
                expires_at,
            })
            .on_conflict((
                schema::shares_shopping_lists::user_id,
                schema::shares_shopping_lists::list_id,
            ))
            .do_update()
            .set(schema::shares_shopping_lists::last_accessed.eq(diesel::dsl::now))
            .returning(Self::as_returning())
            .get_result(&mut mm.pool.get().await?)
            .await
            .map_err(Error::from)
    }

    /// Retrieves a shared shopping list by its link UUID.
    pub async fn get_by_link(mm: &ModelManager, link: Uuid) -> Result<(Self, ShoppingListDetails)> {
        let share = schema::shares_shopping_lists::table
            .filter(schema::shares_shopping_lists::link.eq(link))
            .first::<Self>(&mut mm.pool.get().await?)
            .await
            .map_err(Error::from)?;

        let list = ShoppingListDetails::get(mm, share.list_id, share.user_id).await?;

        Ok((share, list))
    }
}
