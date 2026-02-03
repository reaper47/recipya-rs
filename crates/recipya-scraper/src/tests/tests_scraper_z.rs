#[cfg(test)]
mod tests {
    use schema_org::{
        AggregateRating, AtType, Clip, CreativeWork, DurationOrText, Energy, ImageObject, Mass,
        NutritionInformation, Organization, Rating, Recipe, Review, at_context,
        enums::RestrictedDietEnum,
        field::{
            AggregateRatingRatingValueFieldEnum, ClipDescriptionFieldEnum,
            CreativeWorkImageFieldEnum, ImageObjectCaptionFieldEnum, ImageObjectHeightFieldEnum,
            ImageObjectWidthFieldEnum, RatingRatingValueFieldEnum, RecipeAuthorFieldEnum,
            RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeKeywordsFieldEnum,
            RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
            RecipeRecipeYieldFieldEnum, RecipeVideoFieldEnum, ReviewAuthorFieldEnum,
        },
    };

    use crate::{tests::support::scraper::scrape, websites::Website};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zaatar_and_zaytoun_ok() -> Result<()> {
        let got = scrape(Website::ZaatarAndZaytoun, 0).await?;

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

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zabihahalal_ok() -> Result<()> {
        let got = scrape(Website::ZabihaHalal, 0).await?;

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

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zagleft_ok() -> Result<()> {
        let got = scrape(Website::ZagLeft, 0).await?;

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

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zardyplants_ok() -> Result<()> {
        let got = scrape(Website::ZardyPlants, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![
                AggregateRating {
                    r#type: AtType::AggregateRating.to_opt(),
                    rating_value: vec![
                        AggregateRatingRatingValueFieldEnum::Text("4.9".into()),
                    ],
                    review_count: vec![10],
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

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zarskitchen_ok() -> Result<()> {
        let got = scrape(Website::Zarskitchen, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![AggregateRating {
                r#type: AtType::AggregateRating.to_opt(),
                rating_value: vec![AggregateRatingRatingValueFieldEnum::Text("5.0".into())],
                review_count: vec![1],
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

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zeezest_ok() -> Result<()> {
        let got = scrape(Website::Zeezest, 0).await?;

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
            url: vec!["https://zeezest.com/recipes/watch-multigrain-onion-dosa-with-chutney-recipe-by-zee-zest-1302<number>0".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zenbelly_ok() -> Result<()> {
        let got = scrape(Website::Zenbelly, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![AggregateRating {
                r#type: AtType::AggregateRating.to_opt(),
                rating_value: vec![AggregateRatingRatingValueFieldEnum::Text("5".into())],
                review_count: vec![1],
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

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zenhealth_ok() -> Result<()> {
        let got = scrape(Website::ZenHealth, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![AggregateRating {
                r#type: AtType::AggregateRating.to_opt(),
                rating_value: vec![AggregateRatingRatingValueFieldEnum::Text("5".into())],
                rating_count: vec![8],
                review_count: vec![9],
                ..Default::default()
            }],
            author: vec![RecipeAuthorFieldEnum::new_org("Ros")],
            cook_time: vec![DurationOrText::Text("PT15M".into())],
            description: vec![RecipeDescriptionFieldEnum::Text("Lemongrass tea has a wonderful, lemony, delicious taste. This recipe is so quick and easy. Just add the grass to boiling water and then leave to steep. Serve hot for a soothing drink or cold for a refreshing summer cool down.".into())],
            date_published: vec!["2025-10-22T09:00:00+00:00".into()],
            keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("lemongrass tea".into())],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://zenhealth.net/wp-content/uploads/lemongrass-tea-with-stalk.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zenhealth.net/wp-content/uploads/lemongrass-tea-with-stalk-500x500.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zenhealth.net/wp-content/uploads/lemongrass-tea-with-stalk-500x375.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zenhealth.net/wp-content/uploads/lemongrass-tea-with-stalk-480x270.jpg".into(),
                ),
            ],
            prep_time: vec![DurationOrText::Text("PT2M".into())],
            name: vec!["Lemongrass tea".into()],
            recipe_category: vec!["Drinks".into()],
            recipe_cuisine: vec!["Asian".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 stalk lemongrass ((1 tbsp dried lemongrass))".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text("3 cups water".into()),
                RecipeRecipeIngredientFieldEnum::Text("piece of ginger ((optional))".into()),
                RecipeRecipeIngredientFieldEnum::Text("sweetener of choice ((optional))".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::CreativeWork(CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec![
                        "Remove the outermost skin of the stalk. Wash the lemongrass. Fold the grass onto itself.".into(),
                    ],
                    url: vec!["https://zenhealth.net/lemongrass-tea-recipe/#wprm-recipe-2125-step-0-0".into()],
                    name: vec![
                        "Remove the outermost skin of the stalk. Wash the lemongrass. Fold the grass onto itself.".into(),
                    ],
                    ..Default::default()
                }.into()),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec!["Place a small pot on medium heat. Add water and bring to a boil.".into()],
                    url: vec!["https://zenhealth.net/lemongrass-tea-recipe/#wprm-recipe-2125-step-0-1".into()],
                    name: vec!["Place a small pot on medium heat. Add water and bring to a boil.".into()],
                    ..Default::default()
                }.into()),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec!["Add folded lemongrass stalk. Boil for 15 minutes.".into()],
                    image: vec![CreativeWorkImageFieldEnum::URL(
                        "https://zenhealth.net/wp-content/uploads/lemongrass-tea-ingredients.jpg".into(),
                    )],
                    url: vec!["https://zenhealth.net/lemongrass-tea-recipe/#wprm-recipe-2125-step-0-2".into()],
                    name: vec!["Add folded lemongrass stalk. Boil for 15 minutes.".into()],
                    ..Default::default()
                }.into()),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec!["Turn off heat. Steep for 10 minutes.".into()],
                    image: vec![CreativeWorkImageFieldEnum::URL(
                        "https://zenhealth.net/wp-content/uploads/lemongrass-tea-after-boiling.jpg".into(),
                    )],
                    url: vec!["https://zenhealth.net/lemongrass-tea-recipe/#wprm-recipe-2125-step-0-3".into()],
                    name: vec!["Turn off heat. Steep for 10 minutes.".into()],
                    ..Default::default()
                }.into()),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(CreativeWork {
                    r#type: AtType::HowToStep.to_opt(),
                    text: vec!["Add your favorite sweetener (optional) and serve hot or chilled.".into()],
                    url: vec!["https://zenhealth.net/lemongrass-tea-recipe/#wprm-recipe-2125-step-0-4".into()],
                    name: vec!["Add your favorite sweetener (optional) and serve hot or chilled.".into()],
                    ..Default::default()
                }.into()),
            ],
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("2".into())],
            review: vec![
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "just started on this tea. am looking forward to the benefits. thanks for sharing".into(),
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
                    date_published: vec!["2022-12-21".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Julie Mduduz")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "I heard I should not boil the fresh leaves but soak, stew in a certain boiling temperature. Is this true to reap benefits? And at what temp for how long?".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published: vec!["2021-07-11".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Linda")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "I have some in my backyard garden and I drink it just about any day and any time! Gosh, that \"lemony\" aroma that fills the entire house from the kitchen to the parlour and rooms is so, so sensational every time I brew a flask of the drink. I usually take mine without sweeteners, or with an occasional dash of ginger and honey.".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published: vec!["2021-03-28".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Auwal Gene")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "It's really delicious in uganda we enjoy it too".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published: vec!["2020-12-23".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Ayo Michael Denish")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "We also do not add any sugar to tea; as less as possible is the best for both of us ! Besides, it overpower the beauty of flavors, right ?".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published: vec!["2020-10-07".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("2pots2cook")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Am so glad I read on the lemon grass health benefits, this is awesome".into(),
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
                    date_published: vec!["2020-08-29".into()],
                    author: vec![
                        RecipeAuthorFieldEnum::Organization(
                            Organization {
                                r#type: AtType::Person.to_opt(),
                                name: vec![
                                    "LASISI MOSHOOD O".into(),
                                ],
                                ..Default::default()
                            },
                        ),
                    ],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "I'm from the Caribbean we call lemongrass \"fever grass\" , it's really delicious with some sugar or honey or even if you add some milk to it .".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published: vec!["2020-08-23".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Shelisia")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Made some lemongrass tea this morning. Was delicious, sat on the patio, drank it warm and watched the rain come down.".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published: vec!["2020-07-27".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Carrie Vigil")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec!["Wow this is awesome reading!".into()],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published: vec!["2020-05-11".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Amanda")],
                    ..Default::default()
                },
            ],
            total_time: vec![DurationOrText::Text("PT27M".into())],
            url: vec!["https://zenhealth.net/lemongrass-tea-recipe/".into()],
            video: vec![
                RecipeVideoFieldEnum::Clip(
                    Clip {
                        r#type: AtType::VideoObject.to_opt(),
                        thumbnail_url: vec![
                            "https://i.ytimg.com/vi/fva7uRGNzj4/hqdefault.jpg".into(),
                        ],
                        description: vec![
                            ClipDescriptionFieldEnum::Text(
                                "Lemongrass tea (also called fevergrass tea) has a wonderful, lemony, delicious taste. This recipe is so quick and easy. Just add the lemongrass to boiling water and 10 to 15 minutes later you are done! Serve hot for a soothing drink or cold for a refreshing summer cool down.\n\nThat's exactly how to make lemongrass tea from fresh leaves.\n\nBenefits of lemongrass tea:\n• It improves blood parameters (like red and white blood cell counts, hemoglobin levels)\n• It is antimicrobial\n\nFind more on my blogs: \n- https://wetrinifood.com/lemongrass-tea-recipe/\n- https://zenhealth.net/lemongrass-tea-recipe/\n\nFor more tea recipes, check out: https://wetrinifood.com/tea-recipes/\n\nMusic: www.bensound.com\n\n#lemongrasstea #lemongrass".into(),
                            ),
                        ],
                        name: vec![
                            "How to Make Lemongrass Tea from Fresh Leaves".into(),
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

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zenkimchi_ok() -> Result<()> {
        let got = scrape(Website::ZenKimchi, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            author: vec![RecipeAuthorFieldEnum::new_org("Stewart Ho")],
            cook_time: vec![DurationOrText::Text("PT0H0M".into())],
            description: vec![RecipeDescriptionFieldEnum::Text("Korean Style Arrabbiata Sauce".into())],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL("arrabbiata,Kimchi,Pasta,penne,perilla leaves,red pepper paste,samgyubsal".into()),
            ],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://zenkimchi.com/wp-content/uploads/2014/02/DSC065614-250x250.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zenkimchi.com/wp-content/uploads/2014/02/DSC065614-198x164.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zenkimchi.com/wp-content/uploads/2014/02/DSC065614-320x200.jpg".into(),
                ),
            ],
            name: vec!["Korean Style Arrabbiata Sauce".into()],
            prep_time: vec![DurationOrText::Text("PT0H0M".into())],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("1 1/2 cup of Penne pasta \r".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 cup of ripened Kimchi (chopped)\r".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 Garlic cloves (minced or sliced)\r".into()),
                RecipeRecipeIngredientFieldEnum::Text("6 Perilla leaves (sliced into strips)\r".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 stalk of Green Onion \r".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 tbsp of Red Pepper Paste (Gochujang or ???)\r".into()),
                RecipeRecipeIngredientFieldEnum::Text("150g of Pork Belly or Bacon \r".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 28 oz can (about 3.5 cups) of Crushed Tomato or Tomato Sauce (You can just use your favorite pre-made Pasta Sauce as well)\r".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 tbsp of Olive Oil\r".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/2 a medium sized Onion, sliced (optional)\r".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 cup of sliced mushrooms, sliced (optional) \r".into()),
                RecipeRecipeIngredientFieldEnum::Text("Parmesan Cheese for sprinkling (optional)\r".into()),
                RecipeRecipeIngredientFieldEnum::Text("Red Chili Flakes for sprinkling (optional)".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text("Cook your pasta in salted boiling water until al dante. Drain and set aside when done. \r".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Meanwhile, wash your veggies and cut them as needed. Perilla leaves should be cut into strips, green onion stalk chopped, garlics minced or sliced (depending on your preference). I chose to slice and add some mushroom and onions for added nutrients.\r".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Chop your ripened kimchi, and slice your pork belly (or bacon) into thin slices. If you like bigger, chewier cuts of meat in your pasta, cut accordingly. \r".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Add just a tad bit of olive oil to your pan on medium high heat. The bacon or pork belly will release a lot of fat on its own. Stir for a minute or two before adding the garlic. Stir for another minute. If the pan is too hot make sure to lower your heat temporarily as you don't want to burn your garlic. \r".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Add your chopped kimchi to the pan, stir and cook for a minute.\r".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Add your onions and mushroom (or other vegetables) if you choose to add them. Cook until the onions begin looking translucent.\r".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Add in your green onions and perilla leaves, stir and cook for a minute and then add your tomato paste/sauce. Give it a mix, add the red pepper paste and then mix and cook on medium heat (uncovered) for approximately 6-10 minutes so all the flavors from the pork, garlic, perilla, etc, all incorporate together.\r".into()),
                RecipeRecipeInstructionsFieldEnum::Text("When the sauce has slightly thickened and finished cooking, turn off the heat. Add in your cooked penne and gently stir so all the sauce thoroughly coats the pasta.\r".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Serve the pasta immediately and add Parmesan cheese, chili powder flakes to your liking.".into()),
            ],
            recipe_category: vec!["Modern Korean".into()],
            url: vec!["https://zenkimchi.com/recipes/modern-korean/korean-style-arrabbiata-sauce/".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zestfulkitchen_ok() -> Result<()> {
        let got = scrape(Website::ZestfulKitchen, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![AggregateRating {
                r#type: AtType::AggregateRating.to_opt(),
                rating_value: vec![AggregateRatingRatingValueFieldEnum::Text("5".into())],
                rating_count: vec![6],
                review_count: vec![2],
                ..Default::default()
            }],
            cook_time: vec![DurationOrText::Text("PT5M".into())],
            date_published: vec!["2022-09-08T06:00:00+00:00".into()],
            description: vec![
                RecipeDescriptionFieldEnum::Text("A foolproof smash burger recipe! Quick, easy and loaded with flavor—this is the only burger recipe you&amp;#039;ll need.".into()),
            ],
            image: vec![
                RecipeImageFieldEnum::URL("https://zestfulkitchen.com/wp-content/uploads/2022/08/Smash-Burgers_-2.jpg".into()),
                RecipeImageFieldEnum::URL("https://zestfulkitchen.com/wp-content/uploads/2022/08/Smash-Burgers_-2-500x500.jpg".into()),
                RecipeImageFieldEnum::URL("https://zestfulkitchen.com/wp-content/uploads/2022/08/Smash-Burgers_-2-500x375.jpg".into()),
                RecipeImageFieldEnum::URL("https://zestfulkitchen.com/wp-content/uploads/2022/08/Smash-Burgers_-2-480x270.jpg".into()),
            ],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL("smash burger recipe".into()),
            ],
            name: vec!["Smash Burger Patty".into()],
            nutrition: vec![
                NutritionInformation {
                    calories: vec![Energy::new("565 kcal")],
                    carbohydrate_content: vec![Mass::new("32 g")],
                    cholesterol_content: vec![Mass::new("119 mg")],
                    context: None,
                    fat_content: vec![Mass::new("30 g")],
                    fiber_content: vec![Mass::new("3 g")],
                    protein_content: vec![Mass::new("40 g")],
                    saturated_fat_content: vec![Mass::new("10 g")],
                    serving_size: vec!["1 burger".into()],
                    sodium_content: vec![Mass::new("1625 mg")],
                    sugar_content: vec![Mass::new("7 g")],
                    r#type: AtType::NutritionInformation.to_opt(),
                    ..Default::default()
                },
            ],
            prep_time: vec![DurationOrText::Text("PT10M".into())],
            recipe_category: vec!["Dinner".into()],
            recipe_cuisine: vec!["American".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("¼ cup whole-grain mustard".into()),
                RecipeRecipeIngredientFieldEnum::Text("¼ cup prepared yellow mustard".into()),
                RecipeRecipeIngredientFieldEnum::Text("¼ cup olive oil mayonnaise".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 pound (80/20) ground beef".into()),
                RecipeRecipeIngredientFieldEnum::Text("Morton kosher salt".into()),
                RecipeRecipeIngredientFieldEnum::Text("4 brioche buns, (split)".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 medium yellow onion, (very thinly sliced)".into()),
                RecipeRecipeIngredientFieldEnum::Text("½ head iceberg lettuce, (leaves separated)".into()),
                RecipeRecipeIngredientFieldEnum::Text("3 large kosher dill pickles, (thinly sliced + more for serving)".into()),
                RecipeRecipeIngredientFieldEnum::Text("4 slices American cheese, (such as Kraft Singles)".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "For burger sauce, mix together mustards and mayonnaise; set aside.".into(),
                        ],
                        url: vec![
                            "https://zestfulkitchen.com/smash-burger-recipe/#wprm-recipe-24914-step-0-0".into(),
                        ],
                        name: vec![
                            "For burger sauce, mix together mustards and mayonnaise; set aside.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Divide meat into quarters (about 4 ounces each) and roll into balls. Transfer to a plate or small baking sheet.".into(),
                        ],
                        url: vec![
                            "https://zestfulkitchen.com/smash-burger-recipe/#wprm-recipe-24914-step-0-1".into(),
                        ],
                        name: vec![
                            "Divide meat into quarters (about 4 ounces each) and roll into balls. Transfer to a plate or small baking sheet.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Heat a large (12-inch) cast-iron, stainless steel, or carbon steel skillet over medium-high heat until ripping hot and smoking, about 5 minutes.".into(),
                        ],
                        url: vec![
                            "https://zestfulkitchen.com/smash-burger-recipe/#wprm-recipe-24914-step-0-2".into(),
                        ],
                        name: vec![
                            "Heat a large (12-inch) cast-iron, stainless steel, or carbon steel skillet over medium-high heat until ripping hot and smoking, about 5 minutes.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Toast half of the buns, cut side down, in skillet until lightly golden, about 1 minute. Repeat with remaining buns.".into(),
                        ],
                        url: vec![
                            "https://zestfulkitchen.com/smash-burger-recipe/#wprm-recipe-24914-step-0-3".into(),
                        ],
                        name: vec![
                            "Toast half of the buns, cut side down, in skillet until lightly golden, about 1 minute. Repeat with remaining buns.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Increase heat to high.".into(),
                        ],
                        url: vec![
                            "https://zestfulkitchen.com/smash-burger-recipe/#wprm-recipe-24914-step-0-4".into(),
                        ],
                        name: vec![
                            "Increase heat to high.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Add two beef balls to skillet; season generously with kosher salt. Smash beef balls down with a heavy duty spatula, using a potato masher to press down on top of the spatula for extra leverage, until very thin, about ½-inch thick. Cook, without moving, until bottom sides are crisp and browned, about 1 ½ minutes.".into(),
                        ],
                        url: vec![
                            "https://zestfulkitchen.com/smash-burger-recipe/#wprm-recipe-24914-step-0-5".into(),
                        ],
                        name: vec![
                            "Add two beef balls to skillet; season generously with kosher salt. Smash beef balls down with a heavy duty spatula, using a potato masher to press down on top of the spatula for extra leverage, until very thin, about ½-inch thick. Cook, without moving, until bottom sides are crisp and browned, about 1 ½ minutes.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Press a few onion slices into burger patties with spatula. Scrape up and flip patty.".into(),
                        ],
                        url: vec![
                            "https://zestfulkitchen.com/smash-burger-recipe/#wprm-recipe-24914-step-0-6".into(),
                        ],
                        name: vec![
                            "Press a few onion slices into burger patties with spatula. Scrape up and flip patty.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Place a slice of cheese over burger and continue to cook until cheese is melted and onions on bottom are griddled and golden, 30–60 seconds. Remove from griddle.".into(),
                        ],
                        url: vec![
                            "https://zestfulkitchen.com/smash-burger-recipe/#wprm-recipe-24914-step-0-7".into(),
                        ],
                        name: vec![
                            "Place a slice of cheese over burger and continue to cook until cheese is melted and onions on bottom are griddled and golden, 30–60 seconds. Remove from griddle.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "To assemble, spread 1 tablespoon burger sauce on cuts sides of buns (tops and bottoms). Cover bottom buns with pickle slices then top with burger patty. Add a few slices of raw onion, a couple of folded sheets of iceberg lettuce followed by the top bun.".into(),
                        ],
                        url: vec![
                            "https://zestfulkitchen.com/smash-burger-recipe/#wprm-recipe-24914-step-0-8".into(),
                        ],
                        name: vec![
                            "To assemble, spread 1 tablespoon burger sauce on cuts sides of buns (tops and bottoms). Cover bottom buns with pickle slices then top with burger patty. Add a few slices of raw onion, a couple of folded sheets of iceberg lettuce followed by the top bun.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Wipe out skillet with paper towels before repeating smashing and cooking process with remaining beef balls, onions and cheese.".into(),
                        ],
                        url: vec![
                            "https://zestfulkitchen.com/smash-burger-recipe/#wprm-recipe-24914-step-0-9".into(),
                        ],
                        name: vec![
                            "Wipe out skillet with paper towels before repeating smashing and cooking process with remaining beef balls, onions and cheese.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Serve burgers with pickle spears.".into(),
                        ],
                        url: vec![
                            "https://zestfulkitchen.com/smash-burger-recipe/#wprm-recipe-24914-step-0-10".into(),
                        ],
                        name: vec![
                            "Serve burgers with pickle spears.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
            ],
            recipe_yield: vec![
                RecipeRecipeYieldFieldEnum::Text("4".into()),
                RecipeRecipeYieldFieldEnum::Text("4 burgers".into()),
            ],
            review: vec![
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "These are to die for!!! We cook them on our Blackstone griddle outside, so no mess on the stove. Love the crunchy edges, and do NOT skip the sauce, that’s the best part!!!".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into()),
                            ],
                            ..Default::default()
                        },
                    ],
                    date_published: vec!["2023-05-25".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Dee")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Got damn these burgers are good. Love the technique of pressing the onion into the patty. Keeping the lettuce intact with all the layers as you did gave the burger a nice crunch. The Kraft cheese put them over the top. Thank you!".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into()),
                            ],
                            ..Default::default()
                        },
                    ],
                    date_published: vec!["2023-05-23".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("kt")],
                    ..Default::default()
                },
            ],
            total_time: vec![DurationOrText::Text("PT15M".into())],
            url: vec!["https://zestfulkitchen.com/smash-burger-recipe/".into()],
            video: vec![
                RecipeVideoFieldEnum::Clip(
                    Clip {
                        r#type: AtType::VideoObject.to_opt(),
                        thumbnail_url: vec!["https://content.jwplatform.com/thumbs/HNPx6AFO-720.jpg".into()],
                        description: vec![ClipDescriptionFieldEnum::Text("beef, ground beef, burgers, sandwiches".into())],
                        name: vec!["Smashburgers".into()],
                        ..Default::default()
                    }.into(),
                ),
            ],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zestysouthindiankitchen_ok() -> Result<()> {
        let got = scrape(Website::ZestSouthIndianKitchen, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![AggregateRating {
                r#type: AtType::AggregateRating.to_opt(),
                rating_value: vec![AggregateRatingRatingValueFieldEnum::Text("5".into())],
                review_count: vec![15],
                rating_count: vec![17],
                ..Default::default()
            }],
            author: vec![RecipeAuthorFieldEnum::new_org("Swathi (Ambujom Saraswathy)")],
            cook_time: vec![DurationOrText::Text("PT45M".into())],
            date_published: vec!["2020-08-25T16:45:30+00:00".into()],
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "Spicy Baked zucchini chips are&nbsp; healthy snack recipe that is loaded with flavor and crunch, yet light in calories! Just 5 ingredients and you can create amazing appetizer.".into(),
                ),
            ],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://zestysouthindiankitchen.com/wp-content/uploads/2020/08/Spicy-Baked-Zucchini-chips.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zestysouthindiankitchen.com/wp-content/uploads/2020/08/Spicy-Baked-Zucchini-chips-500x500.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zestysouthindiankitchen.com/wp-content/uploads/2020/08/Spicy-Baked-Zucchini-chips-500x375.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zestysouthindiankitchen.com/wp-content/uploads/2020/08/Spicy-Baked-Zucchini-chips-480x270.jpg".into(),
                ),
            ],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL(
                    "baked zucchini chips, Spicy baked zucchini chips".into(),
                ),
            ],
            name: vec!["Spicy Baked Zucchini chips".into()],
            nutrition: vec![
                NutritionInformation {
                    calories: vec![
                        Energy::new("76 kcal"),
                    ],
                    carbohydrate_content: vec![
                        Mass::new("2 g"),
                    ],
                    fat_content: vec![
                        Mass::new("8 g"),
                    ],
                    fiber_content: vec![
                        Mass::new("1 g"),
                    ],
                    protein_content: vec![
                        Mass::new("1 g"),
                    ],
                    saturated_fat_content: vec![
                        Mass::new("1 g"),
                    ],
                    serving_size: vec![
                        "1 serving".into(),
                    ],
                    sodium_content: vec![
                        Mass::new("4 mg"),
                    ],
                    sugar_content: vec![
                        Mass::new("1 g"),
                    ],
                    r#type: AtType::NutritionInformation.to_opt(),
                    ..Default::default()
                },
            ],
            prep_time: vec![DurationOrText::Text("PT5M".into())],
            recipe_category: vec!["Appetizer".into()],
            recipe_cuisine: vec!["American".into()],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Preheat the oven to 425 F . Line two baking sheets with parchment paper.".into(),
                        ],
                        url: vec![
                            "https://zestysouthindiankitchen.com/spicy-baked-zucchini-chips/#wprm-recipe-16521-step-0-0".into(),
                        ],
                        name: vec![
                            "Preheat the oven to 425 F . Line two baking sheets with parchment paper.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Thinly slice the zucchini with a knife or mandolin.".into(),
                        ],
                        url: vec![
                            "https://zestysouthindiankitchen.com/spicy-baked-zucchini-chips/#wprm-recipe-16521-step-0-1".into(),
                        ],
                        name: vec![
                            "Thinly slice the zucchini with a knife or mandolin.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "In a large bowl add zucchini, combine the oil, salt, pepper, cumin powder, and cayenne pepper. Stir to combine.".into(),
                        ],
                        url: vec![
                            "https://zestysouthindiankitchen.com/spicy-baked-zucchini-chips/#wprm-recipe-16521-step-0-2".into(),
                        ],
                        name: vec![
                            "In a large bowl add zucchini, combine the oil, salt, pepper, cumin powder, and cayenne pepper. Stir to combine.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Toss well so that each slice is coated with the seasoned oil.".into(),
                        ],
                        url: vec![
                            "https://zestysouthindiankitchen.com/spicy-baked-zucchini-chips/#wprm-recipe-16521-step-0-3".into(),
                        ],
                        name: vec![
                            "Toss well so that each slice is coated with the seasoned oil.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Place the zucchini slices on the prepared baking sheets.".into(),
                        ],
                        url: vec![
                            "https://zestysouthindiankitchen.com/spicy-baked-zucchini-chips/#wprm-recipe-16521-step-0-4".into(),
                        ],
                        name: vec![
                            "Place the zucchini slices on the prepared baking sheets.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Bake for 20 minutes watching very closely. When the zucchini starts to show some brown spots remove from the oven and set aside.".into(),
                        ],
                        url: vec![
                            "https://zestysouthindiankitchen.com/spicy-baked-zucchini-chips/#wprm-recipe-16521-step-0-5".into(),
                        ],
                        name: vec![
                            "Bake for 20 minutes watching very closely. When the zucchini starts to show some brown spots remove from the oven and set aside.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Reduce the oven temperature to 180-200 degrees. Return the zucchini to the oven and cook for an additional 20 minutes or until the slices are crispy.".into(),
                        ],
                        url: vec![
                            "https://zestysouthindiankitchen.com/spicy-baked-zucchini-chips/#wprm-recipe-16521-step-0-6".into(),
                        ],
                        name: vec![
                            "Reduce the oven temperature to 180-200 degrees. Return the zucchini to the oven and cook for an additional 20 minutes or until the slices are crispy.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Remove from the oven and cool and enjoy".into(),
                        ],
                        url: vec![
                            "https://zestysouthindiankitchen.com/spicy-baked-zucchini-chips/#wprm-recipe-16521-step-0-7".into(),
                        ],
                        name: vec![
                            "Remove from the oven and cool and enjoy".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
            ],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 zucchini washed and dried".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 tbsp. olive oil".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "Scant 1/2 tsp salt".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 tsp. black pepper".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/4 tsp cumin powder".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "¼ tsp cayenne pepper".into(),
                ),
            ],
            recipe_yield: vec![
                RecipeRecipeYieldFieldEnum::Text("4".into()),
            ],
            review: vec![
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Tried these and they were so crispy and flavorful! Loved the cayenne kick!".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published:vec!["2025-02-14".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Juyali")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "These are insane. I'm obsessed. I love zucchini chips!".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published:vec!["2020-09-06".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Maren")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "I love the level of spice on these zucchini chips! So good".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published:vec!["2020-09-06".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Capri")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "These are great, a nice change from the cheese crusted version I usually do. Love the cumin.".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published:vec!["2020-09-06".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Amanda Marie Boyle")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Perfect, healthy chip recipe!".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published:vec!["2020-09-05".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Leslie")],
                   ..Default::default()
                },
               Review {
                   r#type: AtType::Review.to_opt(),
                   review_body: vec![
                       "These chips are amazing!! Love the spices!".into(),
                   ],
                   review_rating: vec![
                       Rating {
                           r#type: AtType::Rating.to_opt(),
                           rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                           ..Default::default()
                       },
                   ],
                   date_published: vec!["2020-09-04".into()],
                   author: vec![RecipeAuthorFieldEnum::new_org("Emily")],
                   ..Default::default()
               },
               Review {
                   r#type: AtType::Review.to_opt(),
                   review_body: vec![
                       "I love these guilt free chips!".into(),
                   ],
                   review_rating: vec![
                       Rating {
                           r#type: AtType::Rating.to_opt(),
                           rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                           ..Default::default()
                       },
                   ],
                   date_published: vec!["2020-09-04".into()],
                   author: vec![RecipeAuthorFieldEnum::new_org("Sue")],
                   ..Default::default()
               },
               Review {
                   r#type: AtType::Review.to_opt(),
                   review_body: vec![
                       "My friends raved about the apricot.".into(),
                   ],
                   review_rating: vec![
                       Rating {
                           r#type: AtType::Rating.to_opt(),
                           rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                           ..Default::default()
                       },
                   ],
                   date_published: vec!["2020-09-04".into()],
                   author: vec![RecipeAuthorFieldEnum::new_org("Laura")],
                   ..Default::default()
               },
               Review {
                   r#type: AtType::Review.to_opt(),
                   review_body: vec![
                       "Spicy and addictive!".into(),
                   ],
                   review_rating: vec![
                       Rating {
                           r#type: AtType::Rating.to_opt(),
                           rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                           ..Default::default()
                       },
                   ],
                   date_published: vec!["2020-09-04".into()],
                   author: vec![RecipeAuthorFieldEnum::new_org("Debra")],
                   ..Default::default()
               },
               Review {
                   r#type: AtType::Review.to_opt(),
                   review_body: vec![
                       "These make for such a delicious snack - we love zucchini!".into(),
                   ],
                   review_rating: vec![
                       Rating {
                           r#type: AtType::Rating.to_opt(),
                           rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                           ..Default::default()
                       },
                   ],
                   date_published: vec!["2020-09-04".into()],
                   author: vec![RecipeAuthorFieldEnum::new_org("Alexandra")],
                   ..Default::default()
               },
               Review {
                   r#type: AtType::Review.to_opt(),
                   review_body: vec![
                       "What a great and healthy snack idea! Pinning for sure!".into(),
                   ],
                   review_rating: vec![
                       Rating {
                           r#type: AtType::Rating.to_opt(),
                           rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                           ..Default::default()
                       },
                   ],
                   date_published: vec!["2020-09-03".into()],
                   author: vec![RecipeAuthorFieldEnum::new_org("Taleen | Just As Tasty")],
                   ..Default::default()
               },
               Review {
                   r#type: AtType::Review.to_opt(),
                   review_body: vec![
                       "These are so delicious and a perfect recipe for using up a glut of zucchini!".into(),
                   ],
                   review_rating: vec![
                       Rating {
                           r#type: AtType::Rating.to_opt(),
                           rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                           ..Default::default()
                       },
                   ],
                   date_published: vec!["2020-09-03".into()],
                   author: vec![RecipeAuthorFieldEnum::new_org("Robyn")],
                   ..Default::default()
               },
               Review {
                   r#type: AtType::Review.to_opt(),
                   review_body: vec![
                       "These are one of our favorite snacks! They don't last long in our house!".into(),
                   ],
                   review_rating: vec![
                       Rating {
                           r#type: AtType::Rating.to_opt(),
                           rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                           ..Default::default()
                       },
                   ],
                   date_published: vec!["2020-09-03".into()],
                   author: vec![RecipeAuthorFieldEnum::new_org("kerri")],
                   ..Default::default()
               },
               Review {
                   r#type: AtType::Review.to_opt(),
                   review_body: vec![
                       "I wish I had everything on hand to make this right now. This looks really yummy.".into(),
                   ],
                   review_rating: vec![
                       Rating {
                           r#type: AtType::Rating.to_opt(),
                           rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                           ..Default::default()
                       },
                   ],
                   date_published: vec!["2020-08-26".into()],
                   author: vec![RecipeAuthorFieldEnum::new_org("Tasheena")],
                   ..Default::default()
               },
               Review {
                   r#type: AtType::Review.to_opt(),
                   review_body: vec![
                       "The best healthy snack! Love how easy are to make them!".into(),
                   ],
                   review_rating: vec![
                       Rating {
                           r#type: AtType::Rating.to_opt(),
                           rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                           ..Default::default()
                       },
                   ],
                   date_published: vec!["2020-08-26".into()],
                   author: vec![RecipeAuthorFieldEnum::new_org("Catalina")],
                   ..Default::default()
               },
            ],
            total_time: vec![DurationOrText::Text("PT50M".into())],
            url: vec!["https://zestysouthindiankitchen.com/spicy-baked-zucchini-chips/".into()],
            video: vec![
                RecipeVideoFieldEnum::Clip(
                    Clip {
                        r#type:AtType::VideoObject.to_opt(),
                        thumbnail_url: vec![
                            "https://i.ytimg.com/vi/Zu-g6I_OQVk/hqdefault.jpg".into(),
                        ],
                        description: vec![
                            RecipeDescriptionFieldEnum::Text(
                                "Delicious simple, spicy baked zucchini chips, wtih just 5 ingredents you can make delicious sidedish . That too in oven without much cleaning dishes. For full recipes with instructions , https://zestysouthindiankitchen.com/spicy-baked-zucchini-chips/\n\n#zucchinirecipes #bakedchips #zucchinichips".into(),
                            ),
                        ],
                        name: vec![
                            "Spicy Zucchini Chips".into(),
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

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zhangcatherine_ok() -> Result<()> {
        let got = scrape(Website::ZhangCatherine, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![AggregateRating {
                r#type: AtType::AggregateRating.to_opt(),
                best_rating: vec![AggregateRatingRatingValueFieldEnum::Number(5.0)],
                rating_value: vec![AggregateRatingRatingValueFieldEnum::Number(5.0)],
                review_count: vec![1],
                worst_rating: vec![AggregateRatingRatingValueFieldEnum::Number(4.9)],
                url: vec!["https://zhangcatherine.com/pistachio-macarons/#aggregateRating".into()],
                ..Default::default()
            }],
            author: vec![RecipeAuthorFieldEnum::new_org("Zhang Catherine")],
            cook_time: vec![DurationOrText::Text("PT15M".into())],
            description: vec![RecipeDescriptionFieldEnum::Text("Crisp and chewy French macaron shells filled with a sweet and nutty pistachio cream, the only pistachio macarons you'll need".into())],
            image: vec![
                RecipeImageFieldEnum::ImageObject(
                    ImageObject {
                        r#type: AtType::ImageObject.to_opt(),
                        caption: vec![
                            ImageObjectCaptionFieldEnum::Text(
                                "French pistachio Macarons with pistachio cream cheese buttercream filling".into(),
                            ),
                        ],
                        height: vec![ImageObjectHeightFieldEnum::Integer(1500)],
                        width: vec![ImageObjectWidthFieldEnum::Integer(1500)],
                        url: vec![
                            "https://zhangcatherine.com/wp-content/uploads/2022/06/15001500.jpg".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
            ],
            keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Pistachio Macaron Recipe".into())],
            name: vec!["Pistachio Macarons - Catherine Zhang".into()],
            nutrition: vec![NutritionInformation {
                calories: vec![Energy::new("283 Calories")],
                r#type: AtType::NutritionInformation.to_opt(),
                ..Default::default()
            }],
            prep_time: vec![DurationOrText::Text("PT15M".into())],
            recipe_category: vec!["Cookie".into()],
            recipe_cuisine: vec!["French".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("Macaron Shells: almond meal".into()),
                RecipeRecipeIngredientFieldEnum::Text("icing sugar".into()),
                RecipeRecipeIngredientFieldEnum::Text("egg whites".into()),
                RecipeRecipeIngredientFieldEnum::Text("white sugar".into()),
                RecipeRecipeIngredientFieldEnum::Text("green food coloring".into()),
                RecipeRecipeIngredientFieldEnum::Text(
                    "Filling (Pistachio Buttercream): unsalted butter".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text("cream cheese".into()),
                RecipeRecipeIngredientFieldEnum::Text("ground pistachios".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Macaron Shells In a medium sized bowl sieve the almond meal and powdered sugar together If there are large chunks of almond meal remaining in the sieve dispose of them In a large clean bowl add the egg whites, and using an electric mixer beat until foamy Slowly add the sugar and beat until stiff peaks Fold the sieved almond meal and powdered sugar into the meringue in 2 additions, scraping around the bowl and down the centre Continue until you notice that the lines that form when the batter falls back into the bowl slowly start to disappear Transfer to a piping bag fitted with a round tip and pipe out 1.5inch circles Allow to dry for 1-2 hours, or until the surface is matte and dry Bake in a preheated oven at 140°C for 13-15 minutes (I find that it’s usually done at 13, but it depends on your oven so make sure to check! If you give your macarons a wiggle they shouldn’t be moving, that’s when they’re ready) Remove from oven and let cool Pistachio Buttercream Combine pistachios and sugar in a food processor and blitz until a smooth paste (this will take some time) Beat the butter and cream cheese together with an electric mixer or stand mixer, until light and fluffy Add the pistachio paste and whisk until smooth Transfer to a piping bag fitted with a round tip Match similar sized macaron shells with each other On the flat side of one shell pipe a large dollop of buttercream Top with a matching shell Place in airtight container in the fridge for a day to mature (gives deeper flavour and chewy texture!)".into(),
                        ],
                        image: vec![
                            RecipeImageFieldEnum::URL(
                                "https://zhangcatherine.com/wp-content/uploads/2022/06/P5230461.jpg".into(),
                            ),
                        ],
                        name: vec![
                            "How to make Pistachio Macaron Recipe?".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
            ],
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("12".into())],
            review: vec![
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "The crisp shells with the smooth pistachio cream create a refined dessert where the sweetness and nuttiness are in perfect harmony.".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            best_rating: vec![RatingRatingValueFieldEnum::Number(5.0)],
                            rating_value: vec![RatingRatingValueFieldEnum::Number(5.0)],
                            worst_rating: vec![RatingRatingValueFieldEnum::Number(4.9)],
                            ..Default::default()
                        },
                    ],
                    headline: vec![
                        "Nutty and perfectly balanced".into(),
                    ],
                    author: vec![RecipeAuthorFieldEnum::new_org("Zhang Catherine")],
                    ..Default::default()
                }.into(),
            ],
            total_time: vec![DurationOrText::Text("PT30M".into())],
            url: vec!["https://zhangcatherine.com/pistachio-macarons/".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_ziahatchchilecompany_ok() -> Result<()> {
        let got = scrape(Website::ZiaHatchChileCompany, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            name: vec!["Hatch Green Chile Baked Spaghetti".into()],
            date_published: vec!["2025-12-22T17:41:39Z".into()],
            description: vec![RecipeDescriptionFieldEnum::Text("Looking for an easy way to feed a crowd? Our recipe for Hatch Green Chile Baked Spaghetti is a great way to upgrade the everyday pasta dish.".into())],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://ziahatchchileco.com/cdn/shop/articles/DSC03444_E_1200x1200.jpg?v=1766425320".into(),
                ),
            ],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 - full jar Zia Hatch Green Chile, strained".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 lb Spaghetti pasta".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 32oz jar Marinara Sauce of choice".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 large Yellow Onion, diced".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 lb Ground Beef (we recommend no more than 15% fat)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup Ricotta Cheese".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 cups Mozzarella Cheese, shredded".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 large Egg".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup Parmesan Cheese, grated".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "4-5 cloves Garlic, finely diced".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 cup Basil Leaves, chopped".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 1/2 tsp dried Oregano".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 tsp dried Thyme or Rosemary".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 tsp Red Pepper Flakes".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "Butter for greasing the pan".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "Olive oil for cooking the meat".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "Salt and pepper".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "In a large pot or dutch oven, heat oil over medium heat. Add onion and cook for 5-6 minutes until translucent. Add garlic, oregano, thyme, red pepper flakes, a couple teaspoons of salt and a few shakes / grinds of black pepper and cook 1-2 minutes until fragrant but make sure the garlic does not burn.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Add the beef and break up / stir constantly until browned, about 5 more minutes. Once cooked, turn off the heat, add the marinara sauce and Zia Hatch Green Chile and stir until combined.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Set aside and boil a pot of salted water and pre heat the oven to 350 degrees Fahrenheit. Once boiling, cook pasta until 1 minute less than al dente. Drain and add to the marinara / meat sauce and mix. Toss well.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "While water is boiling, crack an egg into a bowl and add ricotta, 1/2 of the parmesan cheese and mix / whisk until smooth.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Grease a 9 x 13 inch baking dish with butter. Transfer about half of the pasta into the baking dish and smooth into an even layer. Use a large spoon to drip the ricotta cheese mixture evenly over the layer, and then sprinkle 1 cup of the mozzarella as well. Add the rest of the pasta and smooth out again.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Cover baking dish with tin foil and cook in the oven for about 40 minutes. After 40 minutes, remove the tin foil, evenly sprinkle the remaining mozzarella and parmesan cheese and bake for another 5-10 minutes or so until the cheese has melted. If you want to brown the mozzarella cheese a bit, turn on the broiler for a minute or two toward the end, watching closely to make sure it does not burn.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Top with basil, cut into squares and serve warm.".into(),
                ),
            ],
            url: vec![
                "https://ziahatchchileco.com/blogs/recipes/hatch-green-chile-baked-spaghetti<number>0".into(),
            ],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zibakitchen_ok() -> Result<()> {
        let got = scrape(Website::ZibaKitchen, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            author: vec![RecipeAuthorFieldEnum::new_person("adeel")],
            cook_time: vec![DurationOrText::Text("PT30H".into())],
            date_published: vec!["2017-11-27".into()],
            description: vec![RecipeDescriptionFieldEnum::Text("Bolani Gandana, is a delicious dish! Gandana (Nira Herb or Chinese Leek) is the key to this dish.Bolani is a traditional dish from Afghanistan and people make it in gatherings with family. They make fire and around the fire, they cook Bolani together and serve it while hot.\n\nHere is how to make Bolani Gandana, The recipe, and youtube Video.\n\nPlease click on the youtube icon in the top right corner of our page to subscribe and watch more videos.".into())],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://zibakitchen.com/wp-content/uploads/2017/11/bolani-750x500.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zibakitchen.com/wp-content/uploads/2017/11/bolani-150x150.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zibakitchen.com/wp-content/uploads/2017/11/bolani-340x191.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zibakitchen.com/wp-content/uploads/2014/06/DSC03970-340x191.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zibakitchen.com/wp-content/uploads/2014/05/DSC03904-340x191.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zibakitchen.com/wp-content/uploads/2014/04/DSC00198-340x191.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zibakitchen.com/wp-content/uploads/2014/03/DSC00036-340x191.jpg".into(),
                ),
            ],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL(
                    "Black Pepper".into(),
                ),
                RecipeKeywordsFieldEnum::TextOrURL(
                    "bolani".into(),
                ),
                RecipeKeywordsFieldEnum::TextOrURL(
                    "Coriander".into(),
                ),
                RecipeKeywordsFieldEnum::TextOrURL(
                    "gandana".into(),
                ),
            ],
            name: vec!["Bolani Gandana".into()],
            prep_time: vec![DurationOrText::Text("PT30M".into())],
            recipe_category: vec!["Breads".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    " Dough For Bolani ".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "Check zibakitchen.com".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    " Nira Herb (Chinese Leek) Gandana".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 a Bunch".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    " Sea Salt".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 Teaspoon".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    " Black Pepper".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 Teaspoon".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    " Coriander Powder ".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 Teaspoon".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    " Extra Virgin Olive Oil".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1-2 Tablespoons".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    " Sesame Oil For cooking".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 Cup".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Cut the Nira Herb (Chinese Leek)into small pieces and wash well. Add pepper, salt, and oil and combine them well. Set aside.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Check for the Dough for Bolani At zibakitchen.com".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Now, make small balls from the dough. Take one ball of dough and roll it out using a rolling pin flat and round. ".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Take a little amount of leek mixture and place it on half side of the rolled dough, and fold another half to close and seal the leek mixture to make a half moon.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Transfer the Bolani to a hot pan and add some Sesame oil to the pan. ".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Cook the Bolani until lightly brown then flip it to cook the other side.".into(),
                ),
            ],
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("6".into())],
            url: vec!["https://zibakitchen.com/bolani-gandana/<number>0".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zoebakes_ok() -> Result<()> {
        let got = scrape(Website::ZoeBakes, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![AggregateRating {
                r#type: AtType::AggregateRating.to_opt(),
                rating_value: vec![AggregateRatingRatingValueFieldEnum::Text("5".into())],
                rating_count: vec![1],
                ..Default::default()
            }],
            date_published: vec!["2019-02-13T12:51:17+00:00".into()],
            description: vec![
                RecipeDescriptionFieldEnum::Text("Cinnamon rolls are a classic breakfast treat for any special occasion. You&amp;#39;ll be surprised how easy this recipe is to make, and the resulting cinnamon rolls are pillowy and delicious, with the perfect amount of cinnamon and sweetness.".into())
            ],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://www.zoebakes.com/wp-content/uploads/2019/02/valentine-cinnamon-rolls-ZoeBakes-9-of-8.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://www.zoebakes.com/wp-content/uploads/2019/02/valentine-cinnamon-rolls-ZoeBakes-9-of-8-500x500.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://www.zoebakes.com/wp-content/uploads/2019/02/valentine-cinnamon-rolls-ZoeBakes-9-of-8-500x375.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://www.zoebakes.com/wp-content/uploads/2019/02/valentine-cinnamon-rolls-ZoeBakes-9-of-8-480x270.jpg".into(),
                ),
            ],
            name: vec!["Cinnamon Rolls".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 1/2 lbs Brioche Dough".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 cup sugar".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 cup brown sugar".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 tbsp cinnamon".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 tsp orange zest".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "6 tbsp butter, melted".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "8 oz cream cheese, room temp".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "6 tbsp confectioners&amp;#39; sugar".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "4 tbsp heavy cream".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 tsp vanilla extract".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 tsp orange zest".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Give the dough 3 or 4 turns to help bind the gluten and give the rolls great texture. Rest the dough for 15-20 minutes.".into(),
                        ],
                        url: vec![
                            "https://www.zoebakes.com/2019/02/13/valentines-day-cinnamon-rolls/#wprm-recipe-14807-step-0-0".into(),
                        ],
                        name: vec![
                            "Give the dough 3 or 4 turns to help bind the gluten and give the rolls great texture. Rest the dough for 15-20 minutes.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Once the dough is ready, roll it to 1/4-inch thick rectangle. Brush the entire surface with the melted butter. In a small bowl mix together the sugars, cinnamon and zest.".into(),
                        ],
                        url: vec![
                            "https://www.zoebakes.com/2019/02/13/valentines-day-cinnamon-rolls/#wprm-recipe-14807-step-0-1".into(),
                        ],
                        name: vec![
                            "Once the dough is ready, roll it to 1/4-inch thick rectangle. Brush the entire surface with the melted butter. In a small bowl mix together the sugars, cinnamon and zest.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Spread the mixture over the butter topped dough. Use your hands to make sure you have an even coat of the sugar. Then roll the dough up, starting at the short end.".into(),
                        ],
                        url: vec![
                            "https://www.zoebakes.com/2019/02/13/valentines-day-cinnamon-rolls/#wprm-recipe-14807-step-0-2".into(),
                        ],
                        name: vec![
                            "Spread the mixture over the butter topped dough. Use your hands to make sure you have an even coat of the sugar. Then roll the dough up, starting at the short end.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Use a&nbsp;Bread Knife,&nbsp;Kitchen Scissors&nbsp;or floss to cut the log into 8,10, or 12 equal pieces.".into(),
                        ],
                        url: vec![
                            "https://www.zoebakes.com/2019/02/13/valentines-day-cinnamon-rolls/#wprm-recipe-14807-step-0-3".into(),
                        ],
                        name: vec![
                            "Use a&nbsp;Bread Knife,&nbsp;Kitchen Scissors&nbsp;or floss to cut the log into 8,10, or 12 equal pieces.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Set the buns on a parchment lined&nbsp;Sheet Pan&nbsp;or in a buttered baking dish. Give them about 1 1/2 to 2-inches between them. It is okay if they rise together in the oven.".into(),
                        ],
                        url: vec![
                            "https://www.zoebakes.com/2019/02/13/valentines-day-cinnamon-rolls/#wprm-recipe-14807-step-0-4".into(),
                        ],
                        name: vec![
                            "Set the buns on a parchment lined&nbsp;Sheet Pan&nbsp;or in a buttered baking dish. Give them about 1 1/2 to 2-inches between them. It is okay if they rise together in the oven.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Loosely cover the buns and let them rest between 1 1/2 to 2 hours. The long rest will insure that you have a fluffy bun. (You can set these up the night before&nbsp;and let them rest overnight in the refrigerator. In the morning take them out and let them sit on the counter for about 45 minutes to an hour.)&nbsp;You may get away with slightly shorter rise, but the buns will not be quite as soft.".into(),
                        ],
                        url: vec![
                            "https://www.zoebakes.com/2019/02/13/valentines-day-cinnamon-rolls/#wprm-recipe-14807-step-0-5".into(),
                        ],
                        name: vec![
                            "Loosely cover the buns and let them rest between 1 1/2 to 2 hours. The long rest will insure that you have a fluffy bun. (You can set these up the night before&nbsp;and let them rest overnight in the refrigerator. In the morning take them out and let them sit on the counter for about 45 minutes to an hour.)&nbsp;You may get away with slightly shorter rise, but the buns will not be quite as soft.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Preheat the oven to 350°F and place the rack in the middle of the oven.".into(),
                        ],
                        url: vec![
                            "https://www.zoebakes.com/2019/02/13/valentines-day-cinnamon-rolls/#wprm-recipe-14807-step-0-6".into(),
                        ],
                        name: vec![
                            "Preheat the oven to 350°F and place the rack in the middle of the oven.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Bake for about 25 to 30 minutes, just until the centers are set when poked with your finger (they should be caramel colored). Let them cool for about 10 minutes.".into(),
                        ],
                        url: vec![
                            "https://www.zoebakes.com/2019/02/13/valentines-day-cinnamon-rolls/#wprm-recipe-14807-step-0-7".into(),
                        ],
                        name: vec![
                            "Bake for about 25 to 30 minutes, just until the centers are set when poked with your finger (they should be caramel colored). Let them cool for about 10 minutes.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
            ],
            url: vec!["https://www.zoebakes.com/2019/02/13/valentines-day-cinnamon-rolls/".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zonacooks_ok() -> Result<()> {
        let got = scrape(Website::ZonaCooks, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![AggregateRating {
                r#type: AtType::AggregateRating.to_opt(),
                rating_value: vec![AggregateRatingRatingValueFieldEnum::Text("5".into())],
                review_count: vec![1],
                rating_count: vec![1],
                ..Default::default()
            }],
            cook_time: vec![DurationOrText::Text("PT30M".into())],
            date_published: vec!["2025-12-15T11:24:13+00:00".into()],
            description: vec![RecipeDescriptionFieldEnum::Text("These jumbo blueberry muffins with sour cream are absolutely the best! They’re super quick to whip up—ready in just 40 minutes! Each bite is filled with juicy blueberries, and the soft, moist center makes them totally irresistible. Here’s a little tip: don’t skip the turbinado sugar on top! It adds such a delightful texture and flavor that really takes the muffins to the next level. Trust me, it’s a total game-changer!".into())],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://zonacooks.com/wp-content/uploads/2025/12/Jumbo-Blueberry-Muffins-5.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zonacooks.com/wp-content/uploads/2025/12/Jumbo-Blueberry-Muffins-5-500x500.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zonacooks.com/wp-content/uploads/2025/12/Jumbo-Blueberry-Muffins-5-500x375.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://zonacooks.com/wp-content/uploads/2025/12/Jumbo-Blueberry-Muffins-5-480x270.jpg".into(),
                ),
            ],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL(
                    "Best Blueberry Muffins with Sour Cream, Easy Blueberry Muffins, Jumbo Blueberry Muffin, Jumbo Blueberry Muffin Recipe, Jumbo Blueberry Muffins".into(),
                ),
            ],
            prep_time: vec![DurationOrText::Text("PT10M".into())],
            name: vec!["Jumbo Blueberry Muffin Recipe".into()],
            nutrition: vec![
                NutritionInformation {
                    calories: vec![
                        Energy::new("552 kcal"),
                    ],
                    carbohydrate_content: vec![
                        Mass::new("79 g"),
                    ],
                    cholesterol_content: vec![
                        Mass::new("96 mg"),
                    ],
                    context: None,
                    fat_content: vec![
                        Mass::new("24 g"),
                    ],
                    fiber_content: vec![
                        Mass::new("3 g"),
                    ],
                    protein_content: vec![
                        Mass::new("8 g"),
                    ],
                    saturated_fat_content: vec![
                        Mass::new("8 g"),
                    ],
                    serving_size: vec!["1".into()],
                    sodium_content: vec![
                        Mass::new("138 mg"),
                    ],
                    sugar_content: vec![
                        Mass::new("44 g"),
                    ],
                    r#type: AtType::NutritionInformation.to_opt(),
                    trans_fat_content: vec![
                        Mass::new("0.4 g"),
                    ],
                    unsaturated_fat_content: vec![
                        Mass::new("14 g"),
                    ],
                },
            ],
            recipe_category: vec!["Side Dish".into()],
            recipe_cuisine: vec!["American".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup all-purpose flour".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/8 teaspoon kosher salt".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 teaspoon baking powder".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 teaspoon ground cinnamon".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 cup granulated sugar".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 large egg (room temperature)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/8 cup cooking oil (like vegetable or canola oil)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 Tablespoons unsalted butter (melted and slightly cooled)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/4 cup sour cream (room temperature)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/4 cup whole milk (room temperature)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 teaspoon vanilla extract (or almond extract)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup fresh blueberries".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "Turbinado sugar (for crunchy tops)".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Preheat the oven to 425℉ and prepare either a jumbo or standard muffin tin with liners. If not using liners, lightly grease each well.".into(),
                        ],
                        url: vec![
                            "https://zonacooks.com/jumbo-blueberry-muffins/#wprm-recipe-37028-step-0-0".into(),
                        ],
                        name: vec![
                            "Preheat the oven to 425℉ and prepare either a jumbo or standard muffin tin with liners. If not using liners, lightly grease each well.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "In a large bowl, whisk together the flour, salt, baking powder and cinnamon.".into(),
                        ],
                        url: vec![
                            "https://zonacooks.com/jumbo-blueberry-muffins/#wprm-recipe-37028-step-0-1".into(),
                        ],
                        name: vec![
                            "In a large bowl, whisk together the flour, salt, baking powder and cinnamon.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Add the blueberries to another bowl and toss with 1/2 tablespoon of the flour mixture and set aside.".into(),
                        ],
                        url: vec![
                            "https://zonacooks.com/jumbo-blueberry-muffins/#wprm-recipe-37028-step-0-2".into(),
                        ],
                        name: vec![
                            "Add the blueberries to another bowl and toss with 1/2 tablespoon of the flour mixture and set aside.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "In a third bowl, whisk together the sugar, egg, oil, melted butter, sour cream, whole milk, and vanilla.".into(),
                        ],
                        url: vec![
                            "https://zonacooks.com/jumbo-blueberry-muffins/#wprm-recipe-37028-step-0-3".into(),
                        ],
                        name: vec![
                            "In a third bowl, whisk together the sugar, egg, oil, melted butter, sour cream, whole milk, and vanilla.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Add the wet mixture to the dry mixture and use a rubber spatula to gently combine.".into(),
                        ],
                        url: vec![
                            "https://zonacooks.com/jumbo-blueberry-muffins/#wprm-recipe-37028-step-0-4".into(),
                        ],
                        name: vec![
                            "Add the wet mixture to the dry mixture and use a rubber spatula to gently combine.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Once combined, fold in the blueberries. Then, fill each well or liner with batter, mostly to the top, and sprinkle generously with the coarse turbinado sugar.".into(),
                        ],
                        url: vec![
                            "https://zonacooks.com/jumbo-blueberry-muffins/#wprm-recipe-37028-step-0-5".into(),
                        ],
                        name: vec![
                            "Once combined, fold in the blueberries. Then, fill each well or liner with batter, mostly to the top, and sprinkle generously with the coarse turbinado sugar.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Bake the muffins for 5 minutes at 425℉, then reduce the temperature to 400℉. Bake jumbo muffins for an additional 25 minutes, and regular muffins for an additional 20 minutes. For either, a toothpick inserted into the center should come out with moist crumbs and no wet batter.".into(),
                        ],
                        url: vec![
                            "https://zonacooks.com/jumbo-blueberry-muffins/#wprm-recipe-37028-step-0-6".into(),
                        ],
                        name: vec![
                            "Bake the muffins for 5 minutes at 425℉, then reduce the temperature to 400℉. Bake jumbo muffins for an additional 25 minutes, and regular muffins for an additional 20 minutes. For either, a toothpick inserted into the center should come out with moist crumbs and no wet batter.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Remove the pan to a wire rack to cool for 5 minutes, then remove each muffin from the well to continue cooling.".into(),
                        ],
                        url: vec![
                            "https://zonacooks.com/jumbo-blueberry-muffins/#wprm-recipe-37028-step-0-7".into(),
                        ],
                        name: vec![
                            "Remove the pan to a wire rack to cool for 5 minutes, then remove each muffin from the well to continue cooling.".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
            ],
            recipe_yield: vec![
                RecipeRecipeYieldFieldEnum::Text("3".into()),
                RecipeRecipeYieldFieldEnum::Text("3 Jumbo or 6 Regular".into()),
            ],
            review: vec![
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "I absolutely love the easy huge muffins. so good!".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![RatingRatingValueFieldEnum::Text("5".into())],
                            ..Default::default()
                        },
                    ],
                    date_published: vec!["2025-12-15".into()],
                    author: vec![RecipeAuthorFieldEnum::new_org("Zona House")],
                    ..Default::default()
                },
            ],
            total_time: vec![DurationOrText::Text("PT40M".into())],
            url: vec!["https://zonacooks.com/jumbo-blueberry-muffins/".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zsuzsaisinthekitchen_ok() -> Result<()> {
        let got = scrape(Website::ZsuzsaIsInTheKitchen, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            author: vec![RecipeAuthorFieldEnum::new_person("Zsuzsa")],
            date_published: vec!["2014-06-04T09:39:00-07:00".into()],
            description: vec![RecipeDescriptionFieldEnum::Text("My cherries are still green, but it won’t take long before we can enjoy this year’s produce if the birds would leave us some. With a large\ngarden and mature fruit trees we are keenly aware that changes are taking\nplace. The last couple of years brought in some never seen species of bugs and\nbirds. Some of the familiar wildlife that used to pass through from spring to\nlate fall disappeared and we have been getting new ones; large flocks are\ncoming through and sometimes eating up everything in sight. We had to replant\nseveral times. Ferocious birds would ransack our birdhouses and beat up on\nlocal species that have been nesting in the backyard for decades. The\nmosquitoes are deadly. There are too many storms and the breeze is a constant\nand we are getting brutal winds even on hot days. Climate change, it’s\nhappening.".into())],
            image: vec![RecipeImageFieldEnum::URL("https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEiU9JY6HBrPtcAUnFUNUbHUrQXzn6510J32gYBDr5J_zyPFQ_Xfz8LOAZVxPQeOHF3k-4M_g3V3SrUSz5yWTg0x8NL9QpQLUubazsSlHwQfKjBK9-7qYJSFIwJMPBDAvFIxAA7Bx6pfQxQ/s1600/cherry+chutney2.JPG".into())],
            name: vec!["CHERRY CHUTNEY".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "5 cups ripe, coarsely chopped cherries".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 apple cored, peeled and chopped".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 tsp whole allspice".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 cinnamon sticks".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 cup\u{a0}red wine vinegar".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 cup red onion, chopped".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "4 garlic clove, diced".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 tsp salt".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 cup honey".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cups raisins".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Prepare canning jars and lids.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "You will be reducing vinegar, open up the windows and make sure you have good ventilation.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Add the chopped cherries and apples to a stainless steel or enameled stockpot.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Tie the allspice and the cinnamon sticks into a cheesecloth bundle.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Add the spice bundle, vinegar, onions, garlic and salt to the stockpot.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "On medium heat cook for about 20 minutes or until the mixture thickens.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Stir the chutney occasionally.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Just as the chutney begins to spit add the honey and stir to combine.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Lower the heat and continue cooking at a simmer for 20 more minutes. Stir often.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Do not let it boil hard and certainly don’t leave it. At this stage the chutney scorches easily.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Add the raisins and bring back to simmer.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Remove from heat.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Remove the spice bag and fill the chutney into sterilized jars leaving 1/2-inch head space.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Process or pour into sterilized jars and place in the fridge for up to 6 weeks.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "► 2023 (52) ► December (3) ► November (10) ► October (2) ► September (21) ► August (3) ► January (13)".into(),
                ),
            ],
            url: vec![
                "https://zsuzsaisinthekitchen.blogspot.com/2014/06/cherry-chutney.html<number>0".into(),
            ],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zumavalley_ok() -> Result<()> {
        let got = scrape(Website::ZumaValley, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            description: vec![RecipeDescriptionFieldEnum::Text(
                "Enjoy this flavorful coconut fruit smoothie anytime of day!".into(),
            )],
            image: vec![RecipeImageFieldEnum::URL(
                "https://zumavalley.com/cdn/shop/articles/RecipeImages_Smoothie_Berry.png?v=1588746443"
                    .into(),
            )],
            name: vec!["Delicious Coconut Berries Smoothie".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("½ cup (1 pack) Zuma Valley Coconut Meat".into()),
                RecipeRecipeIngredientFieldEnum::Text(
                    "⅜ cup (1 pack) Zuma Valley Cold-Pressed Coconut Cream".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text("⅜ cup coconut water".into()),
                RecipeRecipeIngredientFieldEnum::Text("⅝ cup frozen raspberries".into()),
                RecipeRecipeIngredientFieldEnum::Text("¼ cup frozen blueberries".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text("Blend it all ingredients.".into()),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Top with extra berries, Zuma Valley Coconut Meat, and a drizzle of Zuma Valley Coconut Cream on top!".into(),
                ),
            ],
            url: vec![
                "https://zumavalley.com/blogs/smoothies-bowls/coconut-mango-smoothie-bowl<number>0"
                    .into(),
            ],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zuranaz_ok() -> Result<()> {
        let got = scrape(Website::Zuranaz, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            author: vec![RecipeAuthorFieldEnum::new_person("Zurana Masud")],
            date_published: vec!["2021-02-17".into()],
            image: vec![RecipeImageFieldEnum::URL(
                "https://zuranazrecipe.com/wp-content/uploads/2021/02/nutella-donut-3-575x262.png"
                    .into(),
            )],
            name: vec!["Nutella Donut l Stuffed Nutella Doughnut Melt In Mouth".into()],
            recipe_category: vec![
                "Appetizers & Snacks".into(),
                "Desserts, Mishty/Sweets".into(),
                "World Recipes".into(),
            ],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL("nutella donut".into()),
                RecipeKeywordsFieldEnum::TextOrURL("stuffed nutella doughnut".into()),
            ],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "flour/all Purpose Flour/Maida  - 3 cups (360 gram)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text("sugar  - 1/4 cup (50 gram)".into()),
                RecipeRecipeIngredientFieldEnum::Text("salt  - 1/2 tsp".into()),
                RecipeRecipeIngredientFieldEnum::Text("Instant Yeast/dry yeast  - 1 tbs".into()),
                RecipeRecipeIngredientFieldEnum::Text("one egg  - (substitute: 1/4 cup milk)".into()),
                RecipeRecipeIngredientFieldEnum::Text("butter  - 2 tbs".into()),
                RecipeRecipeIngredientFieldEnum::Text("vanilla   - 1/2 tsp".into()),
                RecipeRecipeIngredientFieldEnum::Text("milk  - 1 cup".into()),
                RecipeRecipeIngredientFieldEnum::Text("nutella  - 250 gram".into()),
                RecipeRecipeIngredientFieldEnum::Text(
                    "powder sugar/icing sugar  - as needed for coating".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "in a bowl sieve flour , then mix sugar, salt and yeast.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "whisk together egg, milk, butter and vanilla essence.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text("mix and knead to a soft dough.".into()),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "knead the dough for 6 to 8 minutes, until it's smooth & soft.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "place the dough in a greased bowl, cover and let it rise for one hour , until doubled in bulk.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "divide the dough into 16 individual parts. make smooth round shape 16 balls .(weight: 42 gram per ball)".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "place the ball on a parchment paper and lightly press the every ball.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "cover and rest them for 30 minutes to an hour, until doubled.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text("heat the oil in a heavy frying pan.".into()),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "carefully placed the doughnuts in the oil 2 or 3 at a time and fry until golden brown.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "turn over and cook the second side; each side should take no more than a minute.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "remove from the oil with a slotted spoon and drain on absorbent paper.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "fill the doughnuts with nutella, using your choice of sugar topping or glaze.".into(),
                ),
            ],
            url: vec!["https://zuranazrecipe.com/?p=7861<number>0".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_zweigles_ok() -> Result<()> {
        let got = scrape(Website::Zweigles, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            author: vec![RecipeAuthorFieldEnum::Text("Zweigles".into())],
            cook_time: vec![DurationOrText::Text("PT20M".into())],
            date_published: vec!["2023-08-11".into()],
            description: vec![RecipeDescriptionFieldEnum::Text("Want a hearty and comforting meal you can make in 35 minutes? This Polish Kielbasa Sheet Pan and Potatoes is your answer. Heat up some sauerkraut or throw together a salad to go with your sausage and potatoes and enjoy!".into())],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://zweigles.com/wp-content/uploads/2023/08/20230802_151648-scaled.jpg".into(),
                ),
            ],
            name: vec!["Polish Kielbasa Sheet Pan and Potatoes".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 lb Zweigle’s Smoked Polish Kielbasa".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1.5 lb potatoes (Yukon gold tastes best)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 large white onion".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "Seasoning (paprika, garlic powder, kosher salt, black pepper)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 tbsp olive oil".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 tbsp fresh Italian parsley".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "1. Preheat oven to 400° and line a baking sheet with parchment paper.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "2. Chop potatoes into uniform small pieces, place in a bowl and cover with water. Microwave until potatoes are soft enough to pierce with a fork. Drain and rinse.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "3. Roughly chop onion and add to large bowl.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "4. Cut Polish Kielbasa into 1 inch diagonal pieces and add to bowl.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "5. Add potatoes to the bowl and add seasoning and olive oil. Stir to coat and spread out an even layer on the baking sheet.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "6. Bake until sausage and potatoes are browned (around 20 mins).".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "7. Garnish with sour cream and chopped Italian parsley.".into(),
                ),
            ],
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("4 Servings".into())],
            total_time: vec![DurationOrText::Text("PT35M".into())],
            url: vec!["https://zweigles.com/recipes/polish-kielbasa-sheet-pan-and-potatoes/".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }
}
