DROP TRIGGER IF EXISTS update_app_last_checked_at ON app;

DROP FUNCTION IF EXISTS update_last_checked_at_column;

DROP TABLE IF EXISTS app;

DROP EXTENSION IF EXISTS pg_trgm;

DROP EXTENSION IF EXISTS pg_cron;
