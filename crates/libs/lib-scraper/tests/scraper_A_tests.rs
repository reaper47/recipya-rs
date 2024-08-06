use iso8601::{
    {DateTime, Time},
    Date::YMD,
    Duration::YMDHMS,
};
use url::Url;

use lib_scraper::{
    {schema::RecipeSchema, websites::Website},
    schema::{
        AtContext::SchemaDotOrg, AtType, Container, HowTo, Instructions::ItemList,
        NutritionInformationSchema, Organization, VideoObject, Yield,
    },
};

use crate::helpers::scrape;

mod helpers;

#[test]
fn test_abuelascounter_dot_com() {
    let got = scrape(Website::AbuelasCounterCom);

    let want = RecipeSchema {
        at_context: Some(SchemaDotOrg),
        at_type: Some(Container::Item(AtType::Recipe)),
        author: Some(Organization{at_type: AtType::Person, name: "Abuelas Cuban Counter".to_string(), url: None }),
        cook_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 35, second: 0, millisecond: 0 }),
        date_published: Some(DateTime { date: YMD { year: 2023, month: 10, day: 24 }, time: Time { hour: 19, minute: 45, second: 56, millisecond: 0, tz_offset_hours: 0, tz_offset_minutes: 0 } }),
        description: Some("".to_string()),
        image: Some(Container::Vec(vec![
            Url::parse("https://abuelascounter.com/wp-content/uploads/2023/10/Roasted-Carrot-Soup-Recipe.jpeg").unwrap(),
            Url::parse("https://abuelascounter.com/wp-content/uploads/2023/10/Roasted-Carrot-Soup-Recipe-500x500.jpeg").unwrap(),
            Url::parse("https://abuelascounter.com/wp-content/uploads/2023/10/Roasted-Carrot-Soup-Recipe-500x375.jpeg").unwrap(),
            Url::parse(        "https://abuelascounter.com/wp-content/uploads/2023/10/Roasted-Carrot-Soup-Recipe-480x270.jpeg").unwrap(),

        ])),
        keywords: Some(Container::Vec(vec![
            "abuelau0026#039;s".to_string(),
            "cuban".to_string(),
            "easy recipes".to_string(),
            "healthy recipes".to_string(),
            "hosting".to_string(),
            "roasted carrot soup".to_string(),
            "soups".to_string(),
            "thanksgiving recipes".to_string(),
            "traditional".to_string(),
        ])),
        name: Some("Roasted Carrot Soup".to_string()),
        nutrition: Some(NutritionInformationSchema::default()),
        prep_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 10, second: 0, millisecond: 0 }),
        recipe_category: Some(Container::Vec(vec!["Soups".to_string(), "Thanksgiving".to_string()])),
        recipe_cuisine: Some(Container::Vec(vec!["American".to_string()])),
        recipe_ingredients: Some(vec![
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
            HowTo { at_type: AtType::HowToStep, name: "Preheat oven to 425 degrees.\u{a0}".to_string(), text: "Preheat oven to 425 degrees.\u{a0}".to_string(), url: Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-6536bc41231ae").unwrap(), image: None },
            HowTo { at_type: AtType::HowToStep, name: "Line a sheet pan with parchment paper or a silpat cover. Add all the vegetables, along with the apple, thyme and shallot.\u{a0}Toss in oil and sprinkle with 1 teaspoon of salt and freshly cracked pepper.".to_string(), text: "Line a sheet pan with parchment paper or a silpat cover. Add all the vegetables, along with the apple, thyme and shallot.\u{a0}Toss in oil and sprinkle with 1 teaspoon of salt and freshly cracked pepper.".to_string(), url: Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-6536bc41231af").unwrap(), image: None },
            HowTo { at_type: AtType::HowToStep, name: "Roast everything for 15 minutes. Then add 2 cups of stock. Roast for another 15 minutes or until all the vegetables have completely cooked through and are tender.\u{a0}".to_string(), text: "Roast everything for 15 minutes. Then add 2 cups of stock. Roast for another 15 minutes or until all the vegetables have completely cooked through and are tender.\u{a0}".to_string(), url: Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-6536bc41231b0").unwrap(), image: None },
            HowTo { at_type: AtType::HowToStep, name: "Add everything to the blender (including the liquid) but make sure you leave out the thyme. Before adding the thyme to the blender, strip the thyme sprigs of their leaves. Discard the stems and only add the leaves to the blender.\u{a0}".to_string(), text: "Add everything to the blender (including the liquid) but make sure you leave out the thyme. Before adding the thyme to the blender, strip the thyme sprigs of their leaves. Discard the stems and only add the leaves to the blender.\u{a0}".to_string(), url: Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-6536bc41231b1").unwrap(), image: None },
            HowTo { at_type: AtType::HowToStep, name: "Add another 2 cups of stock. Add a small pinch of freshly grated nutmeg.".to_string(), text: "Add another 2 cups of stock. Add a small pinch of freshly grated nutmeg.".to_string(), url: Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-16981142881391014").unwrap(), image: None },
            HowTo { at_type: AtType::HowToStep, name: "Blend until completely smooth. Use a rubber spatula to move any chunks or pieces from the sides of the blender.\u{a0}If you want it to be a little thinner add another ½-1 cup of liquid or as much as you need to get it to your preferred consistency.\u{a0}".to_string(), text: "Blend until completely smooth. Use a rubber spatula to move any chunks or pieces from the sides of the blender.\u{a0}If you want it to be a little thinner add another ½-1 cup of liquid or as much as you need to get it to your preferred consistency.\u{a0}".to_string(), url: Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-16981142889011019").unwrap(), image: None },
            HowTo { at_type: AtType::HowToStep, name: "Add to a pot keep warm on low heat.\u{a0}Serve and garnish.\u{a0}".to_string(), text: "Add to a pot keep warm on low heat.\u{a0}Serve and garnish.\u{a0}".to_string(), url: Url::parse("https://abuelascounter.com/roasted-carrot-soup/#wpzoom-rcb-direction-step-16981143386011052").unwrap(), image: None }
        ])),
        recipe_yield: Some(Yield::VecStr(vec!["6".to_string(), "6 servings".to_string()])),
        total_time: Some(YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 45, second: 0, millisecond: 0 }),
        video: Some(VideoObject {
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
        }),
        ..Default::default()
    };
    assert_eq!(got, want);
}
