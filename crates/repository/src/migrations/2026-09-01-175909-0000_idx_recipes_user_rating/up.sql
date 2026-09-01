CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_recipes_user_rating
ON recipes (user_id, rating)
WHERE
  rating IS NOT NULL;;
