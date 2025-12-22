// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    additional_images_recipe (id) {
        id -> Int8,
        recipe_id -> Int8,
        image -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    app (id) {
        id -> Int8,
        is_update_available -> Nullable<Bool>,
        updated_at -> Nullable<Timestamp>,
        update_last_checked_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    auth_tokens (id) {
        id -> Int8,
        #[max_length = 12]
        selector -> Nullable<Bpchar>,
        #[max_length = 64]
        hash_validator -> Nullable<Bpchar>,
        expires -> Nullable<Timestamp>,
        user_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    categories (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    categories_recipes (category_id, recipe_id) {
        category_id -> Int8,
        recipe_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    cookbooks (id) {
        id -> Int8,
        title -> Text,
        image -> Nullable<Uuid>,
        count -> Nullable<Int4>,
        user_id -> Nullable<Int8>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    cookbooks_recipes (id) {
        id -> Int8,
        cookbook_id -> Nullable<Int8>,
        recipe_id -> Nullable<Int8>,
        order_index -> Int2,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    counts (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        recipes -> Nullable<Int4>,
        cookbooks -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    cuisines (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    cuisines_recipes (cuisine_id, recipe_id) {
        cuisine_id -> Int8,
        recipe_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    fdc_food_portions (id) {
        id -> Int8,
        value -> Float8,
        measure_unit_id -> Int8,
        modifier -> Nullable<Text>,
        gram_weight -> Float8,
        amount -> Float8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    fdc_food_portions_fdc_foods (id) {
        id -> Int8,
        food_id -> Int8,
        portion_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    fdc_foods (id) {
        id -> Int8,
        food_class -> Text,
        description -> Text,
        food_category -> Text,
        fdc_id -> Int8,
        description_tsv -> Nullable<Tsvector>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    fdc_foods_fdc_nutrients (id) {
        id -> Int8,
        food_id -> Int8,
        nutrient_id -> Int8,
        median -> Nullable<Float8>,
        amount -> Float8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    fdc_nutrients (id) {
        id -> Int8,
        name -> Text,
        unit_name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    ingredients (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    ingredients_recipes (id) {
        id -> Int8,
        ingredient_id -> Int8,
        recipe_id -> Int8,
        section_id -> Int8,
        item_order -> Int2,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    instructions (id) {
        id -> Int8,
        name -> Text,
        duration_seconds -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    instructions_recipes (id) {
        id -> Int8,
        instruction_id -> Int8,
        recipe_id -> Int8,
        section_id -> Int8,
        item_order -> Int2,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    keywords (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    keywords_recipes (keyword_id, recipe_id) {
        keyword_id -> Int8,
        recipe_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    measure_units (id) {
        id -> Int8,
        name -> Text,
        abbreviation -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    measurement_systems (id) {
        id -> Int2,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    nutrition (id) {
        id -> Int8,
        recipe_id -> Int8,
        calories_kcal -> Nullable<Int2>,
        total_carbohydrates -> Nullable<Int2>,
        sugars_g -> Nullable<Int2>,
        protein_g -> Nullable<Int2>,
        total_fat_g -> Nullable<Int2>,
        saturated_fat_g -> Nullable<Int2>,
        unsaturated_fat_g -> Nullable<Int2>,
        cholesterol_mg -> Nullable<Int2>,
        sodium_mg -> Nullable<Int2>,
        fiber_g -> Nullable<Int2>,
        trans_fat_g -> Nullable<Int2>,
        serving_size -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    nutrition_sources (id) {
        id -> Int8,
        name -> Text,
        description -> Text,
        url -> Text,
        country -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    recipe_timelines (id) {
        id -> Int8,
        recipe_id -> Int8,
        user_id -> Int8,
        title -> Text,
        comment -> Nullable<Text>,
        rating -> Nullable<Int2>,
        image -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    recipes (id) {
        id -> Int8,
        name -> Text,
        description -> Nullable<Text>,
        image -> Nullable<Uuid>,
        #[sql_name = "yield"]
        yield_ -> Int2,
        #[max_length = 3]
        language -> Bpchar,
        measurement_system_id -> Int2,
        notes -> Nullable<Text>,
        source -> Text,
        is_favourite -> Bool,
        rating -> Nullable<Int2>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int8,
        fts_combined -> Tsvector,
        fts_category -> Tsvector,
        fts_cuisine -> Tsvector,
        fts_ingredients -> Tsvector,
        fts_instructions -> Tsvector,
        fts_keywords -> Tsvector,
        fts_tools -> Tsvector,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    report_types (id) {
        id -> Int2,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    reports (id) {
        id -> Int8,
        report_type_id -> Int2,
        user_id -> Int8,
        exec_time_ms -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    reports_logs (id) {
        id -> Int8,
        report_id -> Int8,
        title -> Text,
        is_success -> Bool,
        is_warning -> Bool,
        is_error -> Bool,
        error_reason -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    sections (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    shares_cookbooks (id) {
        id -> Int8,
        link -> Text,
        user_id -> Nullable<Int8>,
        cookbook_id -> Nullable<Int8>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    shares_recipes (id) {
        id -> Int8,
        link -> Uuid,
        user_id -> Int8,
        recipe_id -> Int8,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        last_accessed -> Timestamp,
        click_count -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    themes (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    times (id) {
        id -> Int8,
        recipe_id -> Int8,
        prep_seconds -> Int4,
        cook_seconds -> Int4,
        total_seconds -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    tools (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    tools_recipes (id) {
        id -> Int8,
        tool_id -> Int8,
        recipe_id -> Int8,
        quantity -> Int2,
        tool_order -> Int2,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    user_settings (id) {
        id -> Int8,
        user_id -> Int8,
        measurement_system_id -> Int2,
        calculate_nutrition -> Bool,
        convert_automatically -> Bool,
        cookbooks_view -> Int4,
        default_theme -> Int4,
        selected_theme -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    users (id) {
        id -> Int8,
        email -> Text,
        #[max_length = 256]
        password -> Varchar,
        password_salt -> Uuid,
        token_salt -> Uuid,
        is_remember_me -> Bool,
        is_confirmed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    users_categories (user_id, category_id) {
        user_id -> Int8,
        category_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    users_keywords (user_id, keyword_id) {
        user_id -> Int8,
        keyword_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    users_recipes (id) {
        id -> Int8,
        user_id -> Int8,
        recipe_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    videos_recipes (id) {
        id -> Int8,
        video -> Uuid,
        recipe_id -> Int8,
        duration -> Nullable<Interval>,
        content_url -> Nullable<Text>,
        embed_url -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    websites (id) {
        id -> Int8,
        host -> Text,
        url -> Text,
    }
}

diesel::joinable!(additional_images_recipe -> recipes (recipe_id));
diesel::joinable!(auth_tokens -> users (user_id));
diesel::joinable!(categories_recipes -> categories (category_id));
diesel::joinable!(categories_recipes -> recipes (recipe_id));
diesel::joinable!(cookbooks -> users (user_id));
diesel::joinable!(cookbooks_recipes -> cookbooks (cookbook_id));
diesel::joinable!(cookbooks_recipes -> recipes (recipe_id));
diesel::joinable!(counts -> users (user_id));
diesel::joinable!(cuisines_recipes -> cuisines (cuisine_id));
diesel::joinable!(cuisines_recipes -> recipes (recipe_id));
diesel::joinable!(fdc_food_portions -> measure_units (measure_unit_id));
diesel::joinable!(fdc_food_portions_fdc_foods -> fdc_food_portions (portion_id));
diesel::joinable!(fdc_food_portions_fdc_foods -> fdc_foods (food_id));
diesel::joinable!(fdc_foods_fdc_nutrients -> fdc_foods (food_id));
diesel::joinable!(fdc_foods_fdc_nutrients -> fdc_nutrients (nutrient_id));
diesel::joinable!(ingredients_recipes -> ingredients (ingredient_id));
diesel::joinable!(ingredients_recipes -> recipes (recipe_id));
diesel::joinable!(ingredients_recipes -> sections (section_id));
diesel::joinable!(instructions_recipes -> instructions (instruction_id));
diesel::joinable!(instructions_recipes -> recipes (recipe_id));
diesel::joinable!(instructions_recipes -> sections (section_id));
diesel::joinable!(keywords_recipes -> keywords (keyword_id));
diesel::joinable!(keywords_recipes -> recipes (recipe_id));
diesel::joinable!(nutrition -> recipes (recipe_id));
diesel::joinable!(recipe_timelines -> recipes (recipe_id));
diesel::joinable!(recipe_timelines -> users (user_id));
diesel::joinable!(recipes -> measurement_systems (measurement_system_id));
diesel::joinable!(recipes -> users (user_id));
diesel::joinable!(reports -> report_types (report_type_id));
diesel::joinable!(reports -> users (user_id));
diesel::joinable!(reports_logs -> reports (report_id));
diesel::joinable!(shares_cookbooks -> cookbooks (cookbook_id));
diesel::joinable!(shares_cookbooks -> users (user_id));
diesel::joinable!(shares_recipes -> recipes (recipe_id));
diesel::joinable!(shares_recipes -> users (user_id));
diesel::joinable!(times -> recipes (recipe_id));
diesel::joinable!(tools_recipes -> recipes (recipe_id));
diesel::joinable!(tools_recipes -> tools (tool_id));
diesel::joinable!(user_settings -> measurement_systems (measurement_system_id));
diesel::joinable!(user_settings -> themes (selected_theme));
diesel::joinable!(user_settings -> users (user_id));
diesel::joinable!(users_categories -> categories (category_id));
diesel::joinable!(users_categories -> users (user_id));
diesel::joinable!(users_keywords -> keywords (keyword_id));
diesel::joinable!(users_keywords -> users (user_id));
diesel::joinable!(users_recipes -> recipes (recipe_id));
diesel::joinable!(users_recipes -> users (user_id));
diesel::joinable!(videos_recipes -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    additional_images_recipe,
    app,
    auth_tokens,
    categories,
    categories_recipes,
    cookbooks,
    cookbooks_recipes,
    counts,
    cuisines,
    cuisines_recipes,
    fdc_food_portions,
    fdc_food_portions_fdc_foods,
    fdc_foods,
    fdc_foods_fdc_nutrients,
    fdc_nutrients,
    ingredients,
    ingredients_recipes,
    instructions,
    instructions_recipes,
    keywords,
    keywords_recipes,
    measure_units,
    measurement_systems,
    nutrition,
    nutrition_sources,
    recipe_timelines,
    recipes,
    report_types,
    reports,
    reports_logs,
    sections,
    shares_cookbooks,
    shares_recipes,
    themes,
    times,
    tools,
    tools_recipes,
    user_settings,
    users,
    users_categories,
    users_keywords,
    users_recipes,
    videos_recipes,
    websites,
);
