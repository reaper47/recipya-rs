#[cfg(test)]
mod tests {
    use schema_org::{
        AggregateRating, AtType, Clip, DurationOrText, Energy, ImageObject, ItemList, Mass,
        NutritionInformation, Organization, Rating, Recipe, Review, at_context,
        field::{
            AggregateRatingRatingValueFieldEnum, ClipDescriptionFieldEnum,
            ImageObjectHeightFieldEnum, ImageObjectWidthFieldEnum,
            ItemListItemListElementFieldEnum, OrganizationLogoFieldEnum,
            RatingRatingValueFieldEnum, RecipeAuthorFieldEnum, RecipeDescriptionFieldEnum,
            RecipeImageFieldEnum, RecipeKeywordsFieldEnum, RecipePublisherFieldEnum,
            RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
            RecipeRecipeYieldFieldEnum, RecipeVideoFieldEnum, ReviewAuthorFieldEnum,
        },
    };

    use crate::{tests::support::scraper::scrape, websites::Website};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_acecanning_ok() -> Result<()> {
        let got = scrape(Website::AceCanning, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            image: vec![RecipeImageFieldEnum::URL(
                "http://www.acecanning.com/web/wp-content/uploads/2019/12/Puff.jpg".into(),
            )],
            name: vec!["HOMESOY Raspberry Cream Puff".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::ItemList(ItemList {
                    r#type: AtType::ItemList.to_string(),
                    item_list_element: vec![
                        ItemListItemListElementFieldEnum::Text("Daisy Margarine 75gm".into()),
                        ItemListItemListElementFieldEnum::Text("Brown Sugar 95gm".into()),
                        ItemListItemListElementFieldEnum::Text("All Purpose Flour 95gm".into()),
                        ItemListItemListElementFieldEnum::Text("Red Colouring".into()),
                    ],
                    name: vec!["For Topping".into()],
                    number_of_items: vec![4],
                    ..Default::default()
                }),
                RecipeRecipeIngredientFieldEnum::ItemList(ItemList {
                    r#type: AtType::ItemList.to_string(),
                    item_list_element: vec![
                        ItemListItemListElementFieldEnum::Text(
                            "HOMESOY No Sugar Added 250ml".into(),
                        ),
                        ItemListItemListElementFieldEnum::Text("Daisy Margarine 125gm".into()),
                        ItemListItemListElementFieldEnum::Text("All Purpose Flour 180gm".into()),
                        ItemListItemListElementFieldEnum::Text("Eggs 225gm".into()),
                        ItemListItemListElementFieldEnum::Text(
                            "HOMESOY No Sugar Added 1800ml".into(),
                        ),
                        ItemListItemListElementFieldEnum::Text("Egg Yolk 440gm".into()),
                        ItemListItemListElementFieldEnum::Text("Sugar 360gm".into()),
                        ItemListItemListElementFieldEnum::Text("Custard Powder 160gm".into()),
                        ItemListItemListElementFieldEnum::Text("Unsalted Butter 100gm".into()),
                        ItemListItemListElementFieldEnum::Text(
                            "Raspberry Puree (optional) 200gm".into(),
                        ),
                    ],
                    name: vec!["For Choux Pastry".into()],
                    number_of_items: vec![10],
                    ..Default::default()
                }),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::ItemList(ItemList {
                    item_list_element: vec![
                        ItemListItemListElementFieldEnum::Text("Mix all ingredients until uniform.".into()),
                        ItemListItemListElementFieldEnum::Text("Roll dough to 2mm thickness, put in chiller until harden.".into()),
                        ItemListItemListElementFieldEnum::Text("Cut dough into 3cm round shapes and keep in the freezer for later use.".into()),
                    ],
                    name: vec!["For Topping".into()],
                    number_of_items: vec![3],
                    ..Default::default()
                }.into()),
                RecipeRecipeInstructionsFieldEnum::ItemList(ItemList {
                    name: vec!["For Choux Pastry".into()],
                    number_of_items: vec![0],
                    ..Default::default()
                }.into()),
            ],
            url: vec![
                "http://www.acecanning.com/web/recipe_post/raspberry-cream-puff/<number>0".into(),
            ],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_adayinthelifeonthefarm_ok() -> Result<()> {
        let got = scrape(Website::ADayInTheLifeOnTheFarm, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            nutrition: vec![
                NutritionInformation {
                    calories: vec![Energy::new("527.84")],
                    carbohydrate_content: vec![Mass::new("40.45")],
                    cholesterol_content: vec![Mass::new("202.5")],
                    context: None,
                    fat_content: vec![Mass::new("10.84")],
                    fiber_content: vec![Mass::new("7.26")],
                    protein_content: vec![Mass::new("62.25")],
                    saturated_fat_content: vec![Mass::new("2.48")],
                    sodium_content: vec![Mass::new("1120.72")],
                    sugar_content: vec![Mass::new("12.34")],
                    r#type: AtType::NutritionInformation.to_opt(),
                    ..Default::default()
                },
            ],
            recipe_cuisine: vec!["Italian".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("1 rabbit".into()),
                RecipeRecipeIngredientFieldEnum::Text("olive oil".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/2 c. red wine".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 (14 oz) can diced tomatoes".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 (14 oz) can tomato sauce".into()),
                RecipeRecipeIngredientFieldEnum::Text("3 T. garlic".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/4 t. crushed red pepper".into()),
                RecipeRecipeIngredientFieldEnum::Text("salt and pepper, to taste".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 T. Italian Herb Blend".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 t. sugar, if desired".into()),
                RecipeRecipeIngredientFieldEnum::Text("8 oz. dried pasta, cooked per package directions".into()),
            ],
            cook_time: vec![DurationOrText::Text("PT1H45M".into())],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text("Cut the rabbit into 6 portions, season with salt and pepper.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Heat some olive oil in the bottom of a dutch oven. Add the chicken and sear on both sides. Add the garlic and crushed red pepper. Cook until fragrant and garlic begins to turn golden, about 30 seconds".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Add the wine and bring to a boil, scraping up any browned bits stuck to the bottl of the pan, until wine is reduced by half.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Add the tomatoes and tomato sauce. Stir in the herbs, cover, reduce heat to low and simmer for about an hour, until the rabbit is very tender.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Remove the rabbit from the stew, if desired, and remove the meat from the bones. Return the meat to the sauce, taste and add a bit of sugar if the sauce is too acidic.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Cook uncovered for about 45 minutes until sauce is thickened to desired consistency.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Serve over cooked pasta. I like egg noodles as they hold up well to the sauce.".into()),
            ],
            recipe_category: vec!["Entrees, Pasta".into()],
            r#yield: vec![RecipeRecipeYieldFieldEnum::Text("4 large servings".into())],
            prep_time: vec![DurationOrText::Text("PT15M".into())],
            total_time: vec![DurationOrText::Text("PT2H".into())],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL(
                    "Pasta, Rabbit, Stew, Game meat,".into(),
                ),
            ],
            author: vec![RecipeAuthorFieldEnum::new_org("Wendy Klik")],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEi9yNxDT-TQM8LHVf297TWsSJGHP3fs1ANIfI2P-PQ6iavfmIxqJ61AmzbLs9dob7bjwynn5YTIZERLjzzXMWyQxqMF5r4Y_nCEEYenZufGpkhK1xiv9oW9KDgpo0UskjT_BhMkQyv0VdUYRfvKkSEPaEqbU1QfDn257_BCBxAlfS9kD8aOllSrhoo8aw/w400-h266/4.jpg".into(),
                ),
            ],
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "This recipe is inspired by one served at Il Focolore in Ishchia, Campania that was featured on Searching for Italy starring Stanly Tucci. It is rich and delicious. Well worth the time it takes to braise the rabbit and make the \"gravy\".".into(),
                ),
            ],
            name: vec![
                "Coniglio All'Ischitana (Rabbit Stew)".into(),
            ],
            url: vec![
                "https://adayinthelifeonthefarm.blogspot.com/2023/05/coniglio-allischitana-rabbit-stew-and.html".into(),
            ],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_adultbar_ok() -> Result<()> {
        let got = scrape(Website::AdultBar, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            description: vec![RecipeDescriptionFieldEnum::Text("Copy Cat Cocktail shake licorice liqueur bottega apricot liqueur drillaud apricot juice ice licorice sticks black and served cold in a old fashioned glass.".into())],
            keywords: vec![
               RecipeKeywordsFieldEnum::TextOrURL(
                   "copy cat cocktail, licorice liqueur bottega, apricot liqueur drillaud, apricot juice, ice, licorice sticks black, 15% alcohol by volume, making cocktail".into(),
               ),
            ],
            image: vec![RecipeImageFieldEnum::URL("http://AdultBar.com.au/cocktails/images/cocktails/cocktails-old-fashioned-brown.png".into())],
            name: vec!["Copy Cat Cocktail".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("30 ml Licorice Liqueur Bottega".into()),
                RecipeRecipeIngredientFieldEnum::Text("30 ml Apricot Liqueur Drillaud".into()),
                RecipeRecipeIngredientFieldEnum::Text("30 ml Apricot Juice".into()),
                RecipeRecipeIngredientFieldEnum::Text("100 ml Ice".into()),
                RecipeRecipeIngredientFieldEnum::Text("Licorice Sticks Black".into()),
                RecipeRecipeIngredientFieldEnum::Text("15% Alcohol By Volume".into()),
                RecipeRecipeIngredientFieldEnum::Text("30 Proof".into()),
                RecipeRecipeIngredientFieldEnum::Text("1.1 Standard Drinks".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text("Place half the ice into shaker.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Place all the ingredients into shaker.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Shake ingredients together until mixed.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Pour all into glass.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Top glass with ice.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Drop in a licorice stick.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Bewitching".into()),
            ],
            recipe_yield: vec![
                RecipeRecipeYieldFieldEnum::Text("190".into()),
            ],
            url: vec!["http://adultbar.com.au/cocktails/How-To-Make-A/Elvin<number>0".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_allrecipes_ok() -> Result<()> {
        let got = scrape(Website::AllRecipes, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![
                AggregateRating {
                    r#type: AtType::AggregateRating.to_opt(),
                    rating_count: vec![19403],
                    rating_value: vec![AggregateRatingRatingValueFieldEnum::Text("4.6".into())],
                    ..Default::default()
                },
            ],
            author: vec![RecipeAuthorFieldEnum::new_org("Dora")],
            cook_time: vec![DurationOrText::Text("PT10M".into())],
            date_modified: vec!["2026-02-27T14:12:51-05:00".into()],
            date_published: vec!["1998-04-18T16:10:32-04:00".into()],
            description: vec![RecipeDescriptionFieldEnum::Text("This classic chocolate chip cookie recipe makes deliciously buttery cookies with crisp edges, chewy middles, and gooey chocolate chips in every bite.".into())],
            headline: vec!["Best Chocolate Chip Cookies".into()],
            image: vec![
                    RecipeImageFieldEnum::ImageObject(ImageObject {
                        r#type: AtType::ImageObject.to_opt(),
                        width: vec![ImageObjectWidthFieldEnum::Integer(1500)],
                        height: vec![ImageObjectHeightFieldEnum::Integer(1125)],
                        url: vec![
                            "https://www.allrecipes.com/thmb/8xwaWAHtl_QLij6D-G0Z4B1HDVA=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-146-4x3-b108aceffa6043a1ac81c3c5a9b034c8.jpg".into(),
                        ],
                        ..Default::default()
                    }.into()),
            ],
            keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("publisher-tested".into())],
            nutrition: vec![NutritionInformation {
                calories: vec![Energy::new("146 kcal")],
                carbohydrate_content: vec![Mass::new("19 g")],
                cholesterol_content: vec![Mass::new("10 mg")],
                context: None,
                fat_content: vec![Mass::new("8 g")],
                fiber_content: vec![Mass::new("1 g")],
                protein_content: vec![Mass::new("2 g")],
                saturated_fat_content: vec![Mass::new("4 g")],
                serving_size: vec![],
                sodium_content: vec![Mass::new("76 mg")],
                sugar_content: vec![],
                r#type: AtType::NutritionInformation.to_opt(),
                trans_fat_content: vec![],
                unsaturated_fat_content: vec![Mass::new("0 g")],
            }],
            name: vec!["Best Chocolate Chip Cookies".into()],
            prep_time: vec![DurationOrText::Text("PT20M".into())],
            publisher: vec![
                RecipePublisherFieldEnum::Organization(
                    Organization {
                        r#type: AtType::Organization.to_opt(),
                        logo: vec![
                            OrganizationLogoFieldEnum::ImageObject(
                                ImageObject {
                                    r#type: AtType::ImageObject.to_opt(),
                                    width: vec![ImageObjectWidthFieldEnum::Integer(112)],
                                    height: vec![ImageObjectHeightFieldEnum::Integer(112)],
                                    url: vec![
                                        "https://www.allrecipes.com/thmb/Z9lwz1y0B5aX-cemPiTgpn5YB0k=/112x112/filters:no_upscale():max_bytes(150000):strip_icc()/allrecipes_logo_schema-867c69d2999b439a9eba923a445ccfe3.png".into(),
                                    ],
                                    ..Default::default()
                                }.into(),
                            ),
                        ],
                        same_as: vec![
                            "https://www.facebook.com/allrecipes".into(),
                            "https://www.instagram.com/allrecipes/".into(),
                            "https://www.pinterest.com/allrecipes/".into(),
                            "https://www.tiktok.com/@allrecipes".into(),
                            "https://www.youtube.com/user/allrecipes/videos".into(),
                            "https://flipboard.com/@Allrecipes".into(),
                            "https://en.wikipedia.org/wiki/Allrecipes.com".into(),
                            "http://linkedin.com/company/allrecipes.com".into(),
                        ],
                        url: vec![
                            "https://www.allrecipes.com".into(),
                        ],
                        name: vec![
                            "Allrecipes".into(),
                        ],
                        ..Default::default()
                    },
                ),
            ],
            total_time: vec![DurationOrText::Text("PT30M".into())],
            recipe_yield: vec![
                RecipeRecipeYieldFieldEnum::Text("48".into()),
                RecipeRecipeYieldFieldEnum::Text("48 cookies".into()),

            ],
            recipe_category: vec!["Dessert".into()],
            recipe_cuisine: vec!["American".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup butter, softened".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup white sugar".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup packed brown sugar".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 eggs".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 teaspoons vanilla extract".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 teaspoon baking soda".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 teaspoons hot water".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "0.5 teaspoon salt".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "3 cups all-purpose flour".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 cups semisweet chocolate chips".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup chopped walnuts".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Gather your ingredients, making sure your butter is softened, and your eggs are room temperature.",
                    Some("https://www.allrecipes.com/thmb/ikAh8YlzsTfWmVA6G6MRHlq7xtU=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-ddmfs-3188-4x3-step-01-61fd6c03b33f40e0a8bd756cb56ecac1.jpg"),
                    None, None,
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Preheat the oven to 350 degrees F (175 degrees C). Beat butter, white sugar, and brown sugar in a large bowl with an electric mixer until smooth and creamy.",
                    Some("https://www.allrecipes.com/thmb/HZVO7UCA4f6YjwRUxMm2bo5Wz2U=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-2-1191-1b57f642a52849ce84a25c48363c4013.jpg"),
                    None, None,
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Beat in eggs, one at a time, then stir in vanilla.",
                    Some("https://www.allrecipes.com/thmb/vb_CS6Q-CEf0cZySd5HgkUV-T_Y=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-3-121-f73dc5abc3454dfab534d8efc0e966e6.jpg"),
                    None, None,
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Dissolve baking soda in hot water; add to batter along with salt and mix until combined.",
                    Some("https://www.allrecipes.com/thmb/EHPMpRCIHiCbzy9a3hGDZn0Tgyc=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-4-123-abd5290df87f47668cbc8eef6587c292.jpg"),
                    None, None,
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Stir in flour, chocolate chips, and walnuts until a soft dough forms.",
                    Some("https://www.allrecipes.com/thmb/4cwC_GLPXQVWlqI2KH0GUbC1-V4=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-5-125-b755dee648e949129ad022d8b019b405.jpg"),
                    None, None,
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Drop rounded spoonfuls of cookie dough 2 inches apart onto ungreased baking sheets.",
                    Some("https://www.allrecipes.com/thmb/A63M4EgkpKCDtqT1qWgMciQ3-hI=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-6-128-44e1d4355a6a4441a3ba993e2a2b74fa.jpg"),
                    None, None,
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Bake in the preheated oven until edges are lightly browned, about 10 minutes.",
                    Some("https://www.allrecipes.com/thmb/NFy1pucQzBWKkvRGRvcQgFcmu9E=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-7-134-e0952f5171a2434dbb2b3bb53b18648a.jpg"),
                    None, None,
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Cool on the baking sheets briefly before removing to a wire rack to cool completely.",
                    Some("https://www.allrecipes.com/thmb/5tMxJUaTaqQNYD3O-56aG4L9t00=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-8-138-d4f25b55db5c417b9d6ece8cf98700f3.jpg"),
                    None, None,
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Store in an airtight container or serve immediately and enjoy!",
                    Some("https://www.allrecipes.com/thmb/8xwaWAHtl_QLij6D-G0Z4B1HDVA=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-146-4x3-b108aceffa6043a1ac81c3c5a9b034c8.jpg"),
                    None, None,
                ),
            ],
            review: vec![
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "vERRY eXXCELLENT cOOOKIE! I worked 10 years as a baker and the recipe we used at the bake shop was similar except this recipe didn't call for any cream of tartar, so I added it to my version. I added 1/2 tspn. What it does is make the cookie crack better when baking which gives it a better appeal in appearance as appose to that smooth top looking cookie which I don't like. It doesn't do anything for flavor so if you don't care about whether its smooth or cracked then don't bother with the suggestion.".into(),
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
                    author: vec![ReviewAuthorFieldEnum::new_org("Chuck Sampson")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "I've tried a lot of different chocolate chip cookie recipes and this is the best one by far! The only change I've made is to the sugar amounts: 1/2 cup white sugar and 1 1/2 cups brown sugar-I think it makes them chewier. I omit the nuts and in my oven they take about 12 minutes to bake. So yummy!!".into(),
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
                    date_published: vec![
                        "2005-11-17T15:41:13.467Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Jen Gerbrandt")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "These chocolate chip cookies were very, very good. I have been trying chocolate chip cookie recipes forever to find the perfect cookie and this one is very close. They weren't too cake-like and they weren't thin and greasy, they were the perfect in-between cookie. That said, the dough is very thick once you mix all the ingredients together. A bit of advice - if you like a thinner cookie and not so cake-like, make a smaller ball of dough and when you place it on the cookie sheet be sure to flatten it quite a bit as the dough doesn't seem to spread when in the oven. If you like thick cookies, just leave them in the ball and they'll stay big. As the recipe states, they are crisp on the outside and chewy on the inside - still nice and moist the following day. I added some Skor toffee bits to the recipe and they turned out amazing - will definitely make again!! Thanks for sharing.".into(),
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
                    date_published: vec![
                        "2006-01-05T06:43:10.670Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("SPORTSNUT")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Superb. I give 4 1/2 stars to the original recipe since it calls for 50/50 white &amp;amp; brown sugar. Since then, I've used ALL brown sugar and increased the salt to 2/3 teaspoons, and it was PERFECT. The flavor was a lot richer. I've tried 4 recipes from this site and I'm glad I finally found my Master recipe. Some of the other recipes on this site calls for pudding mix in the batter - which baffles me. This is an excellent cookie (soft and chewy) that has only good old-fashioned ingredients. Most importantly, it stays chewy, not hard. Look no further folks, this is the one. (For walnut sized dough balls, and I did roll them for a more uniform look, 11-12 min. was perfect. 10 min. was still a little doughy). TIP - if your cookies sit around for a few days (if that's possible) and gets a little dry, put a fresh slice of sandwich bread in the same container as the cookies and they will moisten right up.".into(),
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
                    date_published: vec![
                        "2008-02-05T15:28:09.803Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("MommyFromSeattle")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Perfect Chocolate Chip recipe. I did not use hot water or nuts and I rolled into two logs and wrapped in plastic wrap. Put both in large freezer bag and sliced and baked off when needed. I baked at 350 for 13 minutes (frozen).".into(),
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
                    date_published: vec![
                        "2022-01-06T22:53:07.457Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("cxgc5b24fd")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "This cookie was amazing! They baked perfectly in my oven at 325 for 12 minutes. I followed a couple suggestions of reducing white sugar to 1/2 C and increasing brown sugar to 1 1/2 C as well as adding 1 C of white chocolate chips. Instead of walnuts I added roasted pecan pieces to mine. My husband took one bite and said \"you're a goddess\" haha This will be my go-to cookie recipe".into(),
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
                    date_published: vec![
                        "2018-08-24T15:16:41.833Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Amy Mendivil")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "I did as another rater suggested: skipped dissolving baking soda in water and increased the vanilla to 1 tablespoon, and changed the sugar to 1/2 cup white and 1.5 cups brown. Baked for 12 minutes at 350 and OMG they taste like they came from a bakery! I have never made better cookies.\n\nI do not own a mixer and therefore make everything by hand, so this recipe seemed daunting at first... but with a little elbow grease, it was a snap.".into(),
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
                    date_published: vec![
                        "2012-05-31T23:06:14.880Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("angelmuffin")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "These are awesome cookies. I followed the advice of others by making the following tweaks: (1) I used a half cup of sugar and one-and-a-half cups of brown sugar; (2) I omitted the water; (3) I added a teaspoon of cream of tartar to the batter; (4) I refrigerated the batter for at least an hour before scooping and baking. The cookies retained their shape and didn't spread when baked. I think the cream of tartar made a difference, and the cookies did not come out too cakey either! This recipe is going in my \"favorites\" cookbook.".into(),
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
                    date_published: vec![
                        "2025-06-21T22:03:07.018Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("litg8r")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "They came out a little more crunchy than I would've liked. I chilled the dough in the fridge for about 10 minutes. I also have a convection oven so I had to do about 14 minutes in the oven to get them to even be slightly brown, let cool on the pan for about 2 minutes before moving to the cooling rack. I used Ghirardelli chocolate chips and had a great flavor with those.".into(),
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
                    date_published: vec![
                        "2024-12-08T05:23:59.990Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Wickedella")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "To make these as Christmas cookies, I used 1 cup plain white sugar and 1 cup of Moreno or pure cane unbleached sugar. I added a few drops peppermint essential oil. Then instead of plain chocolate chips, I used a combination of crushed peppermint candy and chopped up Andes mints. They did take longer to bake than the recipe said, about 15 minutes.".into(),
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
                    date_published: vec![
                        "2024-12-05T21:53:04.482Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Lillian Hughes")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Best cookies I have ever made. I will say that you should weigh your flour in grams. All-Purpose flours weighs 120 grams per cup. If you sift the flour into a cup you will have the right amount but if you just dip the cup in the flour you will get too much. This will end up making the cookies had and not turn out right. Just over 2 cups resulted the in correct weight in grams I needed for the recipe. No one sifts flour anymore and when you are baking it makes a huge difference. I would suggest trying the recipe again weighing the flour if you did not get good results. Also, blend the butter and sugar thoroughly (room temperature no salt added butter is a must). I will definitely make these again. I made exactly as stated.".into(),
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
                    date_published: vec![
                        "2024-09-16T15:57:50.614Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Amy Gates")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "I made them as the recipes stated.. but added semi sweet &amp;amp; white chocolate chips and chopped pecans. The cookies came out perfect and are SO good!!!!".into(),
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
                    date_published: vec![
                        "2026-03-12T21:45:44.898Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Wanda Bartlett")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "I took tips and tricks from allll the nice reviews!! Thank you!! These were perfection!!!".into(),
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
                    date_published: vec![
                        "2026-03-12T00:47:50.908Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Kellyb")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "I made this recipe 4 times by now, and its amazing, but it can harden overtime".into(),
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
                    date_published: vec![
                        "2026-03-10T08:42:26.474Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("parchment pro7")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Totally worked like a charm; now the big question: Does the water trick work with peanut butter cookies?".into(),
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
                    date_published: vec![
                        "2026-03-08T16:26:32.016Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Paleface")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Turned into a soupy goopy mess, recipe doesn’t work at all".into(),
                    ],
                    review_rating: vec![
                        Rating {
                            r#type: AtType::Rating.to_opt(),
                            rating_value: vec![
                                RatingRatingValueFieldEnum::Text("1".into()),
                            ],
                            ..Default::default()
                        },
                    ],
                    date_published: vec![
                        "2026-02-26T18:33:52.844Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("alex")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Great Recipe that works well. The flavor isn't quite as rich as i would like, so I'm going to experiment with using dark brown sugar or perhaps a bit more salt.".into(),
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
                    date_published: vec![
                        "2026-02-23T18:58:53.043Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Kevin")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "This is my go to cookie!! I've been making this recipe for a few years now and it gets better every time. Sometimes I add extras, but I always get compliments on this recipe. Depending if I want them chewy or more crunchy, I'll add another cup of flour and i add milk chocolate chips instead of semi sweet. Must try recipe!".into(),
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
                    date_published: vec![
                        "2026-02-21T04:46:14.787Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("JazzyCress5311")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "This is my first time i actually attempt to make cookies and the recipe genuinely works out 😭😭 im so happy its so good i added coco powder to the dough sooo.. it might look a lil different".into(),
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
                    date_published: vec![
                        "2026-02-18T19:06:16.977Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("yakozawa")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "I only made half of a batch, froze some of the dough, and used a pinch of lemon extract for part of the flavor instead of all vanilla. They are delicious!".into(),
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
                    date_published: vec![
                        "2026-02-18T06:03:53.072Z".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Just Desserts")],
                    ..Default::default()
                },
            ],
            url: vec![
                "https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/".into(),
            ],
            video: vec![
               RecipeVideoFieldEnum::Clip(
                   Clip {
                       r#type: AtType::VideoObject.to_opt(),
                       thumbnail_url: vec![
                           "https://cdn.jwplayer.com/v2/media/qHQSNVCK/thumbnails/tibAvzY8.jpg?width=1280".into(),
                       ],
                       description: vec![
                           ClipDescriptionFieldEnum::Text(
                               "When it comes to fresh and delicious cookies, nothing beats homemade. In this video, Nicole shows you how to make Allrecipes’s top rated recipe for chocolate chip cookies. Using a deliciously sweet cookie dough that’s beaten with brown sugar and vanilla, fold in mini chocolate chips. Once the dough has been chilled in the fridge, distribute it onto a baking sheet and bake for ten minutes in the oven. With a tender texture and a rich, buttery taste, they're the ultimate comfort snack!".into(),
                           ),
                       ],
                       name:vec! [
                           "How to Make the Best Chocolate Chip Cookies".into(),
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
    async fn test_allroadsleadtothekitchen_ok() -> Result<()> {
        let got = scrape(Website::AllRoadsLeadToTheKitchen, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            author: vec![RecipeAuthorFieldEnum::new_person(
                "Heather Schmitt-Gonzalez",
            )],
            cook_time: vec![DurationOrText::Text("PT1H".into())],
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "Chicken wings smothered in a garlicky parmesan sauce are the perfect non-spicy wing offering (and sure to become a fast favorite).".into(),
                ),
            ],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEhlrKfVQ_aQexgyJj2db3Ue3T0HJtkkeFee0l3jsYgxo9rHLtBtL4MHLKicvKGFnW18Dzq-UblnVXn0-J7cO2hz6RXILi_QeG4lGSLU4S3ileFQER8g1UZVkaec1Q52r7X_Y1uai7vezV6F/s1600/Garlic+Parmesan+Wings+recipe.jpg".into(),
                ),
            ],
            name: vec!["Garlic Parmesan Wings".into()],
            prep_time: vec![DurationOrText::Text("PT15M".into())],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("2 to 2.5 pounds chicken wings (see here for ".into()),
                RecipeRecipeIngredientFieldEnum::Text("how to break down a chicken wing".into()),
                RecipeRecipeIngredientFieldEnum::Text(")".into()),
                RecipeRecipeIngredientFieldEnum::Text("vegetable oil, to coat".into()),
                RecipeRecipeIngredientFieldEnum::Text("sea salt, to taste".into()),
                RecipeRecipeIngredientFieldEnum::Text("garlic powder, to taste".into()),
                RecipeRecipeIngredientFieldEnum::Text("freshly ground black pepper, to taste".into()),
                RecipeRecipeIngredientFieldEnum::Text("4 ounces salted butter".into()),
                RecipeRecipeIngredientFieldEnum::Text("4 fat cloves minced garlic".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 teaspoon Italian seasoning".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 cup grated Parmesan cheese".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Preheat oven to 400° F. Line a baking sheet with foil, then set a wire rack on top.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Place the broken down chicken wings in a gallon-sized ziploc baggie. Drizzle them with a light coating of vegetable oil, then season them with salt, garlic powder, and pepper (use enough to flavor them well). Zip the baggie shut and smoosh everything around so the wings are evenly coated.\u{a0}".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Tip them out onto the prepared baking sheet. Slide into the hot oven and bake for 50-60 minutes, or until the wings register 165° F on an instant-read thermometer and are as crispy as you like them on the outside. (Alternately, you could fry them.)".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "While the wings are in the oven, melt the butter, garlic cloves, and Italian seasoning together on the stovetop or in the microwave. Once melted, stir in the Parmesan cheese. Transfer to a bowl large enough to hold the wings.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "When the wings come out of the oven, carefully add them to the bowl with the sauce. Place another bowl upside down over that bowl, hold them together tightly with potholders, and shake to coat them in sauce.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Serve immediately.".into(),
                ),
            ],
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("4".into())],
            total_time: vec![DurationOrText::Text("PT1H15M".into())],
            url: vec![
                "https://www.allroadsleadtothe.kitchen/2019/12/garlic-parmesan-wings.html<number>0"
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
    #[allow(clippy::too_many_lines)]
    async fn test_allthingsmamma_ok() -> Result<()> {
        let got = scrape(Website::AllThingsMamma, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![
                AggregateRating {
                    r#type: AtType::AggregateRating.to_opt(),
                    review_count: vec![
                        3,
                    ],
                    rating_count: vec![
                        39,
                    ],
                    rating_value: vec![
                        AggregateRatingRatingValueFieldEnum::Text(
                            "3.49".into(),
                        ),
                    ],
                    ..Default::default()
                },
            ],
            date_published: vec![
                "2020-12-16T00:00:00+00:00".into(),
            ],
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "Million Dollar Dip is made with only 5 ingredients - mayonaise, cheddar cheese, bacon, green onions, and almonds.".into(),
                ),
            ],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://www.allthingsmamma.com/wp-content/uploads/2020/12/million-dollar-dip.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://www.allthingsmamma.com/wp-content/uploads/2020/12/million-dollar-dip-500x500.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://www.allthingsmamma.com/wp-content/uploads/2020/12/million-dollar-dip-500x375.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://www.allthingsmamma.com/wp-content/uploads/2020/12/million-dollar-dip-480x270.jpg".into(),
                ),
            ],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL(
                    "cheese, dip recipe, easy appetizer".into(),
                ),
            ],
            name: vec!["Million Dollar Dip".into()],
            nutrition: vec![NutritionInformation {
                calories: vec![Energy::new("526 kcal")],
                carbohydrate_content: vec![Mass::new("2 g")],
                cholesterol_content: vec![Mass::new("58 mg")],
                context: None,
                fat_content: vec![Mass::new("53 g")],
                fiber_content: vec![Mass::new("0.4 g")],
                protein_content: vec![Mass::new("10 g")],
                saturated_fat_content: vec![Mass::new("13 g")],
                serving_size: vec!["1 serving".into()],
                sodium_content: vec![Mass::new("587 mg")],
                sugar_content: vec![Mass::new("1 g")],
                r#type: AtType::NutritionInformation.to_opt(),
                trans_fat_content: vec![Mass::new("0.1 g")],
                unsaturated_fat_content: vec![Mass::new("38 g")],
            }],
            prep_time: vec![
                DurationOrText::Text(
                    "PT5M".into(),
                ),
            ],
            recipe_category: vec![
                "Appetizer".into(),
            ],
            recipe_cuisine: vec![
                "American".into(),
            ],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 3/4 cup mayonnaise".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 cups cheddar cheese (shredded)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "5 green onions (diced)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "5 slices bacon (cooked and chopped)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/4 cup almonds (slivered or sliced)".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Combine all of the ingredients into a bowl and mix until fully combined.",
                    None,
                    Some("Combine"),
                    Some("https://www.allthingsmamma.com/million-dollar-dip/#wprm-recipe-36586-step-0-0"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Refrigerate for at least 2 hours and enjoy with your favorite crackers.",
                    None,
                    Some("Chill"),
                    Some("https://www.allthingsmamma.com/million-dollar-dip/#wprm-recipe-36586-step-0-1"),
                ),
            ],
            recipe_yield: vec![
                RecipeRecipeYieldFieldEnum::Text(
                    "8".into(),
                ),
                RecipeRecipeYieldFieldEnum::Text(
                    "8 servings".into(),
                ),
            ],
            review: vec![
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Looks good. Something like I would do for Thanksgiving and Christmas.".into(),
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
                    date_published: vec![
                        "2021-11-05".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Tammie Ann Weaver")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Looks good. Something like I would do for Thanksgiving and Christmas.".into(),
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
                    date_published: vec![
                        "2021-11-05".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Tammie Weaver")],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Way to much mayo. I would start with 3/4 cup and add more if needed. Otherwise awesome dip.\r\nI put it on tortillas and rolled them and put in fridge until cold then sliced them in 1 inch rounds and severed with salsa. Very good.".into(),
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
                    date_published: vec![
                        "2021-10-30".into(),
                    ],
                    author: vec![ReviewAuthorFieldEnum::new_org("Vicky")],
                    ..Default::default()
                },
            ],
            total_time: vec![
                DurationOrText::Text(
                    "PT125M".into(),
                ),
            ],
            url: vec![
                "https://www.allthingsmamma.com/million-dollar-dip/#wprm-recipe-container-36586"
                    .into(),
            ],
            video: vec![
                RecipeVideoFieldEnum::Clip(
                    Clip {
                        r#type: AtType::VideoObject.to_opt(),
                        thumbnail_url: vec![
                            "https://i.ytimg.com/vi/nwjs0Y6BMHo/hqdefault.jpg".into(),
                        ],
                        description: vec![
                            ClipDescriptionFieldEnum::Text(
                                "This Million Dollar Dip is made with just 5 simple ingredients: mayonnaise, cheddar cheese, bacon, green onions, and almonds. It’s the ultimate game-day or holiday appetizer that tastes amazing with crackers, pretzels, or fresh veggies.\n\nIngredients:\n1 3/4 cup mayonnaise\n2 cups cheddar cheese – shredded\n5 Green Onions – diced\n5 slices bacon – cooked and chopped\n1/4 cup Almonds – Slivered or sliced\n\nGet the full recipe: https://www.allthingsmamma.com/million-dollar-dip/".into(),
                            ),
                        ],
                        name: vec![
                            "Million Dollar Dip #diprecipe #appetizerideas #gamedayfood".into(),
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
    async fn test_almanac_ok() -> Result<()> {
        let got = scrape(Website::Almanac, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            author: vec![RecipeAuthorFieldEnum::Organization(Organization {
                r#type: AtType::Person.to_opt(),
                name: vec!["Ken Haedrich".to_string()],
                url: vec!["https://www.almanac.com/user/585856".into()],
                ..Default::default()
            })],
            date_published: vec![
                "2026-01-26T15:30:58-0500".into(),
            ],
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "These Ginger Bran Muffins by Ken Haedrich are hearty, warmly spiced, and just sweet enough to feel like a treat. Moist, tender, and deeply flavorful, they’re ideal for breakfast, snacking, or a cozy afternoon with&nbsp;tea.Made with wheat bran, applesauce, molasses, and both ground and crystallized ginger, they strike a perfect balance between wholesome and&nbsp;indulgent.&nbsp;".into(),
                ),
            ],
            image: vec![
                RecipeImageFieldEnum::ImageObject(ImageObject {
                    r#type: AtType::ImageObject.to_opt(),
                    representative_of_page: vec!["True".into()],
                    height: vec![ImageObjectHeightFieldEnum::Integer(800)],
                    width: vec![ImageObjectWidthFieldEnum::Integer(1200)],
                    url: vec!["https://www.almanac.com/sites/default/files/styles/large/public/recipe-ginger_bran_muffins.jpg?itok=NcHGn24x".into()],
                    ..Default::default()
                }.into()),
            ],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL(
                    "Bread".into(),
                ),
            ],
            name: vec![
                "Ginger Bran Muffins".into(),
            ],
            recipe_category: vec![
                "Bread".into(),
            ],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "1-1⁄2 cups all-purpose flour".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "3⁄4 cup plain oat bran or wheat bran (not packaged bran cereal)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2-1⁄2 teaspoons baking powder".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "3⁄4 teaspoon salt".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1⁄2 teaspoon ground ginger".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1⁄3 cup chopped crystallized ginger".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 large egg".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup sweetened or unsweetened applesauce".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1⁄2 cup packed light-brown sugar".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1⁄3 cup vegetable oil".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1⁄4 cup milk".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 tablespoons molasses".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::new_creative_work("Preheat the oven to 375°F.", None, None, None),
                RecipeRecipeInstructionsFieldEnum::new_creative_work("Butter 10 muffin cups (or 12, for slightly smaller muffins) or line with paper liners.Combine the flour, bran, baking powder, salt, and ground ginger in a large bowl and whisk to blend.", None, None, None),
                RecipeRecipeInstructionsFieldEnum::new_creative_work("Add the crystallized ginger, mix, and make a well.In a separate bowl, whisk the egg until frothy.", None, None, None),
                RecipeRecipeInstructionsFieldEnum::new_creative_work("Add the applesauce, brown sugar, oil, milk, and molasses and whisk to blend.", None, None, None),
                RecipeRecipeInstructionsFieldEnum::new_creative_work("Pour into the well and stir with a wooden spoon until evenly blended.Divide the batter evenly among the prepared cups.Bake for 23 to 25 minutes, or until the muffins form domes that spring back when touched.Transfer the pan to a cooling rack for 5 minutes, then remove the muffins from the pan and place on the rack until ready to serve.", None, None, None),
            ],
            recipe_yield: vec![
                RecipeRecipeYieldFieldEnum::Text(
                    "Makes 10 to 12 muffins.".into(),
                ),
            ],
            url: vec!["https://www.almanac.com/recipe/ginger-bran-muffins".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    async fn test_almondandfig_ok() -> Result<()> {
        let got = scrape(Website::AlmondAndFig, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "Golden, crisp, and filled with spinach and feta — these phyllo cigars are a beloved staple in my kitchen. Our mothers and grandmothers always keep a stash of ma‘ajanat (savory pastries) in the freezer, ready to bake when guests arrive unexpectedly. Some are made with cheese, meat or savory vegetable fillings. Spinach, a winter crop in Palestine, finds its way into comforting stews, fresh salads, and flaky pastries like this — a simple gesture of warmth and hospitality shared around the table.".into(),
                ),
            ],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://images.squarespace-cdn.com/content/v1/65a8c83ff8fd7a06122db371/7e12b7ad-e42a-44b2-b961-8ec4f0f461b5/dc207f_33e0a2f9eac34fcbb9d9d15d7d35d5ea%7Emv2.jpg".into(),
                ),
            ],
            name: vec!["Baked Spinach and feta phyllo cigars".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::new_section("", &[
                        "1 package frozen phyllo dough thawed For the filling",
                        "1 bag frozen spinach thawed",
                        "1 cup crumbled feta",
                        "1 red onion minced (you can sautée it or leave it raw)",
                        "Salt and pepper to taste",
                        "A few grates of fresh nutmeg",
                        "Zest of one Lemon",
                        "2 tsp sumac",
                        "2 tbls pine nuts Nigella seeds or sesame seeds and flaky salt (optional)",
                        "Olive oil for brushing",
                ]),
                RecipeRecipeIngredientFieldEnum::new_section("Dipping Sauce", &[
                        "1 cup Greek yogurt",
                        "1/4 cup crumbled feta",
                        "Juice and zest of one Lemon",
                        "A drizzle of olive oil",
                       "Whirl all ingredients in a blender or a mixer until smooth and fluffy",
                ])
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Filling: Squeeze the spinach until all the water comes out you can do that using a couple of paper towels. Add salt, pepper, nutmeg, sumac, feta, pine nuts, onions, lemon zest.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Take 1 sheet of filo dough (make sure to cover the remaining sheets with a clean kitchen towel to prevent them from drying)".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Cut the sheet of phyllo into 3 even strips and brush with olive oil.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Fold each strip in half creating a double layer.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Place one teaspoon of the filling along the short side of the rectangle and roll creating a cigar ha!".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Brush the top with olive oil and add a sprinkle of nigella seeds and flaky salt on top if you want to be fancy u know.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Brush, fill, roll and repeat until you are done it goes by so fast.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Line the cigars in a parchment lined baking sheet and bake in a 350 oven for about 20 min until golden and crisp.".into(),
                ),
            ],
            url: vec![
                "https://www.almondandfig.com/baked-spinach-and-feta-phyllo-cigars<number>0".into(),
            ],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_alphafoodie_ok() -> Result<()> {
        let got = scrape(Website::AlphaFoodie, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![
                AggregateRating {
                    r#type: AtType::AggregateRating.to_opt(),
                    review_count: vec![
                        1,
                    ],
                    rating_count: vec![
                        1,
                    ],
                    rating_value: vec![
                        AggregateRatingRatingValueFieldEnum::Text(
                            "5".into(),
                        ),
                    ],
                    ..Default::default()
                },
            ],
            author: vec![RecipeAuthorFieldEnum::new_org("Samira")],
            cook_time: vec![
                DurationOrText::Text(
                    "PT25M".into(),
                ),
            ],
            date_published: vec![
                "2026-02-12T15:54:25+00:00".into(),
            ],
            description: vec![RecipeDescriptionFieldEnum::Text(
                "These sheet pan chicken fajitas are bright, juicy, and perfect for weeknights. Toss, roast, and finish with lime for that fajita punch.".into(),
            )],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://www.alphafoodie.com/wp-content/uploads/2026/02/sheet-pan-chicken-fajitas-1-of-1-2.jpeg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://www.alphafoodie.com/wp-content/uploads/2026/02/sheet-pan-chicken-fajitas-1-of-1-2-500x500.jpeg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://www.alphafoodie.com/wp-content/uploads/2026/02/sheet-pan-chicken-fajitas-1-of-1-2-500x375.jpeg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://www.alphafoodie.com/wp-content/uploads/2026/02/sheet-pan-chicken-fajitas-1-of-1-2-480x270.jpeg".into(),
                ),
            ],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL(
                    "baked fajitas, one pan fajitas, oven fajitas, sheet pan fajitas".into(),
                ),
            ],
            name: vec!["Easy Sheet Pan Chicken Fajitas".into()],
            nutrition: vec![NutritionInformation {
                calories: vec![Energy::new("228 kcal")],
                carbohydrate_content: vec![Mass::new("22 g")],
                cholesterol_content: vec![Mass::new("36 mg")],
                context: None,
                fat_content: vec![Mass::new("9 g")],
                fiber_content: vec![Mass::new("3 g")],
                protein_content: vec![Mass::new("16 g")],
                saturated_fat_content: vec![Mass::new("2 g")],
                serving_size: vec!["1 serving".into()],
                sodium_content: vec![Mass::new("493 mg")],
                sugar_content: vec![Mass::new("5 g")],
                r#type: AtType::NutritionInformation.to_opt(),
                trans_fat_content: vec![Mass::new("0.01 g")],
                unsaturated_fat_content: vec![Mass::new("6 g")],
            }],
            prep_time: vec![
                DurationOrText::Text(
                    "PT5M".into(),
                ),
            ],
            recipe_category: vec![
                "Dinner".into(),
            ],
            recipe_cuisine: vec![
                "Mexican".into(),
            ],
            recipe_yield: vec![
                RecipeRecipeYieldFieldEnum::Text(
                    "6".into(),
                ),
            ],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "3 chicken breasts (boneless skinless, cut into thin strips)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "3 bell peppers (mixed colors, cored and sliced)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 yellow onion (large, thinly sliced)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 tablespoons olive oil".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 tablespoons fajita seasoning (see homemade blend below or store‑bought)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "6-8 flour tortillas (small, warmed)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 lime (cut into wedges (plus more for serving))".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "¼ cup cilantro (chopped (plus more for serving))".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup Guacamole (for serving)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 cup pico de gallo (for serving)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1 teaspoon chili powder".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "½ teaspoon smoked paprika".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "½ teaspoon ground cumin".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "½ teaspoon garlic powder".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "½ teaspoon onion powder".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "½ teaspoon fine sea salt".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "¼ teaspoon black pepper".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "¼ teaspoon dried oregano (optional)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "¼ teaspoon Cayenne pepper (optional)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "½ teaspoon sugar (optional)".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Preheat the oven to 425°F and lightly oil a large rimmed sheet pan. Pat the chicken dry and slice against the grain. Slice the peppers and onion into similar thickness for even cooking.",
                    None,
                    Some("Preheat the oven to 425°F and lightly oil a large rimmed sheet pan. Pat the chicken dry and slice against the grain. Slice the peppers and onion into similar thickness for even cooking."),
                    Some("https://www.alphafoodie.com/sheet-pan-chicken-fajitas/#wprm-recipe-148738-step-0-0"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Add the chicken, peppers, and onion to the pan. Drizzle with olive oil and sprinkle with the fajita seasoning. Toss to coat, then spread into a roomy single layer.",
                    None,
                    Some("Add the chicken, peppers, and onion to the pan. Drizzle with olive oil and sprinkle with the fajita seasoning. Toss to coat, then spread into a roomy single layer."),
                    Some("https://www.alphafoodie.com/sheet-pan-chicken-fajitas/#wprm-recipe-148738-step-0-1"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Bake on the top rack for 20 to 25 minutes until the chicken is opaque and the vegetables are tender with caramelized edges. For more char, broil for 1 to 2 minutes at the end.",
                    None,
                    Some("Bake on the top rack for 20 to 25 minutes until the chicken is opaque and the vegetables are tender with caramelized edges. For more char, broil for 1 to 2 minutes at the end."),
                    Some("https://www.alphafoodie.com/sheet-pan-chicken-fajitas/#wprm-recipe-148738-step-0-2"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Wrap the tortillas in foil and warm them in the oven during the last few minutes. Squeeze fresh lime over the fajitas, sprinkle with cilantro, and serve hot with guacamole and pico de gallo.",
                    None,
                    Some("Wrap the tortillas in foil and warm them in the oven during the last few minutes. Squeeze fresh lime over the fajitas, sprinkle with cilantro, and serve hot with guacamole and pico de gallo."),
                    Some("https://www.alphafoodie.com/sheet-pan-chicken-fajitas/#wprm-recipe-148738-step-0-3"),
                ),
            ],
            review: vec![
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Looks delicious!".into(),
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
                    author: vec![ReviewAuthorFieldEnum::new_org("josh")],
                    date_published: vec![
                        "2026-02-17".into(),
                    ],
                    ..Default::default()
                },
            ],
            total_time: vec![
                DurationOrText::Text(
                    "PT30M".into(),
                ),
            ],
            url: vec!["https://www.alphafoodie.com/sheet-pan-chicken-fajitas/".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_alsothecrumbsplease_ok() -> Result<()> {
        let got = scrape(Website::AlsoTheCrumbsPlease, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            aggregate_rating: vec![
                AggregateRating {
                    r#type: AtType::AggregateRating.to_opt(),
                    review_count: vec![
                        4,
                    ],
                    rating_count: vec![
                        14,
                    ],
                    rating_value: vec![
                        AggregateRatingRatingValueFieldEnum::Text("5".into()),
                    ],
                    ..Default::default()
                },
            ],
            cook_time: vec![DurationOrText::Text("PT40M".into())],
            date_published: vec![
                 "2021-12-06T06:00:00+00:00".into(),
            ],
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "These brownies are ultra-fudgy and have the most delicious peanut butter filling. Even if you are a novice home baker, you will love how simple and easy this recipe is. All you need are a few basic ingredients to make this chocolaty treat from scratch.".into(),
                ),
            ],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://www.alsothecrumbsplease.com/wp-content/uploads/2021/11/Peanut-Butter-Brownies-13.jpg".into(),
                ),
            ],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL(
                    "chocolate peanut butter brownie recipe, pb brownies, peanut butter brownies, peanut butter filled brownies, peanut butter swirl brownies".into(),
                ),
            ],
            name: vec!["The Best Chocolate Peanut Butter Brownies".into()],
            nutrition: vec![NutritionInformation {
                calories: vec![Energy::new("348 kcal")],
                carbohydrate_content: vec![Mass::new("32 g")],
                cholesterol_content: vec![Mass::new("47 mg")],
                context: None,
                fat_content: vec![Mass::new("23 g")],
                fiber_content: vec![Mass::new("3 g")],
                protein_content: vec![Mass::new("7 g")],
                saturated_fat_content: vec![Mass::new("11 g")],
                serving_size: vec!["1 serving".into()],
                sodium_content: vec![Mass::new("159 mg")],
                sugar_content: vec![Mass::new("23 g")],
                r#type: AtType::NutritionInformation.to_opt(),
                trans_fat_content: vec![Mass::new("1 g")],
                unsaturated_fat_content: vec![Mass::new("11 g")],
            }],
            prep_time: vec![DurationOrText::Text("PT15M".into())],
            recipe_yield: vec![
                RecipeRecipeYieldFieldEnum::Text("16".into()),
                RecipeRecipeYieldFieldEnum::Text("16 servings".into()),
            ],
            recipe_category: vec![
                "Dessert".into(),
            ],
            recipe_cuisine: vec!["American".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("1 cup creamy peanut butter".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/2 cup powdered sugar, sifted".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/4 cup unsalted butter, melted".into()),
                RecipeRecipeIngredientFieldEnum::Text(
                    "8 oz semi-sweet chocolate bars, roughly chopped".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/2 cup unsalted butter, cut into a few pieces".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "1/4 cup unsweetened cocoa powder, spooned and leveled".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text("2 large eggs".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 cup granulated sugar".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 tsp vanilla extract".into()),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2/3 cup all-purpose flour, spooned and leveled".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text("1/2 tsp salt".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Preheat the oven to 350°F (177°C) and line a 9x9-inch (23x23cm) brownie pan with parchment paper. Set aside.",
                    None,
                    Some(
                        "Preheat the oven to 350°F (177°C) and line a 9x9-inch (23x23cm) brownie pan with parchment paper. Set aside.",
                    ),
                    Some(
                        "https://www.alsothecrumbsplease.com/the-best-peanut-butter-brownies-recipe/#wprm-recipe-6353-step-0-0",
                    ),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "In a large mixing bowl, whisk the peanut butter, powdered sugar, and melted butter until smooth and combined. Set aside.",
                    None,
                    Some(
                        "In a large mixing bowl, whisk the peanut butter, powdered sugar, and melted butter until smooth and combined. Set aside.",
                    ),
                    Some(
                        "https://www.alsothecrumbsplease.com/the-best-peanut-butter-brownies-recipe/#wprm-recipe-6353-step-0-1",
                    ),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Place the chocolate and butter in a microwave-safe bowl and heat until melted. Stir every 15-20 seconds to prevent the chocolate from overheating and burning. Remove the bowl from the microwave, add the cocoa powder and stir until combined. Let rest for 5 minutes.",
                    None,
                    Some(
                        "Place the chocolate and butter in a microwave-safe bowl and heat until melted. Stir every 15-20 seconds to prevent the chocolate from overheating and burning. Remove the bowl from the microwave, add the cocoa powder and stir until combined. Let rest for 5 minutes.",
                    ),
                    Some(
                        "https://www.alsothecrumbsplease.com/the-best-peanut-butter-brownies-recipe/#wprm-recipe-6353-step-0-2",
                    ),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "In a large mixing bowl, beat the eggs, sugar and vanilla for about 2-3 minutes until frothy. Add the chocolate mixture and whisk to combine. Then add the flour and salt and stir just to combine.",
                    None,
                    Some(
                        "In a large mixing bowl, beat the eggs, sugar and vanilla for about 2-3 minutes until frothy. Add the chocolate mixture and whisk to combine. Then add the flour and salt and stir just to combine.",
                    ),
                    Some(
                        "https://www.alsothecrumbsplease.com/the-best-peanut-butter-brownies-recipe/#wprm-recipe-6353-step-0-3",
                    ),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "You can assemble the brownie batter and peanut butter filling in two ways, filled or swirled. Either pour half of the brownie batter into the pan and cover the bottom, then add the peanut butter filling and top with the remaining brownie batter. Or you can alternate spoon dollops of peanut butter filling and chocolate brownie batter into the pan and draw a swirl.",
                    None,
                    Some(
                        "You can assemble the brownie batter and peanut butter filling in two ways, filled or swirled. Either pour half of the brownie batter into the pan and cover the bottom, then add the peanut butter filling and top with the remaining brownie batter. Or you can alternate spoon dollops of peanut butter filling and chocolate brownie batter into the pan and draw a swirl.",
                    ),
                    Some(
                        "https://www.alsothecrumbsplease.com/the-best-peanut-butter-brownies-recipe/#wprm-recipe-6353-step-0-4",
                    ),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Bake for 15 minutes. Then, remove the brownies from the oven and let them cool for 10 minutes. Turn the pan around, return to the oven, and bake for another 15-20 minutes, or until a toothpick comes out slightly dirty with a few crumbs attached.",
                    None,
                    Some(
                        "Bake for 15 minutes. Then, remove the brownies from the oven and let them cool for 10 minutes. Turn the pan around, return to the oven, and bake for another 15-20 minutes, or until a toothpick comes out slightly dirty with a few crumbs attached.",
                    ),
                    Some(
                        "https://www.alsothecrumbsplease.com/the-best-peanut-butter-brownies-recipe/#wprm-recipe-6353-step-0-5",
                    ),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Remove from the oven and let cool in the pan until firm enough to lift them out of the pan without breaking them, about 45-60 minutes. Then transfer to a cooling rack and let cool completely.",
                    None,
                    Some(
                        "Remove from the oven and let cool in the pan until firm enough to lift them out of the pan without breaking them, about 45-60 minutes. Then transfer to a cooling rack and let cool completely.",
                    ),
                    Some(
                        "https://www.alsothecrumbsplease.com/the-best-peanut-butter-brownies-recipe/#wprm-recipe-6353-step-0-6",
                    ),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Cut into squares and serve. Store leftovers in an airtight container for up to 4 days or freeze for up to 3 months.",
                    None,
                    Some(
                        "Cut into squares and serve. Store leftovers in an airtight container for up to 4 days or freeze for up to 3 months.",
                    ),
                    Some(
                        "https://www.alsothecrumbsplease.com/the-best-peanut-butter-brownies-recipe/#wprm-recipe-6353-step-0-7",
                    ),
                ),
            ],
            review: vec![
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Delicious! I used semi-sweet chocolate chips because that's what I had and brown sugar instead of white. Also used Kirkland creamy peanut butter (only peanuts and water). Followed recipe other than that (including taking them out of the oven for 10 mins) and they turned out perfect. Left them in for a total of 40 mins. Thank you for sharing! Will make again for sure.".into(),
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
                    author: vec![ReviewAuthorFieldEnum::new_org("Viv")],
                    date_published: vec![
                        "2025-06-22".into(),
                    ],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "These are incredible!! My friends seemed unsure of the peanut butter at first but then went back for FOURTHS. These are definitely my new favourite, oh and they're so fudgy in the middle!!".into(),
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
                    author: vec![ReviewAuthorFieldEnum::new_org("Sian")],
                    date_published: vec![
                        "2020-06-08".into(),
                    ],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "Thx fir that revipe. Found it on Pinterest and had to give it a try. I appreciate the leasurement were also in metric (i'm in france). And, most if all, the result is really TASTY ! Will def do it again. Thumbs up !".into(),
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
                    author: vec![ReviewAuthorFieldEnum::new_org("Nou Ga")],
                    date_published: vec![
                        "2019-06-21".into(),
                    ],
                    ..Default::default()
                },
                Review {
                    r#type: AtType::Review.to_opt(),
                    review_body: vec![
                        "These are amazing!do you know if these will keep any longer? I can usually keep brownies for a couple of weeks, although normally baked with melted chocolate in the mix so don't know if this makes a difference?".into(),
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
                    author: vec![ReviewAuthorFieldEnum::new_org("Brinda")],
                    date_published: vec![
                        "2019-06-10".into(),
                    ],
                    ..Default::default()
                },
            ],
            total_time: vec![DurationOrText::Text("PT55M".into())],
            url: vec![
                "https://www.alsothecrumbsplease.com/the-best-peanut-butter-brownies-recipe/"
                    .into(),
            ],
            video: vec![
                RecipeVideoFieldEnum::Clip(
                    Clip {
                        r#type: AtType::VideoObject.to_opt(),
                        thumbnail_url: vec![
                            "https://content.jwplatform.com/thumbs/HGYEZsWh-720.jpg".into(),
                        ],
                        description: vec![
                            ClipDescriptionFieldEnum::Text(
                                "Who doesn't love to sink their teeth into gooey peanut butter brownies? These brownies are ultra-fudgy and have the most delicious peanut butter filling. Even if you are a novice home baker, you will love how simple and easy this recipe is. All you need are a few basic ingredients to make this chocolaty treat from scratch.".into(),
                            ),
                        ],
                        name: vec![
                            "Chocolate Peanut Butter Brownies".into(),
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
    #[allow(clippy::too_many_lines)]
    async fn test_alvaradostreetbakery_ok() -> Result<()> {
        let got = scrape(Website::AlvaradoStreetBakery, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "Grilled cheese grew up. Sweet fig butter, crisp green apple, and creamy brie - toasted on bread that can actually hold its own against bold flavors. Fancy enough for company, easy enough for a weeknight.".into(),
                ),
                RecipeDescriptionFieldEnum::Text(
                    "How To: Fig Butter".into(),
                ),
                RecipeDescriptionFieldEnum::Text(
                    "Add ½ cup dried turkish figs to a small blender with 2 tablespoons water and ¼ cup coconut\u{a0}sugar; blend until smooth.".into(),
                ),
            ],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://www.alvaradostreetbakery.com/cdn/shop/articles/ALV-RecipeImages-Apple-Fig_4d3ee45a-249a-4ff7-95d8-497e7a5cb957.jpg?v=1772121623".into(),
                ),
            ],
            name: vec![
                "Apple & Fig Butter Grilled Brie".into(),
            ],
            prep_time: vec![
                DurationOrText::Text("10 minutes".into()),
            ],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 slices of your favorite Alvarado Street Bakery Bread".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "2 tablespoons fig butter (store bought or home\u{ad}made)".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "¼ green apple sliced thin".into(),
                ),
                RecipeRecipeIngredientFieldEnum::Text(
                    "3 slices brie cheese (look for vegan/dairy\u{ad}free brie cheese if preferred)".into(),
                ),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text("Coat one side of each slice of bread with a small amount of cooking oil.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("On the opposite side of one slice, spread on the fig butter, layer with apple and brie slices.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Top with the other slice of bread and toast in a large skillet over medium heat for 3\u{ad}-5 minutes on each side. Enjoy!".into()),
            ],
            recipe_yield: vec![
                RecipeRecipeYieldFieldEnum::Text("1 Serving".into()),
            ],
            url: vec![
                "https://www.alvaradostreetbakery.com/blogs/recipes/apple-fig-butter-grilled-brie<number>0"
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
    #[allow(clippy::too_many_lines)]
    async fn test_alwadi_ok() -> Result<()> {
        let got = scrape(Website::Alwadi, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            cook_time: vec![DurationOrText::Text("20 min".into())],
            description: vec![RecipeDescriptionFieldEnum::Text("Delicious and delightful vegan Falafel balls especially with the tartor sauce.".into())],
            image: vec![RecipeImageFieldEnum::URL("https://www.alwadi.com/ContentFiles/1315Image.jpg".into())],
            name: vec!["Falafel With Tarator Sauce".into()],
            recipe_category: vec!["Side Dish".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("1 Box / 200g Al Wadi Al Akhdar Falafel".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 Cup water".into()),
                RecipeRecipeIngredientFieldEnum::Text("Cooking oil".into()),
                RecipeRecipeIngredientFieldEnum::Text("½ Cup pickled cucumbers, sliced".into()),
                RecipeRecipeIngredientFieldEnum::Text("½ Cup pickled turnips, chunks".into()),
                RecipeRecipeIngredientFieldEnum::Text("¼ Cup fresh parsley, chopped".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 Tomato, cut into wedges".into()),
                RecipeRecipeIngredientFieldEnum::Text("Pita bread (or Lebanese bread)".into()),
                RecipeRecipeIngredientFieldEnum::Text("¼ Cup tarator sauce".into()),
                RecipeRecipeIngredientFieldEnum::Text("Mint leaves, to decorate".into()),
                RecipeRecipeIngredientFieldEnum::Text("Tarator Sauce".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 Cup water".into()),
                RecipeRecipeIngredientFieldEnum::Text("½ Cup lemon juice".into()),
                RecipeRecipeIngredientFieldEnum::Text("½ Cup Al Wadi Al Akhdar Tahina".into()),
                RecipeRecipeIngredientFieldEnum::Text("Salt to Taste".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text("Mix each envelope of Falafel mix with 120ml / ½ cup of water. Let it stand for 10 minutes.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Use a spoon (or a Falafel spoon) to form the falafel paste into flattened balls.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Heat oil over medium-high heat. Fry few falafel at a time until golden brown.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("To test the temperature is right, each ball should sink and rise to the surface and float. Gently remove from oil and drain on paper towels.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Serve Falafel balls with Tarator sauce accompanied by pickles, parsley, tomato, mint, and pita bread.".into()),
            ],
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("2".into())],
            url: vec![
                "https://www.alwadi.com/english/recipes/falafel-with-tarator-sauce?filterIdCategory=&filterIdCollection=&searchKeyword=&source=collection&sourceId=1773<number>0"
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
    #[allow(clippy::too_many_lines)]
    async fn test_annamsrecipes_ok() -> Result<()> {
        let got = scrape(Website::AnnamsRecipes, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            date_published: vec!["2014-10-13T17:32:00+00:00".into()],
            date_modified: vec!["2014-10-13T17:32:00+00:00".into()],
            description: vec![RecipeDescriptionFieldEnum::Text("Chettinad\u{a0}Porivilangai Urundai\u{a0} In my childhood my mom used to make Porivilangai urundai. We don’t like it at that time. Now I asked my mom to make Porivilangai urundai. She is a good cook and she …".into())],
            image: vec![RecipeImageFieldEnum::URL("https://blogger.googleusercontent.com/img/b/R29vZ2xl/AVvXsEixUyfI3-Q_fkN7lFOT9wwq53zX_-i-X5ySoa_u3GWNVX_DGTz3KJm8k6xmidDL3P5M4xCfQMfuhmIfK1sH88zDSj_Hn4ZOiHvnNTMeVt9deDzEZsizdYklOqLGAZSnqcW9P6QuSjjLcCSf/s1600/Porivilangai-Urundai-1.gif".into())],
            name: vec!["Porivilangai Urundai".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("Dry roast and grind:".into()),
                RecipeRecipeIngredientFieldEnum::Text("Boiled rice (Idli rice) – 1 cup".into()),
                RecipeRecipeIngredientFieldEnum::Text("Green gram dal – ½ cup".into()),
                RecipeRecipeIngredientFieldEnum::Text("For urundai:".into()),
                RecipeRecipeIngredientFieldEnum::Text("Ground flour – 3 cups".into()),
                RecipeRecipeIngredientFieldEnum::Text("Jaggery – 250 grams".into()),
                RecipeRecipeIngredientFieldEnum::Text("Coconut – 3 tsp".into()),
                RecipeRecipeIngredientFieldEnum::Text("Fried gram – 3 tsp".into()),
                RecipeRecipeIngredientFieldEnum::Text("Cardamom powder – ¼ tsp".into()),
                RecipeRecipeIngredientFieldEnum::Text("Ghee – 2 tblsp".into()),
                RecipeRecipeIngredientFieldEnum::Text("Water – ¼ cup".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text("Dry roast rice and dal seperately till nice aroma comes. Mix together and grind it in the rice mill. Take 3 cups of flour. Cut coconut in to tiny bits.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Melt jaggery in water and strain it. Heat ghee in a pan and fry cashew and fried gram.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Add jaggery syrup, cardamom powder and flour and mix well and knead into a smooth dough.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Grease palm with little ghee and make balls.".into()),
            ],
            url: vec![
                "https://annamsrecipes.wordpress.com/2014/10/13/porivilangai-urundai/<number>0"
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
    #[allow(clippy::too_many_lines)]
    async fn test_arbuz_ok() -> Result<()> {
        let got = scrape(Website::Arbuz, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            author: vec![RecipeAuthorFieldEnum::new_org("Lola Elise")],
            cook_time: vec![
                DurationOrText::Text("PT40M".into()),
            ],
            date_published: vec![
                "2019-04-06T18:37:24+00:00".into(),
            ],
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "This layered bread is something you have to absolutely try. It is called Qatlama Patir in the Uzbek language, which means Layered flatbread. Its crispy and buttery texture will have you craving for more in no time.".into(),
                ),
            ],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://arbuz.com/wp-content/uploads/2019/04/Flaky-Layered-Flatbread-47.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://arbuz.com/wp-content/uploads/2019/04/Flaky-Layered-Flatbread-47-500x500.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://arbuz.com/wp-content/uploads/2019/04/Flaky-Layered-Flatbread-47-500x375.jpg".into(),
                ),
                RecipeImageFieldEnum::URL(
                    "https://arbuz.com/wp-content/uploads/2019/04/Flaky-Layered-Flatbread-47-480x270.jpg".into(),
                ),
            ],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL(
                    "Bread, Paratha, Layered Bread, Qatlama Patir".into(),
                ),
            ],
            name: vec!["Layered Bread - Qatlama Patir".into()],
            recipe_category: vec![
                "Bread".into(),
            ],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("1 cup hot water".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 cup cold milk".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/2 Tbsp salt".into()),
                RecipeRecipeIngredientFieldEnum::Text("4 cups (loosely measured all-purpose flour)".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 egg (for egg wash (optional))".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 cup melted (clarified butter (substitute with either butter or ghee))".into()),
                RecipeRecipeIngredientFieldEnum::Text("sesame seeds (optional)".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "In a medium bowl place hot water, cold milk, salt, and mix well. Add pre-measured flour and mix until all of the flour is moistened.",
                    None,
                    Some("In a medium bowl place hot water, cold milk, salt, and mix well. Add pre-measured flour and mix until all of the flour is moistened."),
                    Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-0"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Start kneading the dough until smooth. At most you may need an extra tablespoon of flour in order to scrape the sides of the bowl, clean the dough off of your hands and to finish of kneading. This process can also be done in a stand mixer.",
                    None,
                    Some("Start kneading the dough until smooth. At most you may need an extra tablespoon of flour in order to scrape the sides of the bowl, clean the dough off of your hands and to finish of kneading. This process can also be done in a stand mixer."),
                    Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-1"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "The final dough will not be very soft, but it will be pliable and easily manageable. Divide it into two and cover it with a plastic wrap (or lately my favorite – reusable silicone pouches). Let the dough rest for about 15 minutes.",
                    None,
                    Some("The final dough will not be very soft, but it will be pliable and easily manageable. Divide it into two and cover it with a plastic wrap (or lately my favorite – reusable silicone pouches). Let the dough rest for about 15 minutes."),
                    Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-2"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Once the dough has rested, take one dough out and place it on a flat surface where you will be rolling it. Sprinkle some flour on it. Using your knuckles press the dough into a thinner disk. This way you will create a big enough disk for you to be able to roll it into a rolling pin. Latch one section of the dough onto the rolling pin until the entire dough is wrapped around the pin somewhat tightly. Begin expanding the dough by performing front and back repetitive motion with your palms against the rolling pin for a few seconds. As the dough gets bigger, it will feel loose around the rolling pin. Unroll the dough and grab the next section to roll around the rolling pin. Repeat the same process until you have a large, thin disc. You can also use a smaller rolling pin to roll out the thicker sides of the dough in order to create some sort of uniformity.",
                    None,
                    Some("Once the dough has rested, take one dough out and place it on a flat surface where you will be rolling it. Sprinkle some flour on it. Using your knuckles press the dough into a thinner disk. This way you will create a big enough disk for you to be able to roll it into a rolling pin. Latch one section of the dough onto the rolling pin until the entire dough is wrapped around the pin somewhat tightly. Begin expanding the dough by performing front and back repetitive motion with your palms against the rolling pin for a few seconds. As the dough gets bigger, it will feel loose around the rolling pin. Unroll the dough and grab the next section to roll around the rolling pin. Repeat the same process until you have a large, thin disc. You can also use a smaller rolling pin to roll out the thicker sides of the dough in order to create some sort of uniformity."),
                    Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-3"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "You may also stretch the dough out a bit with your hands if you wish to have thinner and much crispier layers. Totally your call. Once the desired size of the dough is reached, you may stop rolling the dough and proceed to the next step.",
                    None,
                    Some("You may also stretch the dough out a bit with your hands if you wish to have thinner and much crispier layers. Totally your call. Once the desired size of the dough is reached, you may stop rolling the dough and proceed to the next step."),
                    Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-4"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "With a tablespoon randomly pour half of the melted, clarified butter all over the dough. Using your hand spread out the butter evenly all over the dough. Let the clarified butter rest for 2 minutes before you roll it onto the pin. When the time is up, wrap the dough around the rolling tightly. Yes, the pin will get a little butter on it. No problem there.",
                    None,
                    Some("With a tablespoon randomly pour half of the melted, clarified butter all over the dough. Using your hand spread out the butter evenly all over the dough. Let the clarified butter rest for 2 minutes before you roll it onto the pin. When the time is up, wrap the dough around the rolling tightly. Yes, the pin will get a little butter on it. No problem there."),
                    Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-5"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Using a sharp knife slice through the middle of the rolling pin. Now you have a big layered strip of dough. Remove the rolling pin and again cut through the middle of the dough creating two thinner strips of stacks of dough.",
                    None,
                    Some("Using a sharp knife slice through the middle of the rolling pin. Now you have a big layered strip of dough. Remove the rolling pin and again cut through the middle of the dough creating two thinner strips of stacks of dough."),
                        Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-6"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Take one strip of stacked dough and gently place it over the other strip. Starting from the long edge of the strips start rolling them towards the other end, creating one chubby roll.",
                    None,
                    Some("Take one strip of stacked dough and gently place it over the other strip. Starting from the long edge of the strips start rolling them towards the other end, creating one chubby roll."),
                    Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-7"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Tuck the edge of the dough under itself, place the dough on the work surface and gently press on it to create yet another disk.",
                    None,
                    Some("Tuck the edge of the dough under itself, place the dough on the work surface and gently press on it to create yet another disk."),
                    Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-8"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Use a small rolling pin to roll out a large pancake measuring about 1/2 an inch thick. Don’t make it too thick and don’t make it too thin either.",
                    None,
                    Some("Use a small rolling pin to roll out a large pancake measuring about 1/2 an inch thick. Don’t make it too thick and don’t make it too thin either."),
                    Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-9"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Using either a chakich (indentation maker in the Uzbek language) or a fork, stab the pancakes creating many indentations. If you would like to use an egg wash to give a nice shin to your bread, do so now. Simply mix 1 egg with 1 tbsp of water and whisk well. Spread the egg wash on the qatlama and decorate it with sesame seeds.",
                    None,
                    Some("Using either a chakich (indentation maker in the Uzbek language) or a fork, stab the pancakes creating many indentations. If you would like to use an egg wash to give a nice shin to your bread, do so now. Simply mix 1 egg with 1 tbsp of water and whisk well. Spread the egg wash on the qatlama and decorate it with sesame seeds."),
                    Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-10"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Repeat the process for the second dough because one qatlama patir is just not enough!",
                    None,
                    Some("Repeat the process for the second dough because one qatlama patir is just not enough!"),
                    Some("https://arbuz.com/recipes/layered-bread/#wprm-recipe-20973-step-0-11"),
                ),
            ],
            recipe_yield: vec![
                RecipeRecipeYieldFieldEnum::Text("2".into()),
                RecipeRecipeYieldFieldEnum::Text("2 large breads".into()),
            ],
            prep_time: vec![DurationOrText::Text("PT35M".into())],
            total_time: vec![DurationOrText::Text("PT75M".into())],
            url: vec!["https://arbuz.com/recipes/layered-bread/".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_argirobarbarigou_ok() -> Result<()> {
        let got = scrape(Website::ArgiroBarbarigou, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            cook_time: vec![DurationOrText::Text("PT15M".into())],
            prep_time: vec![DurationOrText::Text("PT5M".into())],
            description: vec![RecipeDescriptionFieldEnum::Text("Shrimp and saffron pilaf, one of the easiest Greek meals for fasting, an incredible dish worth trying | Argiro Barbarigou English Recipes".into())],
            image: vec![RecipeImageFieldEnum::URL("https://argirobarbarigou.com/wp-content/uploads/2018/05/Shrimp-and-saffron-pilaf.jpg".into())],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL("Dairy Free".into()),
                RecipeKeywordsFieldEnum::TextOrURL("Gluten Free".into()),
                RecipeKeywordsFieldEnum::TextOrURL("Pescetarian".into()),
            ],
            name: vec!["Shrimp and saffron pilaf".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("¼ cup extra-virgin olive oil".into()),
                RecipeRecipeIngredientFieldEnum::Text("12 shelled prawns".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 clove of garlic".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 onion, diced".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 red bell pepper, diced".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 yellow red pepper, diced".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 cups long grain pilaf rice".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 tomatoes, chopped".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 tbsp. fresh parsley, chopped".into()),
                RecipeRecipeIngredientFieldEnum::Text("4 cups water".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 pinches of saffron powder or 3 threads".into()),
                RecipeRecipeIngredientFieldEnum::Text("Salt, freshly ground pepper".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text("Heat the olive oil in a deep skillet and sauté the onion, garlic and peppers for 3-4 minutes.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Careful they do not begin to turn brown.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Add the rice and stir until shiny.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Finish with the tomatoes and water.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Season to taste, lower heat, partially cover and simmer for 10 -12 minutes on a low heat, until the rice begins to soften.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Stir in the shrimp, parsley, and saffron, and simmer for a further 5 minutes.".into()),
            ],
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("4".into())],
            url: vec![
                "https://argirobarbarigou.com/recipes/shrimp-and-saffron-pilaf/<number>0".into(),
            ],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_asicilianpeasantstable_ok() -> Result<()> {
        let got = scrape(Website::ASicilianPeasantsTable, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            date_created: vec!["March 24, 2022".into()],
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "Although this recipe is not traditional Italian cuisine; this family favorite is a quick pasta dish that I created and wanted to share with you.\u{a0} It has a deep, earthy mushroom flavor that I like, and it is easy to prepare. \u{a0}Best of all it can be prepared in 30 minutes or less.\u{a0} Serve as a main course or as a side dish with grilled and roasted meats.\u{a0}".into(),
                ),
            ],
            image: vec![RecipeImageFieldEnum::URL("https://asicilianpeasantstable.com/wp-content/uploads/2022/03/Orzo-scaled-1-1920x1302.jpg".into())],
            keywords: vec![RecipeKeywordsFieldEnum::TextOrURL("Pasta".into())],
            name: vec!["Orzo with Mushrooms".into()],
            recipe_category: vec!["Side Dishes".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("1/2-ounce dried Porcini mushrooms".into()),
                RecipeRecipeIngredientFieldEnum::Text("8 ounces fresh Cremini (Baby Bella) mushrooms, cleaned and quartered".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/2 cup finely diced onion".into()),
                RecipeRecipeIngredientFieldEnum::Text("3 tablespoons extra-virgin olive oil, divided".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 tablespoons butter".into()),
                RecipeRecipeIngredientFieldEnum::Text("16 ounces dried orzo pasta".into()),
                RecipeRecipeIngredientFieldEnum::Text("3 to 3 ½ cups beef or chicken stock".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/3 cup chopped flat-leaf parsley".into()),
                RecipeRecipeIngredientFieldEnum::Text("Salt".into()),
                RecipeRecipeIngredientFieldEnum::Text("Pepper".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Place the dried porcini in a small bowl and cover with 1 cup boiling water.\u{a0} Let sit 1 hour.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Strain the porcinis reserving the soaking the liquid.\u{a0} Coarsely chop the porcinis and set aside.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Melt the butter with 2 tablespoons of the olive oil in a large deep skillet over medium heat.\u{a0} Add the fresh mushrooms and onion until vegetables are soft, about 4 minutes.\u{a0} Transfer to a bowl.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Add the remaining oil into the same skillet.\u{a0} Add the orzo, stirring continuously until the pasta is lightly toasted.".into(),
                ),
                RecipeRecipeInstructionsFieldEnum::Text(
                    "Stir in 2 cups of the stock and the porcini soaking liquid.\u{a0} Add the porcinis, mushroom mixture, and half of the parsley.\u{a0} Bring to a boil, reduce heat to low and cover with a lid.\u{a0} Season with salt and pepper to taste.\u{a0} Add additional stock as needed throughout cooking to keep the pasta from drying out.\u{a0} Cook for approximately 15 minutes or to al dente depending on the pasta size which can vary. \u{a0}Stir in the remaining parsley.\u{a0} Serve with grated parmesan cheese if desired.".into(),
                ),
            ],
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("8".into())],
            url: vec!["https://asicilianpeasantstable.com/2022/03/24/orzo-with-mushrooms/<number>0".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_atreatsaffair_ok() -> Result<()> {
        let got = scrape(Website::ATreatsAffair, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            nutrition: vec![
                NutritionInformation {
                    calories: vec![Energy::new("4785")],
                    fat_content: vec![Mass::new("104")],
                    r#type: AtType::NutritionInformation.to_opt(),
                    ..Default::default()
                },
            ],
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("12 cupcakes".into())],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("1/2 cup butter, room temperature".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 1/2 cup sugar".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 eggs, room temperature".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 teaspoon vanilla extract".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/2 cup pumpkin puree (I used store bought) ".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/4 cup International Delight Pumpkin Pie Spice Coffee Creamer".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 3/4 cup all purpose flour".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 teaspoon baking powder".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 teaspoon pumpkin pie spice ".into()),
                RecipeRecipeIngredientFieldEnum::Text("pinch of salt".into()),
                RecipeRecipeIngredientFieldEnum::Text("!Caramel frosting".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/2 cup butter".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 cup dark brown sugar".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/4 cup milk (whole or 2%) ".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 teaspoon vanilla extract".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 1/2 cups powdered sugar".into()),
            ],
            cook_time: vec![
                DurationOrText::Text("PT25M".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text("Preheat the oven to 350 degrees. Line a cupcake pan with 12 cupcake liners. Set aside".into()),
                RecipeRecipeInstructionsFieldEnum::Text("In a bowl mix the butter and sugar. Add the eggs, pumpkin spice, baking powder, and vanilla and mix until incorporated. Mix in the flour.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Add the pumpkin puree and the International Delight® Pumpkin Pie Spice Coffee Creamer and mix until there are no lumps left. Set aside.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Pour the pumpkin batter into the cupcake liners, Add 1 tbsp of cream cheese mix on top and with the help of a toothpick swirl it.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Take to the oven and bake for 25 minutes or until done.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Take out of the oven and let them cool until they are no longer hot to the touch.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("!Caramel frosting".into()),
                RecipeRecipeInstructionsFieldEnum::Text("In a medium size saucepan, melt 1 cup butter over low flame. ".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Add the brown sugar and continue to cook until it reaches boiling point, stirring constantly. ".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Stir in the milk and continue to cook until it boils again. ".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Remove from heat. Add the vanilla extract. ".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Let the mixture cool until lukewarm, about 30-40 minutes.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Once cooled, gradually stir in powdered sugar. ".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Frost the cupcakes. ".into()),
            ],
            prep_time: vec![
                DurationOrText::Text("PT30M".into()),
            ],
            aggregate_rating: vec![
                AggregateRating {
                    r#type: AtType::AggregateRating.to_opt(),
                    rating_count: vec![0],
                    ..Default::default()
                },
            ],
            date_published: vec![
                "2017-10-06 01:49:03".into(),
            ],
            author: vec![RecipeAuthorFieldEnum::new_org("Roxana Yawgel")],
            image: vec![
                RecipeImageFieldEnum::ImageObject(
                    ImageObject {
                        r#type: AtType::ImageObject.to_opt(),
                        url: vec![
                            "http://atreatsaffair.com/wp-content/uploads/2017/10/pumpkin-cupcakes-with-caramel-frosting-recipe-2.jpg".into(),
                        ],
                        ..Default::default()
                    }.into(),
                ),
            ],
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "Perfect for all pumpkin lovers, these moist and sweet pumpkin cupcakes topped with an easy to make caramel frosting are a fall favorite dessert!".into(),
                ),
            ],
            name: vec![
                "Pumpkin cupcakes with caramel frosting recipe".into(),
            ],
            url: vec![
                "https://atreatsaffair.com/pumpkin-cupcakes-with-caramel-frosting-recipe/".into(),
            ],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }
}
