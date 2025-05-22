-- normalize
select normalize('a' + 'b');
select normalize('a' || 'b', nfc);
select normalize('c', nfd);
select normalize('d', nfkc);
select normalize('e', nfkd);

-- nullif
select nullif(a, b);

-- coalesce
select coalesce(a, 1);
select coalesce(a, b, c, d, 'foo');

-- greater
select greater(a, 1);
select greater(a, b, c, d, 0);

-- least
select least(a, 1);
select least(a, b, c, d, 0);

-- merge_action
select merge_action();



-- overlay
select overlay(a placing b from c for d);
select overlay(a placing b from c);
select overlay();
select overlay(1, 2, 3, a => 4);

-- position
select position('foo' in 'bar');
select position('f' || 'b' in 'fb' || ' ');
select position('c' in a);

-- trim
select trim(both a from a);
select trim(both from a);
select trim(both a);
select trim(leading a);
select trim(trailing a);
select trim(a);

-- substring_fn
select substring(a from b for c);
select substring(a for b from c);
select substring(a from b);
select substring(a for b);
select substring(a similar b escape c);

select substring(a || '', b * 2, c - 1);
select substring('hello world', 1, 5);

-- xmlconcat
select xmlconcat(a, b);

-- xmlelement
select xmlelement(name b);
select xmlelement(name c, xmlattributes(foo, bar as b));
select xmlelement(name d, e, f, g);
select xmlelement(name d, xmlattributes(buzz, foo as c), f, g);

-- xmlexists
select xmlexists('foo' passing 'bar');
select xmlexists('foo' passing bar by ref);
select xmlexists('foo' passing by ref bar);
select xmlexists('foo' passing by ref foo by value);

-- xmlforest
select xmlforest(foo, bar, buzz);

-- xmlparse
select xmlparse(document x preserve whitespace);
select xmlparse(content x strip whitespace);

-- xmlpi
select xmlpi(name foo);
select xmlpi(name foo, bar);

-- xml_root
select xmlroot(a, version foo);
select xmlroot(a, version no value);
select xmlroot(a, version foo, standalone yes);
select xmlroot(a, version foo, standalone no);
select xmlroot(a, version foo, standalone no value);

-- xml_serialize
select xmlserialize(document x as foo indent);
select xmlserialize(document x as foo no indent);

-- json_object
select json_object();
select json_object('{a, 1, b, "def", c, 3.5}');
select json_object('{foo}', '{bar}');
select json_object(array['a', 'b'], array['foo', 'bar']);
select json_object('foo' value 'bar', 'a' value 'b');
select json_object('foo': 'bar', 'a': 'b');

select json_object('foo': 'bar' format json encoding utf8);
select json_object('foo': 'bar' format json);

select json_object('foo': 'bar' null on null);
select json_object('foo': 'bar' absent on null);
select json_object('a': 1, 'b': '1' format json, 'c' value 'c');

select json_object('foo': 'bar' null on null with unique keys);
select json_object('foo': 'bar' null on null with unique);
select json_object('foo': 'bar' null on null without unique keys);
select json_object('foo': 'bar' null on null without unique);

select json_object('foo': 'bar' returning foo);
select json_object('foo': 'bar' returning foo format json);

-- json_array
-- value_expression
select json_array(a);
select json_array(1, 2, 3, 4);
select json_array(((1)));
select json_array(a format json encoding utf8);
select json_array(a format json);
select json_array(a format json null on null);
select json_array(1 returning text format json);
select json_array(a absent on null);
select json_array(a absent on null returning foo format json);
select json_array(1, true, json '{"a":null}');
select json_array(1, b, '3' format json, 4);

-- query_expression
select json_array(select 1);
select json_array(select 1 format json);
select json_array(select 'true' format json returning text format json);
select json_array(select 'true' returning text format json);
select json_array(((select 1)));
select json_array(returning text);
select json_array(returning text format json);

-- json_scalar
select json_scalar(1);

-- exists
select exists(select 1 from t where a = b);

select exists(with t as (select 1) select * from t);

-- exists_values
select exists(values (1));

-- exists_where
select * from t where exists(select null);

-- not_exists
select not exists(select 1);

-- function_call
-- position notation
select buzz('x', 'y');

-- named notation
select f(a => 'foo', b => 'bar');

-- mixed notation
select foo('bar', 'buzz', boo => true);

-- colon equals
select f('b', a := true);

-- extract
select extract(foo from bar || 'buzz');
select extract(year from a);
select extract(month from a);
select extract(day from a);
select extract(hour from a);
select extract(minute from a);
select extract(second from a);
select extract('minute' from a);
select extract(epoch from timestamptz '2013-07-01 12:00:00');
select extract(century from timestamp '2000-12-16 12:21:13');
select extract(isodow from timestamp '2001-02-18 20:38:40');
select extract(isoyear from date '2006-01-01');
select extract(julian from date '2006-01-01');
select extract(microseconds from time '17:12:28.5');
select extract(millennium from timestamp '2001-02-16 20:38:40');
select extract(quarter from timestamp '2001-02-16 20:38:40');

select transaction_timestamp();
select statement_timestamp();
select clock_timestamp();
select timeofday();
select now();

select pg_sleep(1.5);
select pg_sleep_for('5 minutes');
select pg_sleep_until('tomorrow 03:00');

-- aggs
-- order by one param
select array_agg(v order by v desc) from vals;

-- order by param 2
select jsonb_object_agg(k, v order by v) from vals;

-- order by param 2 const
select string_agg(a, ',' order by a) from t;

-- order by param 1
select string_agg(a order by a, ',') from "table";

-- within group
select foo(0.5) within group (order by c) from t;

-- filter expr simple
select count(*) filter (where i < 5) from t;

-- filter expr
select
  count(*) as unfiltered,
  count(*) filter (where i < 5) as filtered
from generate_series(1, 10) as s(i);

select f(all c);

select f(distinct b);

-- variadic_func_calls
-- simple
select f(variadic foo);

select f(variadic array[1, 2, 3]);

-- array
select b(variadic array[]::numeric[]);

-- named
select c(variadic arr => array[1, 2, 3]);

-- last param
select f(a, b, c, variadic array[1, 2, 3]);


-- window_func_calls
-- over partition
select max(a) over (partition by b) as c from t;

-- window name
select max(a) over w_name from t;
