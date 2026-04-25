DROP TRIGGER IF EXISTS update_shopping_list_num_items ON shopping_list_items;

DROP TRIGGER IF EXISTS update_shopping_list_items_position ON shopping_list_items;

DROP TRIGGER IF EXISTS update_shopping_list_items_updated_at ON shopping_list_items;

DROP TRIGGER IF EXISTS update_shopping_lists_updated_at ON shopping_lists;

DROP FUNCTION IF EXISTS update_shopping_list_items_position ();

DROP FUNCTION IF EXISTS update_shopping_list_num_items ();

DROP TABLE IF EXISTS shopping_list_recipes;

DROP TABLE IF EXISTS shopping_list_items;

DROP TABLE IF EXISTS users_shopping_list_labels;

DROP TABLE IF EXISTS shopping_list_labels;

DROP TABLE IF EXISTS shopping_lists;
