CREATE UNIQUE INDEX CONCURRENTLY IF NOT EXISTS idx_shopping_list_items_ingredient_labeled
ON shopping_list_items (
  shopping_list_id,
  lower(ingredient),
  shopping_list_label_id
);
