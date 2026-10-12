DELETE FROM languages WHERE locale = 'es-419';

INSERT INTO
    languages (locale, name, english_name)
VALUES
    ('es-MX', 'Español (México)', 'Spanish (Mexico)');
