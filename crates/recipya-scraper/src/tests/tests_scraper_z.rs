#[cfg(test)]
use schema_org::{
    AggregateRating, AtType, Clip, CreativeWork, DurationOrText, Energy, Mass,
    NutritionInformation, Organization, Rating, Recipe, Review, at_context,
    enums::RestrictedDietEnum,
    field::{
        AggregateRatingRatingValueFieldEnum, RatingRatingValueFieldEnum, RecipeAuthorFieldEnum,
        RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeKeywordsFieldEnum,
        RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
        RecipeRecipeYieldFieldEnum, RecipeVideoFieldEnum, ReviewAuthorFieldEnum,
    },
};

#[cfg(test)]
use crate::{tests::support::scraper::scrape, websites::Website};

#[cfg(test)]
type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[test]
#[tracing_test::traced_test]
#[ignore = "Needs manual testing"]
fn test_zaatar_and_zaytoun_ok() -> Result<()> {
    let got = scrape(Website::ZaatarAndZaytoun, 0)?;

    let want = Recipe {
        context: at_context(),
        r#type: AtType::Recipe.to_opt(),
        aggregate_rating: vec![AggregateRating::new(5.0, 1)],
        author: vec![RecipeAuthorFieldEnum::new_org("Zaatar and Zaytoun")],
        cook_time: vec![DurationOrText::Text("PT30M".into())],
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
                "https://zaatarandzaytoun.com/wp-content/uploads/2023/08/sardines.jpg".into(),
            ),
            RecipeImageFieldEnum::URL(
                "https://zaatarandzaytoun.com/wp-content/uploads/2023/08/sardines-500x500.jpg".into(),
            ),
            RecipeImageFieldEnum::URL(
                "https://zaatarandzaytoun.com/wp-content/uploads/2023/08/sardines-500x375.jpg".into(),
            ),
            RecipeImageFieldEnum::URL(
                "https://zaatarandzaytoun.com/wp-content/uploads/2023/08/sardines-480x270.jpg".into(),
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
        review: vec![
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec!["I find your tips very useful thank you".into()],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2023-08-05".into()],
                author: vec![
                    ReviewAuthorFieldEnum::Organization(
                        Organization {
                            r#type: AtType::Person.to_opt(),
                            name: vec!["Sarina".into()],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
        ],
        url: vec!["https://zaatarandzaytoun.com/sardines/".into()],
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}

#[test]
#[tracing_test::traced_test]
#[ignore = "Needs manual testing"]
fn test_zabihahalal_ok() -> Result<()> {
    let got = scrape(Website::ZabihaHalal, 0)?;

    let want = Recipe {
        context: at_context(),
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
        url: vec!["https://zabihahalal.com/recipes/bbq-chicken-pizza/".into()],
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}

#[test]
#[tracing_test::traced_test]
#[ignore = "Needs manual testing"]
fn test_zagleft_ok() -> Result<()> {
    let got = scrape(Website::ZagLeft, 0)?;

    let want = Recipe {
        context: at_context(),
        r#type: AtType::Recipe.to_opt(),
        author: vec![RecipeAuthorFieldEnum::new_person("Joanie Zisk")],
        cook_time: vec![DurationOrText::Text("PT18M".into())],
        date_modified: vec!["2025-12-17T18:49:44+00:00".into()],
        date_published: vec!["2019-12-05T15:18:00+00:00".into()],
        description: vec![RecipeDescriptionFieldEnum::Text("Banana Oat Pecan Muffins , a muffin recipe I guarantee your whole family will love!&amp;nbsp;".into())],
        image: vec![
            RecipeImageFieldEnum::URL("https://zagleft.com/wp-content/uploads/2019/12/banana-oat-pecan-muffins-zagleft-1-1-scaled.jpg".into()),
            RecipeImageFieldEnum::URL("https://zagleft.com/wp-content/uploads/2019/12/banana-oat-pecan-muffins-zagleft-1-1-500x500.jpg".into()),
            RecipeImageFieldEnum::URL("https://zagleft.com/wp-content/uploads/2019/12/banana-oat-pecan-muffins-zagleft-1-1-500x375.jpg".into()),
            RecipeImageFieldEnum::URL("https://zagleft.com/wp-content/uploads/2019/12/banana-oat-pecan-muffins-zagleft-1-1-480x270.jpg".into()),
        ],
        name: vec!["Banana Oat Pecan Muffins".into()],
        prep_time: vec![DurationOrText::Text("PT10M".into())],
        recipe_category: vec!["Breakfast".into()],
        recipe_cuisine: vec!["Muffins".into()],
        recipe_ingredient: vec![
            RecipeRecipeIngredientFieldEnum::Text("1 egg, (beaten)".into()),
            RecipeRecipeIngredientFieldEnum::Text("3/4 cup milk".into()),
            RecipeRecipeIngredientFieldEnum::Text("1/3 cup butter, (melted)".into()),
            RecipeRecipeIngredientFieldEnum::Text("1/2 teaspoon vanilla".into()),
            RecipeRecipeIngredientFieldEnum::Text("1 1/2 cup all-purpose flour".into()),
            RecipeRecipeIngredientFieldEnum::Text("1 cup old-fashioned oats".into()),
            RecipeRecipeIngredientFieldEnum::Text("1/2 cup sugar".into()),
            RecipeRecipeIngredientFieldEnum::Text("2 teaspoons baking powder".into()),
            RecipeRecipeIngredientFieldEnum::Text("1 teaspoon baking soda".into()),
            RecipeRecipeIngredientFieldEnum::Text("1/2 teaspoon salt".into()),
            RecipeRecipeIngredientFieldEnum::Text("4 cup medium bananas ((about 1 mashed))".into()),
            RecipeRecipeIngredientFieldEnum::Text("1/2 cup pecans, (chopped)".into()),
        ],
        recipe_instructions: vec![
            RecipeRecipeInstructionsFieldEnum::Text("Heat oven to 350 degrees F.".into()),
            RecipeRecipeInstructionsFieldEnum::Text("Combine the egg, milk, butter and vanilla in a large bowl. Set aside.".into()),
            RecipeRecipeInstructionsFieldEnum::Text("In a small bowl, whisk together the flour, oats, sugar, baking powder, baking soda and salt.".into()),
            RecipeRecipeInstructionsFieldEnum::Text("Stir the dry ingredients into the wet ingredients and combine thoroughly.".into()),
            RecipeRecipeInstructionsFieldEnum::Text("Gently fold in the mashed bananas and the chopped pecans.".into()),
            RecipeRecipeInstructionsFieldEnum::Text("Lightly grease a 12-cup standard muffin tin or line with paper liners.".into()),
            RecipeRecipeInstructionsFieldEnum::Text("Divide the batter between the prepared muffin cups.".into()),
            RecipeRecipeInstructionsFieldEnum::Text("Bake for 15-18 minutes, until the tops of the muffins are golden and a toothpick inserted in the center comes out clean.".into()),
            RecipeRecipeInstructionsFieldEnum::Text("Place the pan on a wire rack and let the muffins cool in the pan for 10 minutes.".into()),
            RecipeRecipeInstructionsFieldEnum::Text("Remove from the pan and enjoy!".into()),
        ],
        recipe_yield: vec![
            RecipeRecipeYieldFieldEnum::Text("12".into()),
            RecipeRecipeYieldFieldEnum::Text("12 muffins".into()),
        ],
        total_time: vec![DurationOrText::Text("PT28M".into())],
        url: vec!["https://zagleft.com/banana-oat-pecan-muffins/".into()],
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}

#[test]
#[tracing_test::traced_test]
#[ignore = "Needs manual testing"]
fn test_zardyplants_ok() -> Result<()> {
    let got = scrape(Website::ZardyPlants, 0)?;

    let want = Recipe {
        context: at_context(),
        r#type: AtType::Recipe.to_opt(),
        aggregate_rating: vec![
            AggregateRating {
                r#type: AtType::AggregateRating.to_opt(),
                rating_value: vec![
                    AggregateRatingRatingValueFieldEnum::Text("4.9".into()),
                ],
                ..Default::default()
            },
        ],
        author: vec![RecipeAuthorFieldEnum::Organization(Organization {
            r#type: AtType::Person.to_opt(),
            name: vec!["Liz Madsen".into()],
            url: vec!["https://zardyplants.com/about/".into()],
            ..Default::default()
        })],
        cooking_method: vec!["Stove top".into()],
        cook_time: vec![DurationOrText::Text("PT20M".into())],
        date_published: vec!["2021-02-11".into()],
        description: vec![RecipeDescriptionFieldEnum::Text(
            "Rich, cozy and PACKED with flavor, this easy Vegan Tom Kha Soup is a Thai-inspired recipe that is simple and quick to make.".into(),
        )],
        image: vec![
            RecipeImageFieldEnum::URL(
                "https://zardyplants.com/wp-content/uploads/2021/02/Vegan-Tom-Kha-05-rotated-225x225.jpg".into(),
            ),
            RecipeImageFieldEnum::URL(
                "https://zardyplants.com/wp-content/uploads/2021/02/Vegan-Tom-Kha-05-rotated-260x195.jpg".into(),
            ),
            RecipeImageFieldEnum::URL(
                "https://zardyplants.com/wp-content/uploads/2021/02/Vegan-Tom-Kha-05-rotated-320x180.jpg".into(),
            ),
            RecipeImageFieldEnum::URL(
                "https://zardyplants.com/wp-content/uploads/2021/02/Vegan-Tom-Kha-05-rotated.jpg".into(),
            ),
        ],
        keywords: vec![
            RecipeKeywordsFieldEnum::TextOrURL("Vegan, Gluten-Free, Oil-Free, Nut-Free, Can Be Sugar-Free, Vegan Tom Kha".into()),
        ],
        name: vec!["Vegan Tom Kha Soup (Thai Inspired)".into()],
        nutrition: vec![NutritionInformation {
            calories: vec![Energy::new("409 calories")],
            carbohydrate_content: vec![Mass::new("33.4 g")],
            cholesterol_content: vec![Mass::new("0 mg")],
            context: None,
            fat_content: vec![Mass::new("23.8 g")],
            fiber_content: vec![Mass::new("5.7 g")],
            protein_content: vec![Mass::new("18 g")],
            saturated_fat_content: vec![Mass::new("15.2 g")],
            serving_size: vec!["2 cups".into()],
            sodium_content: vec![Mass::new("227.9 mg")],
            sugar_content: vec![Mass::new("13.1 g")],
            r#type: Some("nutritionInformation".into()),
            trans_fat_content: vec![Mass::new("0 g")],
            unsaturated_fat_content: vec![],
        }],
        prep_time: vec![DurationOrText::Text("PT10M".into())],
        recipe_category: vec!["Entree".into()],
        recipe_cuisine: vec!["Thai".into()],
        recipe_ingredient: vec![
            RecipeRecipeIngredientFieldEnum::Text("1 small white onion, sliced or diced".into()),
            RecipeRecipeIngredientFieldEnum::Text("2 inches (to taste) galangal or ginger, grated".into()),
            RecipeRecipeIngredientFieldEnum::Text("Thai chilies to taste, minced (see note 2) (I used 3)".into()),
            RecipeRecipeIngredientFieldEnum::Text("1 stalk fresh lemongrass if you can find it or 1-2 tbsp lemongrass paste".into()),
            RecipeRecipeIngredientFieldEnum::Text("2-3 tsp Thai red curry paste (to taste)".into()),
            RecipeRecipeIngredientFieldEnum::Text("1 pound shiitake mushrooms, chopped".into()),
            RecipeRecipeIngredientFieldEnum::Text("1 large red bell pepper, halved and thinly sliced".into()),
            RecipeRecipeIngredientFieldEnum::Text("4 cups vegan chicken broth (see note 3), recommend Better Than Bouillon".into()),
            RecipeRecipeIngredientFieldEnum::Text("2 cans coconut milk".into()),
            RecipeRecipeIngredientFieldEnum::Text("2 16-oz blocks extra firm tofu, cut in ½” dice (see note 4)".into()),
            RecipeRecipeIngredientFieldEnum::Text("Juice of 2-ish limes".into()),
            RecipeRecipeIngredientFieldEnum::Text("2-3 tbsp coconut sugar, to taste".into()),
            RecipeRecipeIngredientFieldEnum::Text("Green onions and cilantro to garnish".into()),
            RecipeRecipeIngredientFieldEnum::Text("Kaffir lime leaves if you can find them".into()),
            RecipeRecipeIngredientFieldEnum::Text("3 cloves garlic".into()),
            RecipeRecipeIngredientFieldEnum::Text("8 oz rice vermicelli noodles, optional".into()),
        ],
        recipe_instructions: vec![
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "When making a soup, I like to pre-chop all my vegetables for less stress during cooking time, but that’s just a tip.".into(),
                    ],
                    url: vec![
                        "https://zardyplants.com/recipes/vegan-tom-kha-soup-thai-inspired/#instruction-step-1".into(),
                    ],
                    name: vec!["Prep".into()],
                    ..Default::default()
                }.into(),
            ),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Add the white onion, grated ginger, minced garlic, fresh lemongrass if you have it, and minced Thai chilies to a large nonstick pot. If you cook with oil you could certainly use that. I just add a little water from a small cup any time something starts to stick. Saute over medium high heat, stirring constantly, for 1-2 minutes until fragrant. Add the Thai red curry paste&nbsp;and, if you couldn’t find fresh lemongrass, the lemongrass paste. Stir constantly for another 1-2 minutes.".into(),
                    ],
                    url: vec![
                        "https://zardyplants.com/recipes/vegan-tom-kha-soup-thai-inspired/#instruction-step-2".into(),
                    ],
                    name: vec!["Saute aromatics".into()],
                    ..Default::default()
                }.into(),
            ),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Now add the chopped mushrooms and red bell pepper, and saute for 5 minutes, stirring frequently and adding a splash of water when needed.".into(),
                    ],
                    url: vec![
                        "https://zardyplants.com/recipes/vegan-tom-kha-soup-thai-inspired/#instruction-step-3".into(),
                    ],
                    name: vec!["Add mushrooms and bell pepper".into()],
                    ..Default::default()
                }.into(),
            ),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Add the vegan chicken broth or vegetable broth and coconut milk, turn the heat down to medium, and bring to a high simmer. Add the cubed tofu and let cook for about 6-8 minutes.".into(),
                    ],
                    url: vec![
                        "https://zardyplants.com/recipes/vegan-tom-kha-soup-thai-inspired/#instruction-step-4".into(),
                    ],
                    name: vec!["Add broth".into()],
                    ..Default::default()
                }.into(),
            ),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Meanwhile, if using, soak or cook your rice vermicelli noodles&nbsp;in a different pot.".into(),
                    ],
                    url: vec![
                        "https://zardyplants.com/recipes/vegan-tom-kha-soup-thai-inspired/#instruction-step-5".into(),
                    ],
                    name: vec!["Make the noodles".into()],
                    ..Default::default()
                }.into(),
            ),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Add the coconut sugar (if using), and juice of 2ish limes (to taste). Let warm for a minute or two, then remove from heat.".into(),
                    ],
                    url: vec![
                        "https://zardyplants.com/recipes/vegan-tom-kha-soup-thai-inspired/#instruction-step-6".into(),
                    ],
                    name: vec!["Finish up".into()],
                    ..Default::default()
                }.into(),
            ),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Ladle the soup into bowls and add noodles to each bowl (if using). Garnish with thinly sliced green onions and lots of cilantro (if you like) and a few lime wedges.".into(),
                    ],
                    url: vec![
                        "https://zardyplants.com/recipes/vegan-tom-kha-soup-thai-inspired/#instruction-step-7".into(),
                    ],
                    name: vec!["Serve".into()],
                    ..Default::default()
                }.into(),
            ),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Store leftover soup separately from the noodles. They’ll each keep in the fridge for up to 5 days in airtight containers. The noodles tend to soak up the broth, so I recommend storing separately when you can. This is a GREAT dish to take to work or school though. I often will make it the night before I go to work and take it for lunch the next day. I pack a lime wedge and some cilantro in a reusable stasher bag&nbsp;to add at work.".into(),
                    ],
                    url: vec![
                        "https://zardyplants.com/recipes/vegan-tom-kha-soup-thai-inspired/#instruction-step-8".into(),
                    ],
                    name: vec!["Store".into()],
                    ..Default::default()
                }.into(),
            ),
        ],
        recipe_yield: vec![
            RecipeRecipeYieldFieldEnum::Text("12".into()),
            RecipeRecipeYieldFieldEnum::Text("12 cups".into()),
        ],
        review: vec![
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec!["Delicious! Full of flavor and perfect for this chilly weather!".into()],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2021-02-14".into()],
                author: vec![
                    ReviewAuthorFieldEnum::Organization(
                        Organization {
                            r#type: AtType::Person.to_opt(),
                            name: vec!["Mary".into()],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec!["Just had this tonight. Saw fresh lemongrass at the supermarket and thought, game on. Super yummy. I love the pairing of the sweet of the coconut milk and the spicy of the hot pepper— though I ended up using a jalapeño but it worked out pretty well. I ended up adding the rice noodles and it made quite a filling meal. Excited that there is plenty leftover for tomorrow’s lunch!".into()],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Text("5".into()),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2021-02-17".into()],
                author: vec![
                    ReviewAuthorFieldEnum::Organization(
                        Organization {
                            r#type: AtType::Person.to_opt(),
                            name: vec!["Stephanie".into()],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "Really delicious! This was so easy to make and packed with flavor!".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2021-02-21".into()],
                author: vec![
                    ReviewAuthorFieldEnum::Organization(
                        Organization {
                            r#type: AtType::Person.to_opt(),
                            name: vec!["Erin".into()],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "So delicious! Easy to make and lasted us several meals. Will definitely be repeating this recipe on the regular throughout the colder months.".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Text("5".into()),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2021-03-02".into()],
                author: vec![
                    ReviewAuthorFieldEnum::Organization(
                        Organization {
                            r#type: AtType::Person.to_opt(),
                            name: vec!["Christina".into()],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "Wow. So great! I was able to buy Kaffir lime leaves and lemon grass on line and it was so worth it. Better than the Tom Kha at my favorite veggie Thai place. My whole family loved it and was amazed.".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2021-08-06".into()],
                author: vec![
                    ReviewAuthorFieldEnum::Organization(
                        Organization {
                            r#type: AtType::Person.to_opt(),
                            name: vec!["Carol Booth".into()],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "Looove this!!!! Also, love the way you write these recipe posts! And pictures are terrific! You are my new favorite vegan chef!".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Text("5".into()),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2022-01-30".into()],
                author: vec![
                    ReviewAuthorFieldEnum::Organization(
                        Organization {
                            r#type: AtType::Person.to_opt(),
                            name: vec!["Karen Flynn".into()],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "Hey what stage do you add the kaffir leaves?".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Text("5".into()),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2022-06-17".into()],
                author: vec![
                    ReviewAuthorFieldEnum::Organization(
                        Organization {
                            r#type: AtType::Person.to_opt(),
                            name: vec!["Neo".into()],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "When do you add the lime leaves???".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Text("4".into()),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2025-01-24".into()],
                author: vec![
                    ReviewAuthorFieldEnum::Organization(
                        Organization {
                            r#type: AtType::Person.to_opt(),
                            name: vec!["Cat".into()],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "This is a really great recipe. So yummy! Thanks!".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Text("5".into()),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2023-02-25".into()],
                author: vec![
                    ReviewAuthorFieldEnum::Organization(
                        Organization {
                            r#type: AtType::Person.to_opt(),
                            name: vec!["Andrea".into()],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "Well, I made this even less authentic due to having to sub a few key ingredients... (stores are closed today and really wanted to make this)\r\nOnly had dried mushrooms and soy milk, but figured it's worth a try and am so glad that we did. This soup, even with above modifications, is outstanding. \r\nEasy to make, smells divine and tastes even better.\r\n(Make it without the noddles)\r\nYummy!\r\n\r\nThank you for sharing".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Text("5".into()),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2023-12-26".into()],
                author: vec![
                    ReviewAuthorFieldEnum::Organization(
                        Organization {
                            r#type: AtType::Person.to_opt(),
                            name: vec!["in2insight".into()],
                            ..Default::default()
                        },
                    ),
                ],
                ..Default::default()
            },
        ],
        suitable_for_diet: vec![RestrictedDietEnum::VeganDiet],
        total_time: vec![DurationOrText::Text("PT30M".into())],
        url: vec!["https://zardyplants.com/recipes/vegan-tom-kha-soup-thai-inspired/".into()],
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}

#[test]
#[tracing_test::traced_test]
#[ignore = "Needs manual testing"]
fn test_zarskitchen_ok() -> Result<()> {
    let got = scrape(Website::Zarskitchen, 0)?;

    let want = Recipe {
        context: at_context(),
        r#type: AtType::Recipe.to_opt(),
        aggregate_rating: vec![AggregateRating {
            r#type: AtType::AggregateRating.to_opt(),
            rating_value: vec![AggregateRatingRatingValueFieldEnum::Text("5.0".into())],
            ..Default::default()
        }],
        author: vec![RecipeAuthorFieldEnum::new_org("Zarina")],
        cook_time: vec![DurationOrText::Text("PT".into())],
        date_published: vec!["2021-04-13".into()],
        description: vec![RecipeDescriptionFieldEnum::Text("Wild Garlic Pesto".into())],
        image: vec![RecipeImageFieldEnum::URL(
            "https://zarskitchen.com/wp-content/uploads/2021/04/Zars-Kitchen-Recipe-10.jpg".into(),
        )],
        keywords: vec![RecipeKeywordsFieldEnum::TextOrURL(
            "Wild Garlic Pesto".into(),
        )],
        name: vec!["Wild Garlic Pesto".into()],
        nutrition: vec![
            NutritionInformation {
                calories: vec![Energy::new("200")],
                fat_content: vec![Mass::new("20 grams"),
                ],
                r#type: AtType::NutritionInformation.to_opt(),
                ..Default::default()
            },
        ],
        recipe_ingredient: vec![
            RecipeRecipeIngredientFieldEnum::Text(
                "75g wild garlic".into(),
            ),
            RecipeRecipeIngredientFieldEnum::Text(
                "150g parmesan, grated".into(),
            ),
            RecipeRecipeIngredientFieldEnum::Text(
                "100g rocket".into(),
            ),
            RecipeRecipeIngredientFieldEnum::Text(
                "100g toasted pine nuts".into(),
            ),
            RecipeRecipeIngredientFieldEnum::Text(
                "1 lemon juiced ".into(),
            ),
            RecipeRecipeIngredientFieldEnum::Text(
                "150ml olive oil".into(),
            ),
            RecipeRecipeIngredientFieldEnum::Text(
                "Salt, Pepper &amp; chilli flakes to taste.".into(),
            ),
        ],
        recipe_instructions: vec![
            RecipeRecipeInstructionsFieldEnum::Text(
                "Place all your ingredients in a food processor until you get your desired consistency, add more olive oil if needed.".into(),
            ),
            RecipeRecipeInstructionsFieldEnum::Text(
                "Any leftovers can be kept in the fridge for up to a week in a clean jar, just top with a little oil to create a seal.".into(),
            ),
        ],
        recipe_category: vec!["Pasta".into()],
        recipe_cuisine: vec!["European".into()],
        recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("4".into())],
        prep_time: vec![DurationOrText::Text("PT10M".into())],
        total_time: vec![DurationOrText::Text("PT".into())],
        url: vec!["https://zarskitchen.com/wild-garlic-pesto/".into()],
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}

#[test]
#[tracing_test::traced_test]
#[ignore = "Needs manual testing"]
fn test_zeezest_ok() -> Result<()> {
    let got = scrape(Website::Zeezest, 0)?;

    let want = Recipe {
        context: at_context(),
        r#type: AtType::Recipe.to_opt(),
        author: vec![RecipeAuthorFieldEnum::new_person("Team ZZ")],
        cook_time: vec![DurationOrText::Text("12-15 Min".into())],
        description: vec![
            RecipeDescriptionFieldEnum::Text("Multigrain Onion Dosa is a type of thin and crispy crepe that is made with a fermented batter of rice and lentils.".into()),
        ],
        image: vec![
            RecipeImageFieldEnum::URL(
                "https://assets.zeezest.com/recipes/PROD_Multigrain Onion Dosa with Chutney_SEO_1682270069564_thumb_500.jpeg".into(),
            ),
        ],
        name: vec!["Multigrain Onion Dosa with Chutney" .into()],
        prep_time: vec![DurationOrText::Text("18-20 Min".into())],
        recipe_category: vec!["breakfast".into()],
        recipe_ingredient: vec![
            RecipeRecipeIngredientFieldEnum::new_section("For soaking", vec![
                "¾ cup par boiled rice (Ukhad)",
                "2 tbsp chana dal",
                "2 tbsp urad dal",
                "2 tbsp yellow moong dal",
                "2 tbsp tur dal",
                "2 tbsp split green moong dal",
                "2 tbsp rajgira",
            ]),
            RecipeRecipeIngredientFieldEnum::new_section("For cooking", vec![
                "2 tbsp butter",
                "gun powder",
                "2 tbsp chopped onion",
                "½ tsp chopped green chilli",
                "1 tsp chopped coriander leaves",
            ]),
            RecipeRecipeIngredientFieldEnum::new_section("For Onion tomato chutney", vec![
                "1 tbsp oil",
                "1 tsp urad dal",
                "1 cup diced onion",
                "1 cup diced tomato",
                "1 tbsp Kashmiri red powder",
                "½ inch ginger",
                "2-3 garlic cloves",
                "1 tsp tamarind pulp",
                "salt to taste",
            ]),
            RecipeRecipeIngredientFieldEnum::new_section("For tadka", vec![
                "2 tsp oil",
                "1 tsp mustard seeds",
                "few curry leaves",
            ]),
        ],
        recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Take a bowl and add rice, chana dal, urad dal, yellow moong dal, green moong dal, tur dal, masoor dal, rajgira. Wash it with water and soak it for 4-5 hours.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "In a blender jar, add soaked lentils and rice and grind it into fine paste.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Add water in it and let it rest for 2-3 hours.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Heat oil in a pan, add urad dal, onion, ginger, garlic, tomato and saute it for 1 minute.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Add red chilli, tamarind paste, salt and mix well.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Grind it into a fine paste.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Add salt in the batter before spreading over the pan.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Heat oil in a griddle pan, pour the batter.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Spread the ladleful of batter into a circle.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Add butter and spread it, add onion, green chilli, gunpowder and spread it . Sprinkle coriander leaves fold and serve hot.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Heat oil in a small non-stick pan, add mustard seeds, curry leaves and sauté.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Pour the tadka over the chutney.".into(),
                ),
        ],
        url: vec!["https://zeezest.com/recipes/watch-multigrain-onion-dosa-with-chutney-recipe-by-zee-zest-1302".into()],
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}

#[test]
#[tracing_test::traced_test]
#[ignore = "Needs manual testing"]
fn test_zenbelly_ok() -> Result<()> {
    let got = scrape(Website::Zenbelly, 0)?;

    let want = Recipe {
        context: at_context(),
        r#type: AtType::Recipe.to_opt(),
        aggregate_rating: vec![AggregateRating {
            r#type: AtType::AggregateRating.to_opt(),
            rating_value: vec![AggregateRatingRatingValueFieldEnum::Text("5".into())],
            ..Default::default()
        }],
        author: vec![
            RecipeAuthorFieldEnum::Organization(Organization {
                r#type: AtType::Person.to_opt(),
                name: vec!["zenbelly".into()],
                url: vec!["http://Simone%20Miller".into()],
                ..Default::default()
            }),
        ],
        cooking_method: vec!["roast".into()],
        cook_time: vec![DurationOrText::Text("PT30M".into())],
        date_published: vec!["2023-02-04".into()],
        keywords: vec![
            RecipeKeywordsFieldEnum::TextOrURL("crispy chickpeas".into()),
        ],
        image: vec![
            RecipeImageFieldEnum::URL(
                "https://www.zenbelly.com/wp-content/uploads/2023/02/crispy-chickpeas--225x225.png".into(),
            ),
            RecipeImageFieldEnum::URL(
                "https://www.zenbelly.com/wp-content/uploads/2023/02/crispy-chickpeas--260x195.png".into(),
            ),
            RecipeImageFieldEnum::URL(
                "https://www.zenbelly.com/wp-content/uploads/2023/02/crispy-chickpeas--320x180.png".into(),
            ),
            RecipeImageFieldEnum::URL(
                "https://www.zenbelly.com/wp-content/uploads/2023/02/crispy-chickpeas-.png".into(),
            ),
        ],
        name: vec!["Easy Crispy Chickpeas".into()],
        prep_time: vec![DurationOrText::Text("PT5M".into())],
        recipe_category: vec!["snacks".into()],
        recipe_ingredient: vec![
            RecipeRecipeIngredientFieldEnum::Text(
                "1 14-oz can chickpeas".into(),
            ),
            RecipeRecipeIngredientFieldEnum::Text(
                "extra virgin olive oil".into(),
            ),
            RecipeRecipeIngredientFieldEnum::Text(
                "salt".into(),
            ),
            RecipeRecipeIngredientFieldEnum::Text(
                "seasoning of choice &amp;#8211; anything works here, and you can also just salt them. I especially love them with chili powder or ras el hanout.".into(),
            ),
        ],
        recipe_instructions: vec![
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Heat the oven to 425ºF and line a rimmed baking sheet with parchment paper.".into(),
                    ],
                    url: vec![
                        "https://www.zenbelly.com/crispy-chickpeas/#instruction-step-1".into(),
                    ],
                    ..Default::default()
                }.into(),
            ),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Drain the chickpeas, reserving the liquid from the jar and 10-12 chickpeas if you’d like to use it for vegan mayo. Shake off as much water from the chickpeas as you can, and then transfer to a clean kitchen towel. Roll them around to loosen the skins, and discard any that come off. You can peel each chickpea if you’d like, but you don’t have to. (I started doing it and couldn’t stop and found it to be somewhat cathartic) Get the chickpeas as dry as possible, using paper towels or another kitchen towel.".into(),
                    ],
                    url: vec![
                        "https://www.zenbelly.com/crispy-chickpeas/#instruction-step-2".into(),
                    ],
                    ..Default::default()
                }.into(),
            ),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Transfer the chickpeas to the parchment lined baking sheet and drizzle with enough oil to coat. Sprinkle with salt and toss to coat. Roast for 15 minutes, shaking the pan about halfway through. Turn the heat down to 400 and roast another 10 minutes, or until they’re golden brown and crisp. If they are as brown as you want but could use some mire drying out, turn off the oven and leave them in there with a wooden spoon propping the door ajar.".into(),
                    ],
                    url: vec![
                        "https://www.zenbelly.com/crispy-chickpeas/#instruction-step-3".into(),
                    ],
                    ..Default::default()
                }.into(),
            ),
            RecipeRecipeInstructionsFieldEnum::CreativeWork(
                CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Allow to cool completely and store in an airtight jar. If you get them completely dried out during the cooking process, they should stay crispy, but if they lose their crunch &amp;#8211; pop them in a 350 oven for 5 minutes.".into(),
                    ],
                    url: vec![
                        "https://www.zenbelly.com/crispy-chickpeas/#instruction-step-4".into(),
                    ],
                    ..Default::default()
                }.into(),
            ),
        ],
        recipe_yield: vec![
            RecipeRecipeYieldFieldEnum::Text("1".into()),
            RecipeRecipeYieldFieldEnum::Text("1 cup".into()),
        ],
        review: vec![
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "It’s perfect for a quick and healthy snack that still packs a punch".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        rating_value: vec![RatingRatingValueFieldEnum::Text("5".into()),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2025-09-28".into()],
                author: vec![RecipeAuthorFieldEnum::new_org("blocky blast")],
                ..Default::default()
            },
        ],
        total_time: vec![DurationOrText::Text("PT35M".into())],
        url: vec!["https://www.zenbelly.com/crispy-chickpeas/".into()],
        video: vec![
            RecipeVideoFieldEnum::Clip(
                Clip {
                    r#type:AtType::VideoObject.to_opt(),
                    context: Some("http://schema.org".into()),
                    thumbnail_url: vec![
                        "https://mediavine-res.cloudinary.com/image/upload/s--kypZQCXB--/c_limit,f_auto,fl_lossy,h_1080,q_auto,w_1920/v1675556534/ihysxtd1aghzz78cv8s7.jpg".into(),
                    ],
                    description: vec![
                        RecipeDescriptionFieldEnum::Text(
                            "Easy Crispy Chickpeas".into(),
                        ),
                    ],
                    name: vec![
                        "Easy Crispy Chickpeas".into(),
                    ],
                    ..Default::default()
                }.into(),
            ),
        ],
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
    Ok(())
}
