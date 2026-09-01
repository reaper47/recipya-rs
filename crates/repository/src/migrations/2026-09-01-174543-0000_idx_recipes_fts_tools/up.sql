CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_fts_tools ON recipes USING gin (fts_tools);
