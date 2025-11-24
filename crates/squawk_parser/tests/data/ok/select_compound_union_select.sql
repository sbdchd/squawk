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

select * from t union table t;
table t union table t;
(select 1) union (table t);

(select 2) union select a, b, c into t from t2;

values (1), (2) union values (3), (4);

