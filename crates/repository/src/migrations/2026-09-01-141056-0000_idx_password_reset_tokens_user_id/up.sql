CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_password_reset_tokens_user_id ON password_reset_tokens (user_id);
