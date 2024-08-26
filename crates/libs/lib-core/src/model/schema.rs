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
    category_recipe (id) {
        id -> Int8,
        category_id -> Nullable<Int8>,
        recipe_id -> Int8,
    }
}

diesel::table! {
    cookbook_recipes (id) {
        id -> Int8,
        cookbook_id -> Nullable<Int8>,
        recipe_id -> Nullable<Int8>,
        order_index -> Int2,
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
    counts (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        recipes -> Nullable<Int4>,
        cookbooks -> Nullable<Int4>,
    }
}

diesel::table! {
    cuisine_recipe (id) {
        id -> Int8,
        cuisine_id -> Nullable<Int8>,
        recipe_id -> Int8,
    }
}

diesel::table! {
    cuisines (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    ingredient_recipe (id) {
        id -> Int8,
        ingredient_id -> Int8,
        recipe_id -> Int8,
        ingredient_order -> Int2,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    instruction_recipe (id) {
        id -> Int8,
        instruction_id -> Int8,
        recipe_id -> Int8,
        instruction_order -> Int2,
    }
}

diesel::table! {
    instructions (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    keyword_recipe (id) {
        id -> Int8,
        keyword_id -> Int8,
        recipe_id -> Int8,
    }
}

diesel::table! {
    keywords (id) {
        id -> Int8,
        name -> Text,
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
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        user_id -> Int8,
    }
}

diesel::table! {
    report_logs (id) {
        id -> Int8,
        report_id -> Int8,
        title -> Text,
        is_success -> Bool,
        error_reason -> Text,
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
    share_cookbooks (id) {
        id -> Int8,
        link -> Text,
        user_id -> Nullable<Int8>,
        cookbook_id -> Nullable<Int8>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    share_recipes (id) {
        id -> Int8,
        link -> Text,
        user_id -> Nullable<Int8>,
        recipe_id -> Nullable<Int8>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    time_recipe (id) {
        id -> Int8,
        time_id -> Int8,
        recipe_id -> Int8,
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
    tool_recipe (id) {
        id -> Int8,
        tool_id -> Int8,
        recipe_id -> Int8,
        quantity -> Nullable<Int2>,
        tool_order -> Int2,
    }
}

diesel::table! {
    tools (id) {
        id -> Int8,
        name -> Text,
    }
}

diesel::table! {
    user_category (id) {
        id -> Int8,
        user_id -> Int8,
        category_id -> Int8,
    }
}

diesel::table! {
    user_recipe (id) {
        id -> Int8,
        user_id -> Int8,
        recipe_id -> Int8,
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
    video_recipe (id) {
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
diesel::joinable!(category_recipe -> categories (category_id));
diesel::joinable!(category_recipe -> recipes (recipe_id));
diesel::joinable!(cookbook_recipes -> cookbooks (cookbook_id));
diesel::joinable!(cookbook_recipes -> recipes (recipe_id));
diesel::joinable!(cookbooks -> users (user_id));
diesel::joinable!(counts -> users (user_id));
diesel::joinable!(cuisine_recipe -> cuisines (cuisine_id));
diesel::joinable!(cuisine_recipe -> recipes (recipe_id));
diesel::joinable!(ingredient_recipe -> ingredients (ingredient_id));
diesel::joinable!(ingredient_recipe -> recipes (recipe_id));
diesel::joinable!(instruction_recipe -> instructions (instruction_id));
diesel::joinable!(instruction_recipe -> recipes (recipe_id));
diesel::joinable!(keyword_recipe -> keywords (keyword_id));
diesel::joinable!(keyword_recipe -> recipes (recipe_id));
diesel::joinable!(nutrition -> recipes (recipe_id));
diesel::joinable!(recipes -> users (user_id));
diesel::joinable!(report_logs -> reports (report_id));
diesel::joinable!(reports -> report_types (report_type));
diesel::joinable!(reports -> users (user_id));
diesel::joinable!(share_cookbooks -> cookbooks (cookbook_id));
diesel::joinable!(share_cookbooks -> users (user_id));
diesel::joinable!(share_recipes -> recipes (recipe_id));
diesel::joinable!(share_recipes -> users (user_id));
diesel::joinable!(time_recipe -> recipes (recipe_id));
diesel::joinable!(time_recipe -> times (time_id));
diesel::joinable!(tool_recipe -> recipes (recipe_id));
diesel::joinable!(tool_recipe -> tools (tool_id));
diesel::joinable!(user_category -> categories (category_id));
diesel::joinable!(user_category -> users (user_id));
diesel::joinable!(user_recipe -> recipes (recipe_id));
diesel::joinable!(user_recipe -> users (user_id));
diesel::joinable!(user_settings -> measurement_systems (measurement_system_id));
diesel::joinable!(user_settings -> users (user_id));
diesel::joinable!(video_recipe -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    additional_images_recipe,
    app,
    auth_tokens,
    categories,
    category_recipe,
    cookbook_recipes,
    cookbooks,
    counts,
    cuisine_recipe,
    cuisines,
    ingredient_recipe,
    ingredients,
    instruction_recipe,
    instructions,
    keyword_recipe,
    keywords,
    measurement_systems,
    nutrition,
    recipes,
    report_logs,
    report_types,
    reports,
    share_cookbooks,
    share_recipes,
    time_recipe,
    times,
    tool_recipe,
    tools,
    user_category,
    user_recipe,
    user_settings,
    users,
    video_recipe,
    websites,
);
