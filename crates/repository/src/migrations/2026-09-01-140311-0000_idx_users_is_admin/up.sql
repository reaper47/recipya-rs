CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_users_is_admin ON users(id) WHERE is_admin = true;
