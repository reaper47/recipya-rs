DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'pg_cron') THEN
        IF EXISTS (SELECT 1 FROM cron.job WHERE jobname = 'delete_expired_links') THEN
            PERFORM cron.unschedule('delete_expired_links');
        END IF;
    END IF;
END;
$$;

DROP TRIGGER IF EXISTS insert_user_init_data ON users;

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

DROP FUNCTION IF EXISTS init_user_data_columns;

DROP FUNCTION IF EXISTS delete_orphaned_nutrition;

DROP FUNCTION IF EXISTS update_tools_fts_column;

DROP FUNCTION IF EXISTS update_keywords_fts_column;

DROP FUNCTION IF EXISTS update_category_fts_column;

DROP FUNCTION IF EXISTS update_instructions_fts_column;

DROP FUNCTION IF EXISTS update_ingredients_fts_column;

DROP FUNCTION IF EXISTS update_cuisine_fts_column;

DROP FUNCTION IF EXISTS calculate_total_seconds;

-- safety-assured:start
DROP TABLE IF EXISTS videos_recipes RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS users_recipes RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS users_keywords RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS users_categories RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS tools_recipes RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS shares_recipes RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS recipe_timelines RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS keywords_recipes RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS instructions_recipes RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS ingredients_recipes RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS cuisines_recipes RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS categories_recipes RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS bold_indices_ingredients RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS nutrition_per_serving RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS nutrition_per_100g RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS additional_images_recipe RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS times RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS sections RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS tools RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS keywords RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS instructions RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS ingredients RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS cuisines RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS categories RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS user_settings RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS recipes RESTRICT;
--safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS measurement_systems RESTRICT;
--safety-assured:end

DROP FUNCTION IF EXISTS get_tsv_config (char(3)) CASCADE;
