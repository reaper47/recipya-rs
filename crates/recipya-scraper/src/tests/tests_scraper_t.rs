#[cfg(test)]
mod tests {
    use schema_org::{
        AggregateRating, AtType, CreativeWork, DurationOrText, Energy, Mass, NutritionInformation,
        Organization, Rating, Recipe, Review, at_context,
        field::{
            AggregateRatingBestRatingFieldEnum, AggregateRatingRatingValueFieldEnum,
            CreativeWorkImageFieldEnum, OrganizationDescriptionFieldEnum,
            OrganizationImageFieldEnum, RatingRatingValueFieldEnum, RatingWorstRatingFieldEnum,
            RecipeAuthorFieldEnum, RecipeCreatorFieldEnum, RecipeDescriptionFieldEnum,
            RecipeImageFieldEnum, RecipeInLanguageFieldEnum, RecipeKeywordsFieldEnum,
            RecipePublisherFieldEnum, RecipeRecipeIngredientFieldEnum,
            RecipeRecipeInstructionsFieldEnum, RecipeRecipeYieldFieldEnum,
        },
    };

    use crate::{tests::support::scraper::scrape, websites::Website};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_themodernproper_ok() -> Result<()> {
        let got = scrape(Website::TheModernProper, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            nutrition: vec![
                NutritionInformation {
                    calories: vec![Energy::new("404 calories")],
                    carbohydrate_content: vec![Mass::new("34 grams carbohydrates")],
                    cholesterol_content: vec![Mass::new("158 milligrams cholesterol")],
                    fat_content: vec![Mass::new("14 grams fat")],
                    fiber_content: vec![Mass::new("2 grams fiber")],
                    protein_content: vec![Mass::new("34 grams protein")],
                    saturated_fat_content: vec![Mass::new("2 grams saturated fat")],
                    sodium_content: vec![Mass::new("1394 milligrams sodium")],
                    sugar_content: vec![Mass::new("27 grams sugar")],
                    r#type: AtType::NutritionInformation.to_opt(),
                    ..Default::default()
                },
            ],
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("6".into())],
            recipe_cuisine: vec!["American".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("½ cup honey".into()),
                RecipeRecipeIngredientFieldEnum::Text("½ cup whole grain mustard".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 tablespoons extra-virgin olive oil".into()),
                RecipeRecipeIngredientFieldEnum::Text("¼ cup fresh lemon juice (from 2 lemons)".into()),
                RecipeRecipeIngredientFieldEnum::Text("¼ cup finely chopped fresh cilantro".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 clove garlic, grated".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 tablespoon yellow curry powder".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 teaspoon kosher salt".into()),
                RecipeRecipeIngredientFieldEnum::Text("½ teaspoon freshly cracked black pepper".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 teaspoon crushed red pepper flakes".into()),
                RecipeRecipeIngredientFieldEnum::Text("2½ pounds boneless, skinless chicken thighs".into()),
                RecipeRecipeIngredientFieldEnum::Text("6 Persian cucumbers, cut into 1½- inch pieces and smashed".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 peaches, pitted and thinly sliced".into()),
                RecipeRecipeIngredientFieldEnum::Text("½ cup loosely packed fresh cilantro, chopped, plus more for garnish".into()),
                RecipeRecipeIngredientFieldEnum::Text("½ cup loosely packed fresh mint leaves, roughly chopped, plus more for garnish".into()),
                RecipeRecipeIngredientFieldEnum::Text("¼ cup lemon juice (from 2 lemons)".into()),
                RecipeRecipeIngredientFieldEnum::Text("½ teaspoon kosher salt".into()),
            ],
            cook_time: vec![DurationOrText::Text("PT20M".into())],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Make the chicken. In a large bowl, whisk together the honey, mustard, olive oil, lemon juice, cilantro, garlic, curry powder, salt, pepper, and red pepper until smooth. Reserve ¼ cup of marinade. Add the chicken to the bowl and turn to coat. Cover and let marinate refrigerated for at least 30 minutes.".into(),
                        ],
                        image: vec![
                            CreativeWorkImageFieldEnum::URL(
                                "https://images.themodernproper.com/production/posts/GrilledHoneyMustardChickenwithPeachSalad_3.jpg?w=960&amp;h=960&amp;q=82&amp;fm=jpg&amp;fit=crop&amp;dm=1721145248&amp;s=035076842c0f91ce4ec83a8aa4e50352".into(),
                            ),
                        ],
                        url: vec![
                            "https://themodernproper.com/grilled-honey-mustard-chicken-with-peach-salad#step-1".into(),
                        ],
                        name: vec![
                            "marinate the chicken".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Preheat the grill on medium-low. Remove the chicken from the marinade, allowing any excess to drip off. Place the chicken on the grill and cook until the chicken begins to char, about 4 minutes per side. Brush the chicken with the marinade remaining in the bowl and continue cooking, turning occasionally, until the internal temperature of the chicken reaches 165°F on an instant-read thermometer, about 10 minutes more.".into(),
                        ],
                        image: vec![
                            CreativeWorkImageFieldEnum::URL(
                                "https://images.themodernproper.com/production/posts/GrilledHoneyMustardChickenwithPeachSalad_4.jpg?w=960&amp;h=960&amp;q=82&amp;fm=jpg&amp;fit=crop&amp;dm=1721145274&amp;s=455999835138bf004eec5e43cb352c6c".into(),
                            ),
                        ],
                        url: vec![
                            "https://themodernproper.com/grilled-honey-mustard-chicken-with-peach-salad#step-2".into(),
                        ],
                        name: vec![
                            "grill the chicken".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Meanwhile, make the salad. In a medium bowl, toss the cucumbers, peaches, cilantro, and mint with the lemon juice, ¼ cup reserved honey mustard marinade, and salt. Let marinate at room temperature for about 10 minutes.".into(),
                        ],
                        image: vec![
                            CreativeWorkImageFieldEnum::URL(
                                "https://images.themodernproper.com/production/posts/GrilledHoneyMustardChickenwithPeachSalad_5.jpg?w=960&amp;h=960&amp;q=82&amp;fm=jpg&amp;fit=crop&amp;dm=1721145303&amp;s=e99721a3d9ebb0bb79b5dcba27a68a0e".into(),
                            ),
                        ],
                        url: vec![
                            "https://themodernproper.com/grilled-honey-mustard-chicken-with-peach-salad#step-3".into(),
                        ],
                        name: vec![
                            "make the salad".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
                RecipeRecipeInstructionsFieldEnum::CreativeWork(
                    CreativeWork {
                        r#type: AtType::HowToStep.to_opt(),
                        text: vec![
                            "Arrange the chicken on a large platter and top with the peach salad. Garnish with cilantro and mint and serve family-style.".into(),
                        ],
                        image: vec![
                            CreativeWorkImageFieldEnum::URL(
                                "https://images.themodernproper.com/production/posts/GrilledHoneyMustardChickenwithPeachSalad_9.jpg?w=960&amp;h=960&amp;q=82&amp;fm=jpg&amp;fit=crop&amp;dm=1721145354&amp;s=2c5686198da1538102a27c71034f8843".into(),
                            ),
                        ],
                        url: vec![
                            "https://themodernproper.com/grilled-honey-mustard-chicken-with-peach-salad#step-4".into(),
                        ],
                        name: vec![
                            "serve".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
            ],
            recipe_category: vec![
                "lunch".into(),
                "dinner".into(),
                "dairy-free".into(),
                "gluten-free".into(),
                "grilling".into(),
                "kid-friendly".into(),
            ],
            prep_time: vec![DurationOrText::Text("PT45M".into())],
            total_time: vec![DurationOrText::Text("PT1H5M".into())],
            date_created: vec!["2024-07-15T17:23:35-07:00".into()],
            review: vec![
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "Really good but tooo much flavor. Id cut down on honey/sauce if making again".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        worst_rating: vec![RatingWorstRatingFieldEnum::Number(1.0)],
                        best_rating: vec![AggregateRatingBestRatingFieldEnum::Number(5.0)],
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Number(4.0),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2025-11-05".into()],
                author: vec![RecipeAuthorFieldEnum::new_org("Morgan")],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "What great flavors! Bright &amp; savory this is a wonderful summer dish that doesnt feel heavy. My Dad is a steak and potatoes guy and doesnt stray. He gobbled this up and said to keep the recipe. Served separately but we all ended up mixing it all together. Both the salad and chicken are great on their own but together they are very fulfilling.".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        worst_rating: vec![RatingWorstRatingFieldEnum::Number(1.0)],
                        best_rating: vec![AggregateRatingBestRatingFieldEnum::Number(5.0)],
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Number(5.0),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2025-09-19".into()],
                author: vec![RecipeAuthorFieldEnum::new_org("Jenni")],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "Absolutely delicious! I served the chicken and salad separately but prepare my own plate with the chicken topped with salad as directed, and it was *chefs kiss*. Such a fun way to use peaches, cukes, and herbs from our garden!".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        worst_rating: vec![RatingWorstRatingFieldEnum::Number(1.0)],
                        best_rating: vec![AggregateRatingBestRatingFieldEnum::Number(5.0)],
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Number(5.0),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2025-08-14".into()],
                author: vec![RecipeAuthorFieldEnum::new_org("Megan Vanderflute")],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "This super easy grilled chicken was a huge hit at our family Sunday dinner. Everyone raved. Not one piece left over! Used breasts and thighs and let it marinate overnight. The result was a tender and delicious main course. Will definitely be on the rotation. Thank you for sharing this one!".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        worst_rating: vec![RatingWorstRatingFieldEnum::Number(1.0)],
                        best_rating: vec![AggregateRatingBestRatingFieldEnum::Number(5.0)],
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Number(5.0),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2025-07-12".into()],
                author: vec![RecipeAuthorFieldEnum::new_org("Darian")],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "Weve made this twice already &amp; this recipe is a slam dunk! My three year old cant get enough of the chicken. My husband will tell you Im not the biggest chicken fan &amp; we were both eyeing up that last piece! The acidity, seasoning and sweetness are something I hadnt tried before. Ive sent the link for this recipe to a few friends already!".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        worst_rating: vec![RatingWorstRatingFieldEnum::Number(1.0)],
                        best_rating: vec![AggregateRatingBestRatingFieldEnum::Number(5.0)],
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Number(5.0),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2025-06-21".into()],
                author: vec![RecipeAuthorFieldEnum::new_org("Samantha R.")],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec!["Summer recipe on repeat! Simple and delicious - another winner. Thank you".into()],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        worst_rating: vec![RatingWorstRatingFieldEnum::Number(1.0)],
                        best_rating: vec![AggregateRatingBestRatingFieldEnum::Number(5.0)],
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Number(5.0),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2025-06-21".into()],
                author: vec![RecipeAuthorFieldEnum::new_org("Jessica Roach")],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "My daughter found this recipe and I made it for her and her 2 elementary age sons. This is so delicious! Bonus! Both boys loved the chicken and even like the salad. Her younger said I think I like cilantro now! \r\nDo try smashing the cucumber, it is fun and tastes great. Thank you for this delicious and FUN recipe!".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        worst_rating: vec![RatingWorstRatingFieldEnum::Number(1.0)],
                        best_rating: vec![AggregateRatingBestRatingFieldEnum::Number(5.0)],
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Number(5.0),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2025-06-05".into()],
                author: vec![RecipeAuthorFieldEnum::new_org("Debi Anzini")],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "This was ballerrrrrrrrrr! I added shallots to the lil salad. 15/10 recommend, mad flavor.".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        worst_rating: vec![RatingWorstRatingFieldEnum::Number(1.0)],
                        best_rating: vec![AggregateRatingBestRatingFieldEnum::Number(5.0)],
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Number(5.0),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2024-09-02".into()],
                author: vec![RecipeAuthorFieldEnum::new_org("Nurse Deez")],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "That meal was da bomb! I didnt have mint so it was missing that ingredient so I dont know how the flavor is impacted but I can tell you this meal is perfect for summer! A nice refreshing take on honey mustard".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        worst_rating: vec![RatingWorstRatingFieldEnum::Number(1.0)],
                        best_rating: vec![AggregateRatingBestRatingFieldEnum::Number(5.0)],
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Number(5.0),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2024-08-22".into()],
                author: vec![RecipeAuthorFieldEnum::new_org("Laurie Emerson")],
                ..Default::default()
            },
            Review {
                r#type: AtType::Review.to_opt(),
                review_body: vec![
                    "Outstanding! A great summer dish, so fresh as deliciousI I added Burrata cheese on the side. Thank you!".into(),
                ],
                review_rating: vec![
                    Rating {
                        r#type: AtType::Rating.to_opt(),
                        worst_rating: vec![RatingWorstRatingFieldEnum::Number(1.0)],
                        best_rating: vec![AggregateRatingBestRatingFieldEnum::Number(5.0)],
                        rating_value: vec![
                            RatingRatingValueFieldEnum::Number(5.0),
                        ],
                        ..Default::default()
                    },
                ],
                date_published: vec!["2024-08-21".into()],
                author: vec![RecipeAuthorFieldEnum::new_org("Robin Foreman")],
                ..Default::default()
            },
            ],
            publisher: vec![
                RecipePublisherFieldEnum::Organization(
                    Organization {
                        r#type: AtType::Organization.to_opt(),
                        image: vec![
                            OrganizationImageFieldEnum::URL(
                                "https://images.themodernproper.com/production/about/holly-natalie-2022.jpg?w=960&amp;h=960&amp;q=82&amp;fm=jpg&amp;fit=crop&amp;crop=focalpoint&amp;fp-x=0.5328&amp;fp-y=0.288&amp;dm=1737108511&amp;s=083f4b1eb102697a97dc45c6207aac15".into(),
                            ),
                        ],
                        description: vec![
                            OrganizationDescriptionFieldEnum::Text(
                                "Holly Erickson and Natalie Mortimer became friends over a shared love for food and hosting. The Modern Proper exists to empower people to cherish their loved ones and feed them well by providing a large library of beautiful and delicious recipes.".into(),
                            ),
                        ],
                        alternate_name: vec!["TMP".into()],
                        url: vec!["https://themodernproper.com/about".into()],
                        name: vec!["The Modern Proper".into()],
                        ..Default::default()
                    },
                ),
            ],
            date_modified: vec!["2025-07-30".into()],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL(
                    "grilled honey mustard chicken with peach salad, summer recipe, dairy-free recipe, gluten-free recipe, grilling, kid-friendly".into(),
                ),
            ],
            creator: vec![
                RecipeCreatorFieldEnum::Organization(
                    Organization {
                        r#type: AtType::Organization.to_opt(),
                        image: vec![
                            OrganizationImageFieldEnum::URL(
                                "https://images.themodernproper.com/production/about/holly-natalie-2022.jpg?w=960&amp;h=960&amp;q=82&amp;fm=jpg&amp;fit=crop&amp;crop=focalpoint&amp;fp-x=0.5328&amp;fp-y=0.288&amp;dm=1737108511&amp;s=083f4b1eb102697a97dc45c6207aac15".into(),
                            ),
                        ],
                        description: vec![
                            OrganizationDescriptionFieldEnum::Text(
                                "Holly Erickson and Natalie Mortimer became friends over a shared love for food and hosting. The Modern Proper exists to empower people to cherish their loved ones and feed them well by providing a large library of beautiful and delicious recipes.".into(),
                            ),
                        ],
                        alternate_name: vec![
                            "TMP".into(),
                        ],
                        url: vec![
                            "https://themodernproper.com/about".into(),
                        ],
                        name: vec![
                            "The Modern Proper".into(),
                        ],
                        ..Default::default()
                    },
                ),
            ],
            aggregate_rating: vec![
                AggregateRating {
                    r#type: AtType::AggregateRating.to_opt(),
                    rating_count: vec![19],
                    worst_rating: vec![RatingWorstRatingFieldEnum::Number(1.0)],
                    best_rating: vec![AggregateRatingBestRatingFieldEnum::Number(5.0)],
                    rating_value: vec![AggregateRatingRatingValueFieldEnum::Number(4.9)],
                    ..Default::default()
                },
            ],
            in_language: vec![RecipeInLanguageFieldEnum::Text("en-us".into())],
            date_published: vec!["2024-07-15".into()],
            copyright_year: vec![2024],
            author: vec![
                RecipeAuthorFieldEnum::Organization(
                    Organization {
                        r#type: AtType::Person.to_opt(),
                        url: vec!["https://www.themodernproper.com/about/holly-erickson".into()],
                        name: vec!["Holly Erickson".into()],
                        ..Default::default()
                    },
                ),
                RecipeAuthorFieldEnum::Organization(
                    Organization {
                        r#type: AtType::Person.to_opt(),
                        url: vec!["https://www.themodernproper.com/about/natalie-mortimer".into()],
                        name: vec!["Natalie Mortimer".into()],
                        ..Default::default()
                    },
                ),
            ],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://images.themodernproper.com/production/posts/GrilledHoneyMustardChickenwithPeachSalad_8.jpg?w=960&amp;h=540&amp;q=82&amp;fm=jpg&amp;fit=crop&amp;dm=1721145374&amp;s=ca4acaaea469695db50204a90b098bdd".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://images.themodernproper.com/production/posts/GrilledHoneyMustardChickenwithPeachSalad_8.jpg?w=960&amp;h=720&amp;q=82&amp;fm=jpg&amp;fit=crop&amp;dm=1721145374&amp;s=acab9feabb87325878055612b07ec653".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://images.themodernproper.com/production/posts/GrilledHoneyMustardChickenwithPeachSalad_8.jpg?w=960&amp;h=960&amp;q=82&amp;fm=jpg&amp;fit=crop&amp;dm=1721145374&amp;s=69afbd54b7a56f8d84ee89c0b4cc6e46".into(),
                ),
            ],
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "A little sweet, a little savory and a lot delicious, our grilled honey mustard chicken recipe is all about a homemade honey-mustard marinade that doubles as a delicious dressing for the finished meal.".into(),
                ),
            ],
            url: vec!["https://themodernproper.com/grilled-honey-mustard-chicken-with-peach-salad".into()],
            name: vec!["Grilled Honey Mustard Chicken with Peach Salad".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }
}
