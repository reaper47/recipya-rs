use uuid::Uuid;

use crate::recipe::utils::a_complete_recipe;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[test]
fn test_to_markdown_complete_recipe_ok() -> Result<()> {
    let (mut recipe, _) = a_complete_recipe();
    recipe.recipe.image = Some(Uuid::parse_str("637bcefb-9fa4-4970-b490-b73023ac772f")?);

    let got = recipe.to_markdown("https://example.com/images")?;
    let expected = r"# Best Chinese Kale

![Image of the recipe](https://example.com/images/data/images/637bcefb-9fa4-4970-b490-b73023ac772f.webp)

## Metadata

Prep time: 1h
Cook time: 15m
Total time: **1h 15m**

Category: dinner
Cuisine: thai
Keywords: vegetarian, tofu
Yield: 4

## Description

This is the most delicious recipe!

## Tools

* 1 wok
* 1 frying pan

## Ingredients

### Sauce

* 1 cup blue spinach
* 1/2 tbsp cinnamon

### Main

* 4 pounds top quality chicken filet
* 1/8 cup lemon juice

## Instructions

### Sauce

1. Mix all these ingredients

### Chicken

1. Turn the oven at 300 F
2. Soak the chicken in the lemon juice
3. Bake for 35 minutes

## Nutrition Facts (per 100g)

|                        |          |
|------------------------|----------|
| **Calories**           | 300 kcal |
| **Total fat**          | 6g       |
| - Saturated fat        | 1g       |
| - Unsaturated fat      | 2g       |
| - Saturated fat        | 3g       |
| **Cholesterol**        | 5mg      |
| **Sodium**             | 12mg     |
| **Total carbohydrate** | 55g      |
| - Dietary fiber        | 10g      |
| - Total sugars         | 43g      |
| **Protein**            | 7g      |

Source: https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/
";
    pretty_assertions::assert_eq!(expected, got);
    Ok(())
}

#[test]
fn test_to_text_complete_recipe_ok() -> Result<()> {
    let (mut recipe, _) = a_complete_recipe();
    recipe.recipe.image = Some(Uuid::parse_str("637bcefb-9fa4-4970-b490-b73023ac772f")?);

    let got = recipe.to_text("https://example.com/images")?;
    let expected = r"Best Chinese Kale
-----------------

Image of the recipe: https://example.com/images/data/images/637bcefb-9fa4-4970-b490-b73023ac772f.webp

Metadata
========

Prep time: 1h
Cook time: 15m
Total time: 1h 15m

Category: dinner
Cuisine: thai
Keywords: vegetarian, tofu
Yield: 4

Description
===========

This is the most delicious recipe!

Tools
=====

* 1 wok
* 1 frying pan

Ingredients
===========

Sauce
-----

* 1 cup blue spinach
* 1/2 tbsp cinnamon

Main
----

* 4 pounds top quality chicken filet
* 1/8 cup lemon juice

Instructions
============

Sauce
-----

1. Mix all these ingredients

Chicken
-------

1. Turn the oven at 300 F
2. Soak the chicken in the lemon juice
3. Bake for 35 minutes

Nutrition Facts (per 100g)
==========================

|--------------------|----------|
| Calories           | 300 kcal |
| Total fat          | 6g       |
| - Saturated fat    | 1g       |
| - Unsaturated fat  | 2g       |
| - Saturated fat    | 3g       |
| Cholesterol        | 5mg      |
| Sodium             | 12mg     |
| Total carbohydrate | 55g      |
| - Dietary fiber    | 10g      |
| - Total sugars     | 43g      |
| Protein            | 7g      |

Source: https://www.allrecipes.com/recipe/10813/best-chocolate-chip-cookies/
";
    pretty_assertions::assert_eq!(expected, got);
    Ok(())
}
