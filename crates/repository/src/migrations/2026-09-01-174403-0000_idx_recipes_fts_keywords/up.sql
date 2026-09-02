CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_fts_keywords ON recipes USING gin (fts_keywords);
