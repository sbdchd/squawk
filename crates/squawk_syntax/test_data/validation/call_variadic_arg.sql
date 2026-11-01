-- ok
select concat(variadic array['a']);
select concat('a', variadic array['b']);
select f(a => 1, variadic b => array[2]);
select string_agg(variadic array['a'] order by 1);

-- err
select f(variadic x, b);
select concat(variadic array['a'], 'b');
select f(variadic a => array[1], b => 2);
select f(variadic x, variadic y, z);
