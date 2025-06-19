-- execute
create temp table t on commit drop as
  execute f(a);

-- select
create table t as
  select * from u where c >= b;

-- table
create table t as
  table u;

-- more fields
create local temporary table 
  t (a, b, c)
  using foo
  with (x = 1, b)
  on commit delete rows
  tablespace bar
  as select 1
  with data;

create global temp table 
  if not exists 
  u (a)
  using foo
  without oids
  on commit preserve rows
  tablespace foo
  as select 2
  with no data;

-- unlogged
create unlogged table u
  as select 2;

-- values
create temporary table t as
  values (1);

create temporary table t as
  values (1, 3), (a, 5);

