DO $$
BEGIN
    IF current_database() = 'recipya' THEN
        PERFORM
            cron.unschedule ('delete_expired_links');
    END IF;
END
$$;

DROP TRIGGER IF EXISTS insert_user_init_data ON users;

DROP TRIGGER IF EXISTS update_recipes_modified ON recipes;

DROP TRIGGER IF EXISTS update_users_recipes_recipe_count ON users_recipes;

DROP TRIGGER IF EXISTS decrement_recipes_count_after_delete ON users_recipes;

DROP TRIGGER IF EXISTS update_category_fts ON categories_recipes;

DROP TRIGGER IF EXISTS update_cuisine_fts ON cuisines_recipes;

DROP TRIGGER IF EXISTS update_ingredients_fts ON ingredients_recipes;

DROP TRIGGER IF EXISTS update_instructions_fts ON instructions_recipes;

DROP TRIGGER IF EXISTS update_keywords_fts ON keywords_recipes;

DROP TRIGGER IF EXISTS update_tools_fts ON tools_recipes;

DROP TRIGGER IF EXISTS cleanup_nutrition_per_100g ON nutrition_per_100g;

DROP TRIGGER IF EXISTS cleanup_nutrition_per_serving ON nutrition_per_serving;

DROP FUNCTION IF EXISTS delete_orphaned_nutrition;

DROP FUNCTION IF EXISTS update_tools_fts_column;

DROP FUNCTION IF EXISTS update_keywords_fts_column;

DROP FUNCTION IF EXISTS update_category_fts_column;

DROP FUNCTION IF EXISTS update_instructions_fts_column;

DROP FUNCTION IF EXISTS update_ingredients_fts_column;

DROP FUNCTION IF EXISTS update_cuisine_fts_column;

DROP FUNCTION IF EXISTS decrement_recipe_count;

DROP FUNCTION IF EXISTS update_recipe_count;

DROP FUNCTION IF EXISTS calculate_total_seconds;

DROP FUNCTION IF EXISTS init_user_data_columns;

DROP TABLE IF EXISTS videos_recipes;

DROP TABLE IF EXISTS users_recipes;

DROP TABLE IF EXISTS users_keywords;

DROP TABLE IF EXISTS users_categories;

DROP TABLE IF EXISTS tools_recipes;

DROP TABLE IF EXISTS shares_recipes;

DROP TABLE IF EXISTS recipe_timelines;

DROP TABLE IF EXISTS keywords_recipes;

DROP TABLE IF EXISTS instructions_recipes;

DROP TABLE IF EXISTS ingredients_recipes;

DROP TABLE IF EXISTS cuisines_recipes;

DROP TABLE IF EXISTS categories_recipes;

DROP TABLE IF EXISTS user_settings;

DROP TABLE IF EXISTS sections;

DROP TABLE IF EXISTS tools;

DROP TABLE IF EXISTS times;

DROP TABLE IF EXISTS additional_images_recipe;

DROP TABLE IF EXISTS nutrition_per_serving;

DROP TABLE IF EXISTS nutrition_per_100g;

DROP TABLE IF EXISTS keywords;

DROP TABLE IF EXISTS instructions;

DROP TABLE IF EXISTS ingredients;

DROP TABLE IF EXISTS cuisines;

DROP TABLE IF EXISTS categories;

DROP TABLE IF EXISTS recipes;

DROP TABLE IF EXISTS measurement_systems;

DROP FUNCTION IF EXISTS get_tsv_config (char(3)) CASCADE;

