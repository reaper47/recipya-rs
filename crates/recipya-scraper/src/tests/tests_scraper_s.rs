#[cfg(test)]
mod tests {
    use schema_org::{
        AggregateRating, AtType, Clip, DurationOrText, Energy, ItemList, ListItem, Mass,
        NutritionInformation, Recipe, at_context,
        field::{
            AggregateRatingRatingValueFieldEnum, ClipDescriptionFieldEnum,
            ItemListItemListElementFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
            RecipeInLanguageFieldEnum, RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum,
            RecipeRecipeInstructionsFieldEnum, RecipeRecipeYieldFieldEnum, RecipeVideoFieldEnum,
        },
    };

    use crate::{tests::support::scraper::scrape, websites::Website};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_samsung_food_ok() -> Result<()> {
        let got = scrape(Website::SamsungFood, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            nutrition: vec![
                NutritionInformation {
                    calories: vec![Energy::new("981.77 calories")],
                    carbohydrate_content: vec![Mass::new("53.01 g")],
                    fat_content: vec![Mass::new("61.39 g")],
                    fiber_content: vec![Mass::new("6.12 g")],
                    protein_content: vec![Mass::new("57.44 g")],
                    saturated_fat_content: vec![Mass::new("9.05 g")],
                    serving_size: vec!["4".into()],
                    sugar_content: vec![Mass::new("24.3 g")],
                    r#type: AtType::NutritionInformation.to_opt(),
                    ..Default::default()
                },
            ],
            recipe_yield: vec![RecipeRecipeYieldFieldEnum::Number(4.0)],
            recipe_cuisine: vec!["Asian".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("450g firm tofu. Broken up into bite size".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 tbsp oil".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 tbsp tamari (or soy sauce)".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 tbsp cornflour (aka cornstarch)".into()),
                RecipeRecipeIngredientFieldEnum::Text("1/4 cup chicken style stock, hot".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 tbsp brown sugar".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 tbsp tamari (or soy sauce)".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 tbsp oil".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 tsp ginger powder (or fresh ginger)".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 tbsp chilli flakes".into()),
                RecipeRecipeIngredientFieldEnum::Text("3 garlic cloves, minced".into()),
                RecipeRecipeIngredientFieldEnum::Text("1 tbsp cornflour, mixed with 2 tbsp cold water".into()),
                RecipeRecipeIngredientFieldEnum::Text("2 spring onions, cut into 1 inch slices".into()),
            ],
            cook_time: vec![DurationOrText::Text("PT20M".into())],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::ItemList(
                    ItemList {
                        r#type: AtType::HowToSection.to_string(),
                        item_list_element: vec![
                            ItemListItemListElementFieldEnum::ListItem(
                                ListItem {
                                    r#type: AtType::HowToStep.to_opt(),
                                    text: vec!["Combine all the tofu ingredients in a container and shake/stir will to combine".into()],
                                    ..Default::default()
                                },
                            ),
                            ItemListItemListElementFieldEnum::ListItem(
                                ListItem {
                                    r#type: AtType::HowToStep.to_opt(),
                                    text: vec!["Dissolve the brown sugar in the hot chicken stock then add the tamari. Add extra tamari/soy sauce if you prefer it a little saltier, leave to the side".into()],
                                    ..Default::default()
                                },
                            ),
                            ItemListItemListElementFieldEnum::ListItem(
                                ListItem {
                                    r#type: AtType::HowToStep.to_opt(),
                                    text: vec!["Heat the oil in a large pan then add the tofu and fry until crispy and golden, then remove the tofu from the pan and add the garlic, ginger and chilli flakes and sauté for 1 - 2 minutes.\u{2028}\u{2028}Add the sauce you made earlier and let it simmer for about 1 - 2 minutes then stir in the cornstarch and water mixture.".into()],
                                    ..Default::default()
                                },
                            ),
                            ItemListItemListElementFieldEnum::ListItem(
                                ListItem {
                                    r#type: AtType::HowToStep.to_opt(),
                                    text: vec!["Once the sauce has thickened add the tofu and spring onion to the pan and toss until everything is combined. There shouldn’t be any liquid, only sauce, so keep the heat going until thickened.".into()],
                                    ..Default::default()
                                },
                            ),
                            ItemListItemListElementFieldEnum::ListItem(
                                ListItem {
                                    r#type: AtType::HowToStep.to_opt(),
                                    text: vec!["Serve with rice and enjoy!".into()],
                                    ..Default::default()
                                },
                            ),
                        ],
                        ..Default::default()
                    }.into(),
                ),
            ],
            recipe_category: vec!["Dinner".into()],
            prep_time: vec![DurationOrText::Text("PT5M".into())],
            total_time: vec![DurationOrText::Text("PT25M".into())],
            keywords: vec![
                RecipeKeywordsFieldEnum::TextOrURL("Mongolian Tofu (so easy and so much flavour), Dinner, Asian".into()),
            ],
            aggregate_rating: vec![
                AggregateRating {
                    r#type: AtType::AggregateRating.to_opt(),
                    rating_count: vec![203],
                    rating_value: vec![AggregateRatingRatingValueFieldEnum::Number(5.0)],
                    ..Default::default()
                },
            ],
            in_language: vec![RecipeInLanguageFieldEnum::Text("English".into())],
            video: vec![
                RecipeVideoFieldEnum::Clip(
                    Clip {
                        r#type: AtType::VideoObject.to_opt(),
                        thumbnail_url: vec![
                            "https://p16-common-sign.tiktokcdn-eu.com/tos-alisg-p-0037/7d823d1cb0d746c08940e57844808e09_1696327654~tplv-tiktokx-origin.image?dr=10395&x-expires=1776009600&x-signature=k4IiX2QhoEh9HOHARTkpturrK9g%3D&t=4d5b0474&ps=13740610&shp=81f88b70&shcp=43f4a2f9&idc=no1a".into(),
                        ],
                        description: vec![
                            ClipDescriptionFieldEnum::Text(
                                "Mongolian Tofu (so easy and so much flavour) - Recipe Video from TikTok on Samsung Food".into(),
                            ),
                        ],
                        name: vec!["Mongolian Tofu (so easy and so much flavour)".into()],
                        ..Default::default()
                    }.into(),
                ),
            ],
            image: vec![
                RecipeImageFieldEnum::URL(
                    "https://art.whisk.com/image/upload/fl_progressive,h_600,w_800,c_fill/v1696487693/v3/user-recipes/bb618006fa844e9f3304000aaf33dba3.jpg".into(),
                ),
            ],
            description: vec![
                RecipeDescriptionFieldEnum::Text(
                    "Tofu Recipes Series Episode 6: Mongolian Tofu Recipe\u{2028}\u{2028}The best thing I ever did with tofu was take traditional tasty recipes such as Mongolian beef, and sub the meat with tofu. Tofu is such a flavour absorber and the texture is always the perfect replacement for meat! \n\nI was soo happy with this recipe because my sister, who is NOT a fan of tofu at all, tried it and her face literally lit up in shock at how much she loved it. \n\nThis recipe is roughly based off The Woks of Life website \n\nSave this easy tofu recipe and sent it to someone who loves tofu!".into(),
                ),
            ],
            url: vec!["https://app.samsungfood.com/recipes/107018afe8e54d07e9689b87dd6f857b15f".into()],
            name: vec!["Mongolian Tofu (so easy and so much flavour)".into()],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }
}
