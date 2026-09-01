CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_cuisines_name_trgm ON cuisines USING gin (name gin_trgm_ops);
