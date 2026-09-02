CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_fts_ingredients ON recipes USING gin (fts_ingredients);
