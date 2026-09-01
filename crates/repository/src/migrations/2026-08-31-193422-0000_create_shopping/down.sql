DO $$
BEGIN
  IF EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'pg_cron') THEN
    IF EXISTS (SELECT 1 FROM cron.job WHERE jobname = 'delete_expired_shares_shopping_lists') THEN
      PERFORM cron.unschedule('delete_expired_shares_shopping_lists');
    END IF;
  END IF;
END;
$$;

DROP TRIGGER IF EXISTS update_shopping_list_num_items ON shopping_list_items;

DROP TRIGGER IF EXISTS update_shopping_list_items_position ON shopping_list_items;

DROP TRIGGER IF EXISTS update_shopping_list_items_updated_at ON shopping_list_items;

DROP TRIGGER IF EXISTS update_shopping_lists_updated_at ON shopping_lists;

DROP FUNCTION IF EXISTS update_shopping_list_items_position ();

DROP FUNCTION IF EXISTS update_shopping_list_num_items ();

-- safety-assured:start
ALTER TABLE user_settings
DROP COLUMN IF EXISTS paper_size_id;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS paper_sizes RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS paper_categories RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS shares_shopping_lists RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS shopping_list_recipes RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS shopping_list_items RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS users_shopping_list_labels RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS shopping_list_labels RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS shopping_lists RESTRICT;
-- safety-assured:end

DROP FUNCTION IF EXISTS get_default_paper_size_id ();
