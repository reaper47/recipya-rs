use std::env::temp_dir;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

use base64::Engine;
use libpaprika::{Recipe, RecipeSet};
use tracing::error;
use url::Url;
use uuid::Uuid;

use schema_org::AggregateRating;
use schema_org::field::{
    AggregateRatingRatingValueFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
    RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
};

use crate::Result;
use crate::helpers::{ToRecipeSchema, seconds_to_duration, to_is_based_on, to_yield};

/// Extracts the recipes from a paprikarecipes container.
pub fn parse<R>(mut r: R) -> Result<Vec<schema_org::Recipe>>
where
    R: Read,
{
    let mut content = Vec::new();
    r.read_to_end(&mut content)?;

    let path = temp_dir().join(format!("example-{}.paprikarecipes", Uuid::new_v4()));
    let mut file = File::create(&path)?;
    file.write_all(content.as_slice())?;

    let recipes = RecipeSet::from_file(path)?
        .recipes
        .values()
        .filter(|r| !r.name.is_empty() && !r.ingredients.is_empty() && !r.directions.is_empty())
        .map(ToRecipeSchema::to_recipe_schema)
        .collect();
    Ok(recipes)
}

impl ToRecipeSchema for Recipe {
    #[allow(clippy::too_many_lines)]
    fn to_recipe_schema(&self) -> schema_org::Recipe {
        let (category, keywords) = match self.categories.split_first() {
            Some((category, keywords)) => (Some(category.clone()), keywords.to_vec()),
            None => (None, Vec::new()),
        };

        let instructions: Vec<RecipeRecipeInstructionsFieldEnum> = std::iter::once(
            self.directions
                .split("\n\n")
                .map(|s| RecipeRecipeInstructionsFieldEnum::Text(s.to_string()))
                .collect::<Vec<_>>(),
        )
        .flatten()
        .chain((!self.notes.trim().is_empty()).then(|| {
            RecipeRecipeInstructionsFieldEnum::new_section(
                "Notes",
                self.notes.split("\n\n").collect(),
            )
        }))
        .collect();

        let cook = humantime::parse_duration(&self.cook_time).unwrap_or_default();
        let prep = humantime::parse_duration(&self.prep_time).unwrap_or_default();

        let url = Url::parse(&self.source_url)
            .ok()
            .map(String::from)
            .unwrap_or_default();

        let pictures = self
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
                match File::create(&path) {
                    Ok(mut file) => {
                        if file.write_all(&bytes).is_ok() {
                            Some(path)
                        } else {
                            None
                        }
                    }
                    Err(err) => {
                        error!(?err, "Failed to create file for paprika photo");
                        None
                    }
                }
            })
            .map(RecipeImageFieldEnum::URL)
            .collect::<Vec<_>>();

        let src = if self.source.is_empty() {
            "Imported from Paprika".to_string()
        } else {
            format!("{} [Imported from Paprika]", self.source)
        };

        schema_org::Recipe {
            aggregate_rating: if self.rating == 0 {
                vec![]
            } else {
                vec![AggregateRating {
                    rating_value: vec![AggregateRatingRatingValueFieldEnum::Number(f32::from(
                        self.rating,
                    ))],
                    ..Default::default()
                }]
            },
            cook_time: seconds_to_duration(i32::try_from(cook.as_secs()).unwrap_or_default()),
            date_created: iso8601::Date::from_str(
                self.created.split_whitespace().next().unwrap_or_default(),
            )
            .map_or_else(|_| vec![], |d| vec![d.to_string()]),
            description: if self.description.is_empty() {
                vec![]
            } else {
                vec![RecipeDescriptionFieldEnum::Text(self.description.clone())]
            },
            image: pictures,
            is_based_on: to_is_based_on(&src),
            keywords: keywords
                .into_iter()
                .map(RecipeKeywordsFieldEnum::TextOrURL)
                .collect(),
            name: vec![self.name.clone()],
            prep_time: seconds_to_duration(i32::try_from(prep.as_secs()).unwrap_or_default()),
            recipe_category: vec![category.unwrap_or_default()],
            recipe_ingredient: self
                .ingredients
                .lines()
                .map(|s| RecipeRecipeIngredientFieldEnum::Text(s.into()))
                .collect(),
            recipe_instructions: instructions,
            recipe_yield: to_yield(self.servings.parse().unwrap_or(1)),
            total_time: seconds_to_duration(
                i32::try_from(prep.as_secs() + cook.as_secs()).unwrap_or_default(),
            ),
            url: vec![url],
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
            want.sort_by_key(|r| (*r.name.first().as_ref().unwrap()).clone());
            got.sort_by_key(|r| (*r.name.first().as_ref().unwrap()).clone());
            for (got_item, want_item) in got.iter_mut().zip(want.iter()) {
                got_item.image = want_item.image.clone();
            }
            pretty_assertions::assert_eq!(got, results::example1());
            Ok(())
        }
    }

    mod files {
        use std::io::Cursor;
        use test_fixtures::open_test_file;

        pub fn example1() -> Cursor<Vec<u8>> {
            open_test_file("integrations/example1.paprikarecipes")
        }
    }

    mod results {
        use schema_org::DurationOrText;

        use super::*;

        pub fn example1() -> Vec<schema_org::Recipe> {
            vec![schema_org::Recipe {
                cook_time: seconds_to_duration(3900),
                date_created: vec!["2025-04-15".into()],
                is_based_on: to_is_based_on("Allrecipes.com [Imported from Paprika]"),
                url: vec!["https://www.allrecipes.com/recipe/259353/black-eyed-pea-cornbread/".into()],
                name: vec!["Black-Eyed Pea Cornbread".into()],
                prep_time: seconds_to_duration(900),
                recipe_category: vec!["Japanese".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("cooking spray".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 pound bulk spicy breakfast sausage".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 onion, chopped".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 cup white cornmeal".into()),
                    RecipeRecipeIngredientFieldEnum::Text("½ cup all-purpose flour".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 teaspoon salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("½ teaspoon baking soda".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 cup buttermilk".into()),
                    RecipeRecipeIngredientFieldEnum::Text("½ cup vegetable oil".into()),
                    RecipeRecipeIngredientFieldEnum::Text("2 eggs, lightly beaten".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 (15 ounce) can black-eyed peas, drained".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 (8 ounce) package shredded Cheddar cheese".into()),
                    RecipeRecipeIngredientFieldEnum::Text("¾ cup cream-style corn".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 (4.5 ounce) can chopped green chile peppers".into()),
                    RecipeRecipeIngredientFieldEnum::Text("¼ cup chopped pickled jalapeño peppers".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Preheat the oven to 350 degrees F (175 degrees C). Grease a 9x13-inch baking dish with cooking spray.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Cook sausage and onion in a large skillet over medium heat, stirring until sausage is crumbly and no longer pink, about 5 minutes. Drain on paper towels.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Mix cornmeal, flour, salt, and baking soda together in a large bowl.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Whisk buttermilk, oil, and eggs together in a small bowl. Add to cornmeal mixture, stirring just until moistened. Stir in sausage mixture, black-eyed peas, Cheddar cheese, corn, chile peppers, and jalapeños. Pour batter into the prepared baking dish.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Bake in the preheated oven until a toothpick inserted into center comes out clean, about 1 hour.".into()),
                    RecipeRecipeInstructionsFieldEnum::new_section("Notes", vec!["Nothing special"])
                ],
                recipe_yield: to_yield(12),
                total_time: vec![DurationOrText::Text("PT4800S".into())],
                ..Default::default()
            }, schema_org::Recipe {
                cook_time: seconds_to_duration(2700),
                date_created: vec!["2025-04-15".into()],
                description: vec![RecipeDescriptionFieldEnum::Text("A great recipe!".into())],
                is_based_on: to_is_based_on("Allrecipes.com [Imported from Paprika]"),
                keywords: vec![
                    RecipeKeywordsFieldEnum::TextOrURL("Dinner".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Japanese".into()),
                    RecipeKeywordsFieldEnum::TextOrURL("Meat".into()),
                ],
                url: vec!["https://www.allrecipes.com/recipe/285611/loaded-mashed-potato-casserole/".into()],
                name: vec!["Loaded Mashed Potato Casserole".into()],
                prep_time: seconds_to_duration(1800),
                recipe_category: vec!["Asian".into()],
                recipe_ingredient: vec![
                    RecipeRecipeIngredientFieldEnum::Text("5 pounds potatoes, peeled and cubed".into()),
                    RecipeRecipeIngredientFieldEnum::Text("8 strips bacon".into()),
                    RecipeRecipeIngredientFieldEnum::Text("⅔ cup chopped green or red bell pepper".into()),
                    RecipeRecipeIngredientFieldEnum::Text("½ cup chopped onion, or more to taste".into()),
                    RecipeRecipeIngredientFieldEnum::Text("¾ cup hot milk".into()),
                    RecipeRecipeIngredientFieldEnum::Text("6 tablespoons butter, plus more for greasing".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 teaspoon salt".into()),
                    RecipeRecipeIngredientFieldEnum::Text("½ teaspoon ground black pepper, or to taste".into()),
                    RecipeRecipeIngredientFieldEnum::Text("1 (10 ounce) package sharp Cheddar cheese, cut into chunks".into()),
                    RecipeRecipeIngredientFieldEnum::Text("3 large eggs".into()),
                ],
                recipe_instructions: vec![
                    RecipeRecipeInstructionsFieldEnum::Text("Preheat the oven to 350 degrees F (175 degrees C). Butter 2 large casserole dishes.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Place potatoes into a large pot and cover with salted water; bring to a boil. Reduce heat to medium-low and simmer until tender, 10 to 15 minutes. Drain and transfer to a large bowl.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("While the potatoes are cooking, place bacon in a large skillet and cook over medium-high heat, turning occasionally, until browned and crispy, 10 to 12 minutes. Drain bacon slices on paper towels and crumble when cool enough to handle. Leave 2 tablespoons bacon drippings in the skillet and discard the rest.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Add bell pepper and onion to the drippings; cook and stir over medium heat until tender, about 5 minutes. Remove from the heat.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Combine potatoes, hot milk, butter, salt, and pepper in the bowl of a stand mixer fitted with the paddle attachment; beat first on slow speed, then increasing to medium speed until fluffy, 2 to 3 minutes. Add Cheddar cheese, eggs, bell pepper-onion mixture, and bacon. Beat again at medium speed until cheese is in smaller pieces, about 3 minutes. Fill each of the prepared casserole dishes 2/3 full of potatoes.".into()),
                    RecipeRecipeInstructionsFieldEnum::Text("Bake in the preheated oven until heated through and potatoes have puffed up, 30 to 35 minutes.".into()),
                    RecipeRecipeInstructionsFieldEnum::new_section("Notes", vec![
                        "To make ahead, cool casseroles after step 5. Cover and refrigerate until ready to bake. Remove from the refrigerator and let sit on counter for 1 hour to come to room temperature. Bake in a preheated 350 degree F (175 degree C) oven until heated through and puffy, 30 to 35 minutes. (May take an extra 10 minutes if made ahead!)",
                    ]),
                ],
                recipe_yield: to_yield(16),
                total_time: vec![DurationOrText::Text("PT4500S".into())],
                ..Default::default()
            }]
        }
    }
}
