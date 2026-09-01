CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_fts_category ON recipes USING gin (fts_category);
