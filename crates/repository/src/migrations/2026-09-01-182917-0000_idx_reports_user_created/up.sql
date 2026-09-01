CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_reports_user_created ON reports (user_id, created_at DESC);
