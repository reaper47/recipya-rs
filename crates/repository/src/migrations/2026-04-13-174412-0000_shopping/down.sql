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

ALTER TABLE user_settings
DROP COLUMN IF EXISTS paper_size_id;

DROP TABLE IF EXISTS paper_sizes;

DROP TABLE IF EXISTS paper_categories;

DROP TABLE IF EXISTS shares_shopping_lists;

DROP TABLE IF EXISTS shopping_list_recipes;

DROP TABLE IF EXISTS shopping_list_items;

DROP TABLE IF EXISTS users_shopping_list_labels;

DROP TABLE IF EXISTS shopping_list_labels;

DROP TABLE IF EXISTS shopping_lists;
