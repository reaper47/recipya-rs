CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_shopping_list_items_list_label_position
ON shopping_list_items (shopping_list_id, shopping_list_label_id, position);
