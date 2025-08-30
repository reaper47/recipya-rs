use std::env::temp_dir;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

use base64::Engine;
use libpaprika::{Recipe, RecipeSet};
use recipe_schema::{
    AggregateRating, AtType, CreativeWorkOrUrl, DateOrDateTime, ImageObjectOrUrl, ImageObjectType,
    NumberOrText, RecipeCategory, RecipeSchema, Sections,
};
use tracing::error;
use url::Url;
use uuid::Uuid;

use crate::Result;
use crate::helpers::{
    ToRecipeSchema, seconds_to_duration, sections_to_itemlist, sections_to_vec, to_defined_text,
    to_is_based_on, to_text, to_yield,
};

/// Extracts the recipes from a paprikarecipes container.
pub fn parse<R>(mut r: R) -> Result<Vec<RecipeSchema>>
where
    R: Read,
{
    let mut content = Vec::new();
    r.read_to_end(&mut content)?;

    let path = temp_dir().join(format!("example-{}.paprikarecipes", Uuid::new_v4()));
    let mut file = File::create(path.clone())?;
    file.write_all(content.as_slice())?;

    let recipes = RecipeSet::from_file(path)?
        .recipes
        .values()
        .filter(|r| !r.name.is_empty() && !r.ingredients.is_empty() && !r.directions.is_empty())
        .map(|r| r.to_recipe_schema())
        .collect();
    Ok(recipes)
}

impl ToRecipeSchema for Recipe {
    fn to_recipe_schema(&self) -> RecipeSchema {
        let (category, keywords) = match self.categories.as_slice() {
            [first, rest @ ..] => (Some(first.to_string()), rest.to_vec()),
            [] => (None, Vec::new()),
        };

        let instructions: Vec<(String, Vec<String>)> = std::iter::once((
            "".to_string(),
            self.directions.split("\n\n").map(str::to_string).collect(),
        ))
        .chain((!self.notes.trim().is_empty()).then(|| {
            (
                "Notes".to_string(),
                self.notes.split("\n\n").map(str::to_string).collect(),
            )
        }))
        .collect();

        let cook = humantime::parse_duration(&self.cook_time).unwrap_or_default();
        let prep = humantime::parse_duration(&self.prep_time).unwrap_or_default();

        let ingredients = self.ingredients.lines().collect::<Vec<_>>();
        let url = if let Ok(u) = Url::parse(&self.source_url) {
            Some(CreativeWorkOrUrl::Url(u))
        } else {
            None
        };

        let pictures = {
            let items = self
                .photos
                .iter()
                .filter_map(|p| {
                    base64::engine::general_purpose::STANDARD
                        .decode(&p.data)
                        .ok()
                })
                .filter_map(|bytes| {
                    let path = format!(
                        "{}/{}.image",
                        temp_dir().to_str().unwrap_or_default(),
                        Uuid::new_v4()
                    );
                    match File::create(path.clone()) {
                        Ok(mut file) => {
                            if file.write_all(&bytes).is_ok() {
                                Some(path)
                            } else {
                                None
                            }
                        }
                        Err(err) => {
                            error!("Failed to create file for paprika photo: {err:?}");
                            None
                        }
                    }
                })
                .map(|s| ImageObjectType {
                    at_type: AtType::ImageObject,
                    at_id: Some(s),
                    ..Default::default()
                })
                .collect::<Vec<_>>();

            (!items.is_empty()).then_some(ImageObjectOrUrl::ImageObjects(items))
        };

        RecipeSchema {
            at_context: Default::default(),
            at_type: Some(AtType::Recipe),
            aggregate_rating: if self.rating > 0 {
                Some(AggregateRating {
                    at_type: AtType::AggregateRating,
                    rating_value: Some(NumberOrText::Number(self.rating as f64)),
                    ..Default::default()
                })
            } else {
                None
            },
            cook_time: seconds_to_duration(cook.as_secs() as i32),
            date_created: Some(DateOrDateTime::Date(
                iso8601::Date::from_str(self.created.split_whitespace().next().unwrap_or_default())
                    .unwrap_or_default(),
            )),
            description: to_text(self.description.clone()),
            image: pictures,
            is_based_on: to_is_based_on(if !self.source.is_empty() {
                format!("{} [Imported from Paprika]", self.source)
            } else {
                "Imported from Paprika".to_string()
            }),
            keywords: to_defined_text(keywords.join(",")),
            name: Some(self.name.clone()),
            prep_time: seconds_to_duration(prep.as_secs() as i32),
            recipe_category: RecipeCategory::Text(category.unwrap_or_default()),
            recipe_ingredient: sections_to_vec(Sections::from([(
                "".into(),
                ingredients
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<_>>(),
            )])),
            recipe_instructions: sections_to_itemlist(Sections::from(instructions)),
            recipe_yield: to_yield(self.servings.parse().unwrap_or(1)),
            total_time: seconds_to_duration((prep.as_secs() + cook.as_secs()) as i32),
            main_entity_of_page: url,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_recipes {
        use super::*;

        #[test]
        fn test_example1() -> Result<()> {
            let buf = files::example1();

            let mut got = parse(buf)?;

            let mut want = results::example1();
            want.sort_by_key(|r| r.name.as_ref().unwrap().to_string());
            got.sort_by_key(|r| r.name.as_ref().unwrap().to_string());
            for (got_item, want_item) in got.iter_mut().zip(want.iter()) {
                got_item.image = want_item.image.clone();
            }
            pretty_assertions::assert_eq!(got, results::example1());
            Ok(())
        }
    }

    mod files {
        use std::io::Cursor;
        use testing::utils::open_test_file;

        pub fn example1() -> Cursor<Vec<u8>> {
            open_test_file("integrations/example1.paprikarecipes")
        }
    }

    mod results {
        use super::*;

        use url::Url;

        pub fn example1() -> Vec<RecipeSchema> {
            vec![RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                cook_time: seconds_to_duration(3900),
                date_created: Some(DateOrDateTime::Date(iso8601::Date::from_str("2025-04-15").unwrap_or_default())),
                image: None,
                is_based_on: to_is_based_on("Allrecipes.com [Imported from Paprika]".into()),
                main_entity_of_page: Some(CreativeWorkOrUrl::Url(Url::parse("https://www.allrecipes.com/recipe/259353/black-eyed-pea-cornbread/").expect("Url to be valid"))),
                name: Some("Black-Eyed Pea Cornbread".into()),
                prep_time: seconds_to_duration(900),
                recipe_category: RecipeCategory::Text("Japanese".into()),
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("".into(), vec![
                        "cooking spray".into(),
                        "1 pound bulk spicy breakfast sausage".into(),
                        "1 onion, chopped".into(),
                        "1 cup white cornmeal".into(),
                        "½ cup all-purpose flour".into(),
                        "1 teaspoon salt".into(),
                        "½ teaspoon baking soda".into(),
                        "1 cup buttermilk".into(),
                        "½ cup vegetable oil".into(),
                        "2 eggs, lightly beaten".into(),
                        "1 (15 ounce) can black-eyed peas, drained".into(),
                        "1 (8 ounce) package shredded Cheddar cheese".into(),
                        "¾ cup cream-style corn".into(),
                        "1 (4.5 ounce) can chopped green chile peppers".into(),
                        "¼ cup chopped pickled jalapeño peppers".into(),
                    ])
                ])),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "Preheat the oven to 350 degrees F (175 degrees C). Grease a 9x13-inch baking dish with cooking spray.".into(),
                        "Cook sausage and onion in a large skillet over medium heat, stirring until sausage is crumbly and no longer pink, about 5 minutes. Drain on paper towels.".into(),
                        "Mix cornmeal, flour, salt, and baking soda together in a large bowl.".into(),
                        "Whisk buttermilk, oil, and eggs together in a small bowl. Add to cornmeal mixture, stirring just until moistened. Stir in sausage mixture, black-eyed peas, Cheddar cheese, corn, chile peppers, and jalapeños. Pour batter into the prepared baking dish.".into(),
                        "Bake in the preheated oven until a toothpick inserted into center comes out clean, about 1 hour.".into(),
                    ]),
                    ("Notes".into(), vec!["Nothing special".into()]),
                ])),
                recipe_yield: to_yield(12),
                ..Default::default()
            }, RecipeSchema {
                at_context: Default::default(),
                at_type: Some(AtType::Recipe),
                at_graph: None,
                at_id: None,
                aggregate_rating: None,
                alternate_name: None,
                article_body: None,
                audio: None,
                author: None,
                award: None,
                citation: None,
                comment: None,
                comment_count: None,
                content_rating: None,
                contributor: None,
                cook_time: seconds_to_duration(2700),
                cooking_method: None,
                content_location: None,
                country_of_origin: None,
                credit_text: None,
                date_created: Some(DateOrDateTime::Date(iso8601::Date::from_str("2025-04-15").unwrap_or_default())),
                date_modified: None,
                date_published: None,
                description: to_text("A great recipe!".into()),
                estimated_cost: None,
                headline: None,
                identifier: None,
                image: None,
                in_language: None,
                is_accessible_for_free: false,
                is_based_on: to_is_based_on("Allrecipes.com [Imported from Paprika]".into()),
                is_part_of: None,
                keywords: to_defined_text("Dinner,Japanese,Meat".into()),
                location_created: None,
                main_entity_of_page: Some(CreativeWorkOrUrl::Url(Url::parse("https://www.allrecipes.com/recipe/285611/loaded-mashed-potato-casserole/").expect("Url to be valid"))),
                name: Some("Loaded Mashed Potato Casserole".into()),
                nutrition: None,
                perform_time: None,
                potential_action: None,
                prep_time: seconds_to_duration(1800),
                publisher: None,
                recipe_category: RecipeCategory::Text("Asian".into()),
                recipe_cuisine: None,
                recipe_ingredient: sections_to_vec(Sections::from([
                    ("".into(), vec![
                        "5 pounds potatoes, peeled and cubed".into(),
                        "8 strips bacon".into(),
                        "⅔ cup chopped green or red bell pepper".into(),
                        "½ cup chopped onion, or more to taste".into(),
                        "¾ cup hot milk".into(),
                        "6 tablespoons butter, plus more for greasing".into(),
                        "1 teaspoon salt".into(),
                        "½ teaspoon ground black pepper, or to taste".into(),
                        "1 (10 ounce) package sharp Cheddar cheese, cut into chunks".into(),
                        "3 large eggs".into(),
                    ])
                ])),
                recipe_instructions: sections_to_itemlist(Sections::from([
                    ("".into(), vec![
                        "Preheat the oven to 350 degrees F (175 degrees C). Butter 2 large casserole dishes.".into(),
                        "Place potatoes into a large pot and cover with salted water; bring to a boil. Reduce heat to medium-low and simmer until tender, 10 to 15 minutes. Drain and transfer to a large bowl.".into(),
                        "While the potatoes are cooking, place bacon in a large skillet and cook over medium-high heat, turning occasionally, until browned and crispy, 10 to 12 minutes. Drain bacon slices on paper towels and crumble when cool enough to handle. Leave 2 tablespoons bacon drippings in the skillet and discard the rest.".into(),
                        "Add bell pepper and onion to the drippings; cook and stir over medium heat until tender, about 5 minutes. Remove from the heat.".into(),
                        "Combine potatoes, hot milk, butter, salt, and pepper in the bowl of a stand mixer fitted with the paddle attachment; beat first on slow speed, then increasing to medium speed until fluffy, 2 to 3 minutes. Add Cheddar cheese, eggs, bell pepper-onion mixture, and bacon. Beat again at medium speed until cheese is in smaller pieces, about 3 minutes. Fill each of the prepared casserole dishes 2/3 full of potatoes.".into(),
                        "Bake in the preheated oven until heated through and potatoes have puffed up, 30 to 35 minutes.".into(),
                    ]),
                    ("Notes".into(), vec![
                        "To make ahead, cool casseroles after step 5. Cover and refrigerate until ready to bake. Remove from the refrigerator and let sit on counter for 1 hour to come to room temperature. Bake in a preheated 350 degree F (175 degree C) oven until heated through and puffy, 30 to 35 minutes. (May take an extra 10 minutes if made ahead!)".into()
                    ]),
                ])),
                recipe_yield: to_yield(16),
                ..Default::default()
            }]
        }
    }
}
