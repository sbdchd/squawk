update foo set a = 1;

UPDATE ONLY (foo) AS f SET a = 1, b = DEFAULT FROM bar WHERE f.id = bar.id RETURNING f.*;

UPDATE foo SET (a, b) = ROW (1, DEFAULT), (c, d) = (SELECT x, y FROM bar), payload.field[1][2:3] = 4;

UPDATE a_very_long_schema_name.a_very_long_table_name SET a_very_long_first_column_name = 'a very long replacement value', a_very_long_second_column_name = 'another very long replacement value' WHERE organization_id = 12345 AND status = 'active' RETURNING id, a_very_long_first_column_name;

WITH changed AS (UPDATE foo SET a = 1 WHERE id = 2 RETURNING id) SELECT * FROM changed;

WITH source AS (SELECT id, value FROM incoming) UPDATE foo SET value = source.value FROM source WHERE foo.id = source.id RETURNING foo.id;

/*before*/ UPDATE /*a*/ ONLY /*b*/ (/*c*/ public /*d*/ . /*e*/ foo /*f*/) /*g*/ FOR /*h*/ PORTION /*i*/ OF /*j*/ valid_at /*k*/ FROM /*l*/ 1 /*m*/ TO /*n*/ 2 /*o*/ AS /*p*/ f /*q*/ SET /*r*/ payload /*s*/ . /*t*/ field /*u*/ [/*v*/ 1 /*w*/ : /*x*/ 2 /*y*/] /*z*/ = /*aa*/ 'new' /*ab*/, /*ac*/ (/*ad*/ a /*ae*/, /*af*/ b /*ag*/) /*ah*/ = /*ai*/ ROW /*aj*/ (/*ak*/ 1 /*al*/, /*am*/ DEFAULT /*an*/) /*ao*/ FROM /*ap*/ bar /*aq*/ WHERE /*ar*/ f.id = bar.id /*as*/ RETURNING /*at*/ f.id /*au*/, /*av*/ f.payload /*aw*/;
