CREATE UNIQUE INDEX CONCURRENTLY IF NOT EXISTS idx_shopping_lists_user_name ON shopping_lists (user_id, lower(name));
