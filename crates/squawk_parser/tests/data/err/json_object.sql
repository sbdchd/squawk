-- avoid parser getting stuck
select json_object(]);
select json_object(;);
select json_object(1, ]);
select json_object('foo': 'bar' format json encoding);
