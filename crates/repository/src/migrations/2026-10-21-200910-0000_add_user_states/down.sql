DROP TRIGGER IF EXISTS update_users_states_updated_at ON user_states;
DROP TRIGGER IF EXISTS add_user_states_after_insert ON users;

DROP FUNCTION IF EXISTS add_user_states ();

-- safety-assured:start
DROP TABLE IF EXISTS user_states RESTRICT;
-- safety-assured:end
