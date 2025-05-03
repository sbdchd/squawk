-- with missing table name
create table (
  sensor_id INTEGER
);

-- missing type
create table t (a);

-- trailing comma
create table t (a text,);

-- missing columns / constraints
create table t (,,,,,);

-- a column list with SET DEFAULT is only supported for ON DELETE actions
create table t (
  a int,
  b int references bar on update cascade,
  c int references bar on update set null,
  d int references bar on update set default,
  e int references bar on update set default (a, b, c)
);

-- conflicting options
create unlogged table t (
  a int generated always as identity,
  b int generated always as identity (
    as bigint
    cache 100
    increment by 10
    increment 10
    sequence name foo
    restart with 500
    logged
    unlogged
    start with 10
    start 25
    owned by none
    owned by fooo.bar
    maxvalue 70
    minvalue 150
    no minvalue
    no cycle
    no maxvalue
    cycle
  )
);
