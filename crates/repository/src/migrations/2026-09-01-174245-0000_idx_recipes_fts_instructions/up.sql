CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_fts_instructions ON recipes USING gin (fts_instructions);
