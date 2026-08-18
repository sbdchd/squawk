SELECT * FROM recipe WHERE tag NOT IN ('breakfast', 'snack', 'appetizer');
SELECT * FROM recipe WHERE tag <> ALL (ARRAY['breakfast', 'snack', 'appetizer']);
SELECT * FROM recipe WHERE NOT (tag && ARRAY['breakfast', 'snack', 'appetizer']);
SELECT ARRAY[1] IN (ARRAY[1]);
SELECT tag IN ('breakfast') FROM recipe;
SELECT tag IN (SELECT tag FROM recipe) FROM recipe;
