use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::io::{Read, Seek, copy};
use std::path::{Component, Path, PathBuf};

use schema_org::field::RecipeImageFieldEnum;
use serde_json::{Map, Value, json};
use tracing::{debug, error, info};
use zip::ZipArchive;

use schema_org::Recipe;

use crate::{Error, Result};

/// Parses a Recipya ZIP archive to extract the recipes.
pub fn parse_zip<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let mut archive = ZipArchive::new(r)?;
    let mut entries: HashMap<String, (Option<Recipe>, Vec<PathBuf>)> = HashMap::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();
        let path = Path::new(&file_name);

        let folder = match path.components().next() {
            Some(Component::Normal(c)) => c.to_string_lossy().to_string(),
            _ => continue,
        };

        let ext = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default();

        debug!("Processing file: {file_name} with extension: {ext}");

        let entry = entries.entry(folder).or_default();

        if ext == "json" {
            // parse the JSON recipe file
            let recipe: Recipe = parse_json(file)?;
            debug!("Parsed recipe: {:?}", recipe);
            entry.0 = Some(recipe);
        } else if ext == "webp" {
            // import the image to the recipe
            let dest_path = temp_dir().join(file.name().split('/').next_back().unwrap_or_default());
            let mut dest = File::create(&dest_path)?;
            copy(&mut file, &mut dest)?;
            entry.1.push(dest_path);
        }
    }

    let mut recipes: Vec<Recipe> = Vec::new();
    for (_, (recipe, image_paths)) in entries {
        if let Some(mut recipe) = recipe {
            recipe.image = image_paths
                .into_iter()
                .map(|p| RecipeImageFieldEnum::URL(p.to_str().unwrap_or_default().into()))
                .collect();
            recipes.push(recipe);
        }
    }

    info!("Opened Recipya ZIP archive with {} entries", archive.len());
    Ok(recipes)
}

fn parse_json<R>(r: R) -> Result<Recipe>
where
    R: Read,
{
    // read the JSON recipe
    let mut v: Value = serde_json::from_reader(r).map_err(|err| {
        error!("Failed to read Recipya JSON file: {err}");
        Error::Parse(err.to_string())
    })?;

    // migrate the attributes to match the schema.org Recipe schema
    migrate_recipe(&mut v);

    // convert the JSON value to a Recipe struct
    serde_json::from_value(v).map_err(|err| {
        error!("Failed to migrate Recipya JSON file: {err}");
        Error::Parse(err.to_string())
    })
}

fn migrate_recipe(v: &mut Value) {
    if let Some(yield_value) = v.get_mut("recipeYield") {
        match yield_value {
            Value::Number(n) => {
                // schema-org RecipeRecipeYieldFieldEnum accepts Text or QuantitativeValue
                *yield_value = Value::String(n.to_string());
            }
            Value::Array(items) => {
                for item in items {
                    if let Value::Number(n) = item {
                        *item = Value::String(n.to_string());
                    }
                }
            }
            _ => {}
        }
    }

    if let Some(keywords_value) = v.get_mut("keywords") {
        match keywords_value {
            Value::String(s) => {
                let parts: Vec<Value> = s
                    .split(',')
                    .map(str::trim)
                    .filter(|part| !part.is_empty())
                    .map(|part| Value::String(part.to_string()))
                    .collect();

                if !parts.is_empty() {
                    *keywords_value = Value::Array(parts);
                }
            }
            Value::Array(items) => {
                let mut parts: Vec<Value> = Vec::new();

                for item in items.iter() {
                    if let Value::String(s) = item {
                        parts.extend(
                            s.split(',')
                                .map(str::trim)
                                .filter(|part| !part.is_empty())
                                .map(|part| Value::String(part.to_string())),
                        );
                    } else {
                        parts.push(item.clone());
                    }
                }

                *keywords_value = Value::Array(parts);
            }
            _ => {}
        }
    }

    if let Some(tool_value) = v.get_mut("tool") {
        match tool_value {
            Value::String(name) => {
                *tool_value = migrate_tool_string(name);
            }
            Value::Array(items) => {
                let migrated_items = items
                    .iter()
                    .map(|item| match item {
                        Value::String(name) => migrate_tool_string(name),
                        Value::Object(tool) => migrate_tool_object(tool.clone()),
                        _ => item.clone(),
                    })
                    .collect();

                *tool_value = Value::Array(migrated_items);
            }
            Value::Object(tool) => {
                *tool_value = migrate_tool_object(tool.clone());
            }
            _ => {}
        }
    }
}

fn migrate_tool_string(name: &str) -> Value {
    json!({
        "@type": "HowToTool",
        "name": name,
        "requiredQuantity": 1
    })
}

fn migrate_tool_object(mut tool: Map<String, Value>) -> Value {
    if let Some(text) = tool.remove("text") {
        tool.entry("name").or_insert(text);
    }

    tool.entry("@type")
        .or_insert_with(|| Value::String("HowToTool".into()));

    Value::Object(tool)
}
#[cfg(test)]
mod tests {
    use super::*;

    use schema_org::field::{
        ClipDescriptionFieldEnum, RecipeDescriptionFieldEnum, RecipeImageFieldEnum,
        RecipeKeywordsFieldEnum, RecipeRecipeIngredientFieldEnum,
        RecipeRecipeInstructionsFieldEnum, RecipeRecipeYieldFieldEnum, RecipeToolFieldEnum,
        RecipeVideoFieldEnum,
    };
    use schema_org::{AtType, Clip, DurationOrText, Energy, Mass, NutritionInformation, Recipe};
    use tracing_test::traced_test;

    use test_fixtures::open_test_file;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod test_recipes {
        use super::*;

        #[traced_test]
        #[test]
        fn test_recipya_zip_import_ok() -> Result<()> {
            // Import from the test ZIP file and compare the first recipe to the expected result.
            let buf = open_test_file("integrations/recipya.zip");

            let got = parse_zip(buf)?;

            let want = results::recipes();

            let expected_name = &want[0].name;

            // identify the recipe to compare with the reference recipe
            let got_recipe: Recipe = got
                .iter()
                .find(|r| &r.name == expected_name)
                .cloned()
                .unwrap();
            let want_recipe = &want[0];

            pretty_assertions::assert_eq!(got_recipe.name, want_recipe.name);
            pretty_assertions::assert_eq!(got_recipe.description, want_recipe.description);
            pretty_assertions::assert_eq!(got_recipe.keywords, want_recipe.keywords);
            pretty_assertions::assert_eq!(got_recipe.image.len(), want_recipe.image.len());
            pretty_assertions::assert_eq!(got_recipe.tool, want_recipe.tool);
            pretty_assertions::assert_eq!(
                got_recipe.recipe_ingredient,
                want_recipe.recipe_ingredient
            );
            pretty_assertions::assert_eq!(got_recipe.nutrition, want_recipe.nutrition);
            pretty_assertions::assert_eq!(got_recipe.prep_time, want_recipe.prep_time);
            pretty_assertions::assert_eq!(got_recipe.thumbnail_url, want_recipe.thumbnail_url);
            pretty_assertions::assert_eq!(got_recipe.total_time, want_recipe.total_time);
            pretty_assertions::assert_eq!(got_recipe.recipe_yield, want_recipe.recipe_yield);
            pretty_assertions::assert_eq!(got_recipe.url, want_recipe.url);
            pretty_assertions::assert_eq!(got_recipe.video, want_recipe.video);
            Ok(())
        }
    }

    mod results {
        use schema_org::at_context;

        use super::*;

        pub fn recipes() -> Vec<Recipe> {
            vec![
                Recipe {
                    r#type: AtType::Recipe.to_opt(),
                    context: at_context(),
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("hummus bowl".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("keyword2".into()),
                    ],
                    name: vec!["Quick Hummus Bowls / test".into()],
                    description: vec![RecipeDescriptionFieldEnum::Text("A hummus bowl makes the best easy lunch or dinner: no cooking required! Layer a dollop with crunchy veggie toppings.öäü".into())],
                    recipe_category: vec!["main dish".into()],
                    tool: vec![
                        RecipeToolFieldEnum::new_tool("pan", 1.0),
                        RecipeToolFieldEnum::new_tool("pan2", 1.0)
                    ],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1/3 cup hummus".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "8 English cucumber slices (or standard cucumber, peeled)".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 handful red onion slices (or shallot)"
                                .into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 handful cherry tomatoes, sliced".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 handful Kalamata olives".into(),
                        ),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Place greens and rice in the bowl, if using (try packaged pre-cooked rice for a quick shortcut). If using rice, season it with salt and a drizzle of olive oil.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Top with hummus, sliced cucumber, sliced red onions, sliced tomatoes, olives and feta cheese. Eat with pita wedges, using the hummus as a dip / dressing for the veggies.".into(),
                        ),
                    ],
                    nutrition: vec![NutritionInformation {
                        // r#type: AtType::NutritionInformation.to_opt(),
                        calories: vec![Energy::new("552")],
                        carbohydrate_content: vec![Mass::new("86.2")],
                        cholesterol_content: vec![Mass::new("16.7")],
                        fat_content: vec![Mass::new("17.1")],
                        fiber_content: vec![Mass::new("10.1")],
                        protein_content: vec![Mass::new("17")],
                        saturated_fat_content: vec![Mass::new("4.5")],
                        serving_size: vec!["1".into()],
                        sodium_content: vec![Mass::new("1043.6")],
                        sugar_content: vec![Mass::new("4.9")],
                        ..Default::default()
                    }],
                    prep_time: vec![DurationOrText::Text("PT5M".into())],
                    total_time: vec![DurationOrText::Text("PT5M".into())],
                    date_created: vec!["2026-03-07".into()],
                    date_modified: vec!["2026-03-07".into()],
                    date_published: vec!["2026-03-07".into()],
                    thumbnail_url: vec!["http://0.0.0.0:8078/data/images/thumbnails/b64c2c2c-fb90-4f15-a8a8-34c675534b2d.webp".into()],
                    image: vec![
                        RecipeImageFieldEnum::URL("http://0.0.0.0:8078/data/images/b64c2c2c-fb90-4f15-a8a8-34c675534b2d.webp".into()),
                        RecipeImageFieldEnum::URL("http://0.0.0.0:8078/data/images/c743a816-0947-4772-ac44-9b49fd3e67ad.webp".into()),
                    ],
                    url: vec!["https://www.acouplecooks.com/hummus-bowl/".into()],
                    recipe_yield: vec![RecipeRecipeYieldFieldEnum::Text("1".into())],
                    video: vec![
                        RecipeVideoFieldEnum::Clip(
                            Clip {
                        r#type: AtType::VideoObject.to_opt(),
                        // thumbnail_url: vec!["https://content.jwplatform.com/thumbs/HNPx6AFO-720.jpg".into()],
                        description: vec![ClipDescriptionFieldEnum::Text("A video showing how to cook Gyros".into())],
                        name: vec!["Video #1".into()],
                        ..Default::default()
                    }.into(),
                ),
                    ],

                    ..Default::default()
                },
            ]
        }
    }
}
