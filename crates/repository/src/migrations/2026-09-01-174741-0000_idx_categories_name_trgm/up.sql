CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_categories_name_trgm ON categories USING gin (name gin_trgm_ops);
