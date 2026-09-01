CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_name_trgm ON recipes USING gin (name gin_trgm_ops);
