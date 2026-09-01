CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_user_id_source ON recipes (user_id, source);
