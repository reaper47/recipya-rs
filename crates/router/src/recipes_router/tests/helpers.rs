use axum_test::multipart::{MultipartForm, Part};

use models::recipe::structs::recipe::RecipeForCreate;

#[allow(clippy::too_many_lines)]
pub(super) fn create_form(recipe: &RecipeForCreate) -> MultipartForm {
    let mut form = MultipartForm::new().add_part("title", Part::text(&recipe.name));

    for ingredient in &recipe.ingredients.items_as_text() {
        form = form.add_part("ingredient", Part::text(ingredient));
    }

    for instruction in &recipe.instructions.items_as_text() {
        form = form.add_part("instruction", Part::text(instruction));
    }

    for tool in &recipe.tools {
        form = form.add_part(
            "tool",
            Part::text(format!("{} {}", tool.quantity, tool.name)),
        );
    }

    for keyword in &recipe.keywords {
        form = form.add_part("keyword", Part::text(keyword));
    }

    if let Some(n) = &recipe.r#yield {
        form = form.add_part("yield", Part::text(n.to_string()));
    }

    if let Some(n) = &recipe.rating {
        form = form.add_part("rating", Part::text(n.to_string()));
    }

    if let Some(n) = &recipe.notes {
        form = form.add_part("notes", Part::text(n));
    }

    for image in &recipe.images {
        let file_name = format!("{image}.jpg");
        form = form.add_part(
            "media",
            Part::file_name(Part::text(image.to_string()), file_name),
        );
    }

    for video in &recipe.videos {
        form = form.add_part(
            "media",
            Part::file_name(Part::text(video.video.to_string()), video.video.to_string()),
        );
    }

    if let Some(times) = &recipe.times {
        form = form.add_part("time-prep", Part::text(seconds_to_hms(times.prep_seconds)));
        form = form.add_part("time-cook", Part::text(seconds_to_hms(times.cook_seconds)));
    }

    if let Some(v) = &recipe.category {
        form = form.add_part("category", Part::text(v));
    }

    if let Some(v) = &recipe.cuisine {
        form = form.add_part("cuisine", Part::text(v));
    }

    if let Some(v) = &recipe.description {
        form = form.add_part("description", Part::text(v));
    }

    let source = recipe.source.as_str();
    if !source.is_empty() {
        form = form.add_part("source", Part::text(source));
    }

    if let Some(n) = &recipe.nutrition.per_100g {
        if let Some(v) = n.calories_kcal {
            form = form.add_part("calories-per-100g", Part::text(v.to_string()));
        }
        if let Some(v) = n.total_carbohydrates {
            form = form.add_part("total-carbohydrates-per-100g", Part::text(v.to_string()));
        }
        if let Some(v) = n.sugars_g {
            form = form.add_part("sugars-per-100g", Part::text(v.to_string()));
        }
        if let Some(v) = n.protein_g {
            form = form.add_part("protein-per-100g", Part::text(v.to_string()));
        }
        if let Some(v) = n.total_fat_g {
            form = form.add_part("total-fat-per-100g", Part::text(v.to_string()));
        }
        if let Some(v) = n.saturated_fat_g {
            form = form.add_part("saturated-fat-per-100g", Part::text(v.to_string()));
        }
        if let Some(v) = n.unsaturated_fat_g {
            form = form.add_part("unsaturated-fat-per-100g", Part::text(v.to_string()));
        }
        if let Some(v) = n.cholesterol_mg {
            form = form.add_part("cholesterol-per-100g", Part::text(v.to_string()));
        }
        if let Some(v) = n.sodium_mg {
            form = form.add_part("sodium-per-100g", Part::text(v.to_string()));
        }
        if let Some(v) = n.fiber_g {
            form = form.add_part("fiber-per-100g", Part::text(v.to_string()));
        }
        if let Some(v) = n.trans_fat_g {
            form = form.add_part("trans-fat-per-100g", Part::text(v.to_string()));
        }
    }

    if let Some(n) = &recipe.nutrition.per_serving {
        if let Some(v) = n.nutrition.calories_kcal {
            form = form.add_part("calories-per-serving", Part::text(v.to_string()));
        }
        if let Some(v) = n.nutrition.total_carbohydrates {
            form = form.add_part("total-carbohydrates-per-serving", Part::text(v.to_string()));
        }
        if let Some(v) = n.nutrition.sugars_g {
            form = form.add_part("sugars-per-serving", Part::text(v.to_string()));
        }
        if let Some(v) = n.nutrition.protein_g {
            form = form.add_part("protein-per-serving", Part::text(v.to_string()));
        }
        if let Some(v) = n.nutrition.total_fat_g {
            form = form.add_part("total-fat-per-serving", Part::text(v.to_string()));
        }
        if let Some(v) = n.nutrition.saturated_fat_g {
            form = form.add_part("saturated-fat-per-serving", Part::text(v.to_string()));
        }
        if let Some(v) = n.nutrition.unsaturated_fat_g {
            form = form.add_part("unsaturated-fat-per-serving", Part::text(v.to_string()));
        }
        if let Some(v) = n.nutrition.cholesterol_mg {
            form = form.add_part("cholesterol-per-serving", Part::text(v.to_string()));
        }
        if let Some(v) = n.nutrition.sodium_mg {
            form = form.add_part("sodium-per-serving", Part::text(v.to_string()));
        }
        if let Some(v) = n.nutrition.fiber_g {
            form = form.add_part("fiber-per-serving", Part::text(v.to_string()));
        }
        if let Some(v) = n.nutrition.trans_fat_g {
            form = form.add_part("trans-fat-per-serving", Part::text(v.to_string()));
        }
        if !n.serving_size.is_empty() {
            form = form.add_part("serving-size", Part::text(n.serving_size.clone()));
        }
    }

    form
}

fn seconds_to_hms(seconds: i32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
