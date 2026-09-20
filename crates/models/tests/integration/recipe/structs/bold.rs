use test_db::default_config;
use test_fixtures::insert_user;
use test_harness::create_app_state;

use models::{
    Recipe,
    recipe::structs::{
        recipe::RecipeForCreate,
        section::{Item, SectionComponents, SectionItem},
    },
    settings::UserSettingDetails,
};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub fn a_bare_minimum_recipe() -> RecipeForCreate {
    RecipeForCreate {
        name: "Best Chinese Kale".into(),
        ingredients: SectionComponents::Grouped(vec![
            SectionItem::new(
                "Sauce",
                vec![
                    Item::new("1 cup blue spinach"),
                    Item::new("1/2 tbsp cinnamon"),
                ],
            ),
            SectionItem::new(
                "Main",
                vec![
                    Item::new("4 pounds top quality chicken filet"),
                    Item::new("1/8 cup lemon juice"),
                ],
            ),
        ]),
        instructions: SectionComponents::Grouped(vec![
            SectionItem::new("Sauce", vec![Item::new("Mix all these ingredients")]),
            SectionItem::new(
                "Chicken",
                vec![
                    Item::new("Turn the oven at 300 F"),
                    Item::new("Soak the chicken in the lemon juice"),
                    Item::new("Bake for 35 minutes").with_duration(2100),
                ],
            ),
        ]),
        measurement_system_id: 2,
        ..Default::default()
    }
}

#[tokio::test]
async fn test_bolden_instrucions_on_recipe_create_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;
    let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
    let recipe = a_bare_minimum_recipe();
    let recipe_id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;

    let mut got = Recipe::get(&state.mm, user.id, recipe_id).await?;
    got.bold_ingredients_in_instructions(&state.mm).await?;

    got.instructions.iter_mut().for_each(|item| item.id = None);
    pretty_assertions::assert_eq!(
        got.instructions,
        SectionComponents::Grouped(vec![
            SectionItem::new("Sauce", vec![Item::new("Mix all these ingredients"),]),
            SectionItem::new(
                "Chicken",
                vec![
                    Item::new("Turn the oven at 300 F"),
                    Item::new("Soak the <b>chicken</b> in the <b>lemon juice</b>"),
                    Item::new("Bake for 35 minutes").with_duration(2100),
                ],
            ),
        ])
    );
    Ok(())
}

#[tokio::test]
async fn test_bolden_instructions_on_recipe_update_ok() -> Result<()> {
    let state = create_app_state(default_config()).await;
    let user = insert_user(&state.mm).await?;
    let user_settings = UserSettingDetails::get(&state.mm, user.id).await?;
    let mut recipe = a_bare_minimum_recipe();
    let recipe_id = Recipe::create(&state.mm, user.id, &recipe, &user_settings).await?;
    recipe.instructions = SectionComponents::Grouped(vec![SectionItem::new(
        "Chicken",
        vec![
            Item::new("Wash the blue spinach carefully.").with_id(2),
            Item::new("Soak the chicken in the lemon juice.").with_id(3),
            Item::new("Bake for 35 minutes then sprinkle with cinnamon.").with_id(4),
        ],
    )]);
    Recipe::update(&state.mm, user.id, recipe_id, &mut recipe).await?;

    let mut got = Recipe::get(&state.mm, user.id, recipe_id).await?;
    got.bold_ingredients_in_instructions(&state.mm).await?;

    match &mut got.instructions {
        SectionComponents::Grouped(section_items) => section_items
            .iter_mut()
            .for_each(|item| item.items.iter_mut().for_each(|i| i.id = None)),
        SectionComponents::Flat(items) => items.iter_mut().for_each(|i| i.id = None),
    }
    pretty_assertions::assert_eq!(
        got.instructions,
        SectionComponents::Grouped(vec![SectionItem::new(
            "Chicken",
            vec![
                Item::new("Wash the <b>blue spinach</b> carefully."),
                Item::new("Soak the <b>chicken</b> in the <b>lemon juice</b>."),
                Item::new("Bake for 35 minutes then sprinkle with <b>cinnamon</b>.")
                    .with_duration(2100),
            ],
        ),])
    );
    Ok(())
}
