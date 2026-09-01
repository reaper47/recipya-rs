CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_fts_combined ON recipes USING gin (fts_combined);
