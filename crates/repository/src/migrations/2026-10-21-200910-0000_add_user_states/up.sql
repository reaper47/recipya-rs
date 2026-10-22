CREATE TABLE IF NOT EXISTS user_states (
    user_id uuid PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    is_importing_app bool NOT NULL DEFAULT false,
    is_importing_api bool NOT NULL DEFAULT false,
    updated_at timestamptz NOT NULL DEFAULT now()
);

---
--- Functions
---
CREATE FUNCTION add_user_states () RETURNS TRIGGER AS $$
BEGIN
  INSERT INTO user_states (user_id) values (NEW.id);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

---
--- Triggers
---
CREATE TRIGGER update_user_states_updated_at
BEFORE UPDATE ON user_states FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column ();

CREATE TRIGGER add_user_states_after_insert
AFTER INSERT ON users FOR EACH ROW
EXECUTE FUNCTION add_user_states();

---
--- Inserts
---
INSERT INTO user_states (user_id)
SELECT id FROM users;
