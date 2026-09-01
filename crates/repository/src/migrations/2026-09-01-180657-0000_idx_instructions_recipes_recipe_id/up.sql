CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_instructions_recipes_recipe_id_order
ON instructions_recipes (recipe_id, section_order, item_order);
