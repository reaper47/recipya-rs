ALTER TABLE recipes DROP CONSTRAINT recipes_rating_check;
ALTER TABLE recipe_timelines DROP CONSTRAINT recipe_timelines_rating_check;

ALTER TABLE recipes
ADD CONSTRAINT recipes_rating_check
CHECK (rating IS NULL OR rating BETWEEN 1 AND 5) NOT VALID;

ALTER TABLE recipe_timelines
ADD CONSTRAINT recipe_timelines_rating_check
CHECK (rating IS NULL OR rating BETWEEN 1 AND 5) NOT VALID;
