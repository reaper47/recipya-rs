CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_user_id_name ON recipes (user_id, name);
