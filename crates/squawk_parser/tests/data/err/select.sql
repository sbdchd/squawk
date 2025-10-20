--- via https://duckdb.org/docs/stable/sql/dialect/friendly_sql.html#trailing-commas
select
  42 as x,
  array['a', 'b', 'c',] as y,
  'hello world' as z,
;

-- trailing comma in column list
select * from t as u(a,);

-- missing comma
select a, b c  d, e from t;
--          ^ ^ comma missing
--          \-- this is a label

-- distinct on missing a comma
SELECT DISTINCT ON (a b) a, b, c
    FROM t
    order by a, b desc; 

-- group bys with missing commas
select * from t group by rollup (1 2 3);
select * from t group by cube (1 2 3);
select * from u
  group by grouping sets((1 2) grouping sets((), grouping sets(())));

-- trailing comma in args
select f(1,);

-- missing args
select f(a,,,,,);

-- in can only be used with tuples / sub queries
select 1 in c;

-- type cast must use a string literal
select numeric 1234;

-- missing comma
select array[1 2,3];
-- extra comma
select array[1, ,3];
-- trailing comma
select array[1,2,3,];

-- cast with malformed type mod args
select cast(x as varchar(100 200));
select cast(x as varchar(100, , 200));
select cast(x as t(a, b,));

-- regression test: this would cause the parser to get stuck & panic, now it
-- warns about a missing semicolon
select select;

-- extra comma
select a, from t;

-- trailing comma at EOF
select 1,
