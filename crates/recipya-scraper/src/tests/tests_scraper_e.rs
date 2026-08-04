#[cfg(test)]
mod tests {
    use schema_org::{
        AtType, Recipe, at_context,
        field::{
            RecipeDescriptionFieldEnum, RecipeImageFieldEnum, RecipeRecipeIngredientFieldEnum,
            RecipeRecipeInstructionsFieldEnum,
        },
    };

    use crate::{tests::support::scraper::scrape, websites::Website};

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore = "needs manual testing"]
    #[allow(clippy::too_many_lines)]
    async fn test_eatingbirdfood() -> Result<()> {
        let got = scrape(Website::EliDuke, 0).await?;

        let want = Recipe {
            context: at_context(),
            r#type: AtType::Recipe.to_opt(),
            description: vec![
                RecipeDescriptionFieldEnum::Text("This is by far my most favorite salad in the whole world. It's super crunchy, hearty, filling, and so ridiculously good for you. It's off the chain! You might could call it a non-dairy slaw, and one of my favorite parts is that you get to eat it with a spoon! Fuck Forks!".into()),
                RecipeDescriptionFieldEnum::Text("The ingredient list is a little vague on purpose because this salad is so flexible and it can go in so many different directions. Feel free to add things, swap things, and mix that shit up with reckless abandon. Wanna try some shredded beat? Go for it! How about some chopped up celery? Why not? Goj Berrs? Pistash? Cocoa Nibbies? I want all the goodies! And it's hard to say exactly how much of each thing to chop up. Just feel it out.".into())],
            image: vec![
                RecipeImageFieldEnum::URL("https://eliduke.com/images/recipes/kale-salad-1.jpg".into()),
                RecipeImageFieldEnum::URL("https://eliduke.com/images/recipes/kale-salad-2.jpg".into()),
                RecipeImageFieldEnum::URL("https://eliduke.com/images/recipes/kale-salad-3.jpg".into()),
                RecipeImageFieldEnum::URL("https://eliduke.com/images/recipes/kale-salad-4.jpg".into()),
            ],
            name: vec!["Kale Salad".into()],
            recipe_ingredient: vec![
                RecipeRecipeIngredientFieldEnum::Text("Kale".into()),
                RecipeRecipeIngredientFieldEnum::Text("Cabbage".into()),
                RecipeRecipeIngredientFieldEnum::Text("Carrots".into()),
                RecipeRecipeIngredientFieldEnum::Text("Apple".into()),
                RecipeRecipeIngredientFieldEnum::Text("Nuts".into()),
                RecipeRecipeIngredientFieldEnum::Text("Dressing".into()),
            ],
            recipe_instructions: vec![
                RecipeRecipeInstructionsFieldEnum::Text("Put the nuts (walnuts, sunflower, sesame, etc) in a small frying pan on the stove on medium high heat.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Chop up the kale, cabbage, apple, and carrots as fine as you can. The smaller the better.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("At some point during all that chopping you'll start to smell the nuts. Give them a little shake in the pan and let them go for a bit longer. You're hoping for a light browning.".into()),
                RecipeRecipeInstructionsFieldEnum::Text("Throw each thing into a big bowl as you get them chopped up, and then add the dressing. My preference is Annie's Goddess Dressing with a little Hot Pepper Sesame Oil for flare, but just follow your heart. Mix the dressing in and then sprinkle the roasted nuts on top.".into()),
            ],
            url: vec![
                "https://eliduke.com/recipes/kale-salad/<number>0".into(),
            ],
            ..Default::default()
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }
}
