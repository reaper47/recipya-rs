CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_fdc_foods_description_fts ON fdc_foods USING gin (description_tsv);
