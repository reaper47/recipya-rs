#[cfg(test)]
mod tests {
    use schema_org::{
        AggregateRating, AtType, Clip, DurationOrText, Energy, ImageObject, Mass,
        NutritionInformation, Organization, Rating, Recipe, Review, at_context,
        field::{
            AggregateRatingRatingValueFieldEnum, ClipDescriptionFieldEnum,
            ImageObjectHeightFieldEnum, ImageObjectWidthFieldEnum, OrganizationLogoFieldEnum,
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
    async fn test_allrecipes_dot_com() -> Result<()> {
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
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Preheat the oven to 350 degrees F (175 degrees C). Beat butter, white sugar, and brown sugar in a large bowl with an electric mixer until smooth and creamy.",
                    Some("https://www.allrecipes.com/thmb/HZVO7UCA4f6YjwRUxMm2bo5Wz2U=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-2-1191-1b57f642a52849ce84a25c48363c4013.jpg"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Beat in eggs, one at a time, then stir in vanilla.",
                    Some("https://www.allrecipes.com/thmb/vb_CS6Q-CEf0cZySd5HgkUV-T_Y=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-3-121-f73dc5abc3454dfab534d8efc0e966e6.jpg"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Dissolve baking soda in hot water; add to batter along with salt and mix until combined.",
                    Some("https://www.allrecipes.com/thmb/EHPMpRCIHiCbzy9a3hGDZn0Tgyc=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-4-123-abd5290df87f47668cbc8eef6587c292.jpg"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Stir in flour, chocolate chips, and walnuts until a soft dough forms.",
                    Some("https://www.allrecipes.com/thmb/4cwC_GLPXQVWlqI2KH0GUbC1-V4=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-5-125-b755dee648e949129ad022d8b019b405.jpg"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Drop rounded spoonfuls of cookie dough 2 inches apart onto ungreased baking sheets.",
                    Some("https://www.allrecipes.com/thmb/A63M4EgkpKCDtqT1qWgMciQ3-hI=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-6-128-44e1d4355a6a4441a3ba993e2a2b74fa.jpg"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Bake in the preheated oven until edges are lightly browned, about 10 minutes.",
                    Some("https://www.allrecipes.com/thmb/NFy1pucQzBWKkvRGRvcQgFcmu9E=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-7-134-e0952f5171a2434dbb2b3bb53b18648a.jpg"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Cool on the baking sheets briefly before removing to a wire rack to cool completely.",
                    Some("https://www.allrecipes.com/thmb/5tMxJUaTaqQNYD3O-56aG4L9t00=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-step-8-138-d4f25b55db5c417b9d6ece8cf98700f3.jpg"),
                ),
                RecipeRecipeInstructionsFieldEnum::new_creative_work(
                    "Store in an airtight container or serve immediately and enjoy!",
                    Some("https://www.allrecipes.com/thmb/8xwaWAHtl_QLij6D-G0Z4B1HDVA=/1500x0/filters:no_upscale():max_bytes(150000):strip_icc()/10813-best-chocolate-chip-cookies-mfs-146-4x3-b108aceffa6043a1ac81c3c5a9b034c8.jpg"),
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
}
