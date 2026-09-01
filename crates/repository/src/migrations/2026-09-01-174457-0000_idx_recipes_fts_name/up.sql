CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_fts_name ON recipes USING gin (fts_name);
