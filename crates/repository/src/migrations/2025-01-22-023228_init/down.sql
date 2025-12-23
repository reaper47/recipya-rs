DROP TRIGGER IF EXISTS trig_cookbooks_ad ON cookbooks;

DROP TRIGGER IF EXISTS trig_cookbooks_ai ON cookbooks;

DROP TRIGGER IF EXISTS trig_cookbooks_recipes_ad ON cookbooks_recipes;

DROP TRIGGER IF EXISTS trig_cookbooks_recipes_ai ON cookbooks_recipes;

DROP TRIGGER IF EXISTS trig_app_update_check_auo ON app;

DROP TRIGGER IF EXISTS trig_users_recipes_ad ON users_recipes;

DROP TRIGGER IF EXISTS trig_users_recipes_ai ON users_recipes;

DROP TRIGGER IF EXISTS trig_times_au ON times;

DROP TRIGGER IF EXISTS trig_recipes_au ON recipes;

DROP TRIGGER IF EXISTS trig_users_ai ON users;

DROP TRIGGER IF EXISTS trig_update_category_fts_ai ON categories_recipes;

DROP TRIGGER IF EXISTS trig_update_cuisine_fts_ai ON cuisines_recipes;

DROP TRIGGER IF EXISTS trig_update_ingredients_fts_ai ON ingredients_recipes;

DROP TRIGGER IF EXISTS trig_update_instructions_fts_ai ON instructions_recipes;

DROP TRIGGER IF EXISTS trig_update_keywords_fts_ai ON keywords_recipes;

DROP TRIGGER IF EXISTS trig_update_tools_fts_ai ON tools_recipes;

DROP TRIGGER IF EXISTS update_nutrition_sources_updated_at ON fdc_foods;

DROP FUNCTION IF EXISTS trig_update_search_vectors;

DROP FUNCTION IF EXISTS get_tsv_config (character) CASCADE;

DROP FUNCTION IF EXISTS update_category_fts_func;

DROP FUNCTION IF EXISTS update_cuisine_fts_func;

DROP FUNCTION IF EXISTS update_ingredients_fts_func;

DROP FUNCTION IF EXISTS update_instructions_fts_func;

DROP FUNCTION IF EXISTS update_keywords_fts_func;

DROP FUNCTION IF EXISTS update_tools_fts_func;

DROP FUNCTION IF EXISTS trig_cookbooks_ad_func;

DROP FUNCTION IF EXISTS trig_cookbooks_ai_func;

DROP FUNCTION IF EXISTS trig_cookbooks_recipes_ad_func;

DROP FUNCTION IF EXISTS trig_cookbooks_recipes_ai_func;

DROP FUNCTION IF EXISTS trig_app_update_check_auo_func;

DROP FUNCTION IF EXISTS trig_users_recipes_ad_func;

DROP FUNCTION IF EXISTS trig_users_recipes_ai_func;

DROP FUNCTION IF EXISTS trig_times_au_func;

DROP FUNCTION IF EXISTS trig_recipes_au_func;

DROP FUNCTION IF EXISTS users_ai_func;

DROP FUNCTION IF EXISTS update_nutrition_sources_updated_at_column;

DROP TABLE IF EXISTS videos_recipes;

DROP TABLE IF EXISTS users_recipes;

DROP TABLE IF EXISTS users_categories;

DROP TABLE IF EXISTS users_keywords;

DROP TABLE IF EXISTS tools_recipes;

DROP TABLE IF EXISTS times_recipes;

DROP TABLE IF EXISTS shares_recipes;

DROP TABLE IF EXISTS shares_cookbooks;

DROP TABLE IF EXISTS recipe_timelines;

DROP TABLE IF EXISTS keywords_recipes;

DROP TABLE IF EXISTS instructions_recipes;

DROP TABLE IF EXISTS ingredients_recipes;

DROP TABLE IF EXISTS cuisines_recipes;

DROP TABLE IF EXISTS cookbooks_recipes;

DROP TABLE IF EXISTS categories_recipes;

DROP TABLE IF EXISTS cookbooks;

DROP TABLE IF EXISTS reports_logs;

DROP TABLE IF EXISTS reports;

DROP TABLE IF EXISTS report_types;

DROP TABLE IF EXISTS tools;

DROP TABLE IF EXISTS times;

DROP TABLE IF EXISTS additional_images_recipe;

DROP TABLE IF EXISTS nutrition;

DROP TABLE IF EXISTS recipes;

DROP TABLE IF EXISTS cuisines;

DROP TABLE IF EXISTS categories;

DROP TABLE IF EXISTS ingredients;

DROP TABLE IF EXISTS instructions;

DROP TABLE IF EXISTS keywords;

DROP TABLE IF EXISTS user_settings;

DROP TABLE IF EXISTS counts;

DROP TABLE IF EXISTS auth_tokens;

DROP TABLE IF EXISTS users;

DROP TABLE IF EXISTS sections;

DROP TABLE IF EXISTS websites;

DROP TABLE IF EXISTS measurement_systems;

DROP TABLE IF EXISTS app;

DROP TABLE IF EXISTS themes;

DROP TABLE IF EXISTS fdc_foods_fdc_nutrients;

DROP TABLE IF EXISTS fdc_food_portions_fdc_foods;

DROP TABLE IF EXISTS fdc_food_portions;

DROP TABLE IF EXISTS fdc_foods;

DROP TABLE IF EXISTS fdc_nutrients;

DROP TABLE IF EXISTS measure_units;

DROP TABLE IF EXISTS nutrition_sources;
