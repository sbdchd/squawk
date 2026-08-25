delete from foo;

DELETE FROM foo AS f USING bar b, another_extremely_long_table_name_with_many_characters baz WHERE f.id = b.id RETURNING f.id, f.name, f.created_at;

DELETE FROM foo * f WHERE CURRENT OF delete_cursor RETURNING WITH (OLD AS the_extremely_long_previous_row_value, NEW AS the_extremely_long_updated_row_value) the_extremely_long_previous_row_value.*;

DELETE FROM ONLY (foo) f;

DELETE FROM foo FOR PORTION OF valid_at FROM 1 TO 2;

DELETE FROM foo FOR PORTION OF valid_at FROM 1 TO 2 WHERE organization_id = 12345 AND status = 'inactive' AND archived_at IS NOT NULL RETURNING id, valid_at;

DELETE FROM foo FOR PORTION OF valid_at (1 + 2);

WITH doomed AS (SELECT id FROM foo) DELETE FROM foo USING doomed WHERE foo.id = doomed.id;

WITH deleted AS (DELETE FROM foo WHERE id = 1 RETURNING id) SELECT * FROM deleted;

/*before*/ DELETE /*a*/ FROM /*b*/ foo /*c*/ FOR /*d*/ PORTION /*e*/ OF /*f*/ valid_at /*g*/ FROM /*h*/ 1 /*i*/ TO /*j*/ 2 /*k*/ AS /*l*/ f /*m*/ USING /*n*/ bar /*o*/ b /*p*/, /*q*/ baz /*r*/ WHERE /*s*/ f.id = b.id /*t*/ RETURNING /*u*/ WITH /*v*/ (/*w*/ OLD /*x*/ AS /*y*/ o /*z*/, /*aa*/ NEW /*ab*/ AS /*ac*/ n /*ad*/) /*ae*/ o.id /*af*/, /*ag*/ n.id /*ah*/;
