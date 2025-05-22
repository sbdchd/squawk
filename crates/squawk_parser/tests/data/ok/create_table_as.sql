-- execute
create temp table t on commit drop as
  execute f(a);

-- select
create table t as
  select * from u where c >= b;

-- table
create table t as
  table u;

-- values
create temporary table t as
  values (1);

create temporary table t as
  values (1, 3), (a, 5);

