CREATE UNIQUE INDEX CONCURRENTLY IF NOT EXISTS idx_shopping_list_labels_name ON shopping_list_labels (lower(name));
