ALTER TABLE user_settings
ADD COLUMN timezone text NOT NULL DEFAULT 'UTC';
