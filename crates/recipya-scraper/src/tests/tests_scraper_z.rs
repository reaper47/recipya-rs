#[cfg(test)]
use schema_org::{
    AggregateRating, AtType, CreativeWork, DurationOrText, NutritionInformation, Organization,
    Recipe, at_context,
    enums::RestrictedDietEnum,
    field::{
        RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
        RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum, RecipeRecipeYieldFieldEnum,
    },
};

#[cfg(test)]
use crate::{tests::support::scraper::scrape, websites::Website};

#[cfg(test)]
type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[test]
#[tracing_test::traced_test]
#[ignore = "Needs manual testing"]
fn test_zaatar_and_zaytoun_dot_com() -> Result<()> {
    let got = scrape(Website::ZaatarAndZaytounDotCom, 0)?;

    let want = Recipe {
        context: at_context(),
        r#type: AtType::Recipe.to_opt(),
        aggregate_rating: vec![AggregateRating::new(5.0, 1)],
        author: vec![RecipeAuthorFieldEnum::new_person("Zaatar and Zaytoun")],
        cook_time: vec![DurationOrText::Text("30 minutes".into())],
        date_modified: vec!["2023-08-16T09:55:44+00:00".into()],
        date_published: vec!["2023-08-05T19:42:35+00:00".into()],
        description: vec![RecipeDescriptionFieldEnum::Text(
            "Spaghetti with sardines and black olives in a light tomato sauce".into(),
        )],
        keywords: vec![
            RecipeKeywordsFieldEnum::TextOrURL("Light".into()),
            RecipeKeywordsFieldEnum::TextOrURL("Pasta".into()),
            RecipeKeywordsFieldEnum::TextOrURL("Quick and Easy".into()),
        ],
        image: vec![
            RecipeImageFieldEnum::URL(
                "https://zaatarandzaytoun.com/wp-content/uploads/2023/06/sardines-pasta-44-scaled.jpg".into(),
            ),
            RecipeImageFieldEnum::URL(
                "https://zaatarandzaytoun.com/wp-content/uploads/2018/09/mama-e1537703662257.jpg".into(),
            ),
            RecipeImageFieldEnum::URL(
                "https://zaatarandzaytoun.com/wp-content/uploads/2018/09/mama-e1537703662257.jpg".into(),
            ),
        ],
        name: vec!["Pasta with Sardines and Olives".into()],
        recipe_category: vec!["Main Course".into()],
        recipe_cuisine: vec!["Lebanese".into(), "Mediterranean".into()],
        recipe_ingredient: vec![
            RecipeRecipeIngredientFieldEnum::new_section(
                "To make the tomato sauce",
                vec![
                    "4-6 tomatoes",
                    "½ teaspoon tomato concentrate",
                    "1 clove garlic whole",
                    "pinch dried oregano",
                    "salt to taste",
                    "drizzle mild olive oil",
                ],
            ),
            RecipeRecipeIngredientFieldEnum::new_section(
                "Other",
                vec![
                    "Spaghetti for 2 people approx 100g",
                    "1 tin sardines approx 75g",
                    "handful black olives",
                    "squeeze of lemon",
                    "salt to taste",
                ],
            ),
        ],
        recipe_instructions: vec![
               RecipeRecipeInstructionsFieldEnum::Text(
                   "In a medium pot, add the roughly chopped tomatoes, a drizzle of mild olive oil, tomato concentrate and a sprinkle of salt along with the clove of garlic. Bring to the boil on medium high heat for around 20 minutes".into(),
               ),
               RecipeRecipeInstructionsFieldEnum::Text(
                   "Take out the garlic clove and blitz using a hand blender. Set aside".into(),
               ),
               RecipeRecipeInstructionsFieldEnum::Text(
                   "Cook the spaghetti as per packet instructions using well salted boiling water".into(),
               ),
               RecipeRecipeInstructionsFieldEnum::Text(
                   "As the spaghetti is cooking, pit the olives and drain the sardines. Transfer ½ cup of tomato sauce to a deep pan and heat on low. 2 minutes before the pasta is done, transfer to the tomato sauce".into(),
               ),
               RecipeRecipeInstructionsFieldEnum::Text(
                   "Add the sardines, olives, squeeze of lemon and tiny sprinkle of salt. Warm through for a few minutes, turning gently so as not to break up the sardines too much.".into(),
               ),
               RecipeRecipeInstructionsFieldEnum::Text(
                   "Taste to adjust any seasoning as necessary and transfer to a pasta bowl. Drizzle on optional extra virgin olive oil or a sprinkle of cayenne pepper".into(),
               ),
    ],
        recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("2".into())],
        url: vec!["https://zaatarandzaytoun.com/sardines/".into()],
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}

#[test]
#[tracing_test::traced_test]
#[ignore = "Needs manual testing"]
fn test_zabihahalal_dot_com() -> Result<()> {
    let got = scrape(Website::ZabihaHalal, 0)?;

    let want = Recipe {
        context: Some(String::from("https://schema.org/")),
        r#type: AtType::Recipe.to_opt(),
        author: vec![RecipeAuthorFieldEnum::Organization(Organization {
            r#type: AtType::Person.to_opt(),
            name: vec!["Maple Lodge Farms".into()],
            ..Default::default()
        })],
        date_published: vec!["2020-01-29".into()],
        description: vec![RecipeDescriptionFieldEnum::Text(
            "&lt;p&gt;BBQ chicken breast is the best thing to happen to pizza since mozzarella cheese. And topped with bell peppers and red onion? Fuhgeddaboudit!&lt;/p&gt; ".into(),
        )],
        image: vec![RecipeImageFieldEnum::URL("https://zabihahalal.com/wp-content/uploads/2023/06/BBQ-Chicken-Pizza-004-1-650x650-1-150x150.jpg".into())],
        keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("BBQ Chicken Pizza".into())],
        name: vec!["BBQ Chicken Pizza".into()],
        nutrition: vec![
                NutritionInformation {
                    serving_size: vec!["2".into()],
                    r#type: AtType::NutritionInformation.to_opt(),
                    ..Default::default()
                },
            ],
        prep_time: vec![DurationOrText::Text("PT60M".into())],
        recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("2".into())],
        recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("1 piece Boneless, Skinless Chicken Breasts".into()),
                RecipeRecipeIngredientFieldEnum::Text("Salt & pepper".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/2 tsp dried oregano".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/2 tsp dried basil".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/2 tsp garlic powder".into()),
                RecipeRecipeIngredientFieldEnum::Text("Pizza".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 pre-baked pizza crust".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 cup barbecue sauce".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/2 cup sliced red bell peppers".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 cup chopped red onion".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 cups shredded Colby-Monterey Jack cheese".into()),
            ],
        recipe_instructions: vec![
            RecipeRecipeInstructionsFieldEnum::CreativeWork(Box::new(CreativeWork::new(AtType::HowToStep, "1. Grilled Chicken"))),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(Box::new(CreativeWork::new(AtType::HowToStep, "2. Mix together all seasonings in mixing bowl."))),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(Box::new(CreativeWork::new(AtType::HowToStep, "3. Add seasonings to both sides of the raw Zabiha Halal Boneless, Skinless Chicken Breasts"))),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(Box::new(CreativeWork::new(AtType::HowToStep, "4. Spray grill pan with cooking spray, and preheat on the stovetop over medium high heat for about one minute."))),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(Box::new(CreativeWork::new(AtType::HowToStep, "5.Place the chicken breasts on the hot grill pan."))),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(Box::new(CreativeWork::new(AtType::HowToStep, "6. Cook 6 minutes on each side or until done."))),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(Box::new(CreativeWork::new(AtType::HowToStep, "7. Pizza- Preheat oven to 350 degrees F"))),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(Box::new(CreativeWork::new(AtType::HowToStep, "8. Cut pre cooked chicken, red bell peppers and onions. Grate cheese."))),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(Box::new(CreativeWork::new(AtType::HowToStep, "9.Place pizza crust on a medium baking sheet. Spread barbecue sauce over the crust and top with cheese, chicken, onion and red bell peppers."))),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(Box::new(CreativeWork::new(AtType::HowToStep, "10.Bake in the preheated oven for 10 minutes or until cheese looks well melted."))),
        ],
        suitable_for_diet: vec![RestrictedDietEnum::HalalDiet],
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}

#[test]
fn test_zeit_dot_de() {
    todo!();
}

#[test]
fn test_zenbelly_dot_com() {
    todo!();
}
