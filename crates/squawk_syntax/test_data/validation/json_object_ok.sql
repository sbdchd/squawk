select json_object();
select json_object(returning jsonb);
select json_object('a', 'b');
select json_object(a => 'x', b => 'y');
select json_object('k': 'v', 'a': 'b');
select json_object('k' value 'v', 'a' value 'b' absent on null with unique keys returning text);
