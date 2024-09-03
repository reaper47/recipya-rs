use crate::model::schema;
use crate::{
    ctx::Ctx,
    model::{Error, ModelManager, Result},
    name_entity_with_relations,
};
use diesel::internal::derives::multiconnection::chrono;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use uuid::Uuid;

#[derive(Associations, Debug, Queryable, Identifiable, PartialEq, Selectable)]
#[diesel(belongs_to(super::user::User))]
#[diesel(table_name = super::schema::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<Uuid>,
    pub yield_: i16,
    pub language: String,
    pub source: Option<String>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    pub user_id: i64,
}

pub struct RecipeForCreate {
    // For the recipe table
    pub name: String,
    pub description: Option<String>,
    pub image: Option<Uuid>,
    pub yield_: i16,
    pub source: Option<String>,

    // For association tables
    pub category: String,
    pub cuisine: Option<String>,
    pub ingredients: Sections,
    pub instructions: Sections,
    pub keywords: Vec<String>,
    // pub nutrition: Nutrition,
    // pub times: Times,
    // pub tools: Tools,
}

#[derive(Associations, Insertable)]
#[diesel(belongs_to(super::user::User))]
#[diesel(table_name = super::schema::recipes)]
pub(in crate::model) struct RecipeForInsert {
    pub(in crate::model) name: String,
    pub(in crate::model) description: Option<String>,
    pub(in crate::model) image: Option<Uuid>,
    pub(in crate::model) yield_: i16,
    pub(in crate::model) language: String,
    pub(in crate::model) source: Option<String>,
    pub(in crate::model) user_id: i64,
}

#[derive(Debug, PartialEq)]
pub struct RecipeWithData {
    pub recipe: Recipe,
    pub category: String,
    pub cuisine: Option<String>,
    pub ingredients: Sections,
    pub instructions: Sections,
    pub keywords: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Sections {
    WithHeaders(Vec<Section>),
    WithoutHeaders(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Section {
    pub name: String,
    pub items: Vec<String>,
}

name_entity_with_relations!(Category, categories, categories_recipes);
name_entity_with_relations!(Cuisine, cuisines, cuisines_recipes);
name_entity_with_relations!(Keyword, keywords, keywords_recipes);

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = super::schema::ingredients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::ingredients)]
struct IngredientForInsert {
    name: String,
}

#[derive(Identifiable, Selectable, Queryable, Insertable, Associations)]
#[diesel(belongs_to(Ingredient), belongs_to(Recipe), belongs_to(Section))]
#[diesel(table_name = super::schema::ingredients_recipes)]
#[diesel(primary_key(ingredient_id, recipe_id))]
pub struct IngredientRecipe {
    pub ingredient_id: i64,
    pub recipe_id: i64,
    pub section_id: Option<i64>,
    pub item_order: i16,
}

/// Recipe backend model controller.
pub struct RecipeBmc;

impl RecipeBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, recipe_c: RecipeForCreate) -> Result<i64> {
        let mut conn = mm.connection().await?;

        let res = conn
            .transaction::<i64, diesel::result::Error, _>(|conn| {
                Box::pin(async move {
                    let language = whatlang::detect_lang(&format!(
                        "{} {} {} {}",
                        recipe_c.name,
                        if let Some(text) = &recipe_c.description {
                            text
                        } else {
                            ""
                        },
                        match &recipe_c.ingredients {
                            Sections::WithoutHeaders(vec) => vec.join(" "),
                            Sections::WithHeaders(vec) => vec
                                .iter()
                                .flat_map(|section| section.items.iter())
                                .cloned()
                                .collect::<Vec<String>>()
                                .join(" "),
                        },
                        match recipe_c.instructions {
                            Sections::WithoutHeaders(vec) => vec.join(" "),
                            Sections::WithHeaders(vec) => vec
                                .iter()
                                .flat_map(|section| section.items.iter())
                                .cloned()
                                .collect::<Vec<String>>()
                                .join(" "),
                        },
                    ))
                    .unwrap_or_else(|| whatlang::Lang::Eng);

                    let recipe_id = diesel::insert_into(schema::recipes::table)
                        .values(&RecipeForInsert {
                            name: recipe_c.name,
                            description: recipe_c.description,
                            image: recipe_c.image,
                            yield_: recipe_c.yield_,
                            language: language.code().to_string(),
                            source: recipe_c.source,
                            user_id: ctx.user_id(),
                        })
                        .returning(schema::recipes::id)
                        .get_result(&mut *conn)
                        .await?;

                    // Category
                    let category_id = diesel::insert_into(schema::categories::table)
                        .values(&CategoryForInsert {
                            name: String::from(recipe_c.category.clone()),
                        })
                        .on_conflict(schema::categories::dsl::name)
                        .do_update()
                        .set(schema::categories::dsl::name.eq(recipe_c.category))
                        .returning(schema::categories::dsl::id)
                        .get_result(&mut *conn)
                        .await?;

                    diesel::insert_into(schema::categories_recipes::table)
                        .values(&CategoryRecipe {
                            category_id,
                            recipe_id,
                        })
                        .execute(&mut *conn)
                        .await?;

                    // Cuisine
                    if let Some(cuisine) = recipe_c.cuisine {
                        let cuisine_id = diesel::insert_into(schema::cuisines::table)
                            .values(&CuisineForInsert { name: cuisine })
                            .returning(schema::cuisines::dsl::id)
                            .get_result(&mut *conn)
                            .await?;

                        diesel::insert_into(schema::cuisines_recipes::table)
                            .values(&CuisineRecipe {
                                cuisine_id,
                                recipe_id,
                            })
                            .execute(&mut *conn)
                            .await?;
                    }

                    // TODO: Ingredients
                    match &recipe_c.ingredients {
                        Sections::WithoutHeaders(ingredients) => {
                            let ids: Vec<IngredientRecipe> =
                                diesel::insert_into(schema::ingredients::table)
                                    .values(
                                        &ingredients
                                            .iter()
                                            .map(|name| IngredientForInsert {
                                                name: String::from(name),
                                            })
                                            .collect::<Vec<IngredientForInsert>>(),
                                    )
                                    .returning(schema::ingredients::dsl::id)
                                    .get_results::<i64>(&mut *conn)
                                    .await?
                                    .iter()
                                    .enumerate()
                                    .map(|(idx, &ingredient_id)| IngredientRecipe {
                                        ingredient_id,
                                        recipe_id,
                                        section_id: None,
                                        item_order: idx as i16,
                                    })
                                    .collect();

                            diesel::insert_into(schema::ingredients_recipes::table)
                                .values(ids)
                                .execute(&mut *conn)
                                .await?;
                        }
                        Sections::WithHeaders(sections) => for section in sections.iter() {},
                    };

                    // TODO: Instructions

                    // Keywords
                    for keyword in recipe_c.keywords.iter() {
                        let keyword_id = diesel::insert_into(schema::keywords::table)
                            .values(&KeywordForInsert {
                                name: String::from(keyword),
                            })
                            .on_conflict(schema::keywords::dsl::name)
                            .do_update()
                            .set(schema::keywords::dsl::name.eq(String::from(keyword)))
                            .returning(schema::keywords::dsl::id)
                            .get_result(&mut *conn)
                            .await?;

                        diesel::insert_into(schema::keywords_recipes::table)
                            .values(&KeywordRecipe {
                                keyword_id,
                                recipe_id,
                            })
                            .execute(&mut *conn)
                            .await?;
                    }

                    // TODO: Nutrition

                    // TODO: Times

                    // TODO: Tools

                    Ok(recipe_id)
                })
            })
            .await?;

        Ok(res)
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<RecipeWithData> {
        let mut conn = mm.connection().await?;

        let (recipe, category, cuisine, keywords) = schema::recipes::table
            .inner_join(schema::categories_recipes::table.inner_join(schema::categories::table))
            .left_join(schema::cuisines_recipes::table.left_join(schema::cuisines::table))
            .left_join(schema::keywords_recipes::table.left_join(schema::keywords::table))
            .select((
                schema::recipes::all_columns,
                schema::categories::name,
                schema::cuisines::name.nullable(),
                schema::keywords::name.nullable(),
            ))
            .first::<(Recipe, String, Option<String>, Option<String>)>(&mut *conn)
            .await
            .optional()?
            .ok_or(Error::EntityNotFound {
                entity: "recipe",
                id,
            })?;

        Ok(RecipeWithData {
            recipe,
            category,
            cuisine,
            ingredients: Sections::WithHeaders(vec![]),
            instructions: Sections::WithHeaders(vec![]),
            keywords: match keywords {
                None => None,
                Some(_) => Some(
                    schema::keywords_recipes::table
                        .filter(schema::keywords_recipes::dsl::recipe_id.eq(id))
                        .inner_join(schema::keywords::table)
                        .select(schema::keywords::name)
                        .load::<String>(&mut *conn)
                        .await?,
                ),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::store::test_db::TestDb;

    pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        let db = TestDb::new().await?;
        let (mm, ctx) = db.setup(1);
        let fx_name = "Best Chinese Kale".to_string();
        let fx_category = "dessert".to_string();
        let fx_description = Some("You cannot find anything better than that".to_string());
        let fx_yield = 4;
        let fx_cuisine = Some("thai".to_string());
        let fx_keywords = vec!["vegetarian".to_string(), "tofu".to_string()];
        /*let fx_ingredients = Sections::WithHeaders(vec![
            Section {
                header: "Sauce".to_string(),
                items: vec![
                    "1 cup blue spinach".to_string(),
                    "1/2 tbsp cinnamon".to_string(),
                ],
            },
            Section {
                header: "Main".to_string(),
                items: vec![
                    "4 pounds top quality chicken filet".to_string(),
                    "1/8 cup lemon juice".to_string(),
                ],
            },
        ]);*/
        let fx_ingredients = Sections::WithoutHeaders(vec![
            "1 cup blue spinach".to_string(),
            "1/2 tbsp cinnamon".to_string(),
            "4 pounds top quality chicken filet".to_string(),
            "1/8 cup lemon juice".to_string(),
        ]);
        let fx_instructions = Sections::WithHeaders(vec![
            Section {
                name: "Sauce".to_string(),
                items: vec!["Mix all these ingredients".to_string()],
            },
            Section {
                name: "Chicken".to_string(),
                items: vec![
                    "Turn the oven at 300 F".to_string(),
                    "Soak the chicken in the lemon juice".to_string(),
                    "Bake for 35 minutes".to_string(),
                ],
            },
        ]);

        let got_id = RecipeBmc::create(
            &Ctx::new(1).unwrap(),
            &mm,
            RecipeForCreate {
                name: fx_name.clone(),
                description: fx_description.clone(),
                image: None,
                yield_: fx_yield,
                source: None,

                category: fx_category.clone(),
                cuisine: fx_cuisine.clone(),
                ingredients: fx_ingredients.clone(),
                instructions: fx_instructions.clone(),
                keywords: fx_keywords.clone(),
            },
        )
        .await?;

        pretty_assertions::assert_eq!(got_id, 1);
        let got = RecipeBmc::get(&ctx, &mm, got_id).await?;
        let want = RecipeWithData {
            recipe: Recipe {
                id: 1,
                name: fx_name,
                description: fx_description,
                image: None,
                yield_: fx_yield,
                language: "eng".to_string(),
                source: None,
                user_id: 1,
                created_at: got.recipe.created_at,
                updated_at: got.recipe.updated_at,
            },
            category: fx_category,
            cuisine: fx_cuisine,
            ingredients: fx_ingredients,
            instructions: fx_instructions,
            keywords: Some(fx_keywords),
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }
}
