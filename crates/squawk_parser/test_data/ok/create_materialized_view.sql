-- simple
create materialized view t
  as select 1;

-- full
create materialized view if not exists foo.bar
  (a, b, c)
  using u
  with (x = 10, bar, buzz = true)
  tablespace t
  as select 1, 2, 3
  with no data;

-- table
create materialized view t
  as table u;

-- values
create materialized view t
  as values (1), (2, 2);

