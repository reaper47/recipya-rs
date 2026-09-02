DROP TRIGGER IF EXISTS update_users_updated_at ON users;

DROP FUNCTION IF EXISTS update_updated_at_column CASCADE;

-- safety-assured:start
DROP TABLE IF EXISTS refresh_tokens RESTRICT;
-- safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS password_reset_tokens RESTRICT;
-- safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS email_verification_tokens RESTRICT;
-- safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS downloads RESTRICT;
-- safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS users RESTRICT;
-- safety-assured:end
-- safety-assured:start
DROP TABLE IF EXISTS themes RESTRICT;
-- safety-assured:end
