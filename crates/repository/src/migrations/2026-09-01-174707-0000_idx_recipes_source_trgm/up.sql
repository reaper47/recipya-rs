CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_source_trgm ON recipes USING gin (source gin_trgm_ops);
