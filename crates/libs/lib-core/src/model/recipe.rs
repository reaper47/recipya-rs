use crate::model::schema;
use crate::{
    ctx::Ctx,
    model::{Error, ModelManager, Result},
    name_entity_with_relations,
};
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
    pub keywords: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum Sections {
    WithHeaders(Vec<Section>),
    WithoutHeaders(Vec<String>),
}

#[derive(Debug, PartialEq)]
pub struct Section {
    pub header: String,
    pub items: Vec<String>,
}

name_entity_with_relations!(Category, categories, categories_recipes);
name_entity_with_relations!(Cuisine, cuisines, cuisines_recipes);

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
                        match recipe_c.ingredients {
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
                            name: "".to_string(),
                        })
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

                    // Ingredients

                    // Instructions

                    // Keywords

                    // Nutrition

                    // Times

                    // Tools

                    Ok(recipe_id)
                })
            })
            .await?;

        Ok(res)
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<RecipeWithData> {
        let recipe = schema::recipes::dsl::recipes
            .find(id)
            .select(Recipe::as_select())
            .first(&mut *mm.connection().await?)
            .await
            .optional()?
            .ok_or(Error::EntityNotFound {
                entity: "recipe",
                id,
            })?;

        Ok(RecipeWithData {
            recipe,
            category: "".to_string(),
            cuisine: None,
            ingredients: Sections::WithHeaders(vec![]),
            instructions: Sections::WithHeaders(vec![]),
            keywords: vec![],
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

        let got_id = RecipeBmc::create(
            &Ctx::new(1).unwrap(),
            &mm,
            RecipeForCreate {
                name: "Best Chinese Kale".to_string(),
                description: Some("You cannot find anything better than that".to_string()),
                image: None,
                yield_: 4,
                source: None,

                category: "dessert".to_string(),
                cuisine: Some("thai".to_string()),
                ingredients: Sections::WithHeaders(vec![]),
                instructions: Sections::WithHeaders(vec![]),
                keywords: vec![],
            },
        )
        .await?;

        pretty_assertions::assert_eq!(got_id, 1);
        let got = RecipeBmc::get(&ctx, &mm, got_id).await?;
        let want = RecipeWithData {
            recipe: Recipe {
                id: 1,
                name: "Best Chinese Kale".to_string(),
                description: Some("You cannot find anything better than that".to_string()),
                image: None,
                yield_: 4,
                language: "eng".to_string(),
                source: None,
                user_id: 1,
            },
            category: "".to_string(),
            cuisine: None,
            ingredients: Sections::WithHeaders(vec![]),
            instructions: Sections::WithHeaders(vec![]),
            keywords: vec![],
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }
}
