DROP TRIGGER IF EXISTS increment_cookbook_count ON cookbooks_recipes;

DROP TRIGGER IF EXISTS increment_counts_cookbook ON cookbooks;

DROP TRIGGER IF EXISTS decrement_counts_cookbook ON cookbooks;

DROP TRIGGER IF EXISTS decrement_cookbook_count ON cookbooks_recipes;

DROP FUNCTION IF EXISTS decrement_cookbooks_count_column;

DROP FUNCTION IF EXISTS decrement_counts_cookbook_column;

DROP FUNCTION IF EXISTS increment_counts_cookbook_column;

DROP FUNCTION IF EXISTS increment_cookbooks_count_column;

DROP TABLE IF EXISTS shares_cookbooks;

DROP TABLE IF EXISTS cookbooks_recipes;

DROP TABLE IF EXISTS cookbooks;

