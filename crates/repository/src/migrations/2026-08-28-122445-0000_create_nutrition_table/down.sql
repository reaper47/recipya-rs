DROP TRIGGER IF EXISTS update_nutrition_sources_updated_on ON fdc_foods;

DROP FUNCTION IF EXISTS update_nutrition_sources_updated_on_column CASCADE;

-- safety-assured:start
DROP TABLE IF EXISTS fdc_food_portions_fdc_foods RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS fdc_food_portions RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS fdc_foods_fdc_nutrients RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS fdc_nutrients RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS fdc_foods RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS nutrition_sources RESTRICT;
-- safety-assured:end

-- safety-assured:start
DROP TABLE IF EXISTS nutrition RESTRICT;
-- safety-assured:end
