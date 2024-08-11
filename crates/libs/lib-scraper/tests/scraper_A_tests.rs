use iso8601::{
    Date::YMD,
    Duration::YMDHMS,
    {DateTime, Time},
};
use url::Url;

use crate::helpers::scrape;

use lib_scraper::{
    schema::{
        common::{
            AggregateRating, ClipOrVideoObject, CreativeWorkOrItemListOrText::ItemList,
            CreativeWorkOrUrl, CreativeWorkType, DateOrDateTime, DefinedTermOrTextOrUrl, HowTo,
            ImageObjectOrUrl, ImageObjectType, OrganizationOrPerson, OrganizationType,
            QuantitativeValueOrText, QuantitativeValueType, ReviewRating, ReviewType,
            TextOrTextObject, VideoObjectType,
        },
        nutrition::{Energy, Mass, NutritionInformationSchema, RestrictedDiet},
        recipe::{RecipeCategory, RecipeCuisine, RecipeSchema},
        AtContext::SchemaDotOrg,
        AtType,
    },
    websites::Website,
};

mod helpers;

#[test]
fn test_abuelascounter_dot_com() {
    let got = scrape(Website::AbuelasCounterCom);

    let want = RecipeSchema {
        at_context: SchemaDotOrg,
        at_type: Some(AtType::Recipe),
        author: Some(OrganizationType { at_type: Some(AtType::Person), name: Some("Abuelas Cuban Counter".to_string()), ..Default::default() }),
        cook_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 35, second: 0, millisecond: 0 }),
        date_published: Some(DateOrDateTime::DateTime(DateTime {
            date: YMD {
                year: 2023,
                month: 10,
                day: 24,
            },
            time: Time {
                hour: 19,
                minute: 45,
                second: 56,
                millisecond: 0,
                tz_offset_hours: 0,
                tz_offset_minutes: 0,
            },
        })),
        description: Some(TextOrTextObject::Text("".to_string())),
        image: Some(ImageObjectOrUrl::Url(Url::parse("https://abuelascounter.com/wp-content/uploads/2023/10/Roasted-Carrot-Soup-Recipe.jpeg").unwrap())),
        keywords: Some(DefinedTermOrTextOrUrl::Text(vec![
            "abuelau0026#039;s".to_string(),
            "cuban".to_string(),
            "easy recipes".to_string(),
            "healthy recipes".to_string(),
            "hosting".to_string(),
            "roasted carrot soup".to_string(),
            "soups".to_string(),
            "thanksgiving recipes".to_string(),
            "traditional".to_string(),
        ].join(","))),
        name: Some("Roasted Carrot Soup".to_string()),
        nutrition: Some(NutritionInformationSchema { at_type: Some(AtType::NutritionInformation), ..Default::default() }),
        prep_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 10, second: 0, millisecond: 0 }),
        recipe_category: RecipeCategory::Text("Soups".to_string()),
        recipe_cuisine: Some(RecipeCuisine::Text("American".to_string())),
        recipe_ingredient: Some(vec![
            "3 cups of carrots, that have been peeled and diced".to_string(),
            "1 ½ cups of butternut squash, that has been peeled and diced".to_string(),
            "1 apple, peeled and diced (we like to use gala or granny smith apples)".to_string(),
            "3 shallots, cut in quarters".to_string(),
            "6 sprigs of thyme".to_string(),
            "4 tablespoons of olive oil or avocado oil".to_string(),
            "Freshly grated nutmeg".to_string(),
            "3 ½ to 4 ½ cups of chicken or vegetable stock".to_string(),
            "Salt and freshly cracked pepper".to_string(),
            "Garnish: chives, sour cream, Calabrian chili peppers".to_string(),
        ]),
        recipe_instructions: Some(ItemList(vec![
            HowTo {
                at_type: AtType::HowToStep,
                name: Some("Preheat oven to 425 degrees.".to_string()),
                text: "Preheat oven to 425 degrees.".to_string(),
                url: Some(Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-6536bc41231ae").unwrap()),
                image: Some(ImageObjectOrUrl::ImageObject(ImageObjectType::default())),
            },
            HowTo {
                at_type: AtType::HowToStep,
                name: Some("Line a sheet pan with parchment paper or a silpat cover. Add all the vegetables, along with the apple, thyme and shallot. Toss in oil and sprinkle with 1 teaspoon of salt and freshly cracked pepper.".to_string()),
                text: "Line a sheet pan with parchment paper or a silpat cover. Add all the vegetables, along with the apple, thyme and shallot. Toss in oil and sprinkle with 1 teaspoon of salt and freshly cracked pepper.".to_string(),
                url: Some(Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-6536bc41231af").unwrap()),
                image: Some(ImageObjectOrUrl::ImageObject(ImageObjectType::default())),
            },
            HowTo {
                at_type: AtType::HowToStep,
                name: Some("Roast everything for 15 minutes. Then add 2 cups of stock. Roast for another 15 minutes or until all the vegetables have completely cooked through and are tender.".to_string()),
                text: "Roast everything for 15 minutes. Then add 2 cups of stock. Roast for another 15 minutes or until all the vegetables have completely cooked through and are tender.".to_string(),
                url: Some(Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-6536bc41231b0").unwrap()),
                image: Some(ImageObjectOrUrl::ImageObject(ImageObjectType::default())),
            },
            HowTo {
                at_type: AtType::HowToStep,
                name: Some("Add everything to the blender (including the liquid) but make sure you leave out the thyme. Before adding the thyme to the blender, strip the thyme sprigs of their leaves. Discard the stems and only add the leaves to the blender.".to_string()),
                text: "Add everything to the blender (including the liquid) but make sure you leave out the thyme. Before adding the thyme to the blender, strip the thyme sprigs of their leaves. Discard the stems and only add the leaves to the blender.".to_string(),
                url: Some(Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-6536bc41231b1").unwrap()),
                image: Some(ImageObjectOrUrl::ImageObject(ImageObjectType::default())),
            },
            HowTo {
                at_type: AtType::HowToStep,
                name: Some("Add another 2 cups of stock. Add a small pinch of freshly grated nutmeg.".to_string()),
                text: "Add another 2 cups of stock. Add a small pinch of freshly grated nutmeg.".to_string(),
                url: Some(Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-16981142881391014").unwrap()),
                image: Some(ImageObjectOrUrl::ImageObject(ImageObjectType::default())),
            },
            HowTo {
                at_type: AtType::HowToStep,
                name: Some("Blend until completely smooth. Use a rubber spatula to move any chunks or pieces from the sides of the blender. If you want it to be a little thinner add another ½-1 cup of liquid or as much as you need to get it to your preferred consistency.".to_string()),
                text: "Blend until completely smooth. Use a rubber spatula to move any chunks or pieces from the sides of the blender. If you want it to be a little thinner add another ½-1 cup of liquid or as much as you need to get it to your preferred consistency.".to_string(),
                url: Some(Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-16981142889011019").unwrap()),
                image: Some(ImageObjectOrUrl::ImageObject(ImageObjectType::default())),
            },
            HowTo {
                at_type: AtType::HowToStep,
                name: Some("Add to a pot keep warm on low heat. Serve and garnish.".to_string()),
                text: "Add to a pot keep warm on low heat. Serve and garnish.".to_string(),
                url: Some(Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-16981143386011052").unwrap()),
                image: Some(ImageObjectOrUrl::ImageObject(ImageObjectType::default())),
            }
        ])),
        recipe_yield: QuantitativeValueOrText::QuantitativeValue(QuantitativeValueType { value: 6 }),
        total_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 45, second: 0, millisecond: 0 }),
        video: Some(ClipOrVideoObject::VideoObject(VideoObjectType {
            at_type: AtType::VideoObject,
            name: "Roasted Carrot Soup".to_string(),
            description: "Roasted Carrot Soup".to_string(),
            thumbnail_url: vec![
                Url::parse("https://abuelascounter.com/wp-content/uploads/2023/10/Roasted-Carrot-Soup-Recipe.jpeg").unwrap(),
                Url::parse("https://abuelascounter.com/wp-content/uploads/2023/10/Roasted-Carrot-Soup-Recipe-500x500.jpeg").unwrap(),
                Url::parse("https://abuelascounter.com/wp-content/uploads/2023/10/Roasted-Carrot-Soup-Recipe-500x375.jpeg").unwrap(),
                Url::parse("https://abuelascounter.com/wp-content/uploads/2023/10/Roasted-Carrot-Soup-Recipe-480x270.jpeg").unwrap(),
            ],
            content_url: Url::parse("https://youtu.be/g63Nto5ld-k").unwrap(),
            embed_url: Url::parse("https://youtu.be/g63Nto5ld-k").unwrap(),
            duration: None,
            upload_date: Some(DateTime { date: YMD { year: 2023, month: 10, day: 24 }, time: Time { hour: 19, minute: 45, second: 56, millisecond: 0, tz_offset_hours: 0, tz_offset_minutes: 0 } }),
        })),
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
}

#[test]
fn test_acouple_cooks_dot_com() {
    let got = scrape(Website::ACoupleCooksCom);

    let want = RecipeSchema {
        at_context: SchemaDotOrg,
        at_id: Some("https://www.acouplecooks.com/shaved-brussels-sprouts/#recipe".to_string()),
        at_type: Some(AtType::Recipe),
        aggregate_rating: Some(
            AggregateRating {
                at_type: AtType::AggregateRating,
                rating_value: Some(5),
                review_count: Some("3".to_string()),
                ..Default::default()
            },
        ),
        author: Some(OrganizationType {
            at_type: Some(AtType::Person),
            name: Some("Sonja Overhiser".to_string()),
            url: Some(Url::parse("https://www.acouplecooks.com/about/").unwrap()),
            ..Default::default()
        }),
        cooking_method: Some("Shredded".to_string()),
        cook_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 7, second: 0, millisecond: 0 }),
        date_published: Some(DateOrDateTime::Date(YMD { year: 2022, month: 3, day: 23 })),
        description: Some(TextOrTextObject::Text("This shaved Brussels sprouts recipe make a tasty side dish that's full of texture and flavor! Shredded Brussels are quick and crowd-pleasing.".to_string())),
        image: Some(ImageObjectOrUrl::Url(Url::parse("https://www.acouplecooks.com/wp-content/uploads/2022/03/Shredded-Brussels-Sprouts-001.jpg").unwrap())),
        is_part_of: Some(CreativeWorkOrUrl::CreativeWork(CreativeWorkType {
            at_id: Some(Url::parse("https://www.acouplecooks.com/shaved-brussels-sprouts/#article").unwrap()),
        })),
        keywords: Some(DefinedTermOrTextOrUrl::Text(vec![
            "Shaved Brussels sprouts".to_string(),
            "Shaved Brussels sprouts recipe".to_string(),
            "shredded Brussel sprouts".to_string(),
            "shredded Brussels sprouts".to_string(),
        ].join(", "))),
        main_entity_of_page: Some(CreativeWorkOrUrl::Url(Url::parse("https://www.acouplecooks.com/shaved-brussels-sprouts/").unwrap())),
        name: Some("Easy Shaved Brussels Sprouts".to_string()),
        nutrition: Some(NutritionInformationSchema {
            at_type: Some(AtType::Unspecified),
            calories: Some(Energy::Str("149 calories".to_string())),
            carbohydrate_content: Some(Mass::Str("14.6 g".to_string())),
            cholesterol_content: Some(Mass::Str("3.6 mg".to_string())),
            fat_content: Some(Mass::Str("9.2 g".to_string())),
            fiber_content: Some(Mass::Str("6.5 g".to_string())),
            protein_content: Some(Mass::Str("6.5 g".to_string())),
            saturated_fat_content: Some(Mass::Str("2.1 g".to_string())),
            sodium_content: Some(Mass::Str("271.1 mg".to_string())),
            sugar_content: Some(Mass::Str("3 g".to_string())),
            trans_fat_content: Some(Mass::Str("0 g".to_string())),
            ..Default::default()
        }),
        prep_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 10, second: 0, millisecond: 0 }),
        recipe_category: RecipeCategory::Text("Side dish".to_string()),
        recipe_cuisine: Some(RecipeCuisine::Text("Vegetables".to_string())),
        recipe_ingredient: Option::from(vec![
            "1 pound Brussels sprouts (off the stalk)".to_string(),
            "2 cloves garlic, minced".to_string(),
            "1 small shallot, minced".to_string(),
            "1/4 cup shredded Parmesan cheese (omit for vegan)".to_string(),
            "½ teaspoon kosher salt, plus more to taste".to_string(),
            "2 tablespoons olive oil".to_string(),
            "1/4 cup Italian panko (optional, omit for gluten-free or use GF panko)".to_string(),
        ]),
        recipe_instructions: Some(ItemList(vec![
            HowTo {
                at_type: AtType::HowToStep,
                text: "Shave the Brussels sprouts:\n\nWith a knife: Remove any tough outer layers with your fingers. With a large Chef’s knife, cut the Brussels sprout in half lengthwise. Place the cut side down and thinly slice cross-wise to create shreds. Separate the shreds with your fingers. Discard the root end.".to_string(),
                url: Some(Url::parse("https://www.acouplecooks.com/shaved-brussels-sprouts/#instruction-step-1").unwrap()),
                ..Default::default()
            },
            HowTo {
                at_type: AtType::HowToStep,
                text: "With a food processor (fastest!): Use a food processor with the shredding disc attachment blade. (Here&amp;#8217;s a video.)".to_string(),
                url: Some(Url::parse("https://www.acouplecooks.com/shaved-brussels-sprouts/#instruction-step-2").unwrap()),
                ..Default::default()
            },
            HowTo {
                at_type: AtType::HowToStep,
                text: "With a mandolin: Slice the whole Brussels sprouts with a mandolin, taking proper safety precautions to keep your fingers away from the blade. (Here&amp;#8217;s a video.)".to_string(),
                url: Some(Url::parse("https://www.acouplecooks.com/shaved-brussels-sprouts/#instruction-step-3").unwrap()),
                ..Default::default()
            },
            HowTo {
                at_type: AtType::HowToStep,
                text: "In a medium bowl, stir together the minced garlic, shallot, Parmesan cheese, and kosher salt.".to_string(),
                url: Some(Url::parse("https://www.acouplecooks.com/shaved-brussels-sprouts/#instruction-step-4").unwrap()),
                ..Default::default()
            },
            HowTo {
                at_type: AtType::HowToStep,
                text: "In a large skillet, heat the olive oil over medium high heat. Add the Brussels sprouts and cook for 4 minutes, stirring only occasionally, until tender and browned. Stir in the Parmesan mixture and cook additional 3 to 4 minutes until lightly browned and fragrant. Remove the heat and if desired, stir in the panko. Taste and add additional salt as necessary.".to_string(),
                url: Some(Url::parse("https://www.acouplecooks.com/shaved-brussels-sprouts/#instruction-step-5").unwrap()),
                ..Default::default()
            },
        ])),
        recipe_yield: QuantitativeValueOrText::Text("4".to_string()),
        review: Some([
            ReviewType {
                at_type: AtType::Review,
                review_rating: ReviewRating {
                    at_type: AtType::Rating,
                    rating_value: "5".to_string(),
                },
                author: OrganizationOrPerson::Organization(OrganizationType {
                    at_type: Some(AtType::Person),
                    name: Some("Larry Harmon".to_string()),
                    ..Default::default()
                }),
                date_published: "2022-04-02".to_string(),
                review_body: "Thanks Sonja!\r\nI just made this for lunch and it has converted me.\r\nI didn't think I liked brussel sprouts until now.\r\nMy daughter, who does love brussel sprouts, said it is now her favorite recipe. \r\n\r\nLarry".to_string(),
            },
            ReviewType {
                at_type: AtType::Review,
                review_rating: ReviewRating {
                    at_type: AtType::Rating,
                    rating_value: "5".to_string(),
                },
                author: OrganizationOrPerson::Organization(OrganizationType {
                    at_type: Some(AtType::Person),
                    name: Some("Alpana Hoffman".to_string()),
                    ..Default::default()
                }),
                date_published: "2022-11-25".to_string(),
                review_body: "I made this recipe on thanksgiving and our guests loved it.  Awesome!  Thank you very much".to_string(),
            },
            ReviewType {
                at_type: AtType::Review,
                review_rating: ReviewRating {
                    at_type: AtType::Rating,
                    rating_value: "5".to_string(),
                },
                author: OrganizationOrPerson::Organization(OrganizationType {
                    at_type: Some(AtType::Person),
                    name: Some("Gretchen g".to_string()),
                    ..Default::default()
                }),
                date_published: "2023-10-15".to_string(),
                review_body: "Fantastic! I cooked as is along with  beets and put in oven after for 15 minutes at 345 to get everything to cook a bit more. Was perfection!!".to_string(),
            },
        ].into()),
        suitable_for_diet: RestrictedDiet::VegetarianDiet,
        total_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 17, second: 0, millisecond: 0 }),
        url: Some(Url::parse("https://www.acouplecooks.com/shaved-brussels-sprouts/").unwrap()),
        ..Default::default()
    };
    pretty_assertions::assert_eq!(got, want);
}
