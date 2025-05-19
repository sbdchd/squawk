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

-- trailing comma in args
select f(1,);

-- missing args
select f(a,,,,,);

-- in can only be used with tuples / sub queries
select 1 in c;

-- type cast must use a string literal
select numeric 1234;

-- trailing comma at EOF
select 1,
