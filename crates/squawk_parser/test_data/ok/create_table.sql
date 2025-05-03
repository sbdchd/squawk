-- create_table
-- simple
create table t (
  a text,
  b integer
);

-- with schema prefix
create table foo.t (
  a text,
  b integer
);

-- like source_table
create table t (
  a text,
  like large_data_table,
  b integer
);

-- like option
create table t (
  a text,
  like bar.b including comments
);

-- like options
create table t (
  a text,
  like bar.b including comments including constraints excluding defaults excluding generated excluding identity excluding indexes excluding statistics including storage excluding all
);

-- with prefix options
create global temporary table if not exists t (
  a text
);
create local temp table t (
  a text
);

-- unlogged
create unlogged table t (
  a text
);

-- inherits from parent
create table t (a int)
inherits (foo.bar, bar, buzz);

-- partition range
create table t (a int)
partition by range (
  foo collate "fr_FR" text_pattern_ops, 
  bar,
  extract(month from b),
  (a || b) collate buzz
);

-- partition list
create table t (b int)
partition by hash (z);

-- partition hash
create table t (b int)
partition by list (z, b);

-- using
create table t (a int)
using bar;

-- with
create table t (a int)
with (
  foo,
  bar = 1,
  buzz.bar = false
);

-- without
create table t (a int)
without OIDS;

-- on commit
create table t (a int)
on commit preserve rows;

create table t (a int)
on commit delete rows;

create table t (a int)
on commit drop;

-- tablespace
create table t (a int)
tablespace bar;


-- like_column
create table u (
  a text, 
  "like" a, 
  like t
);

-- create_table_column_constraints
-- not null
create table t (
  b int not null
);

-- null
create table t (
  b int null
);

-- default
create table t (
  b int default 100
);

-- unique constraint
create table t (
  a int unique,
  b int unique nulls not distinct,
  c int unique with (
    fillfactor,
    toast_tuple_target = 100,
    parallel_workers = 5,
    autovacuum_enabled = false,
    autovacuum_vacuum_cost_delay = 10.1
    -- others omitted
  ),
  d int unique using index tablespace foo
);

-- references actions order
create table t (
  b int references foo
    on update no action
    on delete no action
);

-- primary key
create table t (
  a int primary key with ( autovacuum_enabled ),
  a int primary key
);

-- check constraint
create table t (
  b int check (b > 10),
  c int check (c > 10) no inherit
);

-- defer
create table t (
  a int check (a > 10) deferrable,
  b int not null not deferrable,
  c int not null initially deferred,
  d int not null initially immediate
);

-- constraint
create table t (
  b int constraint foo null
);

-- generated stored
create unlogged table t (
  a int,
  b int generated always as (
    a * 2
  ) stored
);

-- create_table_table_constraints

-- named constraint
create table t (
  a int,
  b text,
  constraint foo check (a > b) no inherit
);

-- check constraint
create table t (
  a int,
  b text,
  check (a > b)
);

-- unique constraint
create table t (
  a int,
  b text,
  unique nulls not distinct (a, b) with ( bar = false )
);

-- unique with name constraint
create table t (
  a int,
  b text,
  unique (a) deferrable initially deferred
);

-- primary key constraint
create table t (
  a int,
  b text,
  primary key (a, b) using index tablespace bar
);

-- exclude constraint
create table t (
  a int,
  b text,
  exclude using btree ( a with f.buzz.> ) where ( a > 10 )
);

-- exclude constraint multiple exclusions
create table t (
  a int,
  b text,
  exclude using btree ( a with buzz.>, b with < ) 
    where ( a > 10 and b like '%foo' )
);

-- exclude constraint all clauses exclusions
create table t (
  a int,
  b text,
  exclude using btree ( a with > ) 
    include (a, b)
    with (x = 10, z)
    using index tablespace foo
    where ( a > 10 and b like '%foo' )
    deferrable
);

-- foreign key constraint
create table t (
  a int,
  b text,
  foreign key ( a, b ) references bizz.bar ( a, b )
);

-- everything
create table t (
  a int,
  b text,
  constraint foo check (a > b) no inherit,
  check (a > b),
  unique nulls not distinct (a, b) with ( bar = false ),
  primary key (a, b) using index tablespace bar,
  exclude using btree ( a with buzz.> ) where ( a > 10 ),
  foreign key ( a, b ) references bizz.bar ( a, b ),
  unique (a) deferrable initially deferred,
  check (i * 2 >= o) not deferrable initially immediate
);


-- multi_dimensional_arrays
CREATE TABLE sal_emp (
    name            text,
    pay_by_quarter  integer[],
    schedule        text[][]
);

-- array_with_params
CREATE TABLE tictactoe (
    squares   integer[3][3]
);

-- array_alt_syntax
CREATE TABLE tictactoe (
    pay_by_quarter  integer ARRAY[4],
    pay_by_quarter  integer ARRAY
);

-- create_table_of_type
-- simple
create table t of bar.foo;

-- with column defs
create table t of bar.foo (
  a,
  b not null,
  c with options not null,
  check (a > b)
);

-- create_table_partition_of
-- default
create table t
partition of foo.bar default;

-- partition with
create table t
partition of foo.bar 
for values with (modulus 1, remainder 1);

-- partition in
create table t
partition of foo.bar 
for values in ('bar', 'buzz');

-- partition from to
create table t
partition of foo.bar
for values from ('bar') to ('buzz');

-- missing entries
create table t ();

-- regression
CREATE TABLE sensors_uncompressed (
  sensor_id INTEGER, 
  ts TIMESTAMPTZ NOT NULL, 
  value REAL
);
