CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_fdc_foods_fdc_nutrients_fdc_id
ON fdc_foods_fdc_nutrients (food_id)
INCLUDE (nutrient_id, amount, min, max);
