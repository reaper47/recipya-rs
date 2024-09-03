// @generated automatically by Diesel CLI.

diesel::table! {
    additional_images_recipe (id) {
        id -> Int8,
        recipe_id -> Int8,
        image -> Uuid,
    }
}

diesel::table! {
    app (id) {
        id -> Int8,
        is_update_available -> Nullable<Bool>,
        updated_at -> Nullable<Timestamp>,
        update_last_checked_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
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
    categories (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    categories_recipes (id) {
        id -> Int8,
        category_id -> Nullable<Int8>,
        recipe_id -> Int8,
    }
}

diesel::table! {
    cookbooks (id) {
        id -> Int8,
        title -> Text,
        image -> Nullable<Uuid>,
        count -> Nullable<Int4>,
        user_id -> Nullable<Int8>,
    }
}

diesel::table! {
    cookbooks_recipes (id) {
        id -> Int8,
        cookbook_id -> Nullable<Int8>,
        recipe_id -> Nullable<Int8>,
        order_index -> Int2,
    }
}

diesel::table! {
    counts (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        recipes -> Nullable<Int4>,
        cookbooks -> Nullable<Int4>,
    }
}

diesel::table! {
    cuisines (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    cuisines_recipes (id) {
        id -> Int8,
        cuisine_id -> Nullable<Int8>,
        recipe_id -> Int8,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    ingredients_recipes (id) {
        id -> Int8,
        ingredient_id -> Int8,
        recipe_id -> Int8,
        section_id -> Nullable<Int8>,
        item_order -> Int2,
    }
}

diesel::table! {
    instructions (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    instructions_recipes (id) {
        id -> Int8,
        instruction_id -> Int8,
        recipe_id -> Int8,
        section_id -> Nullable<Int8>,
        item_order -> Int2,
    }
}

diesel::table! {
    keywords (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    keywords_recipes (id) {
        id -> Int8,
        keyword_id -> Int8,
        recipe_id -> Int8,
    }
}

diesel::table! {
    measurement_systems (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    nutrition (id) {
        id -> Int8,
        recipe_id -> Nullable<Int8>,
        calories -> Nullable<Int2>,
        total_carbohydrates -> Nullable<Int2>,
        sugars -> Nullable<Int2>,
        protein -> Nullable<Int2>,
        total_fat -> Nullable<Int2>,
        saturated_fat -> Nullable<Int2>,
        unsaturated_fat -> Nullable<Int2>,
        cholesterol -> Nullable<Int2>,
        sodium -> Nullable<Int2>,
        fiber -> Nullable<Int2>,
        trans_fat -> Nullable<Int2>,
        serving_size -> Nullable<Text>,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int8,
        name -> Text,
        description -> Nullable<Text>,
        image -> Nullable<Uuid>,
        #[sql_name = "yield"]
        yield_ -> Int2,
        #[max_length = 3]
        language -> Bpchar,
        source -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int8,
    }
}

diesel::table! {
    report_types (id) {
        id -> Int2,
        name -> Text,
    }
}

diesel::table! {
    reports (id) {
        id -> Int8,
        report_type -> Int2,
        user_id -> Int8,
        exec_time_ms -> Int8,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    reports_logs (id) {
        id -> Int8,
        report_id -> Int8,
        title -> Text,
        is_success -> Bool,
        error_reason -> Text,
    }
}

diesel::table! {
    sections (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    shares_cookbooks (id) {
        id -> Int8,
        link -> Text,
        user_id -> Nullable<Int8>,
        cookbook_id -> Nullable<Int8>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    shares_recipes (id) {
        id -> Int8,
        link -> Text,
        user_id -> Nullable<Int8>,
        recipe_id -> Nullable<Int8>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    times (id) {
        id -> Int8,
        prep_seconds -> Nullable<Int4>,
        cook_seconds -> Nullable<Int4>,
        total_seconds -> Nullable<Int4>,
    }
}

diesel::table! {
    times_recipes (id) {
        id -> Int8,
        time_id -> Int8,
        recipe_id -> Int8,
    }
}

diesel::table! {
    tools (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    tools_recipes (id) {
        id -> Int8,
        tool_id -> Int8,
        recipe_id -> Int8,
        quantity -> Nullable<Int2>,
        tool_order -> Int2,
    }
}

diesel::table! {
    user_settings (id) {
        id -> Int8,
        user_id -> Int8,
        measurement_system_id -> Nullable<Int8>,
        calculate_nutrition -> Bool,
        convert_automatically -> Bool,
        cookbooks_view -> Nullable<Int4>,
    }
}

diesel::table! {
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
    users_categories (id) {
        id -> Int8,
        user_id -> Int8,
        category_id -> Int8,
    }
}

diesel::table! {
    users_recipes (id) {
        id -> Int8,
        user_id -> Int8,
        recipe_id -> Int8,
    }
}

diesel::table! {
    videos_recipes (id) {
        id -> Int4,
        video -> Uuid,
        recipe_id -> Int8,
        duration -> Nullable<Interval>,
        content_url -> Nullable<Text>,
        embed_url -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
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
diesel::joinable!(ingredients_recipes -> ingredients (ingredient_id));
diesel::joinable!(ingredients_recipes -> recipes (recipe_id));
diesel::joinable!(ingredients_recipes -> sections (section_id));
diesel::joinable!(instructions_recipes -> instructions (instruction_id));
diesel::joinable!(instructions_recipes -> recipes (recipe_id));
diesel::joinable!(instructions_recipes -> sections (section_id));
diesel::joinable!(keywords_recipes -> keywords (keyword_id));
diesel::joinable!(keywords_recipes -> recipes (recipe_id));
diesel::joinable!(nutrition -> recipes (recipe_id));
diesel::joinable!(recipes -> users (user_id));
diesel::joinable!(reports -> report_types (report_type));
diesel::joinable!(reports -> users (user_id));
diesel::joinable!(reports_logs -> reports (report_id));
diesel::joinable!(shares_cookbooks -> cookbooks (cookbook_id));
diesel::joinable!(shares_cookbooks -> users (user_id));
diesel::joinable!(shares_recipes -> recipes (recipe_id));
diesel::joinable!(shares_recipes -> users (user_id));
diesel::joinable!(times_recipes -> recipes (recipe_id));
diesel::joinable!(times_recipes -> times (time_id));
diesel::joinable!(tools_recipes -> recipes (recipe_id));
diesel::joinable!(tools_recipes -> tools (tool_id));
diesel::joinable!(user_settings -> measurement_systems (measurement_system_id));
diesel::joinable!(user_settings -> users (user_id));
diesel::joinable!(users_categories -> categories (category_id));
diesel::joinable!(users_categories -> users (user_id));
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
    ingredients,
    ingredients_recipes,
    instructions,
    instructions_recipes,
    keywords,
    keywords_recipes,
    measurement_systems,
    nutrition,
    recipes,
    report_types,
    reports,
    reports_logs,
    sections,
    shares_cookbooks,
    shares_recipes,
    times,
    times_recipes,
    tools,
    tools_recipes,
    user_settings,
    users,
    users_categories,
    users_recipes,
    videos_recipes,
    websites,
);
