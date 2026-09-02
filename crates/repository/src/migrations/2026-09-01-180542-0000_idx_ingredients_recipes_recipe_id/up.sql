CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_ingredients_recipes_recipe_id_order
ON ingredients_recipes (recipe_id, section_order, item_order);
