CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_keywords_name_trgm ON keywords USING gin (name gin_trgm_ops);
