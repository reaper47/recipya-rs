DELETE FROM languages WHERE locale = 'es-MX';

INSERT INTO
    languages (locale, name, english_name)
VALUES
('es-419', 'Español (Latinoamérica)', 'Spanish (Latin America)');
