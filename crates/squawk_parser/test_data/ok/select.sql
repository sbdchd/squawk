-- parens_and_unions
(select 1);

((select 2));

select (((select 1)) + 3);

SELECT NULL UNION SELECT NULL UNION SELECT 1;

(SELECT NULL UNION SELECT NULL) UNION SELECT 1;

-- alias_clause
-- simple
select * from (select 1, 2, 3) as t(a, b, c);
-- from alias
select foo from bar as t;

-- non-ident column and table names
select * from (select 1, 2, 3) as target(target, function);
-- from column aliases
select col1 from bar as t(col1, col2);

-- w/o as
select * from (select 1, 2, 3) target(target, function);
-- from alias shorthand
select t.* from bar t;

-- date_funcs
-- current_date
select current_date;

-- current_time
select current_time;
select current_time(10);

-- current_timestamp
select current_timestamp;
select current_timestamp(5);

-- localtime
select localtime;
select localtime(5);

-- localtimestamp
select localtimestamp;
select localtimestamp(5);

-- array_select
-- array with subquery
select array(select oid from pg_proc where proname like 'bytea%');

-- positional_param
select $1;

select * from t where c = $1;

-- any_and_all
-- all
select * from t where c = all(c);

-- any
select * from t where c = any(c);

select * from t where c = any(array['a', 'b', 'c']);

select 1 = any(select 1);

select 1 = any(with t as (select 1) select * from t);

select 1 = any(values (1), (3));

-- some
select 1 > some(select 1);

-- all
select 1 = all(select 1);

-- all with array
select * from t
where not (tag = all(array['a', 'b', 'c']));

-- some, an alias for any
select * from t where c = some(c);

-- any_values
select a = any(values (1, 2), (b, 3), (c, d));

-- field_exprs
select mytable.mycolumn;
select $1.somecolumn;
select (rowfunction(a,b)).col3;

select (compositecol).somefield;
select (mytable.compositecol).somefield;

select (compositecol).*;

-- case_expr
-- simple
select case when $1 then 1 when $2 then 2 end from t;

-- generic conditional case
select a,
  case when a = 1 then 'one'
    when a = 2 then 'two'
    else 'other'
  end
from test;

-- ommitted else
select a,
  case when a = 1 then 'one'
    when a = 2 then 'two'
  end
from test;

-- switch style case
select a,
  case a when 1 then 'one'
    when 2 then 'two'
    else 'other'
  end
from test;

-- col_labels
select foo as bar from t;

select foo bar from t;

select foo bar, b, x y from t;

-- select
-- ident
select foo;

-- all
select all;

-- multi columns
select 1, 2;

-- select_with_quoted_ident
select t."b.c" from t;

-- select_with_from_clause
-- from tablename
select foo from bar;

-- from only
select a from only t;

-- from field expr
select a from foo.bar;

-- from with everything
select * from t * as r(a, b, c) tablesample sample_method (10, 23, 50) repeatable (42);

-- from select stmt
select * from (select 1);
select * from lateral (select 1) as t(a, b, c);

-- function call complex
select a, b from foo(bar, buzz) with ordinality as t(a, b);
select a, b from foo(bar, buzz) with ordinality t(a, b);

-- function with alias
select * from foo(bar, buzz) as t;
select * from foo(bar, buzz) t;
select * from foo(bar, buzz) as t(a, b);
select * from foo(bar, buzz) t(a, b);

-- function with column def
select * from json_to_record('{"a": 1, "b": "c"}') t(a int, b text);
select * from f() as t(a int, b text);
select * from f() as (a int, b text);

-- function with collate
select * from f() as (a int, b text collate foo.bar.buzz);
select * from f() as (a int, b text collate "bar");

-- multiple tables
select * from a, b;
select * from bar x, buzz y;
select * from foo as f, boo as b;

-- from_item

-- table_name from_item
select * from only t as z(a, b, c);
select * from t * as z(a, b, c);
select * from t z(a, b, c);
select * from t z;
select * from foo.t z;
select * from t;

-- with_query_name
select * from t;
select * from t as b(x, y, z);
select * from t as b;
select * from t b;

-- lateral select
select * from lateral (select 1) as t(a, b, c);
select * from (select 1) as t(a, b, c);
select * from (select 1) t;
select * from (select 1);

-- lateral function_name

select * from lateral f();
select * from f() t;
select * from f() with ordinality as t(a, b, c);
select * from f() as t(a, b, c);
select * from f() t(a, b, c);
select * from lateral f(a, b);
select * from f() as t(a int, b text);
select * from f() t(a int, b text);


-- lateral rows from(
select * from lateral rows from(f(a, b)) as (x int, y text, z int8);
select * from rows from(f(a, b));
select * from rows from(f());
select * from rows from(f()) with ordinality;
select * from rows from(f()) with ordinality as t;
select * from rows from(f()) as t(a, b, c);
select * from rows from(f()) t(a, b, c);

-- select_with_where_clause
-- simple
select 1 where 1;

-- select_with_limit_clause
-- simple
select 1 limit 1;

-- select_with_orderby_clause
-- simple
select 1 order by 1;


-- nulls
select 1 order by 1 nulls first;
select 1 order by 1 nulls last;


select 1 order by 1 using > nulls last;

-- select_window_clause
-- simple
select 1 window w as ();

-- with window def order by
select 1 window w as (order by 1);

-- with window def frame_start, frame_end
select 1 window w as (range 1 preceding);
select 1 window w as (rows 1 preceding);
select 1 window w as (groups 1 preceding);

-- with window def frame_exclusion
select 1 window w as (rows 1 preceding exclude current row);
select 1 window w as (rows 1 preceding exclude group);
select 1 window w as (rows 1 preceding exclude ties);
select 1 window w as (rows 1 preceding exclude no others);


-- select_having_clause
-- simple
select 1 having true;


-- select_with_group_by_clause
-- parens
select 1 group by ();

-- simple expr
select 1 group by 1;

-- multi expr
select 1, 2 group by 1, 2;

-- all
select 1 group by all 1;

-- distinct
select 1 group by distinct 1;

-- rollup
select 1 group by rollup (1, 2);

-- rollup multi
select 1 group by rollup (1, 2), rollup (3);

-- cube
select 1 group by distinct cube (1, 2, 3);

-- grouping sets
select 1 group by grouping sets (
  (1, 2, 3),
  (1, 2),
  (1),
  ()
);

-- select_with_offset_clause
-- simple
select 1 offset 1;

-- rows
select 1 offset 1 row;
select 1 offset 1 rows;


-- select_with_fetch_clause
-- first
select 1 fetch first 3 rows only;

-- next
select 1 order by 1 fetch next 1 row with ties;

-- simple_expr
-- simple nested select
select (select 1) + (select 2);

select -(select 1);

-- select_with_locking_clause
-- nowait
select 1 for share of a, b, c;

-- multiple clauses
select 1 for update nowait for update skip locked;

-- select from
select * from t where x not in (1, 2, 3);

-- composite_types
select row('fuzzy dice', 42, 1.99);
select ('fuzzy dice', 42, 1.99);

-- join
-- simple join
select * from t join t2 as tb on tb.id = t.id;

-- left
select * from t left join t2 using (id);
select * from t left join t2 using (id, foo);

-- right
select * from t right join t2 using (id);

-- full
select * from t full join t2 using (id);

-- multi conditions
select * from t join t2 on t2.team_id = t.team_id and t2.id = t.org_id;

-- using w/ join alias
SELECT * from t join t2 using (id) as foo;

-- cross join
SELECT c.color_name, s.size_name FROM colors c CROSS JOIN sizes s;

-- inner join on true
select f.fruit_name, c.color_name from fruits f inner join colors c on (true);

-- multiple join clauses
select 1 from t
left join u using (id)
left join k using (event);

-- natural
SELECT * FROM employees NATURAL JOIN departments;

-- pg docs

-- using
SELECT f.title, f.did, d.name, f.date_prod, f.kind
    FROM distributors d JOIN films f USING (did);

select * from t join t2 using (a_id);

-- lateral
SELECT m.name AS mname, pname
FROM manufacturers m LEFT JOIN LATERAL get_product_names(m.id) pname ON true;

-- table
-- simple
table t;

-- only
table only t;

-- star
table t *;

-- nested
select (table t);

-- union
-- simple
select 1 union select 2;

-- all
select 1 union all select 2;

-- distinct
select 1 union distinct select 2;

-- multi
select 1 union select 2 union select 3;


-- intersect
-- simple
select 1 intersect select 2;

-- all
select 1 intersect all select 2;

-- distinct
select 1 intersect distinct select 2;


-- except
-- simple
select 1 except select 2;

-- all
select 1 except all select 2;

-- distinct
select 1 except distinct select 2;

-- ident_edge_cases
-- select keywords
select 1 as select;
select 1 as from;
select 1 as where;

select 1 all;

-- column labels that are also operators
select 1 is;

select 1 and;

select 1 or;

select 1 collate;

select foo.bar null;
select foo.bar default;

select default;
select c default;
select null;
select c null;

-- select_special_funcs
-- collation
select collation for ( b + c );

-- current_role
select current_role;

-- current_user
select current_user;

-- session_user
select session_user;

-- system_user
select system_user;

-- user
select user;

-- current_catalog
select current_catalog;

-- current_schema
select current_schema;

-- order_by_with_custom_op
select * from t order by a using >>>;

-- order_by_regression
SELECT sensor_id, DATE_TRUNC('day', ts) AS day, MAX(value) AS max_value, MIN(value) AS min_value 
FROM sensors_uncompressed 
WHERE ts >= DATE '2023-12-21' AND ts < DATE '2023-12-22'
GROUP BY sensor_id, DATE_TRUNC('day', ts) 
ORDER BY sensor_id, day;

-- select_from_user_table
select * from user;
