SELECT * FROM recipe WHERE tag IN ARRAY['breakfast', 'snack', 'appetizer'];
SELECT * FROM recipe WHERE tag NOT IN ARRAY['breakfast', 'snack', 'appetizer'];
SELECT * FROM recipe WHERE tag IN 'breakfast';
SELECT * FROM recipe WHERE tag NOT IN $1;
SELECT * FROM recipe WHERE tag IN choices;
SELECT * FROM recipe WHERE tag NOT IN get_choices();
SELECT * FROM recipe WHERE tag IN ROW('breakfast', 'snack');
