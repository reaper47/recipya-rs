#!/bin/sh
set -e

echo "shared_preload_libraries = 'pg_cron'" >> "$PGDATA/postgresql.conf"
echo "cron.database_name = 'recipya'" >> "$PGDATA/postgresql.conf"
