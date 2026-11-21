ALTER TABLE refresh_tokens ALTER COLUMN expires_at SET DEFAULT (now() + interval '7 days');
