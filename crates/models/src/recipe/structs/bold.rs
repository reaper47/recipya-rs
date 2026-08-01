use std::collections::HashMap;

use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use ingredient::IngredientParser;
use itertools::Itertools;

use repository::{ModelManager, schema};
use support::strings::{self, insert_space_after_leading_number};

use crate::{Error, Recipe, RecipeDetails, Result};

/// Represents the bold formatting types.
pub enum BoldFileType {
    Html,
    Markdown,
}

/// Struct associated with the `bold_indices_ingredients` SQL table.
///
/// It stores the indices of text to be displayed in bold.
#[derive(Debug, Eq, PartialEq, Queryable, Associations, Identifiable, Selectable)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::bold_indices_ingredients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BoldInstructionIndex {
    id: i64,
    recipe_id: i64,
    instruction_id: i64,
    start_index: i32,
    end_index: i32,
}

impl BoldInstructionIndex {
    /// Retrieves all bold instruction indices for the given instruction IDs.
    pub async fn get_all(mm: &ModelManager, recipe_id: i64, ids: &[i64]) -> Result<Vec<Self>> {
        schema::bold_indices_ingredients::table
            .filter(schema::bold_indices_ingredients::recipe_id.eq(recipe_id))
            .filter(schema::bold_indices_ingredients::instruction_id.eq_any(ids))
            .select(Self::as_select())
            .load(&mut mm.pool.get().await?)
            .await
            .map_err(|err| Error::Diesel(err.to_string()))
    }
}

#[derive(Debug, Associations, Insertable)]
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = schema::bold_indices_ingredients)]
struct BoldInstructionIndexForInsert {
    recipe_id: i64,
    instruction_id: i64,
    start_index: i32,
    end_index: i32,
}

impl BoldInstructionIndex {
    /// Inserts a list of new bold instruction indices into the database.
    ///
    /// The connection is meant to be reused. Thus, this function is to be used
    /// within a function that has already acquired a connection from the pool.
    pub async fn insert<C>(
        conn: &mut C,
        recipe_id: i64,
        ingredients: &[String],
        instructions: HashMap<String, i64>,
    ) -> Result<()>
    where
        C: AsyncConnection<Backend = diesel::pg::Pg>,
    {
        diesel::delete(schema::bold_indices_ingredients::table)
            .filter(schema::bold_indices_ingredients::recipe_id.eq(recipe_id))
            .execute(conn)
            .await
            .map_err(|err| Error::Support(err.to_string()))?;

        let instructions = instructions.into_iter().collect_vec();
        let instruction_strings = instructions.iter().map(|(s, _)| s.clone()).collect_vec();
        let instruction_refs = instruction_strings.iter().map(String::as_str).collect_vec();

        let ingredient_strings = ingredients
            .iter()
            .map(|s| {
                IngredientParser::new(false)
                    .from_str(&insert_space_after_leading_number(s))
                    .name
                    .trim()
                    .to_string()
            })
            .collect_vec();

        let ingredient_refs = ingredient_strings.iter().map(String::as_str).collect_vec();

        let bolds = strings::find_indexes(instruction_refs.as_slice(), ingredient_refs.as_slice())
            .map_err(|err| Error::Support(err.to_string()))?;

        let values = instructions
            .into_iter()
            .map(|(_, id)| id)
            .collect_vec()
            .into_iter()
            .zip(bolds)
            .flat_map(|(instruction_id, bold)| {
                bold.into_iter()
                    .filter(|(start_index, end_index)| end_index > start_index)
                    .map(|(start_index, end_index)| BoldInstructionIndexForInsert {
                        recipe_id,
                        instruction_id,
                        start_index,
                        end_index,
                    })
                    .collect_vec()
            })
            .collect_vec();

        diesel::insert_into(schema::bold_indices_ingredients::table)
            .values(values)
            .execute(conn)
            .await?;

        Ok(())
    }
}

impl RecipeDetails {
    /// Adds bold indicators to the instructions.
    pub async fn bold_ingredients_in_instructions(&mut self, mm: &ModelManager) -> Result<()> {
        let ids = self
            .instructions
            .iter()
            .filter_map(|item| item.id)
            .collect_vec();

        let mut inserts: HashMap<i64, Vec<(usize, &str)>> = HashMap::new();
        for bold in BoldInstructionIndex::get_all(mm, self.recipe.id, ids.as_slice()).await? {
            inserts
                .entry(bold.instruction_id)
                .or_default()
                .extend_from_slice(&[
                    (usize::try_from(bold.start_index).unwrap_or_default(), "<b>"),
                    (usize::try_from(bold.end_index).unwrap_or_default(), "</b>"),
                ]);
        }

        for (instruction_id, mut indices) in inserts {
            self.instructions
                .iter_mut()
                .find(|item| item.id == Some(instruction_id))
                .and_then(|item| {
                    indices.sort_by_key(|b| std::cmp::Reverse(b.0));
                    for (idx, insert) in indices {
                        item.text.insert_str(idx, insert);
                    }
                    None::<()>
                });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_db::TestDb;
    use test_utils::{create_app_state, insert_user};

    use super::*;
    use crate::{
        recipe::structs::{
            recipe::RecipeForCreate,
            section::{Item, SectionComponents, SectionItem},
        },
        settings::UserSettingDetails,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    pub fn a_bare_minimum_recipe() -> RecipeForCreate {
        RecipeForCreate {
            name: "Best Chinese Kale".into(),
            ingredients: SectionComponents::Grouped(vec![
                SectionItem::new(
                    "Sauce",
                    vec![
                        Item::new("1 cup blue spinach"),
                        Item::new("1/2 tbsp cinnamon"),
                    ],
                ),
                SectionItem::new(
                    "Main",
                    vec![
                        Item::new("4 pounds top quality chicken filet"),
                        Item::new("1/8 cup lemon juice"),
                    ],
                ),
            ]),
            instructions: SectionComponents::Grouped(vec![
                SectionItem::new("Sauce", vec![Item::new("Mix all these ingredients")]),
                SectionItem::new(
                    "Chicken",
                    vec![
                        Item::new("Turn the oven at 300 F"),
                        Item::new("Soak the chicken in the lemon juice"),
                        Item::new("Bake for 35 minutes").with_duration(2100),
                    ],
                ),
            ]),
            measurement_system_id: 2,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_bolden_instrucions_on_recipe_create_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;
        let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let recipe = a_bare_minimum_recipe();
        let recipe_id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

        let mut got = Recipe::get(&state.mm, user.id, recipe_id).await?;
        got.bold_ingredients_in_instructions(&state.mm).await?;

        pretty_assertions::assert_eq!(
            got.instructions,
            SectionComponents::Grouped(vec![
                SectionItem::new(
                    "Sauce",
                    vec![Item::new("Mix all these ingredients").with_id(1),]
                ),
                SectionItem::new(
                    "Chicken",
                    vec![
                        Item::new("Turn the oven at 300 F").with_id(2),
                        Item::new("Soak the <b>chicken</b> in the <b>lemon juice</b>").with_id(3),
                        Item::new("Bake for 35 minutes")
                            .with_duration(2100)
                            .with_id(4),
                    ],
                ),
            ])
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_bolden_instructions_on_recipe_update_ok() -> Result<()> {
        let (_test_db, config) = TestDb::new(None).await?;
        let state = create_app_state(config.clone()).await;
        let user = insert_user(config.clone()).await?;
        let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
        let mut recipe = a_bare_minimum_recipe();
        let recipe_id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;
        recipe.instructions = SectionComponents::Grouped(vec![SectionItem::new(
            "Chicken",
            vec![
                Item::new("Wash the blue spinach carefully.").with_id(2),
                Item::new("Soak the chicken in the lemon juice.").with_id(3),
                Item::new("Bake for 35 minutes then sprinkle with cinnamon.").with_id(4),
            ],
        )]);
        Recipe::update(&state.mm, user.id, recipe_id, &mut recipe).await?;

        let mut got = Recipe::get(&state.mm, user.id, recipe_id).await?;
        got.bold_ingredients_in_instructions(&state.mm).await?;

        pretty_assertions::assert_eq!(
            got.instructions,
            SectionComponents::Grouped(vec![SectionItem::new(
                "Chicken",
                vec![
                    Item::new("Wash the <b>blue spinach</b> carefully.").with_id(5),
                    Item::new("Soak the <b>chicken</b> in the <b>lemon juice</b>.").with_id(6),
                    Item::new("Bake for 35 minutes then sprinkle with <b>cinnamon</b>.")
                        .with_duration(2100)
                        .with_id(7),
                ],
            ),])
        );
        Ok(())
    }
}
