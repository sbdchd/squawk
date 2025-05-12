SELECT (
    (SELECT id FROM code_categories WHERE "language" = @language::char(4) ORDER BY "id" ASC LIMIT 1)
    UNION
    (SELECT id FROM code_categories WHERE "language" = 'nl-NL' ORDER BY "id" ASC LIMIT 1)
) LIMIT 1;

-- version without parentheses.
SELECT (
    (SELECT id FROM code_categories WHERE "language" = @language::char(4) ORDER BY "id" ASC LIMIT 1)
    UNION
    SELECT id FROM code_categories WHERE "language" = 'nl-NL' ORDER BY "id" ASC LIMIT 1
) LIMIT 1;
