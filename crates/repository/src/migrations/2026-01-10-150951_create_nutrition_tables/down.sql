DROP TRIGGER IF EXISTS update_nutrition_sources_updated_on ON fdc_foods;

DROP TABLE IF EXISTS fdc_food_portions_fdc_foods;

DROP TABLE IF EXISTS fdc_food_portions;

DROP TABLE IF EXISTS fdc_foods_fdc_nutrients;

DROP TABLE IF EXISTS fdc_nutrients;

DROP TABLE IF EXISTS fdc_foods;

DROP TABLE IF EXISTS nutrition_sources;

DROP TABLE IF EXISTS nutrition;

DROP FUNCTION IF EXISTS update_nutrition_sources_updated_on_column CASCADE;

