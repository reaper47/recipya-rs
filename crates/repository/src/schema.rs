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
        updated_at -> Timestamptz,
        update_last_checked_at -> Timestamptz,
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
        expires -> Nullable<Timestamptz>,
        user_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    bold_indices_ingredients (id) {
        id -> Int8,
        recipe_id -> Int8,
        instruction_id -> Int8,
        start_index -> Int4,
        end_index -> Int4,
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
        user_id -> Nullable<Uuid>,
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
        user_id -> Nullable<Uuid>,
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

    downloads (id) {
        id -> Int8,
        user_id -> Uuid,
        token -> Uuid,
        file_path -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    email_verification_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Text,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    fdc_food_portions (id) {
        id -> Int8,
        value -> Float8,
        modifier -> Text,
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
        amount -> Float8,
        min -> Nullable<Float8>,
        max -> Nullable<Float8>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    fdc_nutrients (id) {
        id -> Int8,
        fdc_id -> Int8,
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
        section_order -> Int2,
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
        section_order -> Int2,
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

    levels (id) {
        id -> Int2,
        name -> Text,
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
        is_precalculated_by_source -> Bool,
        calories_kcal -> Nullable<Int2>,
        total_carbohydrates -> Nullable<Float8>,
        sugars_g -> Nullable<Float8>,
        protein_g -> Nullable<Float8>,
        total_fat_g -> Nullable<Float8>,
        saturated_fat_g -> Nullable<Float8>,
        unsaturated_fat_g -> Nullable<Float8>,
        cholesterol_mg -> Nullable<Float8>,
        sodium_mg -> Nullable<Float8>,
        fiber_g -> Nullable<Float8>,
        trans_fat_g -> Nullable<Float8>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    nutrition_per_100g (id) {
        id -> Int8,
        recipe_id -> Int8,
        nutrition_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    nutrition_per_serving (id) {
        id -> Int8,
        recipe_id -> Int8,
        nutrition_id -> Int8,
        serving_size -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    nutrition_sources (id) {
        id -> Int2,
        name -> Text,
        description -> Text,
        url -> Text,
        country -> Text,
        created_on -> Date,
        updated_on -> Nullable<Date>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    paper_categories (id) {
        id -> Int2,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    paper_sizes (id) {
        id -> Int2,
        paper_category_id -> Int2,
        name -> Text,
        height_mm -> Float8,
        width_mm -> Float8,
        height_in -> Float8,
        width_in -> Float8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    password_reset_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Text,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    recipe_timelines (id) {
        id -> Int8,
        recipe_id -> Int8,
        user_id -> Uuid,
        title -> Text,
        comment -> Nullable<Text>,
        rating -> Nullable<Int2>,
        image -> Nullable<Uuid>,
        created_at -> Timestamptz,
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
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        user_id -> Uuid,
        fts_combined -> Tsvector,
        fts_category -> Tsvector,
        fts_cuisine -> Tsvector,
        fts_ingredients -> Tsvector,
        fts_instructions -> Tsvector,
        fts_keywords -> Tsvector,
        fts_name -> Tsvector,
        fts_tools -> Tsvector,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    refresh_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        is_remember_me -> Bool,
        token -> Text,
        expires_at -> Timestamptz,
        is_used -> Bool,
        used_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    report_types_primary (id) {
        id -> Int2,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    report_types_secondary (id) {
        id -> Int2,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    report_types_tertiary (id) {
        id -> Int2,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    reports (id) {
        id -> Int8,
        report_type_primary_id -> Int2,
        report_type_secondary_id -> Nullable<Int2>,
        report_type_tertiary_id -> Nullable<Int2>,
        items_total -> Int4,
        items_success -> Int4,
        items_skipped -> Int4,
        items_failed -> Int4,
        user_id -> Uuid,
        total_exec_time_ms -> Int8,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    reports_logs (id) {
        id -> Int8,
        seq_num -> Int4,
        report_id -> Int8,
        entity_name -> Text,
        recipe_id -> Nullable<Int8>,
        level_id -> Int2,
        error_code -> Nullable<Text>,
        error_reason -> Nullable<Text>,
        exec_time_ms -> Int8,
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
        user_id -> Nullable<Uuid>,
        cookbook_id -> Nullable<Int8>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    shares_recipes (id) {
        id -> Int8,
        link -> Uuid,
        user_id -> Uuid,
        recipe_id -> Int8,
        created_at -> Timestamptz,
        expires_at -> Timestamp,
        last_accessed -> Timestamptz,
        click_count -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    shares_shopping_lists (id) {
        id -> Int8,
        link -> Uuid,
        user_id -> Uuid,
        list_id -> Uuid,
        created_at -> Timestamptz,
        expires_at -> Timestamp,
        last_accessed -> Timestamptz,
        click_count -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    shopping_list_items (id) {
        id -> Int8,
        shopping_list_id -> Uuid,
        ingredient -> Text,
        quantity -> Nullable<Text>,
        notes -> Nullable<Text>,
        shopping_list_label_id -> Int8,
        position -> Int4,
        is_checked -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    shopping_list_labels (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    shopping_list_recipes (shopping_list_item_id, recipe_id) {
        shopping_list_item_id -> Int8,
        recipe_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    shopping_lists (id) {
        id -> Uuid,
        name -> Text,
        num_items -> Int8,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        user_id -> Uuid,
        measurement_system_id -> Int2,
        nutrition_source_id -> Int2,
        convert_automatically -> Bool,
        cookbooks_view -> Int4,
        default_theme -> Int4,
        selected_theme -> Int4,
        paper_size_id -> Int2,
        timezone -> Text,
        bold_ingredients -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    users (id) {
        id -> Uuid,
        email -> Text,
        #[max_length = 255]
        password_hash -> Varchar,
        password_salt -> Uuid,
        token_salt -> Uuid,
        is_remember_me -> Bool,
        is_email_verified -> Bool,
        is_admin -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    users_categories (user_id, category_id) {
        user_id -> Uuid,
        category_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    users_keywords (user_id, keyword_id) {
        user_id -> Uuid,
        keyword_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    users_recipes (id) {
        id -> Int8,
        user_id -> Uuid,
        recipe_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector as Tsvector;

    users_shopping_list_labels (user_id, label_id) {
        user_id -> Uuid,
        label_id -> Int8,
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
        created_at -> Timestamptz,
    }
}

diesel::joinable!(additional_images_recipe -> recipes (recipe_id));
diesel::joinable!(auth_tokens -> users (user_id));
diesel::joinable!(bold_indices_ingredients -> instructions (instruction_id));
diesel::joinable!(bold_indices_ingredients -> recipes (recipe_id));
diesel::joinable!(categories_recipes -> categories (category_id));
diesel::joinable!(categories_recipes -> recipes (recipe_id));
diesel::joinable!(cookbooks -> users (user_id));
diesel::joinable!(cookbooks_recipes -> cookbooks (cookbook_id));
diesel::joinable!(cookbooks_recipes -> recipes (recipe_id));
diesel::joinable!(counts -> users (user_id));
diesel::joinable!(cuisines_recipes -> cuisines (cuisine_id));
diesel::joinable!(cuisines_recipes -> recipes (recipe_id));
diesel::joinable!(downloads -> users (user_id));
diesel::joinable!(email_verification_tokens -> users (user_id));
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
diesel::joinable!(nutrition_per_100g -> nutrition (nutrition_id));
diesel::joinable!(nutrition_per_100g -> recipes (recipe_id));
diesel::joinable!(nutrition_per_serving -> nutrition (nutrition_id));
diesel::joinable!(nutrition_per_serving -> recipes (recipe_id));
diesel::joinable!(paper_sizes -> paper_categories (paper_category_id));
diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(recipe_timelines -> recipes (recipe_id));
diesel::joinable!(recipe_timelines -> users (user_id));
diesel::joinable!(recipes -> measurement_systems (measurement_system_id));
diesel::joinable!(recipes -> users (user_id));
diesel::joinable!(refresh_tokens -> users (user_id));
diesel::joinable!(reports -> report_types_primary (report_type_primary_id));
diesel::joinable!(reports -> report_types_secondary (report_type_secondary_id));
diesel::joinable!(reports -> report_types_tertiary (report_type_tertiary_id));
diesel::joinable!(reports -> users (user_id));
diesel::joinable!(reports_logs -> levels (level_id));
diesel::joinable!(reports_logs -> recipes (recipe_id));
diesel::joinable!(reports_logs -> reports (report_id));
diesel::joinable!(shares_cookbooks -> cookbooks (cookbook_id));
diesel::joinable!(shares_cookbooks -> users (user_id));
diesel::joinable!(shares_recipes -> recipes (recipe_id));
diesel::joinable!(shares_recipes -> users (user_id));
diesel::joinable!(shares_shopping_lists -> shopping_lists (list_id));
diesel::joinable!(shares_shopping_lists -> users (user_id));
diesel::joinable!(shopping_list_items -> shopping_list_labels (shopping_list_label_id));
diesel::joinable!(shopping_list_items -> shopping_lists (shopping_list_id));
diesel::joinable!(shopping_list_recipes -> recipes (recipe_id));
diesel::joinable!(shopping_list_recipes -> shopping_list_items (shopping_list_item_id));
diesel::joinable!(shopping_lists -> users (user_id));
diesel::joinable!(times -> recipes (recipe_id));
diesel::joinable!(tools_recipes -> recipes (recipe_id));
diesel::joinable!(tools_recipes -> tools (tool_id));
diesel::joinable!(user_settings -> measurement_systems (measurement_system_id));
diesel::joinable!(user_settings -> nutrition_sources (nutrition_source_id));
diesel::joinable!(user_settings -> themes (selected_theme));
diesel::joinable!(user_settings -> users (user_id));
diesel::joinable!(users_categories -> categories (category_id));
diesel::joinable!(users_categories -> users (user_id));
diesel::joinable!(users_keywords -> keywords (keyword_id));
diesel::joinable!(users_keywords -> users (user_id));
diesel::joinable!(users_recipes -> recipes (recipe_id));
diesel::joinable!(users_recipes -> users (user_id));
diesel::joinable!(users_shopping_list_labels -> shopping_list_labels (label_id));
diesel::joinable!(users_shopping_list_labels -> users (user_id));
diesel::joinable!(videos_recipes -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    additional_images_recipe,
    app,
    auth_tokens,
    bold_indices_ingredients,
    categories,
    categories_recipes,
    cookbooks,
    cookbooks_recipes,
    counts,
    cuisines,
    cuisines_recipes,
    downloads,
    email_verification_tokens,
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
    levels,
    measurement_systems,
    nutrition,
    nutrition_per_100g,
    nutrition_per_serving,
    nutrition_sources,
    paper_categories,
    paper_sizes,
    password_reset_tokens,
    recipe_timelines,
    recipes,
    refresh_tokens,
    report_types_primary,
    report_types_secondary,
    report_types_tertiary,
    reports,
    reports_logs,
    sections,
    shares_cookbooks,
    shares_recipes,
    shares_shopping_lists,
    shopping_list_items,
    shopping_list_labels,
    shopping_list_recipes,
    shopping_lists,
    themes,
    times,
    tools,
    tools_recipes,
    user_settings,
    users,
    users_categories,
    users_keywords,
    users_recipes,
    users_shopping_list_labels,
    videos_recipes,
);
