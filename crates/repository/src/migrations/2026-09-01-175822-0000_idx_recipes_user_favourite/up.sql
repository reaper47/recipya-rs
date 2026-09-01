CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_user_favourite
ON recipes (user_id, is_favourite)
WHERE
  is_favourite = true;
