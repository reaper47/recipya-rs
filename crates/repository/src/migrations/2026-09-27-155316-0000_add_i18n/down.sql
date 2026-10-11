DROP TRIGGER IF EXISTS set_default_user_language_id_before_insert ON user_settings;

DROP FUNCTION IF EXISTS set_default_language_id();

-- safety-assured:start
ALTER TABLE user_settings DROP COLUMN IF EXISTS language_id;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS languages RESTRICT;
-- safety-assured:end
