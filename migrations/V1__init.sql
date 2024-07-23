---
--- Other
---
CREATE TABLE app
(
    id                     SERIAL PRIMARY KEY,
    is_update_available    BOOLEAN   DEFAULT FALSE,
    updated_at             TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_last_checked_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE measurement_systems
(
    id   SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE websites
(
    id   SERIAL PRIMARY KEY,
    host TEXT UNIQUE NOT NULL,
    url  TEXT UNIQUE NOT NULL
);

---
--- Users
---
CREATE TABLE users
(
    id              SERIAL PRIMARY KEY,
    email           TEXT NOT NULL UNIQUE CHECK ( LOWER(email) = email ),
    hashed_password TEXT NOT NULL,
    is_confirmed    BOOLEAN   DEFAULT FALSE,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE auth_tokens
(
    id             SERIAL PRIMARY KEY,
    selector       CHAR(12),
    hash_validator CHAR(64),
    expires        TIMESTAMP DEFAULT (CURRENT_TIMESTAMP + INTERVAL '1 month'),
    user_id        INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE counts
(
    id        SERIAL PRIMARY KEY,
    user_id   INTEGER REFERENCES users (id) ON DELETE CASCADE,
    recipes   INTEGER DEFAULT 0,
    cookbooks INTEGER DEFAULT 0
);

CREATE TABLE user_settings
(
    id                    SERIAL PRIMARY KEY,
    user_id               INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    measurement_system_id INTEGER REFERENCES measurement_systems (id) ON DELETE CASCADE DEFAULT 1,
    calculate_nutrition   BOOLEAN NOT NULL                                              DEFAULT FALSE,
    convert_automatically BOOLEAN NOT NULL                                              DEFAULT FALSE,
    cookbooks_view        INTEGER                                                       DEFAULT 0
);

---
--- Recipes
---
CREATE TABLE recipes
(
    id          SERIAL PRIMARY KEY,
    name        TEXT    NOT NULL,
    description TEXT,
    image       UUID,
    yield       SMALLINT  DEFAULT 1,
    url         TEXT      DEFAULT 'Unknown',
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    user_id     INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE categories
(
    id   SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE cuisines
(
    id   SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE ingredients
(
    id   SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE instructions
(
    id   SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE keywords
(
    id   SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE nutrition
(
    id                  SERIAL PRIMARY KEY,
    recipe_id           INTEGER REFERENCES recipes (id) ON DELETE CASCADE,
    calories            SMALLINT DEFAULT 0,
    total_carbohydrates SMALLINT DEFAULT 0,
    sugars              SMALLINT DEFAULT 0,
    protein             SMALLINT DEFAULT 0,
    total_fat           SMALLINT DEFAULT 0,
    saturated_fat       SMALLINT DEFAULT 0,
    unsaturated_fat     SMALLINT DEFAULT 0,
    cholesterol         SMALLINT DEFAULT 0,
    sodium              SMALLINT DEFAULT 0,
    fiber               SMALLINT DEFAULT 0,
    trans_fat           SMALLINT DEFAULT 0,
    serving_size        TEXT
);

CREATE TABLE additional_images_recipe
(
    id        SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    image     UUID    NOT NULL,
    UNIQUE (recipe_id, image)
);

-- CREATE TABLE shadow_last_inserted_recipe
-- (
--     row         INTEGER PRIMARY KEY,
--     id          INTEGER NOT NULL,
--     name        TEXT    NOT NULL,
--     description TEXT,
--     source      TEXT
-- );

CREATE TABLE times
(
    id            SERIAL PRIMARY KEY,
    prep_seconds  INTEGER DEFAULT 0,
    cook_seconds  INTEGER DEFAULT 0,
    total_seconds INTEGER GENERATED ALWAYS AS (prep_seconds + cook_seconds) STORED,
    UNIQUE (prep_seconds, cook_seconds)
);

CREATE TABLE tools
(
    id   SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

---
--- Reports
---

CREATE TABLE report_types
(
    id   SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE reports
(
    id           SERIAL PRIMARY KEY,
    report_type  SMALLINT NOT NULL REFERENCES report_types (id) ON DELETE CASCADE,
    user_id      INTEGER  NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    exec_time_ms INTEGER  NOT NULL,
    created_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE report_logs
(
    id           SERIAL PRIMARY KEY,
    report_id    INTEGER NOT NULL REFERENCES reports (id) ON DELETE CASCADE,
    title        TEXT    NOT NULL,
    is_success   BOOLEAN NOT NULL,
    error_reason TEXT    NOT NULL
);

---
--- Cookbooks
---
CREATE TABLE cookbooks
(
    id      SERIAL PRIMARY KEY,
    title   TEXT NOT NULL,
    image   UUID,
    count   INTEGER DEFAULT 0,
    user_id INTEGER REFERENCES users (id) ON DELETE CASCADE,
    UNIQUE (title, user_id)
);

---
--- Association tables
---

CREATE TABLE category_recipe
(
    id          SERIAL PRIMARY KEY,
    category_id INTEGER DEFAULT 1 REFERENCES categories (id) ON DELETE SET DEFAULT,
    recipe_id   INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    UNIQUE (category_id, recipe_id)
);

CREATE TABLE cookbook_recipes
(
    id          SERIAL PRIMARY KEY,
    cookbook_id INTEGER REFERENCES cookbooks (id) ON DELETE CASCADE,
    recipe_id   INTEGER REFERENCES recipes (id) ON DELETE CASCADE,
    order_index SMALLINT NOT NULL,
    UNIQUE (cookbook_id, recipe_id)
);

CREATE TABLE cuisine_recipe
(
    id         SERIAL PRIMARY KEY,
    cuisine_id INTEGER DEFAULT 1 REFERENCES cuisines (id) ON DELETE SET DEFAULT,
    recipe_id  INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    UNIQUE (cuisine_id, recipe_id)
);

CREATE TABLE ingredient_recipe
(
    id               SERIAL PRIMARY KEY,
    ingredient_id    INTEGER  NOT NULL REFERENCES ingredients (id) ON DELETE CASCADE,
    recipe_id        INTEGER  NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    ingredient_order SMALLINT NOT NULL
);

CREATE TABLE instruction_recipe
(
    id                SERIAL PRIMARY KEY,
    instruction_id    INTEGER  NOT NULL REFERENCES instructions (id) ON DELETE CASCADE,
    recipe_id         INTEGER  NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    instruction_order SMALLINT NOT NULL
);

CREATE TABLE keyword_recipe
(
    id         SERIAL PRIMARY KEY,
    keyword_id INTEGER NOT NULL REFERENCES keywords (id) ON DELETE CASCADE,
    recipe_id  INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    UNIQUE (recipe_id, keyword_id)
);

CREATE TABLE share_cookbooks
(
    id          SERIAL PRIMARY KEY,
    link        TEXT NOT NULL,
    user_id     INTEGER REFERENCES users (id) ON DELETE CASCADE,
    cookbook_id INTEGER REFERENCES cookbooks (id) ON DELETE CASCADE,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (link, cookbook_id)
);

CREATE TABLE share_recipes
(
    id         SERIAL PRIMARY KEY,
    link       TEXT NOT NULL,
    user_id    INTEGER REFERENCES users (id) ON DELETE CASCADE,
    recipe_id  INTEGER REFERENCES recipes (id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (link, recipe_id)
);

CREATE TABLE time_recipe
(
    id        SERIAL PRIMARY KEY,
    time_id   INTEGER NOT NULL REFERENCES times (id) ON DELETE SET NULL,
    recipe_id INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    UNIQUE (recipe_id, time_id)
);

CREATE TABLE tool_recipe
(
    id         SERIAL PRIMARY KEY,
    tool_id    INTEGER  NOT NULL REFERENCES tools (id) ON DELETE CASCADE,
    recipe_id  INTEGER  NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    quantity   SMALLINT DEFAULT 1,
    tool_order SMALLINT NOT NULL,
    UNIQUE (recipe_id, tool_id)
);

CREATE TABLE user_category
(
    id          SERIAL PRIMARY KEY,
    user_id     INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES categories (id) ON DELETE CASCADE,
    UNIQUE (user_id, category_id)
);

CREATE TABLE user_recipe
(
    id        SERIAL PRIMARY KEY,
    user_id   INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    recipe_id INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    UNIQUE (user_id, recipe_id)
);

CREATE TABLE video_recipe
(
    id          INTEGER PRIMARY KEY,
    video       UUID    NOT NULL,
    recipe_id   INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    duration    INTERVAL,
    content_url TEXT,
    embed_url   TEXT,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (video, content_url, embed_url, recipe_id)
);

---
--- Functions
---
CREATE FUNCTION trig_users_ai_func()
    RETURNS TRIGGER
AS
$$
BEGIN
    INSERT INTO counts (user_id)
    VALUES (NEW.id);

    INSERT INTO user_category (user_id, category_id)
    SELECT NEW.id, id
    FROM categories
    WHERE name IN (
                   'uncategorized', 'appetizers', 'bread', 'breakfasts', 'condiments',
                   'dessert', 'lunch', 'main dish', 'salad', 'side dish',
                   'snacks', 'soups', 'stews'
        );
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION trig_recipes_au_func()
    RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE recipes
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION trig_times_au_func()
    RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE times
    SET total_seconds = NEW.prep_seconds + NEW.cook_seconds
    WHERE id = NEW.id;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION trig_user_recipe_ai_func()
    RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE counts
    SET recipes = recipes + 1
    WHERE id = NEW.user_id;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION trig_app_update_check_auo_func()
    RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE app
    SET update_last_checked_at = CURRENT_TIMESTAMP
    WHERE id = 1;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION trig_user_recipe_ad_func()
    RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE counts
    SET recipes = recipes - 1
    WHERE id = OLD.user_id;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION trig_cookbook_recipes_ai_func()
    RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE cookbooks
    SET count = count + 1
    WHERE NEW.cookbook_id = id;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION trig_cookbooks_ai_func()
    RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE counts
    SET cookbooks = cookbooks + 1
    WHERE user_id = NEW.user_id;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION trig_cookbooks_ad_func()
    RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE counts
    SET cookbooks = cookbooks - 1
    WHERE user_id = OLD.user_id;

    --     DELETE
--     FROM cookbooks_fts
--     WHERE id = OLD.id
--       AND user_id = OLD.user_id;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION trig_cookbook_recipes_ad_func()
    RETURNS TRIGGER
AS
$$
BEGIN
    UPDATE cookbooks
    SET count = count - 1
    WHERE OLD.cookbook_id = id;
END;
$$ LANGUAGE plpgsql;

---
--- Triggers
---
CREATE TRIGGER trig_users_ai
    AFTER INSERT
    ON users
    FOR EACH ROW
EXECUTE FUNCTION trig_users_ai_func();

CREATE TRIGGER trig_recipes_au
    AFTER UPDATE
    ON recipes
    FOR EACH ROW
EXECUTE FUNCTION trig_recipes_au_func();

CREATE TRIGGER trig_times_au
    AFTER UPDATE
    ON times
    FOR EACH ROW
EXECUTE FUNCTION trig_times_au_func();

CREATE TRIGGER trig_user_recipe_ai
    AFTER INSERT
    ON user_recipe
    FOR EACH ROW
EXECUTE FUNCTION trig_user_recipe_ai_func();

CREATE TRIGGER trig_user_recipe_ad
    AFTER DELETE
    ON user_recipe
    FOR EACH ROW
EXECUTE FUNCTION trig_user_recipe_ad_func();

CREATE TRIGGER trig_app_update_check_auo
    AFTER UPDATE OF is_update_available
    ON app
    FOR EACH ROW
EXECUTE FUNCTION trig_app_update_check_auo_func();

CREATE TRIGGER trig_cookbook_recipes_ai
    AFTER INSERT
    ON cookbook_recipes
    FOR EACH ROW
EXECUTE FUNCTION trig_cookbook_recipes_ai_func();

CREATE TRIGGER trig_cookbook_recipes_ad
    AFTER DELETE
    ON cookbook_recipes
    FOR EACH ROW
EXECUTE FUNCTION trig_cookbook_recipes_ad_func();

CREATE TRIGGER trig_cookbooks_ai
    AFTER INSERT
    ON cookbooks
    FOR EACH ROW
EXECUTE FUNCTION trig_cookbooks_ai_func();

CREATE TRIGGER trig_cookbooks_ad
    AFTER DELETE
    ON cookbooks
    FOR EACH ROW
EXECUTE FUNCTION trig_cookbooks_ad_func();

-- CREATE TRIGGER trig_shadow_last_inserted_recipe_ai
--     AFTER INSERT
--     ON shadow_last_inserted_recipe
--     FOR EACH ROW
-- BEGIN
-- INSERT INTO recipes_fts (id,
--                          user_id,
--                          name,
--                          description,
--                          category,
--                          cuisine,
--                          ingredients,
--                          instructions,
--                          keywords,
--                          tools,
--                          source)
-- VALUES (NEW.id,
--         (SELECT user_id FROM user_recipe AS ur WHERE ur.recipe_id = NEW.id),
--         NEW.name,
--         NEW.description,
--         (SELECT c.name
--          FROM category_recipe AS cr
--                   JOIN categories AS c ON cr.category_id = c.id
--          WHERE cr.recipe_id = NEW.id),
--         (SELECT c.name
--          FROM cuisine_recipe AS cr
--                   JOIN categories AS c ON cr.cuisine_id = c.id
--          WHERE cr.recipe_id = NEW.id),
--         (SELECT COALESCE((SELECT GROUP_CONCAT(ingredient_name, '<!---->')
--                           FROM (SELECT DISTINCT ingredients.name AS ingredient_name
--                                 FROM ingredient_recipe
--                                          JOIN ingredients ON ingredients.id = ingredient_recipe.ingredient_id
--                                 WHERE ingredient_recipe.recipe_id = NEW.id
--                                 ORDER BY ingredient_order)), '')),
--         (SELECT COALESCE((SELECT GROUP_CONCAT(instruction_name, '<!---->')
--                           FROM (SELECT DISTINCT instructions.name AS instruction_name
--                                 FROM instruction_recipe
--                                          JOIN instructions ON instructions.id = instruction_recipe.instruction_id
--                                 WHERE instruction_recipe.recipe_id = NEW.id
--                                 ORDER BY instruction_order)), '')),
--         (SELECT COALESCE((SELECT GROUP_CONCAT(keyword_name, ',')
--                           FROM (SELECT DISTINCT keywords.name AS keyword_name
--                                 FROM keyword_recipe
--                                          JOIN keywords ON keywords.id = keyword_recipe.keyword_id
--                                 WHERE keyword_recipe.recipe_id = NEW.id)), '')),
--         (SELECT GROUP_CONCAT(name)
--          FROM (SELECT tool_recipe.quantity || ' ' || tools.name AS name
--                FROM tool_recipe
--                         JOIN tools ON tool_recipe.tool_id = tools.id
--                WHERE tool_recipe.recipe_id = NEW.id
--                ORDER BY tool_recipe.tool_order)),
--         NEW.source);
-- END;
--
-- CREATE TRIGGER shadow_last_inserted_recipe_delete
--     AFTER DELETE
--     ON shadow_last_inserted_recipe
--     FOR EACH ROW
-- BEGIN
-- DELETE
-- FROM recipes_fts
-- WHERE id = OLD.id;
-- END;

-- CREATE TRIGGER trig_users_ad
--     AFTER DELETE
--     ON users
--     FOR EACH ROW
-- EXECUTE FUNCTION trig_users_ad_func();
--
-- CREATE FUNCTION trig_users_ad_func()
--     RETURNS TRIGGER
-- AS
-- $$
-- BEGIN
--     DELETE
--     FROM recipes
--     WHERE id IN (SELECT id
--                  FROM recipes_fts AS r
--                  WHERE r.user_id = OLD.id);
--
--     DELETE
--     FROM recipes_fts
--     WHERE user_id = OLD.id;
--
--     DELETE
--     FROM cookbooks
--     WHERE user_id = OLD.id;
--
--     DELETE
--     FROM cookbooks_fts
--     WHERE user_id = OLD.id;
-- END;
-- $$ LANGUAGE plpgsql;

---
--- TODO: Search
---
-- CREATE TRIGGER trig_recipe_ad
--     AFTER DELETE
--     ON recipes
--     FOR EACH ROW
-- EXECUTE FUNCTION trig_recipe_ad_func();
--
-- CREATE FUNCTION trig_users_ai_func()
--     RETURNS TRIGGER
-- AS
-- $$
-- BEGIN
--     DELETE
--     FROM recipes_fts
--     WHERE id = OLD.id;
-- END;
-- $$ LANGUAGE plpgsql;
--
-- CREATE
-- VIRTUAL
-- TABLE cookbooks_fts
-- USING fts5
-- (
--     id,
--     user_id,
--     title
-- );
--
-- CREATE
-- VIRTUAL
-- TABLE recipes_fts
-- USING fts5
-- (
--     id,
--     user_id,
--     name,
--     description,
--     category,
--     cuisine,
--     ingredients,
--     instructions,
--     keywords,
--     tools,
--     source
-- );
--
-- CREATE TRIGGER trig_update_recipe_buo
--     BEFORE UPDATE OF id
--     ON recipes
--     FOR EACH ROW
-- BEGIN
-- UPDATE recipes_fts
-- SET name         = NEW.name,
--     description  = NEW.description,
--     category     = (SELECT c.name
--                     FROM category_recipe AS cr
--                              JOIN categories AS c ON cr.category_id = c.id
--                     WHERE cr.recipe_id = NEW.id),
--     cuisine      = (SELECT c.name
--                     FROM cuisine_recipe AS cr
--                              JOIN cuisines c ON cr.cuisine_id = c.id
--                     WHERE cr.recipe_id = NEW.id),
--     ingredients  = (SELECT COALESCE((SELECT GROUP_CONCAT(ingredient_name, '<!---->')
--                                      FROM (SELECT DISTINCT ingredients.name AS ingredient_name
--                                            FROM ingredient_recipe
--                                                     JOIN ingredients ON ingredients.id = ingredient_recipe.ingredient_id
--                                            WHERE ingredient_recipe.recipe_id = NEW.id
--                                            ORDER BY ingredient_order)), '')),
--     instructions = (SELECT COALESCE((SELECT GROUP_CONCAT(instruction_name, '<!---->')
--                                      FROM (SELECT DISTINCT instructions.name AS instruction_name
--                                            FROM instruction_recipe
--                                                     JOIN instructions ON instructions.id = instruction_recipe.instruction_id
--                                            WHERE instruction_recipe.recipe_id = NEW.id
--                                            ORDER BY instruction_order)), '')),
--     keywords     = (SELECT COALESCE((SELECT GROUP_CONCAT(keyword_name, ',')
--                                      FROM (SELECT DISTINCT keywords.name AS keyword_name
--                                            FROM keyword_recipe
--                                                     JOIN keywords ON keywords.id = keyword_recipe.keyword_id
--                                            WHERE keyword_recipe.recipe_id = NEW.id)), '')),
--     tools        = (SELECT GROUP_CONCAT(name)
--                     FROM (SELECT tool_recipe.quantity || ' ' || tools.name AS name
--                           FROM tool_recipe
--                                    JOIN tools ON tool_recipe.tool_id = tools.id
--                           WHERE tool_recipe.recipe_id = NEW.id
--                           ORDER BY tool_recipe.tool_order)),
--     source       = NEW.url
-- WHERE id = NEW.id;
-- END;

---
--- Inserts
---
INSERT INTO report_types (name)
VALUES ('import');

INSERT INTO app (id)
VALUES (1);

-- INSERT INTO cookbooks_fts (id, user_id, title)
-- VALUES (NEW.id, NEW.user_id, NEW.title);
-- END;

INSERT INTO measurement_systems (name)
VALUES ('imperial'),
       ('metric');

INSERT INTO user_settings (user_id, measurement_system_id)
SELECT id, 1
FROM users;

INSERT INTO categories (name)
VALUES ('uncategorized'),
       ('appetizers'),
       ('bread'),
       ('breakfasts'),
       ('condiments'),
       ('dessert'),
       ('lunch'),
       ('main dish'),
       ('salad'),
       ('side dish'),
       ('snacks'),
       ('soups'),
       ('stews');

INSERT INTO websites (host, url)
VALUES ('15gram.be', 'https://15gram.be/recepten'),
       ('750g.com', 'https://www.750g.com'),
       ('101cookbooks.com', 'https://101cookbooks.com'),
       ('claudia.abril.com', 'https://www.claudia.abril.com.br/receitas'),
       ('aberlehome.com', 'https://aberlehome.com'),
       ('abuelascounter.com', 'https://abuelascounter.com'),
       ('acouplecooks.com', 'https://www.acouplecooks.com'),
       ('addapinch.com', 'https://addapinch.com'),
       ('afghankitchenrecipes.com', 'http://www.afghankitchenrecipes.com'),
       ('ah.nl', 'https://www.ah.nl/allerhande'),
       ('akispetretzikis.com', 'https://akispetretzikis.com'),
       ('allrecipes.com', 'https://www.allrecipes.com'),
       ('altonbrown.com', 'https://altonbrown.com'),
       ('amazingribs.com', 'https://amazingribs.com'),
       ('ambitiouskitchen.com', 'https://www.ambitiouskitchen.com'),
       ('archanaskitchen.com', 'https://www.archanaskitchen.com'),
       ('argiro.gr', 'https://www.argiro.gr'),
       ('arla.se', 'https://www.arla.se/recept'),
       ('atelierdeschefs.fr', 'https://www.atelierdeschefs.fr'),
       ('averiecooks.com', 'https://www.averiecooks.com'),
       ('bakingmischief.com', 'https://bakingmischief.com'),
       ('baking-sense.com', 'https://www.baking-sense.com'),
       ('bbc.co.uk', 'https://www.bbc.co.uk/food/recipes'),
       ('bbcgoodfood.com', 'https://www.bbcgoodfood.com/recipes'),
       ('bergamot.app', 'https://www.bergamot.app'),
       ('bettycrocker.com', 'https://www.bettycrocker.com/recipes'),
       ('biancazapatka', 'https://biancazapatka.com'),
       ('bigoven.com', 'https://www.bigoven.com'),
       ('blueapron.com', 'https://www.blueapron.com/cookbook'),
       ('bluejeanchef.com', 'https://bluejeanchef.com/'),
       ('brianlagerstrom.com', 'https://www.brianlagerstrom.com'),
       ('briceletbaklava.ch', 'https://briceletbaklava.ch'),
       ('bodybuilding.com', 'https://www.bodybuilding.com/recipes'),
       ('bonappetit.com', 'https://www.bonappetit.com'),
       ('bongeats.com', 'https://www.bongeats.com/'),
       ('bowlofdelicious.com', 'https://www.bowlofdelicious.com'),
       ('budgetbytes.com', 'https://www.budgetbytes.com'),
       ('cafedelites.com', 'https://cafedelites.com'),
       ('castironketo.net', 'https://www.castironketo.net'),
       ('cdkitchen.com', 'https://www.cdkitchen.com'),
       ('chefkoch.de', 'https://www.chefkoch.de'),
       ('chefnini.com', 'https://www.chefnini.com'),
       ('chefsavvy.com', 'https://chefsavvy.com'),
       ('closetcooking.com', 'https://www.closetcooking.com'),
       ('comidinhasdochef.com', 'https://comidinhasdochef.com'),
       ('cookeatshare.com', 'https://cookeatshare.com'),
       ('cookieandkate.com', 'https://cookieandkate.com'),
       ('cookpad.com', 'https://cookpad.com'),
       ('cook-talk.com', 'https://cook-talk.com'),
       ('coop.se', 'https://www.coop.se/recept'),
       ('copykat.com', 'https://copykat.com'),
       ('costco.com', 'https://www.costco.com'),
       ('countryliving.com', 'https://www.countryliving.com'),
       ('creativecanning.com', 'https://creativecanning.com'),
       ('cucchiaio.it', 'https://www.cucchiaio.it/'),
       ('cuisineaz.com', 'https://www.cuisineaz.com'),
       ('cybercook.com.br', 'https://cybercook.com.br'),
       ('davidlebovitz.com', 'https://www.davidlebovitz.com'),
       ('delish.com', 'https://www.delish.com'),
       ('ditchthecarbs.com', 'https://www.ditchthecarbs.com'),
       ('domesticate-me.com', 'https://domesticate-me.com'),
       ('downshiftology.com', 'https://downshiftology.com'),
       ('dr.dk', 'https://www.dr.dk/mad/opskrift'),
       ('eatingbirdfood.com', 'https://www.eatingbirdfood.com'),
       ('eatingwell.com', 'https://www.eatingwell.com'),
       ('eatsmarter.com', 'https://eatsmarter.com'),
       ('eatwell101.com', 'https://www.eatwell101.com'),
       ('eatwhattonight', 'https://eatwhattonight.com'),
       ('elavegan.com', 'https://elavegan.com'),
       ('emmikochteinfach.de', 'https://emmikochteinfach.de'),
       ('epicurious.com', 'https://www.epicurious.com'),
       ('errenskitchen.com', 'https://www.errenskitchen.com'),
       ('expressen.se', 'https://www.expressen.se/alltommat/recept'),
       ('farmhousedelivery.com', 'https://recipes.farmhousedelivery.com'),
       ('farmhouseonboone.com', 'https://www.farmhouseonboone.com'),
       ('fattoincasadabenedetta.it', 'https://www.fattoincasadabenedetta.it'),
       ('fifteenspatulas.com', 'https://www.fifteenspatulas.com'),
       ('finedininglovers.com', 'https://www.finedininglovers.com'),
       ('fitmencook.com', 'https://fitmencook.com'),
       ('food.com', 'https://www.food.com'),
       ('food52.com', 'https://food52.com/recipes'),
       ('foodandwine.com', 'https://www.foodandwine.com'),
       ('foodbag.be', 'https://www.foodbag.be/nl/home'),
       ('foodnetwork.co.uk', 'https://foodnetwork.co.uk/recipes'),
       ('foodrepublic.com', 'https://www.foodrepublic.com'),
       ('forksoverknives.com', 'https://www.forksoverknives.com'),
       ('forktospoon.com', 'https://forktospoon.com'),
       ('franzoesischkochen.de', 'https://www.franzoesischkochen.de'),
       ('fredriksfika.allas.se', 'https://fredriksfika.allas.se'),
       ('gesund-aktiv.com', 'https://www.gesund-aktiv.com/rezepte'),
       ('giallozafferano.com', 'https://www.giallozafferano.com'),
       ('gimmesomeoven.com', 'https://www.gimmesomeoven.com'),
       ('globo.com', 'https://receitas.globo.com'),
       ('gonnawantseconds.com', 'https://www.gonnawantseconds.com'),
       ('goodfooddiscoveries.com', 'https://goodfooddiscoveries.com'),
       ('goodhousekeeping.com', 'https://www.goodhousekeeping.com'),
       ('grandfrais.com', 'https://www.grandfrais.com/recettes'),
       ('greatbritishchefs.com', 'https://www.greatbritishchefs.com'),
       ('grimgrains.com', 'https://grimgrains.com'),
       ('grouprecipes.com', 'http://www.grouprecipes.com'),
       ('halfbakedharvest.com', 'https://www.halfbakedharvest.com'),
       ('handletheheat.com', 'https://handletheheat.com'),
       ('hassanchef.com', 'https://www.hassanchef.com'),
       ('headbangerskitchen.com', 'https://headbangerskitchen.com'),
       ('heatherchristo.com', 'https://heatherchristo.com'),
       ('hellofresh.com', 'https://www.hellofresh.com/recipes'),
       ('homechef.com', 'https://www.homechef.com/our-menu'),
       ('hostthetoast.com', 'https://hostthetoast.com'),
       ('ica.se', 'https://www.ica.se/recept'),
       ('im-worthy.com', 'https://im-worthy.com'),
       ('indianhealthyrecipes.com', 'https://www.indianhealthyrecipes.com'),
       ('innit.com', 'https://www.innit.com/meal'),
       ('insanelygoodrecipes.com', 'https://insanelygoodrecipes.com'),
       ('inspiralized.com', 'https://inspiralized.com'),
       ('jamieoliver.com', 'https://www.jamieoliver.com'),
       ('jimcooksfoodgood.com', 'https://jimcooksfoodgood.com'),
       ('joyfoodsunshine.com', 'https://joyfoodsunshine.com'),
       ('juliegoodwin.com.au', 'https://juliegoodwin.com.au'),
       ('justataste.com', 'https://www.justataste.com'),
       ('justbento.com', 'https://justbento.com'),
       ('justonecookbook.com', 'https://www.justonecookbook.com'),
       ('kennymcgovern.com', 'https://kennymcgovern.com'),
       ('kingarthurbaking.com', 'https://www.kingarthurbaking.com'),
       ('kitchenstories.com', 'https://www.kitchenstories.com'),
       ('kochbar.de', 'https://www.kochbar.de/rezept'),
       ('kochbucher.com', 'https://kochbucher.com'),
       ('koket.se', 'https://www.koket.se'),
       ('kptncook.com', 'https://www.kptncook.com'),
       ('kuchnia-domowa.pl', 'https://www.kuchnia-domowa.pl'),
       ('kwestiasmaku.com', 'https://www.kwestiasmaku.com'),
       ('latelierderoxane.com', 'https://www.latelierderoxane.com'),
       ('leanandgreenrecipes.net', 'https://leanandgreenrecipes.net'),
       ('lecker.de', 'https://www.lecker.de'),
       ('lecremedelacrumb.com', 'https://www.lecremedelacrumb.com'),
       ('lifestyleofafoodie.com', 'https://lifestyleofafoodie.com'),
       ('littlespicejar.com', 'https://littlespicejar.com'),
       ('livelytable.com', 'https://livelytable.com'),
       ('livingthegreenlife.com', 'https://livingthegreenlife.com'),
       ('lovingitvegan.com', 'https://lovingitvegan.com'),
       ('maangchi.com', 'https://www.maangchi.com'),
       ('madensverden.dk', 'https://madensverden.dk'),
       ('madsvin.com', 'https://madsvin.com'),
       ('marthastewart.com', 'https://www.marthastewart.com'),
       ('matprat.no', 'https://www.matprat.no'),
       ('meljoulwan.com', 'https://meljoulwan.com'),
       ('melskitchencafe.com', 'https://www.melskitchencafe.com'),
       ('mindmegette.hu', 'https://www.mindmegette.hu'),
       ('minimalistbaker.com', 'https://minimalistbaker.com'),
       ('ministryofcurry.com', 'https://ministryofcurry.com'),
       ('misya.info', 'https://www.misya.info'),
       ('momsdish.com', 'https://momsdish.com'),
       ('momswithcrockpots.com', 'https://momswithcrockpots.com'),
       ('monsieur-cuisine.com', 'https://www.monsieur-cuisine.com'),
       ('motherthyme.com', 'https://www.motherthyme.com'),
       ('moulinex.fr', 'https://www.moulinex.fr/recette'),
       ('mundodereceitasbimby.com.pt', 'https://www.mundodereceitasbimby.com.pt'),
       ('mybakingaddiction.com', 'https://www.mybakingaddiction.com'),
       ('mykitchen101.com', 'https://mykitchen101.com'),
       ('mykitchen101en.com', 'https://mykitchen101en.com'),
       ('myplate.gov', 'https://www.myplate.gov/recipes'),
       ('myrecipes.com', 'https://www.myrecipes.com'),
       ('ninjatestkitchen.eu', 'https://ninjatestkitchen.eu'),
       ('nourishedbynutrition.com', 'https://nourishedbynutrition.com'),
       ('nosalty.hu', 'https://www.nosalty.hu'),
       ('nrk.no', 'https://www.nrk.no/mat'),
       ('number-2-pencil.com', 'https://www.number-2-pencil.com'),
       ('nutritionfacts.org', 'https://nutritionfacts.org/recipes'),
       ('nytimes.com', 'https://cooking.nytimes.com/recipes'),
       ('ohsheglows.com', 'https://ohsheglows.com'),
       ('omnivorescookbook.com', 'https://omnivorescookbook.com'),
       ('onceuponachef.com', 'https://www.onceuponachef.com'),
       ('owen-han.com', 'https://www.owen-han.com'),
       ('paleorunningmomma.com', 'https://www.paleorunningmomma.com'),
       ('panelinha.com.br', 'https://www.panelinha.com.br'),
       ('paninihappy.com', 'https://paninihappy.com'),
       ('persnicketyplates.com', 'https://www.persnicketyplates.com'),
       ('pickuplimes.com', 'https://www.pickuplimes.com'),
       ('pingodoce.pt', 'https://www.pingodoce.pt'),
       ('pinkowlkitchen.com', 'https://pinkowlkitchen.com'),
       ('platingpixels.com', 'https://www.platingpixels.com'),
       ('ploetzblog.de', 'https://www.ploetzblog.de/rezepte'),
       ('plowingthroughlife.com', 'https://plowingthroughlife.com'),
       ('popsugar.co.uk', 'https://www.popsugar.co.uk/food'),
       ('practicalselfreliance.com', 'https://practicalselfreliance.com'),
       ('pressureluckcooking.com', 'https://pressureluckcooking.com'),
       ('primaledgehealth.com', 'https://www.primaledgehealth.com'),
       ('projectgezond.nl', 'https://www.projectgezond.nl'),
       ('przepisy.pl', 'https://www.przepisy.pl'),
       ('purelypope.com', 'https://purelypope.com'),
       ('purplecarrot.com', 'https://www.purplecarrot.com'),
       ('rachlmansfield.com', 'https://rachlmansfield.com'),
       ('rainbowplantlife.com', 'https://rainbowplantlife.com'),
       ('realsimple.com', 'https://www.realsimple.com'),
       ('recettes.qc.ca', 'https://www.recettes.qc.ca'),
       ('receitasnestle.com.br', 'https://www.receitasnestle.com.br'),
       ('recipecommunity.com.au', 'https://www.recipecommunity.com.au'),
       ('reciperunner.com', 'https://reciperunner.com'),
       ('recipetineats.com', 'https://www.recipetineats.com'),
       ('redhousespice.com', 'https://redhousespice.com'),
       ('reishunger.de', 'https://www.reishunger.de'),
       ('rezeptwelt.de', 'https://www.rezeptwelt.de'),
       ('ricetta.it', 'https://ricetta.it'),
       ('rosannapansino.com', 'https://rosannapansino.com/blogs/recipes'),
       ('rutgerbakt.nl', 'https://rutgerbakt.nl/alle-recepten'),
       ('saboresajinomoto.com.br', 'https://www.saboresajinomoto.com.br'),
       ('sallysbakingaddiction.com', 'https://sallysbakingaddiction.com'),
       ('sallys-blog.de', 'https://sallys-blog.de/rezepte'),
       ('saltpepperskillet.com', 'https://saltpepperskillet.com'),
       ('saveur.com', 'https://www.saveur.com'),
       ('seriouseats.com', 'https://www.seriouseats.com'),
       ('simple-veganista.com', 'https://simple-veganista.com'),
       ('simply-cookit.com', 'https://www.simply-cookit.com'),
       ('simplyquinoa.com', 'https://www.simplyquinoa.com'),
       ('simplyrecipes.com', 'https://www.simplyrecipes.com'),
       ('simplywhisked.com', 'https://www.simplywhisked.com'),
       ('skinnytaste.com', 'https://www.skinnytaste.com'),
       ('sobors.hu', 'https://sobors.hu/receptek'),
       ('southerncastiron.com', 'https://southerncastiron.com/category/recipes'),
       ('southernliving.com', 'https://www.southernliving.com'),
       ('spendwithpennies.com', 'https://www.spendwithpennies.com'),
       ('staysnatched.com', 'https://www.staysnatched.com'),
       ('steamykitchen.com', 'https://steamykitchen.com'),
       ('streetkitchen.co', 'https://streetkitchen.co'),
       ('sunbasket.com', 'https://sunbasket.com'),
       ('sundpaabudget.dk', 'https://sundpaabudget.dk'),
       ('sunset.com', 'https://www.sunset.com'),
       ('sweetcsdesigns.com', 'https://sweetcsdesigns.com'),
       ('sweetpeasandsaffron.com', 'https://sweetpeasandsaffron.com'),
       ('tasteofhome.com', 'https://www.tasteofhome.com'),
       ('tastesbetterfromscratch.com', 'https://tastesbetterfromscratch.com'),
       ('tastesoflizzyt.com', 'https://www.tastesoflizzyt.com'),
       ('tasty.co', 'https://tasty.co'),
       ('tastykitchen.com', 'https://tastykitchen.com'),
       ('tesco.com', 'https://realfood.tesco.com'),
       ('theclevercarrot.com', 'https://www.theclevercarrot.com'),
       ('thecookingguy.com', 'https://www.thecookingguy.com'),
       ('theexpertguides.com', 'https://theexpertguides.com'),
       ('thehappyfoodie.co.uk', 'https://thehappyfoodie.co.uk'),
       ('thekitchencommunity.org', 'https://thekitchencommunity.org/'),
       ('thekitchenmagpie.com', 'https://www.thekitchenmagpie.com'),
       ('thekitchn.com', 'https://www.thekitchn.com'),
       ('themagicalslowcooker.com', 'https://www.themagicalslowcooker.com/'),
       ('themodernproper.com', 'https://themodernproper.com'),
       ('thenutritiouskitchen.co', 'https:///thenutritiouskitchen.co'),
       ('thepioneerwoman.com', 'https://www.thepioneerwoman.com'),
       ('therecipecritic.com', 'https://therecipecritic.com'),
       ('thespruceeats.com', 'https://www.thespruceeats.com'),
       ('thevintagemixer.com', 'https://www.thevintagemixer.com'),
       ('thewoksoflife.com', 'https://thewoksoflife.com'),
       ('thinlicious.com', 'https://thinlicious.com/'),
       ('tidymom.net', 'https://tidymom.net'),
       ('timesofindia.com', 'https://recipes.timesofindia.com/recipes'),
       ('tine.no', 'https://www.tine.no/oppskrifter'),
       ('tudogostoso.com', 'https://www.tudogostoso.com.br'),
       ('twopeasandtheirpod.com', 'https://www.twopeasandtheirpod.com'),
       ('uitpaulineskeuken.nl', 'https://uitpaulineskeuken.nl'),
       ('usapears.org', 'https://usapears.org'),
       ('valdemarsro.dk', 'https://www.valdemarsro.dk'),
       ('vanillaandbean.com', 'https://vanillaandbean.com'),
       ('vegetarbloggen.no', 'https://www.vegetarbloggen.no'),
       ('vegolosi.it', 'https://www.vegolosi.it'),
       ('vegrecipesofindia.com', 'https://www.vegrecipesofindia.com'),
       ('waitrose.com', 'https://www.waitrose.com/'),
       ('watchwhatueat.com', 'https://www.watchwhatueat.com'),
       ('wearenotmartha.com', 'https://wearenotmartha.com'),
       ('weightwatchers.com', 'https://www.weightwatchers.com'),
       ('wellplated.com', 'https://www.wellplated.com'),
       ('whatsgabycooking.com', 'https://whatsgabycooking.com'),
       ('wholefoodsmarket.co.uk', 'https://www.wholefoodsmarket.co.uk/'),
       ('wikibooks.org', 'https://en.wikibooks.org'),
       ('m.wikibooks.org', 'https://en.m.wikibooks.org'),
       ('woop.co.nz', 'https://woop.co.nz'),
       ('ye-mek.net', 'https://ye-mek.net'),
       ('zeit.de', 'https://www.zeit.de'),
       ('zenbelly.com', 'https://www.zenbelly.com'),
       ('puurgezond.nl', 'https://www.puurgezond.nl'),
       ('jaimyskitchen.nl', 'https://jaimyskitchen.nl'),
       ('leukerecepten.nl', 'https://www.leukerecepten.nl'),
       ('bettybossi.ch', 'https://www.bettybossi.ch'),
       ('reddit.com', 'https://www.reddit.com'),
       ('marmiton.org', 'https://www.marmiton.org'),
       ('yumelise.fr', 'https://www.yumelise.fr'),
       ('lidl-kochen.de', 'https://www.lidl-kochen.de/rezeptwelt'),
       ('all-clad.com', 'https://www.all-clad.com'),
       ('francescakookt.nl', 'https://www.francescakookt.nl'),
       ('quitoque.fr', 'https://www.quitoque.fr'),
       ('kuchynalidla.sk', 'https://kuchynalidla.sk'),
       ('myjewishlearning.com', 'https://www.myjewishlearning.com'),
       ('drinkoteket.se', 'https://drinkoteket.se'),
       ('24kitchen.nl', 'https://www.24kitchen.nl/recepten'),
       ('ah.be', 'https://www.ah.be'),
       ('aflavorjournal.com', 'https://aflavorjournal.com'),
       ('aldi.com.au', 'https://www.aldi.com.au/recipes'),
       ('alexandracooks.com', 'https://alexandracooks.com'),
       ('alittlebityummy.com', 'https://alittlebityummy.com'),
       ('allthehealthythings.com', 'https://allthehealthythings.com'),
       ('aniagotuje.pl', 'https://aniagotuje.pl'),
       ('americastestkitchen.com', 'https://www.americastestkitchen.com'),
       ('angielaeats.com', 'https://www.angielaeats.com'),
       ('antilliaans-eten.nl', 'https://www.antilliaans-eten.nl'),
       ('avocadoskillet.com', 'https://avocadoskillet.com'),
       ('bakels.com.au', 'https://www.bakels.com.au'),
       ('barefeetinthekitchen.com', 'https://barefeetinthekitchen.com'),
       ('beyondkimchee.com', 'https://beyondkimchee.com'),
       ('bottomlessgreens.com', 'https://bottomlessgreens.com'),
       ('breadtopia.com', 'https://breadtopia.com'),
       ('britishbakels.co.uk', 'https://www.britishbakels.co.uk/recipes'),
       ('chatelaine.com', 'https://chatelaine.com'),
       ('chejorge.com', 'https://chejorge.com'),
       ('chetnamakan.co.uk', 'https://chetnamakan.co.uk'),
       ('chinesecookingdemystified.substack.com', 'https://chinesecookingdemystified.substack.com'),
       ('colruyt.be', 'https://www.colruyt.be'),
       ('culy.nl', 'https://www.culy.nl'),
       ('cuisineandtravel.com', 'https://cuisineandtravel.com'),
       ('daringgourmet.com', 'https://daringgourmet.com'),
       ('dherbs.com', 'https://www.dherbs.com'),
       ('damndelicious.net', 'https://damndelicious.net'),
       ('dinnerthendessert.com', 'https://dinnerthendessert.com'),
       ('dinneratthezoo.com', 'https://www.dinneratthezoo.com'),
       ('dish.co.nz', 'https://dish.co.nz'),
       ('donnahay.com.au', 'https://www.donnahay.com.au'),
       ('dreenaburton.com', 'https://dreenaburton.com'),
       ('elephantasticvegan.com', 'https://elephantasticvegan.com'),
       ('entertainingwithbeth.com', 'https://entertainingwithbeth.com'),
       ('etenvaneefke.nl', 'http://www.etenvaneefke.nl'),
       ('evolvingtable.com', 'https://www.evolvingtable.com'),
       ('familyfoodonthetable', 'https://www.familyfoodonthetable.com'),
       ('feastingathome.com', 'https://feastingathome.com'),
       ('felix.kitchen', 'https://felix.kitchen'),
       ('findingtimeforcooking.com', 'https://findingtimeforcooking.com'),
       ('foodal.com', 'https://foodal.com'),
       ('foodbymaria.com', 'https://foodbymaria.com'),
       ('foodiecrush.com', 'https://foodiecrush.com'),
       ('food-guide.canada.ca', 'https://food-guide.canada.ca/en/recipes'),
       ('foolproofliving.com', 'https://foolproofliving.com'),
       ('gastroplant.com', 'https://gastroplant.com'),
       ('gazoakleychef.com', 'https://www.gazoakleychef.com'),
       ('glutenfreetables.com', 'https://glutenfreetables.com'),
       ('goodeatings.com', 'https://goodeatings.com'),
       ('goodto.com', 'https://goodto.com'),
       ('gourmettraveller.com.au', 'https://www.gourmettraveller.com.au'),
       ('gousto.co.uk', 'https://www.gousto.co.uk'),
       ('greenevi.com', 'https://greenevi.com'),
       ('gurki.no', 'https://gurki.no'),
       ('healthylittlefoodies.com', 'https://www.healthylittlefoodies.com'),
       ('hellofresh.se', 'https://www.hellofresh.se/recipes'),
       ('homebrewanswers.com', 'https://homebrewanswers.com'),
       ('inbloombakery.com', 'https://inbloombakery.com'),
       ('instantpot.com', 'https://instantpot.com'),
       ('jaroflemons.com', 'https://jaroflemons.com'),
       ('jocooks.com', 'https://www.jocooks.com'),
       ('joythebaker.com', 'https://joythebaker.com'),
       ('jumbo.com', 'https://www.jumbo.com/recepten'),
       ('keepinitkind.com', 'https://keepinitkind.com'),
       ('kitchenaid.com', 'https://kitchenaid.com.au'),
       ('kitchensanctuary.com', 'https://www.kitchensanctuary.com'),
       ('kookjij.nl', 'https://www.kookjij.nl'),
       ('kristineskitchenblog.com', 'https://kristineskitchenblog.com'),
       ('lahbco.com', 'https://www.lahbco.com'),
       ('lekkerensimpel.com', 'https://www.lekkerensimpel.com/gougeres'),
       ('lidl.nl', 'https://recepten.lidl.nl/recept'),
       ('lithuanianintheusa.com', 'https://lithuanianintheusa.com'),
       ('loveandlemons.com', 'https://www.loveandlemons.com/'),
       ('madewithlau.com', 'https://www.madewithlau.com'),
       ('mccormick.com', 'https://www.mccormick.com'),
       ('mexicanmademeatless.com', 'https://mexicanmademeatless.com'),
       ('modernhoney.com', 'https://www.modernhoney.com'),
       ('momontimeout.com', 'https://www.momontimeout.com'),
       ('mygingergarlickitchen.com', 'https://www.mygingergarlickitchen.com'),
       ('mykoreankitchen.com', 'https://mykoreankitchen.com'),
       ('natashaskitchen.com', 'https://natashaskitchen.com'),
       ('nigella.com', 'https://nigella.com'),
       ('notenoughcinnamon.com', 'https://www.notenoughcinnamon.com'),
       ('ohmyveggies.com', 'https://ohmyveggies.com'),
       ('okokorecepten.nl', 'https://www.okokorecepten.nl'),
       ('onesweetappetite.com', 'https://onesweetappetite.com'),
       ('parsleyandparm.com', 'https://parsleyandparm.com'),
       ('plentyvegan.com', 'https://plentyvegan.com'),
       ('potatorolls.com', 'https://potatorolls.com'),
       ('purewow.com', 'https://www.purewow.com'),
       ('radiofrance.fr', 'https://www.radiofrance.fr'),
       ('recipegirl.com', 'https://www.recipegirl.com'),
       ('robinasbell.com', 'https://robinasbell.com'),
       ('saltandlavender.com', 'https://www.saltandlavender.com'),
       ('sarahsveganguide.com', 'https://sarahsveganguide.com'),
       ('savorynothings.com', 'https://www.savorynothings.com'),
       ('smittenkitchen.com', 'https://smittenkitchen.com'),
       ('spiceboxtravels.com', 'https://spiceboxtravels.com'),
       ('tasteatlas.com', 'https://www.tasteatlas.com'),
       ('thatvegandad.com', 'https://www.thatvegandad.net'),
       ('thecookierookie.com', 'https://www.thecookierookie.com'),
       ('thefoodflamingo.com', 'https://thefoodflamingo.com'),
       ('theguccha.com', 'https://www.theguccha.com'),
       ('theheartysoul.com', 'https://theheartysoul.com'),
       ('thesaltymarshmallow.com', 'https://thesaltymarshmallow.com'),
       ('twosleevers.com', 'https://twosleevers.com'),
       ('unsophisticook.com', 'https://unsophisticook.com'),
       ('vegan-pratique.fr', 'https://vegan-pratique.fr/recettes/banana-bread/');
