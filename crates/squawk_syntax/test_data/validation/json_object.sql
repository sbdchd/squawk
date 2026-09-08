-- `key: value` pairs can't be followed by args
select json_object('k': 'v', a => 'x');
select json_object('k' value 'v', a := 'x');
select json_object('k': 'v', 'a', 'b');

-- and args can't be followed by `key: value` pairs
select json_object('a', 'b', 'k': 'v');
select json_object(a => 'x', 'k': 'v');
