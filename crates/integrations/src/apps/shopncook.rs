use std::{
    borrow::Cow,
    fmt::Write,
    io::{Read, Seek},
    result,
};

use itertools::Itertools;
use scraper::{ElementRef, Html, Node, Selector};
use winnow::ModalResult;

use schema_org::{
    AtType, DurationOrText, Energy, Mass, NutritionInformation, Recipe, at_context,
    field::{RecipeImageFieldEnum, RecipeKeywordsFieldEnum, RecipeYieldFieldEnum},
};

use crate::{
    Error, Result,
    apps::{
        helpers::{Ingredient, Instruction, Parsers, ToSections, parse_archive_helper, read_file},
        mealmaster,
    },
    helpers::to_is_based_on,
};

#[derive(Default)]
struct RecipeComponents<'a> {
    title: Cow<'a, str>,
    images: Vec<Cow<'a, str>>,
    category: Option<Cow<'a, str>>,
    keywords: Vec<Cow<'a, str>>,
    r#yield: Option<Cow<'a, str>>,
    prep_time: Option<Cow<'a, str>>,
    total_time: Option<Cow<'a, str>>,
    source: Option<Cow<'a, str>>,
    ingredients: Vec<Ingredient<'a>>,
    instructions: Vec<Instruction<'a>>,
    nutrition: Option<NutritionComponents<'a>>,
}

struct NutritionComponents<'a> {
    calories: Option<Cow<'a, str>>,
    protein: Option<Cow<'a, str>>,
    fat: Option<Cow<'a, str>>,
    cholesterol: Option<Cow<'a, str>>,
    carbs: Option<Cow<'a, str>>,
    fiber: Option<Cow<'a, str>>,
    sugars: Option<Cow<'a, str>>,
    sodium: Option<Cow<'a, str>>,
    trans_fat: Option<Cow<'a, str>>,
}

impl From<NutritionComponents<'_>> for NutritionInformation {
    fn from(n: NutritionComponents) -> Self {
        let to_mass = |o: Option<Cow<'_, str>>| {
            o.filter(|s| !s.is_empty())
                .map_or(Vec::new(), |s| vec![Mass::new(s)])
        };

        Self {
            calories: n
                .calories
                .filter(|s| !s.is_empty())
                .map_or(Vec::new(), |s| vec![Energy::new(s)]),
            carbohydrate_content: to_mass(n.carbs),
            cholesterol_content: to_mass(n.cholesterol),
            context: at_context(),
            fat_content: to_mass(n.fat),
            fiber_content: to_mass(n.fiber),
            protein_content: to_mass(n.protein),
            sodium_content: to_mass(n.sodium),
            sugar_content: to_mass(n.sugars),
            r#type: AtType::NutritionInformation.to_opt(),
            trans_fat_content: to_mass(n.trans_fat),
            ..Default::default()
        }
    }
}

impl TryFrom<RecipeComponents<'_>> for Recipe {
    type Error = String;

    fn try_from(r: RecipeComponents<'_>) -> result::Result<Self, Self::Error> {
        if r.ingredients.is_empty() {
            return Err("ingredients are empty".to_string());
        }

        Ok(Self {
            image: r
                .images
                .into_iter()
                .map(|s| RecipeImageFieldEnum::URL(s.to_string()))
                .collect(),
            name: vec![r.title.to_string()],
            recipe_category: r.category.map_or(Vec::new(), |s| vec![s.to_string()]),
            keywords: r
                .keywords
                .into_iter()
                .map(|s| RecipeKeywordsFieldEnum::TextOrURL(s.to_string()))
                .collect(),
            recipe_yield: r.r#yield.map_or(Vec::new(), |s| {
                vec![RecipeYieldFieldEnum::Text(s.to_string())]
            }),
            prep_time: r
                .prep_time
                .map_or(Vec::new(), |s| vec![DurationOrText::Text(s.to_string())]),
            total_time: r
                .total_time
                .map_or(Vec::new(), |s| vec![DurationOrText::Text(s.to_string())]),
            is_based_on: to_is_based_on(&r.source.unwrap_or_default()),
            recipe_ingredient: r.ingredients.to_sections(),
            recipe_instructions: r.instructions.to_sections(),
            nutrition: r.nutrition.map_or(Vec::new(), |n| vec![n.into()]),
            ..Default::default()
        })
    }
}

/// Parses an `AccuChef` zip archive.
pub fn parse_archive<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    parse_archive_helper(
        r,
        &Parsers {
            html: Some(parse_html),
            mealmaster: Some(mealmaster::parse),
            scx: Some(parse_scx),
            txt: Some(parse_txt),
            ..Default::default()
        },
    )
}

/// Parses a `ShopNCook` text file.
pub fn parse_txt<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;

    Ok(vec![
        parse_txt_helper(&mut content.as_str())
            .map_err(|err| Error::Parse(err.to_string()))?
            .try_into()
            .map_err(|err: String| Error::Parse(err))?,
    ])
}

fn parse_txt_helper<'s>(input: &mut &'s str) -> ModalResult<RecipeComponents<'s>> {
    todo!()
    // seq! {RecipeComponents {
    //     title: parse_txt_title,
    //     category: opt(parse_txt_category),
    //     r#yield: opt(parse_txt_yield),
    //     prep_time: opt(parse_txt_prep),
    //     total_time: opt(parse_txt_total),
    //     ingredients: parse_txt_ingredients,
    //     instructions: parse_txt_instructions,
    //     notes: parse_txt_notes,
    //     ..Default::default()
    // }}
    // .parse_next(input)
}

/// Parses an HTML `ShopNCook` recipe file.
#[allow(clippy::too_many_lines)]
pub fn parse_html<R>(r: R) -> Result<Vec<Recipe>>
where
    R: Read + Seek,
{
    let content = read_file(r)?;
    let doc = Html::parse_document(&content);

    let sel_name = Selector::parse("h1.rec-title").unwrap();
    let sel_img = Selector::parse("div.recipe-img img").unwrap();

    let sel_ingredients = Selector::parse("div.ingredients tr").unwrap();
    let sel_ingredient_heading = Selector::parse("td.ingr-heading").unwrap();
    let sel_ingredient_quant = Selector::parse("td.ingr-quant").unwrap();
    let sel_ingredient_name = Selector::parse("span.ingr-name").unwrap();
    let sel_ingredient_comment = Selector::parse("span.ingr-comment").unwrap();
    let sel_instructions = Selector::parse("div.ingredients ~ p").unwrap();

    let sel_info = Selector::parse("div.recipe-info tr").unwrap();
    let sel_info_prompt = Selector::parse("td.recipe-info-prompt").unwrap();
    let sel_info_data = Selector::parse("td.recipe-info-data").unwrap();
    let sel_nut = Selector::parse("div.nutfacts").unwrap();

    Ok(doc
        .select(&Selector::parse(".recipe-box").unwrap())
        .map(|recipe_box| {
            let mut category = String::new();
            let mut keywords = Vec::new();
            let mut r#yield: Option<String> = None;
            let mut prep_time: Option<String> = None;
            let mut total_time: Option<String> = None;
            let mut source: Option<String> = None;

            for row in recipe_box.select(&sel_info) {
                if let Some(prompt) = row.select(&sel_info_prompt).next()
                    && let Some(data) = row.select(&sel_info_data).next()
                {
                    let prompt_text = prompt.text().next();
                    let data_text = data.text().next().unwrap_or_default();

                    if prompt_text.is_some_and(|s| s.starts_with("Category:")) {
                        let parts = data_text.split('|').collect_vec();
                        category = parts.first().cloned().unwrap_or_default().into();
                        keywords = parts.iter().skip(1).cloned().collect();
                    }

                    if prompt_text.is_some_and(|s| s.starts_with("Yield:")) {
                        r#yield = Some(data_text.into());
                    }

                    if prompt_text.is_some_and(|s| s.starts_with("Preparation time:")) {
                        prep_time = Some(data_text.into());
                    }

                    if prompt_text.is_some_and(|s| s.starts_with("Total time:")) {
                        total_time = Some(data_text.into());
                    }

                    if prompt_text.is_some_and(|s| s.starts_with("Source:")) {
                        source = Some(data_text.into());
                    }
                }
            }

            let mut ingredients = Vec::new();
            let mut instructions = Vec::new();

            for row in recipe_box.select(&sel_ingredients) {
                let heading = row
                    .select(&sel_ingredient_heading)
                    .next()
                    .map(|t| t.text().collect::<String>());
                let quant = row
                    .select(&sel_ingredient_quant)
                    .next()
                    .map(|t| t.text().collect::<String>());
                let name = row
                    .select(&sel_ingredient_name)
                    .next()
                    .map(|t| t.text().collect::<String>());
                let comment = row
                    .select(&sel_ingredient_comment)
                    .next()
                    .map(|t| t.text().collect::<String>());

                if heading.is_some() {
                    for part in row.select(&sel_ingredient_heading).flat_map(split_by_br) {
                        if part.chars().count() > 30 {
                            instructions.push(Instruction::Line(Cow::Owned(part)));
                        } else {
                            ingredients.push(Ingredient::Section(Cow::Owned(
                                part.trim_end_matches(':').to_string(),
                            )));
                        }
                    }
                }

                if let Some(name) = name {
                    let mut s = String::new();

                    if let Some(q) = quant {
                        write!(s, "{q} {name}").unwrap();
                        if let Some(c) = comment {
                            write!(s, ", {c}").unwrap();
                        }
                        ingredients.push(Ingredient::Line(Cow::Owned(s)));
                    } else {
                        write!(s, "{name}").unwrap();
                        if let Some(c) = comment {
                            write!(s, ", {c}").unwrap();
                        }
                        ingredients.push(Ingredient::Line(Cow::Owned(s)));
                    }
                }
            }

            for ins in recipe_box.select(&sel_instructions).flat_map(split_by_br) {
                if ins.ends_with(':') {
                    instructions.push(Instruction::Section(Cow::Owned(
                        ins.trim_end_matches(':').to_string(),
                    )));
                } else {
                    instructions.push(Instruction::Line(Cow::Owned(ins)));
                }
            }

            RecipeComponents {
                title: recipe_box
                    .select(&sel_name)
                    .next()
                    .map(|t| t.text().collect::<String>())
                    .map(Cow::Owned)
                    .unwrap_or_default(),
                images: recipe_box
                    .select(&sel_img)
                    .map(|el| el.attr("src").unwrap_or_default())
                    .filter(|s| !s.is_empty())
                    .map(|s| Cow::Borrowed(s.trim_start_matches("shopncook/")))
                    .collect(),
                category: if category.is_empty() {
                    None
                } else {
                    Some(Cow::Owned(category))
                },
                keywords: keywords.into_iter().map(Cow::Borrowed).collect(),
                r#yield: r#yield.map(Cow::Owned),
                prep_time: prep_time.map(Cow::Owned),
                total_time: total_time.map(Cow::Owned),
                source: source.map(Cow::Owned),
                ingredients,
                instructions,
                nutrition: recipe_box.select(&sel_nut).next().map(|el| {
                    let text = el.text().collect::<String>();
                    let parts = text
                        .trim_start_matches("Nutritional facts per serving (daily value):")
                        .trim_start_matches("Nutritional facts (daily value):")
                        .split(';')
                        .map(str::trim)
                        .collect_vec();

                    let extract_nut = |prefix: &str| -> Option<String> {
                        parts.iter().find(|s| s.starts_with(prefix)).map(|s| {
                            let prefix_parts = prefix.split(' ').collect_vec();
                            let subparts = s
                                .split(' ')
                                .filter(|s| !s.trim().is_empty() && !prefix_parts.contains(s))
                                .collect_vec();

                            subparts
                                .first()
                                .filter(|s| **s != "0g" && **s != "0mg")
                                .cloned()
                                .map(|s| s.trim().to_string())
                                .unwrap_or_default()
                        })
                    };

                    NutritionComponents {
                        calories: extract_nut("Calories").map(Cow::Owned),
                        protein: extract_nut("Protein").map(Cow::Owned),
                        fat: extract_nut("Total Fat").map(Cow::Owned),
                        cholesterol: extract_nut("Chol.").map(Cow::Owned),
                        carbs: extract_nut("Carb.").map(Cow::Owned),
                        fiber: extract_nut("Fiber").map(Cow::Owned),
                        sugars: extract_nut("Sugars").map(Cow::Owned),
                        sodium: extract_nut("Sodium").map(Cow::Owned),
                        trans_fat: extract_nut("Trans fat").map(Cow::Owned),
                    }
                }),
                ..Default::default()
            }
        })
        .filter_map(|r| Recipe::try_from(r).ok())
        .collect())
}

fn split_by_br(container: ElementRef) -> Vec<String> {
    let mut lines = Vec::new();
    let mut curr = String::new();

    for node in container.descendants() {
        match node.value() {
            Node::Element(el) if el.name() == "br" => {
                if !curr.trim().is_empty() {
                    lines.push(curr.trim().to_string());
                }
                curr.clear();
            }
            Node::Text(t) => curr.push_str(t),
            _ => {}
        }
    }
    if !curr.trim().is_empty() {
        lines.push(curr.trim().to_string());
    }
    lines
}

/// Parses a SCX `ShopNCook` recipe file.
pub fn parse_scx<R>(mut r: R) -> Result<Vec<Recipe>>
where
    R: Read,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests_recipes {
        use std::io::Cursor;

        use super::*;

        #[test]
        fn test_html_ok() -> Result<()> {
            let buf = Cursor::new(files::html());

            let got = parse_html(buf)?;

            pretty_assertions::assert_eq!(got.len(), results::all_recipes().len());
            pretty_assertions::assert_eq!(got, results::all_recipes());
            Ok(())
        }

        #[test]
        fn test_scx_ok() -> Result<()> {
            todo!()
        }

        #[test]
        fn test_txt1_ok() -> Result<()> {
            todo!()
        }

        #[test]
        fn test_txt2_ok() -> Result<()> {
            todo!()
        }
    }

    mod files {
        pub fn html<'a>() -> &'a str {
            r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd"><html xmlns=http://www.w3.org/1999/xhtml><head><meta http-equiv=Content-Type content="text/html; charset=utf-8"/><title>Recipes</title><style type=text/css><!-- body,div,p,h1,h2,h3,h4,h5,h6,ul,li,dl,dt,dd,form,input,table,img{padding:0;margin:0}body{background:#989898;font:normal 12px/16px verdana;color:#666666;padding:20px 0}img{border:0;vertical-align:middle}ul{list-style:none}p{padding-bottom:16px}a:link,a:visited,a:hover{text-decoration:none}a:link,a:visited{color:#009900}a:hover,b,strong{color:#ab0101}h1{font:normal 25px georgia;color:#ab0101;padding:0 0 10px 0}h2{font:normal 20px georgia;color:#000;padding:0 0 20px 0}.menu h1{text-align:center}.menu h2{text-align:center;font:italic 16px georgia}.nutfacts{color:#000;font:normal 12px/16px georgia;background:#f7f7f7;padding:10px;border:1px solid #ebebeb}.nutfacts ul{padding:0 0 0 30px}#wrapper{width:980px;margin:0 auto}.logo{background:#fff;text-align:center;width:940px;padding:20px;float:left}.message{background:#ffffdd;color:#000000;width:940px;padding:20px 20px 40px 20px;float:left;margin-bottom:10px}.recipe-header,.menu-header,.recipe-box{background:#fff url(data:image/jpeg;base64,/9j/4AAQSkZJRgABAgAAZABkAAD/7AARRHVja3kAAQAEAAAAUAAA/+4ADkFkb2JlAGTAAAAAAf/bAIQAAgICAgICAgICAgMCAgIDBAMCAgMEBQQEBAQEBQYFBQUFBQUGBgcHCAcHBgkJCgoJCQwMDAwMDAwMDAwMDAwMDAEDAwMFBAUJBgYJDQsJCw0PDg4ODg8PDAwMDAwPDwwMDAwMDA8MDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwM/8AAEQgAHgAeAwERAAIRAQMRAf/EAGIAAAIDAQAAAAAAAAAAAAAAAAAHAwYICQEBAAAAAAAAAAAAAAAAAAAAABAAAgIBAwIFBQAAAAAAAAAAAgMBBAARQQYTBTFxwRJCsTIjFAcRAQAAAAAAAAAAAAAAAAAAAAD/2gAMAwEAAhEDEQA/AOmGAYENmyimhtq00UV0DJtac6CIxvOAkLP9H/Y5P2+zoxfYKDDjoj97Pesl9U43092sD64D1wIbNlFNDbVpooroGTa050ERjecDOHM+Zv5G+atWSR2dBfiV4E0o+Z+kbeeBRMDZVmyimhtq00UV0DJtac6CIxvOBnDmfM38jfNWrJI7OgvxK8CaUfM/SNvPAomAYDb/AKPZ5PY0/Z7eyh2BbPanQ1n1D2NvTItNfjE/XAUmAYBgf//Z) no-repeat bottom right;width:940px;padding:20px 20px 40px 20px;float:left;margin-bottom:10px}.recipe-header ul{font-weight:bold;padding:0 0 0 80px}.recipe-header li{list-style:decimal}.recipe-header li li{list-style:circle}.recipe-header li ul{padding:0 0 0 20px}.recipe-header li{padding:6px 0}.menu-header ul{font-weight:bold;margin:0 auto;text-align:center}.menu-header li{list-style:none;font:bold 16px georgia;color:#ab0101}.menu-header li li{list-style:none;font:bold 12px verdana;color:#000}.menu-header li li li{list-style:none;font:normal 12px verdana;color:#666}.menu-header li{padding:6px 0}.menu-date,italic,i,em{color:#000}.menu-date{padding-top:20px}.menu-date{font-style:italic}.chapter{background:#989898;color:#fff;padding:10px 20px;width:940px;float:left}.chapter h1{color:#fff;page-break-before:always}.recipe-info{page-break-inside:avoid;width:680px;float:left;padding-bottom:10px}.recipe-info table{width:680px;padding:0 0 5px 0}td.recipe-info-prompt{width:170px;color:#ab0101}td.recipe-info-data{width:500;padding-left:10px}.recipe-info dl{width:680px;padding:0 0 5px 0;float:left}.recipe-info dt{width:170px;float:left;color:#ab0101}.recipe-info dd{width:500px;padding-left:10px;float:right}.ingredients{float:left;width:940px;padding:10px 0}.ingredients dl{width:940px;padding:0 0 10px 0;float:left}.ingredients dt{width:180px;float:left;color:#000;text-align:right}.ingredients dd{width:750px;padding-left:10px;float:right}.ingredients table{width:940px;padding:0 0 10px 0}td.ingr-quant{width:180px;color:#000;text-align:right;padding-right:10px}.nutfacts dl{width:680px;padding:0 0 10px 0;line-height:.6em}.nutfacts dt{width:140px}.nutfacts dd{width:540px;padding-left:10px;float:right}.nutfacts table{padding:0 0 10px 0}.nutfacts tr{line-height:.6em}.nutfacts td{padding-left:10px}td.nutrient-name{width:140px}.recipe-img{page-break-inside:avoid;width:250px;padding-right:10px;float:left}.clear{float:left;width:940px;padding:10px 0}.snc-footer{text-align:right;font-size:small}.rec-title{page-break-before:always}--></style> <style type=text/css media=print><!-- .ingredients,.recipe-header,.menu-header,.recipe-box,clear{float:none}--></style></head><body><div id=wrapper> <div class=recipe-header><h1>Index</h1><ul><li><a href=#recipe0> Welcome</a></li><li><a href=#recipe1> Overview</a></li><li><a href=#recipe2>California rolls</a></li><li><a href=#recipe3>Cooked sushi rice</a></li><li><a href=#recipe4>Crab or shrimp Louis salad</a></li><li><a href=#recipe5>Mabodofu</a></li><li><a href=#recipe6>Sushi vinegar</a></li><li><a href=#recipe7>Tiramisu</a></li></ul></div> <div class=recipes><div class=recipe-box><div class=clear><a name=recipe0></a><h1 class=rec-title>Welcome</h1></div><div class=clear><div class=recipe-img><a href=shopncook/img0high.jpg><img src=shopncook/img0.jpg alt="  Welcome"/></a></div><div class=recipe-info><table></table></div></div><div class=clear><div class=ingredients><table></table></div><p>Welcome to Shop'NCook software!<br/> <br/> I created Shop'NCook to solve once for all the problem of planning for meals and organizing the grocery shopping. Shop'NCook comes in three flavors:<br/> - Shop'NCook Home - recipe management, grocery shopping, nutritional analysis;<br/> - Shop'NCook Menu - all of the Home edition plus a meal planner and a menu library;<br/> - Shop'NCook Pro - all of the Menu edition plus recipe and menu costing features.<br/> <br/> Click on "Overview" on the left for a summary of the main features. You can also find a tutorial at http://www.shopncook.com/slideShow.html . To know more on specific topics and to keep up-to-date with the latest news, visit the blog at http://www.shopncook.com/blog/ .<br/> <br/> To contact me, write to customerservice@rufenacht.com .<br/> <br/> I wish you a lot of fun with Shop'NCook!<br/> <br/> Mathilde Rufenacht<br/> Author of Shop'NCook</p></div><div class=clear><div class=nutfacts></div></div></div><div class=recipe-box><div class=clear><a name=recipe1></a><h1 class=rec-title>Overview</h1></div><div class=clear><div class=recipe-info><table></table></div></div><div class=clear><div class=ingredients><table></table></div><p>TIP: Double-click on Overview on the left to open it in a separate window and refer to it while learning your way with Shop'NCook. You can also access a detailed user manual in the Help menu.<br/> <br/> VISUALIZE A RECIPE<br/> Select a cookbook on the navigation panel. Click on a recipe title to view it. The ingredients that are underlined are links that refer to another recipe. Click on the link to view the linked recipe. Click on the picture to see it full size. Display the "Shopping items" tab to see the list of the ingredients. Double-click on a recipe title to open it in a different window. <br/> <br/> RESIZE A RECIPE<br/> Input a new serving size or yield and click on Calculate to resize the recipe.<br/> <br/> FIND A RECIPE<br/> Click the search button of the mini-toolbar (bottom left) to search your recipes. Click "Select All" to search all the cookbooks.<br/> <br/> Search for a recipe on the Internet by clicking on the Get Recipe button of the mini-toolbar.<br/> <br/> ADD A RECIPE<br/> Select an unlocked cookbook in the Cookbooks section, click the New button and type the recipe in the wizard. It will recognize and format automatically the ingredients for you.<br/> <br/> To add a recipe from a file or from internet, select it with the mouse and drag & drop it on the recipe-display panel to add it to Shop'NCook. You can add a picture in the same way.<br/> <br/> If you have your own collection of recipes in text format, you can easily add it to the software in the following way:<br/> 1. Separate each recipe by a line of seven hyphens or more (-------).<br/> 2. Copy the text of the file to the clipboard<br/> 3. Select "Import recipes from clipboard" in the File menu.<br/> <br/> There are also many cookbooks that you can import with a single click from the Browser tab.<br/> <br/> PLAN YOUR MENUS (Menu and Pro editions)<br/> To plan your meals, drag your recipes to the calendar on the left and organize them in menu. By clicking on the Edit button, you can set the number of servings of a menu to adjust automatically the size of its recipes. You will find examples of menus in the Library tab of the Calendar.<br/> <br/> To display your menus, select the Menus section in the main window. You can set in the toolbar the number of days of menus you want to display.<br/> <br/> MAKE YOUR GROCERY LIST<br/> Use the Add button of the toolbar to add to the shopping list the ingredients of a recipe, of a menu or of your whole meal planning period. To have a greater control on what gets added to the shopping list, display the "Shopping items" tab of the Cookbooks or Menus sections.<br/> <br/> Shop'NCook gives a lot of flexibility for the format of your shopping lists. You can modify your options in the Preferences. See also the help for more information.<br/> <br/> AISLE-ORDER YOUR SHOPPING LISTS (Menu and Pro editions)<br/> Use the + button of the mini-toolbar to create a supermarket. Edit the supermarket and change the order of the aisles to match your store. You can also input the brand of the article, size or selling unit, or look them up in the barcode database for fast input. In the shopping list view, assign your articles to the supermarket of your choice to get an aisle-ordered list.<br/> <br/> NUTRITIONAL ANALYSIS<br/> The nutritional analysis is automatically calculated and displayed at the bottom of your recipes and menus. Click the Nutrition tab of the Cookbooks or Menus sections too see the break-down by ingredients. You can select in the Preferences the nutrients you want displayed. Only the ingredients displayed in black are included. The meaning of the colors is as follows:<br/> <br/> - Red: the ingredients is not in the database or has not been recognized;<br/> - Blue: the quantity is not specified or unknow of the program for the given ingredient;<br/> - Green: no nutritional information is associated to the ingredient.<br/> <br/> To turn an ingredient black, click the magic wand button and the software will guide you.<br/> <br/> COSTING (Pro edition only)<br/> Click the Costing tab of the Recipe or Menu managers to display the cost break-down per ingredient. The ingredients in color are not included in the costing, similarly to the nutritional analysis above. Click the magic wands to get help turning the ingredients black.<br/> <br/> Food cost data is entered by editing the main database and the supermarkets in the Supermarkets section. You can also import one of the predefined cost database by selecting "Import Cost Data" in the Costing menu.<br/> <br/> Get the best price for your shopping list by clicking on the supermarket button in the Shopping list view and selecting Best Value.<br/> <br/> SHARING<br/> Shop'NCook offers several functions to make your recipes et shopping list easily accessible:<br/> 1. The Send button to send your recipes, menus and shopping lists by e-mail. It allows you also to transfer them practically to your mobile phone.<br/> 2. Uploading your recipes to the Internet with the button Share of the Recipe manager. The uploaded recipes are accessibles to every Shop'NCook user from the Direct Access section, from Shop'NCook iPhone application, as well as on our online database at http://www.DirectAccessRecipes.com/ .<br/> 3. The free software Shop'NCook Reader to share your cookbooks with family and friends.<br/> <br/> SYNCHRONIZATION (requires a synchronization account)<br/> Click the synchronization button of the mini-toolbar to synchronize your recipes and shopping list between all your computers and devices (iPhone, iPad, iPod) and access them from anywhere.<br/> <br/> NEED MORE?<br/> Shop'NCook offers much more. For more information on any of above topics, click on the Help button of the mini-toolbar.</p></div><div class=clear><div class=nutfacts></div></div></div><div class=recipe-box><div class=clear><a name=recipe2></a><h1 class=rec-title>California rolls</h1></div><div class=clear><div class=recipe-img><a href=shopncook/img2high.jpg><img src=shopncook/img2.jpg alt="California rolls"/></a></div><div class=recipe-info><table><tr><td class=recipe-info-prompt>Category:</td><td class=recipe-info-data>Japanese|Rice|Main dishes|Appetizers|Shop'NCook</td></tr><tr><td class=recipe-info-prompt>Yield:</td><td class=recipe-info-data>6 servings of 4</td></tr><tr><td class=recipe-info-prompt>Preparation time:</td><td class=recipe-info-data>25 minutes</td></tr><tr><td class=recipe-info-prompt>Total time:</td><td class=recipe-info-data>40 minutes</td></tr><tr><td class=recipe-info-prompt>Food cost per portion:</td><td class=recipe-info-data>$0.88</td></tr><tr><td class=recipe-info-prompt>Price per serving:</td><td class=recipe-info-data>$3.14</td></tr><tr><td class=recipe-info-prompt>Source:</td><td class=recipe-info-data>An Introduction to Japanese Home Cooking by Mathilde Rufenacht</td></tr></table></div></div><div class=clear><div class=ingredients><table><tr><td class=ingr-heading colspan=2>Strictly speaking, California rolls are not Japanese - you would be hard pressed to find them in Japan, - but they are a marvelous example how cooking techniques can be deliciously adapted to local ingredients. I give this recipe here as a beacon for all experimental cooks:</td></tr><tr><td class=ingr-quant>12 cups</td><td><span class=ingr-name>cooked sushi rice</span><span class=ingr-sep> - </span><span class=ingr-comment>see separate recipe</span></td></tr><tr><td class=ingr-quant>6 sheets</td><td><span class=ingr-name>nori</span><span class=ingr-sep> - </span><span class=ingr-comment>seaweed</span></td></tr><tr><td class=ingr-heading colspan=2>Filling:</td></tr><tr><td class=ingr-quant>1 medium</td><td><span class=ingr-name>cucumber</span></td></tr><tr><td class=ingr-quant>1</td><td><span class=ingr-name>avocado</span></td></tr><tr><td class=ingr-quant>1/2 tbsp</td><td><span class=ingr-name>lemon juice</span></td></tr><tr><td class=ingr-quant>9</td><td><span class=ingr-name>imitation crab stick</span><span class=ingr-sep> - </span><span class=ingr-comment>or cooked snow crab meat</span></td></tr><tr><td class=ingr-heading colspan=2>Sides:</td></tr><tr><td class=ingr-quant></td><td><span class=ingr-name>wasabi</span></td></tr><tr><td class=ingr-quant></td><td><span class=ingr-name>soy sauce</span></td></tr><tr><td class=ingr-quant></td><td><span class=ingr-name>gari</span><span class=ingr-sep> - </span><span class=ingr-comment>pickled ginger, see separated recipe</span></td></tr></table></div><p>Prepare the sushi rice according to above recipe using 2 cups of Japanese rice.<br/> <br/> <br/> Preparation of the filling:<br/> <br/> Wash and peel the cucumber. Seed it and cut it in thin long strips. Cut the avocado in two, remove the pit and peel, and cut it likewise. Sprinkle a few drops of lemon on it to keep it from darkening. Cut the crab sticks in two lengthwise.<br/> <br/> <br/> Rolling:<br/> <br/> Put a nori sheet in the center of a dry bamboo rolling mat. Take about one sixth of the rice and spread it evenly on the nori leaving one inch at the far end of the nori sheet, in order to have a quarter inch-thick layer of rice on the sheet. To handle the rice, wet your fingers with water mixed with a little bit of vinegar. Dispose one sixth of the cucumber and avocado strips on a two-inch strip in the center of the rice. Arrange the crab sticks on top. Start rolling by taking the end of the rolling mat nearest to you and bringing it over the end of the layer of rice. Press to make a tight roll while pulling back the end of the bamboo mat as neccessary. With a wet towel, push softly on the sides of the roll to arrange the filling. If necessary, wet a little the end of the nori sheet to help closing the roll. Remove the rolling mat. Cut 1 inch-thick slices with a wet knife. Repeat for the other nori sheets.<br/> <br/> Serve with wasabi, soy sauce and gari at the side. The soy sauce is poured in small individual plates and a little bit of wasabi is dissolved in it. Dip the sushi in it as you eat.<br/> <br/> California rolls can be served as appetizer or as main dish. Many variations of filling are possible. Try for example cooked shrimps, raw tuna, shiitake nimono, meat for gyuudon (see separate recipes).<br/> <br/></p></div><div class=clear><div class=nutfacts>Nutritional facts per serving (daily value): Calories 393.952kcal; Protein 6.781g (14%); Total Fat 0.575g (1%)(Sat. 0.119g (1%)); Chol. 0mg (0%); Carb. 88.699g (30%); Fiber 2.854g (11%); Sugars 11.97g; Calcium 20.939mg (2%); Iron 1.684mg (9%); Sodium 1085.715mg (45%); Vit. C 2.734mg (5%); Vit. A 188.118IU (4%); Trans fat 0g</div></div></div><div class=recipe-box><div class=clear><a name=recipe3></a><h1 class=rec-title>Cooked sushi rice</h1></div><div class=clear><div class=recipe-img><a href=shopncook/img3high.jpg><img src=shopncook/img3.jpg alt="Cooked sushi rice"/></a></div><div class=recipe-info><table><tr><td class=recipe-info-prompt>Category:</td><td class=recipe-info-data>Side dishes|Rice|Japanese|Shop'NCook</td></tr><tr><td class=recipe-info-prompt>Yield:</td><td class=recipe-info-data>12 cups</td></tr><tr><td class=recipe-info-prompt>Source:</td><td class=recipe-info-data>An Introduction to Japanese Home Cooking by Mathilde Rufenacht</td></tr></table></div></div><div class=clear><div class=ingredients><table><tr><td class=ingr-heading colspan=2>This rice is used as base for sushi and sushi rolls.<br/> Cook:</td></tr><tr><td class=ingr-quant>3 cups</td><td><span class=ingr-name>Japanese rice</span><span class=ingr-sep> - </span><span class=ingr-comment>but use 10% less water than usual.</span></td></tr><tr><td class=ingr-heading colspan=2>Add:</td></tr><tr><td class=ingr-quant>about 1/2 cup + 2 tbsp</td><td><span class=ingr-name>sushi vinegar</span><span class=ingr-sep> - </span><span class=ingr-comment>see separate recipe</span></td></tr></table></div><p>Mix well with a cutting and folding movement.<br/></p></div><div class=clear><div class=nutfacts>Nutritional facts (daily value): Calories 2311.5kcal; Protein 37.795g (76%); Total Fat 3.053g (5%)(Sat. 0.61g (3%)); Chol. 0mg (0%); Carb. 519.977g (173%); Fiber 15.54g (62%); Sugars 66.533g; Calcium 65.717mg (7%); Iron 8.968mg (50%); Sodium 6499.183mg (271%); Vit. C 0mg (0%); Vit. A 0IU (0%); Trans fat 0g</div></div></div><div class=recipe-box><div class=clear><a name=recipe4></a><h1 class=rec-title>Crab or shrimp Louis salad</h1></div><div class=clear><div class=recipe-img><a href=shopncook/img4high.jpg><img src=shopncook/img4.jpg alt="Crab or shrimp Louis salad"/></a></div><div class=recipe-info><table><tr><td class=recipe-info-prompt>Category:</td><td class=recipe-info-data>Seafood|Salads|Shop'NCook</td></tr><tr><td class=recipe-info-prompt>Yield:</td><td class=recipe-info-data>4 servings</td></tr><tr><td class=recipe-info-prompt>Food cost per portion:</td><td class=recipe-info-data>$3.27</td></tr><tr><td class=recipe-info-prompt>Price per serving:</td><td class=recipe-info-data>$11.69</td></tr></table></div></div><div class=clear><div class=ingredients><table><tr><td class=ingr-heading colspan=2>SALAD:</td></tr><tr><td class=ingr-quant>1</td><td><span class=ingr-name>lettuce</span><span class=ingr-sep> - </span><span class=ingr-comment>torn</span></td></tr><tr><td class=ingr-quant>2 c.</td><td><span class=ingr-name>crab or shrimp meat</span></td></tr><tr><td class=ingr-quant>2</td><td><span class=ingr-name>Tomato</span><span class=ingr-sep> - </span><span class=ingr-comment>cut in wedges</span></td></tr><tr><td class=ingr-quant>2</td><td><span class=ingr-name>Egg</span><span class=ingr-sep> - </span><span class=ingr-comment>cut in wedges</span></td></tr><tr><td class=ingr-quant>1 can</td><td><span class=ingr-name>Asparagus</span></td></tr><tr><td class=ingr-quant>1</td><td><span class=ingr-name>Avocado</span><span class=ingr-sep> - </span><span class=ingr-comment>sliced</span></td></tr><tr><td class=ingr-heading colspan=2>LOUIS DRESSING:</td></tr><tr><td class=ingr-quant>1/2 c.</td><td><span class=ingr-name>refrigerated style French dressing</span></td></tr><tr><td class=ingr-quant>1/3 c.</td><td><span class=ingr-name>bottled chile sauce or catsup</span></td></tr><tr><td class=ingr-quant>2 tbsp.</td><td><span class=ingr-name>mayonnaise</span></td></tr><tr><td class=ingr-quant>1/2 tsp.</td><td><span class=ingr-name>Worcestershire sauce</span></td></tr><tr><td class=ingr-quant>1 tbsp.</td><td><span class=ingr-name>lemon juice</span></td></tr><tr><td class=ingr-quant>1/4 tsp.</td><td><span class=ingr-name>black pepper</span></td></tr></table></div><p>Prepare and chill dressing by mixing all the ingredients. On serving plate arrange salad greens, mount seafood in center and garnish with vegetables. Serve with Louis dressing. <br/> <br/></p></div><div class=clear><div class=nutfacts>Nutritional facts per serving (daily value): Calories 418.414kcal; Protein 19.484g (39%); Total Fat 30.137g (46%)(Sat. 4.365g (22%)); Chol. 151.386mg (50%); Carb. 21.952g (7%); Fiber 6.378g (26%); Sugars 12.82g; Calcium 141.798mg (14%); Iron 3.484mg (19%); Sodium 1110.429mg (46%); Vit. C 38.348mg (64%); Vit. A 8151.482IU (163%); Trans fat 0.018g</div></div></div><div class=recipe-box><div class=clear><a name=recipe5></a><h1 class=rec-title>Mabodofu</h1></div><div class=clear><div class=recipe-img><a href=shopncook/img5high.jpg><img src=shopncook/img5.jpg alt="Mabodofu"/></a></div><div class=recipe-info><table><tr><td class=recipe-info-prompt>Category:</td><td class=recipe-info-data>Main dishes|Meat|Chinese|Tofu|Shop'NCook</td></tr><tr><td class=recipe-info-prompt>Yield:</td><td class=recipe-info-data>2 servings</td></tr><tr><td class=recipe-info-prompt>Food cost per portion:</td><td class=recipe-info-data>$2.26</td></tr><tr><td class=recipe-info-prompt>Price per serving:</td><td class=recipe-info-data>$8.06</td></tr><tr><td class=recipe-info-prompt>Source:</td><td class=recipe-info-data>Otoko no tame no ryouri no ki sou</td></tr></table></div></div><div class=clear><div class=ingredients><table><tr><td class=ingr-quant>400 g</td><td><span class=ingr-name>tofu</span></td></tr><tr><td class=ingr-quant>200 g</td><td><span class=ingr-name>ground pork</span></td></tr><tr><td class=ingr-quant>1 heaped tbsp</td><td><span class=ingr-name>salad oil</span></td></tr><tr><td class=ingr-quant>1 - 1 1/2 tsp</td><td><span class=ingr-name>toban djan</span><span class=ingr-sep> - </span><span class=ingr-comment>(chili bean paste, can be replaced by crushed chili pepper)</span></td></tr><tr><td class=ingr-quant>1 tsp</td><td><span class=ingr-name>finely chopped garlic</span></td></tr><tr><td class=ingr-quant>1 tsp</td><td><span class=ingr-name>finely chopped ginger root</span></td></tr><tr><td class=ingr-quant>3 tbsp</td><td><span class=ingr-name>chopped green onion</span></td></tr><tr><td class=ingr-quant>1 tsp</td><td><span class=ingr-name>sesame oil</span></td></tr><tr><td class=ingr-heading colspan=2>A:</td></tr><tr><td class=ingr-quant>2 - 2 1/2 tbsp</td><td><span class=ingr-name>miso</span></td></tr><tr><td class=ingr-quant>1 1/2 tbsp</td><td><span class=ingr-name>soy sauce</span></td></tr><tr><td class=ingr-quant>2 tbsp</td><td><span class=ingr-name>sake</span></td></tr><tr><td class=ingr-quant>1 - 1 1/2 tbsp</td><td><span class=ingr-name>sugar</span></td></tr><tr><td class=ingr-quant>2/3 cup</td><td><span class=ingr-name>water</span></td></tr><tr><td class=ingr-quant>a little</td><td><span class=ingr-name>gara soup no moto</span><span class=ingr-sep> - </span><span class=ingr-comment>(Chinese chicken bouillon)</span></td></tr><tr><td class=ingr-heading colspan=2>Diluted maizena:</td></tr><tr><td class=ingr-quant>1 tbsp</td><td><span class=ingr-name>maizena</span></td></tr><tr><td class=ingr-quant>2 tbsp</td><td><span class=ingr-name>water</span></td></tr></table></div><p>1. Wrap the tofu in paper towel and heat in microwave oven 3 min. 30 s to remove some of the water. Cool. Cut in 2cm cubes.<br/> 2. Mix ingredients A.<br/> 3. Cook over medium heat in a little oil the toubanjan, add garlic and ginger, without letting them burn.<br/> 4. Over high heat, add the minced meat and separate it with the back of a laddle. When hard to separate, add a little bit of sake.<br/> 5. When the meat is cooked, add the green onion and cook. Add ingredients A. Mix well over high heat.<br/> 6. When it starts boiling, add the tofu.<br/> 7. When it boils again, tilt the skillet and add in two to three times the diluted maizena in the soup.<br/> 8. To finish, add the sesame oil and serve with Japanese rice.<br/></p></div><div class=clear><div class=nutfacts>Nutritional facts per serving (daily value): Calories 625.154kcal; Protein 34.42g (69%); Total Fat 40.761g (63%)(Sat. 10.024g (50%)); Chol. 72mg (24%); Carb. 26.006g (9%); Fiber 1.691g (7%); Sugars 13.72g; Calcium 105.191mg (11%); Iron 3.757mg (21%); Sodium 1584.638mg (66%); Vit. C 2.398mg (4%); Vit. A 385.615IU (8%); Trans fat 0.041g</div></div></div><div class=recipe-box><div class=clear><a name=recipe6></a><h1 class=rec-title>Sushi vinegar</h1></div><div class=clear><div class=recipe-img><a href=shopncook/img6high.jpg><img src=shopncook/img6.jpg alt="Sushi vinegar"/></a></div><div class=recipe-info><table><tr><td class=recipe-info-prompt>Category:</td><td class=recipe-info-data>Sauces & dressings|Japanese|Shop'NCook</td></tr><tr><td class=recipe-info-prompt>Yield:</td><td class=recipe-info-data>675 ml</td></tr><tr><td class=recipe-info-prompt>Preparation time:</td><td class=recipe-info-data>10 min</td></tr><tr><td class=recipe-info-prompt>Total time:</td><td class=recipe-info-data>10 min</td></tr><tr><td class=recipe-info-prompt>Source:</td><td class=recipe-info-data>An Introduction to Japanese Home Cooking by Mathilde Rufenacht</td></tr></table></div></div><div class=clear><div class=ingredients><table><tr><td class=ingr-heading colspan=2>Prepare a bottle in advance to use when making sushi rice.</td></tr><tr><td class=ingr-quant>450 ml</td><td><span class=ingr-name>rice vinegar</span></td></tr><tr><td class=ingr-quant>300 g</td><td><span class=ingr-name>sugar</span></td></tr><tr><td class=ingr-quant>75 g</td><td><span class=ingr-name>salt</span></td></tr></table></div><p>Mix in a skillet over low to medium heat until the sugar and salt are diluted. Do not boil. Put back in the original vinegar bottle, close the lid and put at once in cold water to cool as fast as possible in order to keep the flavor of the vinegar. To not mix it with usual rice vinegar, donät forget to put a label with "Sushi vinegar" and the date on it.<br/></p></div><div class=clear><div class=nutfacts>Nutritional facts (daily value): Calories 1161kcal; Protein 0g (0%); Total Fat 0g (0%)(Sat. 0g (0%)); Chol. 0mg (0%); Carb. 299.94g (100%); Fiber 0g (0%); Sugars 299.4g; Calcium 21mg (2%); Iron 0.398mg (2%); Sodium 29071.5mg (1,211%); Vit. C 0mg (0%); Vit. A 0IU (0%); Trans fat 0g</div></div></div><div class=recipe-box><div class=clear><a name=recipe7></a><h1 class=rec-title>Tiramisu</h1></div><div class=clear><div class=recipe-img><a href=shopncook/img7high.jpg><img src=shopncook/img7.jpg alt="Tiramisu"/></a></div><div class=recipe-info><table><tr><td class=recipe-info-prompt>Category:</td><td class=recipe-info-data>Desserts|Italian|Shop'NCook</td></tr><tr><td class=recipe-info-prompt>Yield:</td><td class=recipe-info-data>4 servings of about 200 g</td></tr><tr><td class=recipe-info-prompt>Preparation time:</td><td class=recipe-info-data>15 minutes</td></tr><tr><td class=recipe-info-prompt>Total time:</td><td class=recipe-info-data>2 hours</td></tr><tr><td class=recipe-info-prompt>Food cost per portion:</td><td class=recipe-info-data>$2.76</td></tr><tr><td class=recipe-info-prompt>Price per serving:</td><td class=recipe-info-data>$9.85</td></tr><tr><td class=recipe-info-prompt>Source:</td><td class=recipe-info-data>Anna</td></tr></table></div></div><div class=clear><div class=ingredients><table><tr><td class=ingr-quant>500 g</td><td><span class=ingr-name>mascarpone</span></td></tr><tr><td class=ingr-quant>5</td><td><span class=ingr-name>eggs</span><span class=ingr-sep> - </span><span class=ingr-comment>(Anna uses 5 yolks and 3 whites)</span></td></tr><tr><td class=ingr-quant>5 tbsp</td><td><span class=ingr-name>sugar</span><span class=ingr-sep> - </span><span class=ingr-comment>(Anna uses 3 tbsp)</span></td></tr><tr><td class=ingr-quant>about 20</td><td><span class=ingr-name>ladyfingers</span></td></tr><tr><td class=ingr-heading colspan=2>Beat the egg yolks, cream with the sugar. Mix well with the mascarpone. Whip the egg whites until stiff (but not dry). Incorporate delicately to the mascarpone mass.<br/> Prepare:</td></tr><tr><td class=ingr-quant>1/2 cup</td><td><span class=ingr-name>strong coffee</span></td></tr><tr><td class=ingr-heading colspan=2>Mix in:</td></tr><tr><td class=ingr-quant></td><td><span class=ingr-name>cognac or some other alcool, replace by orange juice when preparing for children</span></td></tr><tr><td class=ingr-heading colspan=2>Line the serving dish with a layer of savoyar that you have shortly dipped in the coffee mix. Top with a layer of mascarpone mix. Repeat one or two times until you have used all the mascarpone. To finish, dust with:</td></tr><tr><td class=ingr-quant></td><td><span class=ingr-name>cocoa powder</span></td></tr></table></div><p>Cool in the fridge 2 hours.</p></div><div class=clear><div class=nutfacts>Nutritional facts per serving (daily value): Calories 777.477kcal; Protein 21.093g (42%); Total Fat 53.749g (83%)(Sat. 27.724g (139%)); Chol. 491.55mg (164%); Carb. 54.119g (18%); Fiber 0.55g (2%); Sugars 19.962g; Calcium 183.507mg (18%); Iron 3.546mg (20%); Sodium 626.007mg (26%); Vit. C 0mg (0%); Vit. A 2322.05IU (46%); Trans fat 0.024g</div></div></div></div><div class=footer></div><div class=snc-footer><a href=http://www.shopncook.com>Generated with Shop'NCook Pro 4.0</a></div></div></body></html>"#
        }

        pub fn scx<'a>() -> &'a str {
            todo!()
        }

        pub fn txt1<'a>() -> &'a str {
            todo!()
        }

        pub fn txt2<'a>() -> &'a str {
            todo!()
        }
    }

    mod results {
        use schema_org::field::{
            RecipeRecipeIngredientFieldEnum, RecipeRecipeInstructionsFieldEnum,
        };

        use super::*;

        pub fn all_recipes() -> Vec<Recipe> {
            vec![
                Recipe {
                    name: vec!["California rolls".into()],
                    image: vec![RecipeImageFieldEnum::URL("img2.jpg".into())],
                    recipe_category: vec!["Japanese".into()],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("Rice".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Main dishes".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Appetizers".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Shop'NCook".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("6 servings of 4".into())],
                    prep_time: vec![DurationOrText::Text("25 minutes".into())],
                    total_time: vec![DurationOrText::Text("40 minutes".into())],
                    is_based_on: to_is_based_on(
                        "An Introduction to Japanese Home Cooking by Mathilde Rufenacht",
                    ),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text("12 cups cooked sushi rice, see separate recipe".into()),
                        RecipeRecipeIngredientFieldEnum::Text("6 sheets nori, seaweed".into()),
                        RecipeRecipeIngredientFieldEnum::new_section("Filling", &[
                            "1 medium cucumber",
                            "1 avocado",
                            "1/2 tbsp lemon juice",
                            "9 imitation crab stick, or cooked snow crab meat",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("Sides", &[
                            "wasabi",
                            "soy sauce",
                            "gari, pickled ginger, see separated recipe",
                        ])
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("Strictly speaking, California rolls are not Japanese - you would be hard pressed to find them in Japan, - but they are a marvelous example how cooking techniques can be deliciously adapted to local ingredients. I give this recipe here as a beacon for all experimental cooks:".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Prepare the sushi rice according to above recipe using 2 cups of Japanese rice.".into()),
                        RecipeRecipeInstructionsFieldEnum::new_section("Preparation of the filling", vec![
                            "Wash and peel the cucumber. Seed it and cut it in thin long strips. Cut the avocado in two, remove the pit and peel, and cut it likewise. Sprinkle a few drops of lemon on it to keep it from darkening. Cut the crab sticks in two lengthwise.",
                        ]),
                        RecipeRecipeInstructionsFieldEnum::new_section("Rolling", vec![
                            "Put a nori sheet in the center of a dry bamboo rolling mat. Take about one sixth of the rice and spread it evenly on the nori leaving one inch at the far end of the nori sheet, in order to have a quarter inch-thick layer of rice on the sheet. To handle the rice, wet your fingers with water mixed with a little bit of vinegar. Dispose one sixth of the cucumber and avocado strips on a two-inch strip in the center of the rice. Arrange the crab sticks on top. Start rolling by taking the end of the rolling mat nearest to you and bringing it over the end of the layer of rice. Press to make a tight roll while pulling back the end of the bamboo mat as neccessary. With a wet towel, push softly on the sides of the roll to arrange the filling. If necessary, wet a little the end of the nori sheet to help closing the roll. Remove the rolling mat. Cut 1 inch-thick slices with a wet knife. Repeat for the other nori sheets.",
                            "Serve with wasabi, soy sauce and gari at the side. The soy sauce is poured in small individual plates and a little bit of wasabi is dissolved in it. Dip the sushi in it as you eat.",
                            "California rolls can be served as appetizer or as main dish. Many variations of filling are possible. Try for example cooked shrimps, raw tuna, shiitake nimono, meat for gyuudon (see separate recipes).",
                        ]),
                    ],
                    nutrition: vec![NutritionInformation {
                        calories: vec![Energy::new("393.952kcal")],
                        protein_content: vec![Mass::new("6.781g")],
                        fat_content: vec![Mass::new("0.575g")],
                        carbohydrate_content: vec![Mass::new("88.699g")],
                        fiber_content: vec![Mass::new("2.854g")],
                        sugar_content: vec![Mass::new("11.97g")],
                        sodium_content: vec![Mass::new("1085.715mg")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        context: at_context(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Recipe {
                    name: vec!["Cooked sushi rice".into()],
                    image: vec![RecipeImageFieldEnum::URL("img3.jpg".into())],
                    recipe_category: vec!["Side dishes".into()],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("Rice".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Japanese".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Shop'NCook".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("12 cups".into())],
                    is_based_on: to_is_based_on(
                        "An Introduction to Japanese Home Cooking by Mathilde Rufenacht",
                    ),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::new_section("Cook", &["3 cups Japanese rice, but use 10% less water than usual."]),
                        RecipeRecipeIngredientFieldEnum::new_section("Add", &[
                            "about 1/2 cup + 2 tbsp sushi vinegar, see separate recipe",
                        ]),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text("This rice is used as base for sushi and sushi rolls.".into()),
                        RecipeRecipeInstructionsFieldEnum::Text("Mix well with a cutting and folding movement.".into()),
                    ],
                    nutrition: vec![NutritionInformation {
                        calories: vec![Energy::new("2311.5kcal")],
                        protein_content: vec![Mass::new("37.795g")],
                        fat_content: vec![Mass::new("3.053g")],
                        carbohydrate_content: vec![Mass::new("519.977g")],
                        fiber_content: vec![Mass::new("15.54g")],
                        sugar_content: vec![Mass::new("66.533g")],
                        sodium_content: vec![Mass::new("6499.183mg")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        context: at_context(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Recipe {
                    name: vec!["Crab or shrimp Louis salad".into()],
                    image: vec![RecipeImageFieldEnum::URL("img4.jpg".into())],
                    recipe_category: vec!["Seafood".into()],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("Salads".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Shop'NCook".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("4 servings".into())],
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::new_section("SALAD", &[
                            "1 lettuce, torn",
                            "2 c. crab or shrimp meat",
                            "2 Tomato, cut in wedges",
                            "2 Egg, cut in wedges",
                            "1 can Asparagus",
                            "1 Avocado, sliced",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("LOUIS DRESSING", &[
                            "1/2 c. refrigerated style French dressing",
                            "1/3 c. bottled chile sauce or catsup",
                            "2 tbsp. mayonnaise",
                            "1/2 tsp. Worcestershire sauce",
                            "1 tbsp. lemon juice",
                            "1/4 tsp. black pepper",
                        ]),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Prepare and chill dressing by mixing all the ingredients. On serving plate arrange salad greens, mount seafood in center and garnish with vegetables. Serve with Louis dressing.".into(),
                        ),
                    ],
                    nutrition: vec![NutritionInformation {
                        calories: vec![Energy::new("418.414kcal")],
                        protein_content: vec![Mass::new("19.484g")],
                        fat_content: vec![Mass::new("30.137g")],
                        cholesterol_content: vec![Mass::new("151.386mg")],
                        carbohydrate_content: vec![Mass::new("21.952g")],
                        fiber_content: vec![Mass::new("6.378g")],
                        sugar_content: vec![Mass::new("12.82g")],
                        sodium_content: vec![Mass::new("1110.429mg")],
                        trans_fat_content: vec![Mass::new("0.018g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        context: at_context(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Recipe {
                    name: vec!["Mabodofu".into()],
                    image: vec![RecipeImageFieldEnum::URL("img5.jpg".into())],
                    recipe_category: vec!["Main dishes".into()],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("Meat".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Chinese".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Tofu".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Shop'NCook".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("2 servings".into())],
                    is_based_on: to_is_based_on(
                        "Otoko no tame no ryouri no ki sou",
                    ),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "400 g tofu".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "200 g ground pork".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 heaped tbsp salad oil".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 - 1 1/2 tsp toban djan, (chili bean paste, can be replaced by crushed chili pepper)".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 tsp finely chopped garlic".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 tsp finely chopped ginger root".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "3 tbsp chopped green onion".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "1 tsp sesame oil".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::new_section("A", &[
                            "2 - 2 1/2 tbsp miso",
                            "1 1/2 tbsp soy sauce",
                            "2 tbsp sake",
                            "1 - 1 1/2 tbsp sugar",
                            "2/3 cup water",
                            "a little gara soup no moto, (Chinese chicken bouillon)",
                        ]),
                        RecipeRecipeIngredientFieldEnum::new_section("Diluted maizena", &[
                            "1 tbsp maizena",
                            "2 tbsp water",
                        ])
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Wrap the tofu in paper towel and heat in microwave oven 3 min. 30 s to remove some of the water. Cool. Cut in 2cm cubes.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Mix ingredients A.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Cook over medium heat in a little oil the toubanjan, add garlic and ginger, without letting them burn.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Over high heat, add the minced meat and separate it with the back of a laddle. When hard to separate, add a little bit of sake.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "When the meat is cooked, add the green onion and cook. Add ingredients A. Mix well over high heat.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "When it starts boiling, add the tofu.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "When it boils again, tilt the skillet and add in two to three times the diluted maizena in the soup.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "To finish, add the sesame oil and serve with Japanese rice.".into(),
                        ),
                    ],
                    nutrition: vec![NutritionInformation {
                        calories: vec![Energy::new("625.154kcal")],
                        protein_content: vec![Mass::new("34.42g")],
                        fat_content: vec![Mass::new("40.761g")],
                        cholesterol_content: vec![Mass::new("72mg")],
                        carbohydrate_content: vec![Mass::new("26.006g")],
                        fiber_content: vec![Mass::new("1.691g")],
                        sugar_content: vec![Mass::new("13.72g")],
                        sodium_content: vec![Mass::new("1584.638mg")],
                        trans_fat_content: vec![Mass::new("0.041g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        context: at_context(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Recipe {
                    name: vec!["Sushi vinegar".into()],
                    image: vec![RecipeImageFieldEnum::URL("img6.jpg".into())],
                    recipe_category: vec!["Sauces & dressings".into()],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("Japanese".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Shop'NCook".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("675 ml".into())],
                    prep_time: vec![DurationOrText::Text("10 min".into())],
                    total_time: vec![DurationOrText::Text("10 min".into())],
                    is_based_on: to_is_based_on(
                        "An Introduction to Japanese Home Cooking by Mathilde Rufenacht",
                    ),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "450 ml rice vinegar".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "300 g sugar".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "75 g salt".into(),
                        ),
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Prepare a bottle in advance to use when making sushi rice.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Mix in a skillet over low to medium heat until the sugar and salt are diluted. Do not boil. Put back in the original vinegar bottle, close the lid and put at once in cold water to cool as fast as possible in order to keep the flavor of the vinegar. To not mix it with usual rice vinegar, donät forget to put a label with \"Sushi vinegar\" and the date on it.".into(),
                        ),
                    ],
                    nutrition: vec![NutritionInformation {
                        calories: vec![Energy::new("1161kcal")],
                        carbohydrate_content: vec![Mass::new("299.94g")],
                        sugar_content: vec![Mass::new("299.4g")],
                        sodium_content: vec![Mass::new("29071.5mg")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        context: at_context(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Recipe {
                    name: vec!["Tiramisu".into()],
                    image: vec![RecipeImageFieldEnum::URL("img7.jpg".into())],
                    recipe_category: vec!["Desserts".into()],
                    keywords: vec![
                        RecipeKeywordsFieldEnum::TextOrURL("Italian".into()),
                        RecipeKeywordsFieldEnum::TextOrURL("Shop'NCook".into()),
                    ],
                    recipe_yield: vec![RecipeYieldFieldEnum::Text("4 servings of about 200 g".into())],
                    prep_time: vec![DurationOrText::Text("15 minutes".into())],
                    total_time: vec![DurationOrText::Text("2 hours".into())],
                    is_based_on: to_is_based_on("Anna"),
                    recipe_ingredient: vec![
                        RecipeRecipeIngredientFieldEnum::Text(
                            "500 g mascarpone".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "5 eggs, (Anna uses 5 yolks and 3 whites)".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "5 tbsp sugar, (Anna uses 3 tbsp)".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::Text(
                            "about 20 ladyfingers".into(),
                        ),
                        RecipeRecipeIngredientFieldEnum::new_section("Prepare", &["1/2 cup strong coffee"]),
                        RecipeRecipeIngredientFieldEnum::new_section("Mix in", &[
                            "cognac or some other alcool, replace by orange juice when preparing for children",
                            "cocoa powder",
                        ])
                    ],
                    recipe_instructions: vec![
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Beat the egg yolks, cream with the sugar. Mix well with the mascarpone. Whip the egg whites until stiff (but not dry). Incorporate delicately to the mascarpone mass.".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Line the serving dish with a layer of savoyar that you have shortly dipped in the coffee mix. Top with a layer of mascarpone mix. Repeat one or two times until you have used all the mascarpone. To finish, dust with:".into(),
                        ),
                        RecipeRecipeInstructionsFieldEnum::Text(
                            "Cool in the fridge 2 hours.".into(),
                        ),
                    ],
                    nutrition: vec![NutritionInformation {
                        calories: vec![Energy::new("777.477kcal")],
                        protein_content: vec![Mass::new("21.093g")],
                        fat_content: vec![Mass::new("53.749g")],
                        cholesterol_content: vec![Mass::new("491.55mg")],
                        carbohydrate_content: vec![Mass::new("54.119g")],
                        fiber_content: vec![Mass::new("0.55g")],
                        sugar_content: vec![Mass::new("19.962g")],
                        sodium_content: vec![Mass::new("626.007mg")],
                        trans_fat_content: vec![Mass::new("0.024g")],
                        r#type: AtType::NutritionInformation.to_opt(),
                        context: at_context(),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ]
        }

        pub fn html() -> Vec<Recipe> {
            todo!()
        }

        pub fn scx() -> Vec<Recipe> {
            todo!()
        }

        pub fn txt1() -> Vec<Recipe> {
            todo!()
        }

        pub fn txt2() -> Vec<Recipe> {
            todo!()
        }
    }
}
