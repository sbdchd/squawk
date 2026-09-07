-- avoid parser getting stuck
select json_object(]);
select json_object(;);
select json_object(1, ]);
select json_object('foo': 'bar' format json encoding);

-- JSON array value lists cannot end with a comma
select json_array('' format json, 2, returning j);
select json_array(3,);
select json_array(1, '' format json,);
select json_array(select 1 format json,);

-- malformed JSON_OBJECTAGG options
select json_objectagg(k::v with unique);

-- JSON PASSING lists cannot end with a comma
select json_value(x, '$.a' passing y as z,);
select json_query(x, '$.a' passing y as z, returning json);
select json_exists(x, '$.a' passing y as z,);
select * from json_table(x, '$.a' passing y as z, columns (a int));
